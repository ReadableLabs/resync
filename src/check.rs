use aho_corasick::AhoCorasick;
use pathdiff::diff_paths;

use crate::tools::{get_latest_line, print_symbol, check_control, unix_time_diff};
use crate::parsers::get_parser;
use crate::info::{get_line_info, get_commit_diff};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs::metadata;
use std::fs::{read_to_string};
use std::time::{SystemTime, UNIX_EPOCH};
use git2::{Repository, Oid};
use crate::parsers::types::SymbolSpan;

/// All the flags resync saves for an out of sync comment
pub struct SyncInfo {
    last_edit: u64,
    time_diff: String,
    commit_diff: usize,
    line: usize,
    character: usize,
    function: SymbolSpan,
    comment: SymbolSpan
}

pub struct Checker {
    repo: Repository,
    working_dir: PathBuf,
    ac: AhoCorasick,
    porcelain: bool
}

impl Checker {
    pub fn new(repo: Repository, working_dir: PathBuf, ac: AhoCorasick, porcelain: bool) -> Self {
        Self { repo, working_dir, ac, porcelain }
    }

    pub fn check_file(&self, file: PathBuf) {
        // let last_edit = std::fs::metadata(file)
        //     .unwrap()
        //     .modified()
        //     .unwrap()
        //     .duration_since(UNIX_EPOCH)
        //     .unwrap().as_millis();

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

        let relative_path = diff_paths(&file, self.working_dir.as_path()).unwrap();

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

            let line = function.start.line - 1;
            let character = function.start.character;

            if self.porcelain != true {
                println!("{}", time_diff);
                println!("{} commits since update", commit_diff); // change red green or yellow text changed a lot, little, or changed
                println!("{}:{}:{}", file.display(), line, character);
                print_symbol(&function, &comment, &file, ext);
            }

            else {
                // println!("{}\n{} commits since update\n{}:{}:{}\n{}\n{}", time_diff, commit_diff, file.display(), line, character, comment.start.line, comment.end.line);
                let file_name = file.file_name().and_then(OsStr::to_str).unwrap();
                println!("{}\n{}\n{}\n{}\n{}\n{}", time_diff, commit_diff, relative_path.display(), file_name, comment.start.line, comment.end.line);
            }
        }
    }

    pub fn check_dir(&self, dir: &Path) {
    }
}

pub fn check_file(repo: &Repository, working_dir: &Path, file: &Path, ac: &AhoCorasick, porcelain: &bool) {
}