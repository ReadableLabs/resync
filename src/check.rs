use aho_corasick::AhoCorasick;
use pathdiff::diff_paths;
use walkdir::WalkDir;

use crate::tools::{get_latest_line, check_control, unix_time_diff};
use crate::parsers::get_parser;
use crate::info::{get_line_info, get_commit_diff};
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};
use std::fs::File;
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
        
        // let file_name = file.file_name().and_then(OsStr::to_str).unwrap();
        // println!("{}", &file.display());

        let key = format!("{}:time", file.display().to_string());

        let last_checked = match self.db.get::<u128>(&key) {
            Some(time) => {
                time
            },
            None => 0,
        };

        if last_edit > last_checked {
            return true;
        }

        return false;
    }

    /// returns sync info
    pub fn check_file(&mut self, file: PathBuf, ignore_files: &mut Vec<String>) {
        let mut patterns: Vec<String> = vec![".git".to_string(), ".swp".to_string(), "node_modules".to_string(), "target".to_string()]; // TODO: add global pattern list, or read gitignore
        patterns.extend(ignore_files.iter().cloned());

        if self.ac.is_match(file.to_str().unwrap()) {
            return;
        }

        let ext = match file.extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => {
                return;
            }
        };


        let formatter = get_formatter(&self.porcelain);

        let relative_path = diff_paths(&file, self.working_dir.as_path()).unwrap();

        let parser = match get_parser(&file, &patterns) {
            Some(parser) => parser,
            _ => {
                return;
            }
        };


        if !self.should_check(&file) {
            let symbols = match self.db.get::<Vec<SyncInfo>>(&format!("{}:info", &file.display())) {
                Some(symbol) => symbol,
                None => {
                    return;
                }
            };

            for symbol in symbols {
                formatter.output(&symbol.function, &symbol.comment, &file, &ext, &symbol.time_diff, &symbol.commit_diff);
            }

            return;
        }

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

        let all_funs = match parser.parse(&file) {
            Ok(funs) => funs,
            Err(_) => {
                if self.porcelain == false {
                    println!("Failed to parse file, Skipping");
                }
                return;
            }
        };

        let mut all_symbols = Vec::<SyncInfo>::new();

        // make a module which checks all of these, checkall, which you can implement
        for (comment, function) in all_funs {
            // TODO: make these return a result and skip if fail
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

            let symbol = SyncInfo {
                function,
                comment,
                commit_diff,
                time_diff
            };

            all_symbols.push(symbol);

        }

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        let key = format!("{}:time", file.display().to_string());
        self.db.set::<u128>(&key, &current_time).expect("Failed to set last time");

        if all_symbols.is_empty() {
            return;
        }

        self.db.set::<Vec<SyncInfo>>(&format!("{}:info", &file.display()), &all_symbols).expect("Failed to set last time");

    }

    fn check_gitignore(&self) -> Vec<String> {
        let gitignore = Path::join(&self.working_dir, ".gitignore");

        if !Path::exists(&gitignore) {
            return vec![];
        }

        let file = File::open(gitignore).expect("Failed to open gitignore");

        let buf = BufReader::new(file);

        buf.lines()
        .map(|f| f.expect("Failed to read gitignore"))
        .collect()
    }

    pub fn check_working_dir(&mut self) {

        let mut ignore_files = self.check_gitignore();

        for file in WalkDir::new(&self.working_dir).into_iter().filter_map(|e| e.ok()) {
            self.check_file(file.path().to_path_buf(), &mut ignore_files);
        }
    }
}
