use aho_corasick::AhoCorasick;
use pathdiff::diff_paths;
use walkdir::WalkDir;

use crate::tools::{get_latest_line, print_symbol, check_control, unix_time_diff};
use crate::parsers::get_parser;
use crate::info::{get_line_info, get_commit_diff};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs::{read_to_string};
use std::time::{SystemTime, UNIX_EPOCH};
use git2::{Repository, Oid};
use crate::parsers::types::SymbolSpan;
use pickledb::PickleDb;
use serde::{Serialize, Deserialize};
use crate::formatters::get_formatter;

/// All the flags resync saves for an out of sync comment
#[derive(Serialize, Deserialize)]
pub struct SyncInfo {
    time_diff: String,
    commit_diff: usize,
    function: SymbolSpan,
    comment: SymbolSpan
}

pub struct Checker {
    repo: Repository,
    working_dir: PathBuf,
    ac: AhoCorasick,
    porcelain: bool,
    db: PickleDb
}

impl Checker {
    pub fn new(repo: Repository, working_dir: PathBuf, ac: AhoCorasick, porcelain: bool, db: PickleDb) -> Self {
        Self { repo, working_dir, ac, porcelain, db }
    }

    fn should_check(&self, file: &PathBuf) -> bool {
        let last_edit = std::fs::metadata(&file)
            .unwrap()
            .modified()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap().as_millis();
        
        let file_name = file.file_name().and_then(OsStr::to_str).unwrap();
        let last_checked = match self.db.get::<u128>(format!("{}:time", &file_name).as_str()) {
            Some(time) => time,
            None => 0
        };

        if last_edit > last_checked {
            return true;
        }

        return false;
    }

    /// returns sync info
    pub fn check_file(&mut self, file: PathBuf) {
        // failed to get last edit, just continue

        let patterns = [".git", ".swp", "node_modules", "target"]; // TODO: add global pattern list, or read gitignore
        // let f = file.path().to_str().unwrap();
        if self.ac.is_match(file.to_str().unwrap()) {
            return;
        }

        // check if file is directory
        let ext = match file.extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => {
                return;
            }
        };


        let formatter = get_formatter(&self.porcelain);

        let file_name = file.file_name().and_then(OsStr::to_str).unwrap();

        let relative_path = diff_paths(&file, self.working_dir.as_path()).unwrap();

        if !self.should_check(&file) {
            let symbol = match self.db.get::<SyncInfo>(format!("{}:info", file_name).as_str()) {
                Some(symbol) => symbol,
                None => {
                    return;
                }
            };

            formatter.output(&symbol.function, &symbol.comment, &relative_path, &ext, &symbol.time_diff, &symbol.commit_diff);
            // ok

            // println!("{}\n{}\n{}\n{}\n{}\n{}", symbol.time_diff, symbol.commit_diff, symbol.relative_path.display(), file_name, comment.start.line, comment.end.line);
            // time diff, commit_diff, relative path, file name, comment start, comment end
        }


        // if there is already something, get the file and print it
        // else, continue

        let parser = match get_parser(&file, &patterns) {
            Some(parser) => parser,
            _ => {
                return;
            }
        };

        let read = match read_to_string(&file) {
            Ok(read) => read,
            Err(e) => {
                println!("{:#?}", file);
                println!("{}", e);
                println!("Failed to read file {}, skipping", file.display());
                return;
            }
        };


        // let repo = Repository::open(working_dir).expect("Failed to open repo");

        let blame_lines = match get_line_info(&self.repo, &relative_path) {
            Ok(lines) => lines,
            Err(e) => {

                if self.porcelain != true {
                    println!("{}", e);
                    println!("Failed checking {}, continuing", file.display());
                }
                return;
            }
        };

        let all_funs = match parser.parse(&read) {
            Ok(funs) => funs,
            Err(e) => {
                println!("Failed to parse file. Error {}. Skipping", e);
                return;
            }
        };

        // make a module which checks all of these, checkall, which you can implement
        for (comment, function) in all_funs {
            let comment_idx = get_latest_line(&blame_lines, &comment);
            let fun_idx = get_latest_line(&blame_lines, &function);

            let comment_info = blame_lines.get(&comment_idx).expect("Failed to get comment from blame lines");
            let function_info = blame_lines.get(&fun_idx).expect("Failed to get function from blame lines");

            // if the comment has been edited before, or at the same time as the function has
            if function_info.time <= comment_info.time {
                continue;
            }

            // helps show less false positives by only showing functions which have a lot of different commits
            if check_control(&blame_lines, &function) {
                continue;
            }

            let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
            let time_diff = unix_time_diff(current_time.into(), comment_info.time.into());
            let commit_diff = get_commit_diff(&self.repo, &Oid::from_str(&comment_info.commit).unwrap()).expect("Failed to get commit diff");

            formatter.output(&function, &comment, &file, &ext, &time_diff, &commit_diff);
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            let symbol = SyncInfo {
                function,
                comment,
                commit_diff,
                time_diff
            };
            self.db.set::<u128>(format!("{}:time", &file_name).as_str(), &current_time).expect("Failed to set last time");
            self.db.set::<SyncInfo>(format!("{}:info", &file_name).as_str(), &symbol).expect("Failed to set last time");
        }
    }

    pub fn check_dir(&mut self, dir: &Path) {
        for file in WalkDir::new(&self.working_dir).into_iter().filter_map(|e| e.ok()) {
            self.check_file(file.path().to_path_buf());
        }
    }
}
