use std::collections::HashMap;
use std::path::Path;
use git2::{Repository, Blame, BlameOptions, Error};

pub fn get_line_info(path: &Path, file: &Path) -> Result<HashMap<i32, i32>, Error> { // TODO: make blame oldest and newest commit be equivalent to head id passed in
    let mut lines: HashMap<i32, i32> = HashMap::new();

    let repo = Repository::open(path)?;
    let mut blame_opts = BlameOptions::new();
    let blame = repo.blame_file(file, Some(&mut blame_opts))?;
    return Ok(lines);
}
