use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::Path;
use git2::{Repository, BlameOptions, Error, BranchType, Oid};

pub struct LineInfo {
    pub time: u64,
    pub commit: String,
}

pub fn get_line_info(repo: &Repository, file: &Path) -> Result<HashMap<usize, LineInfo>, Error> {
    let mut lines: HashMap<usize, LineInfo> = HashMap::new();

    let head = repo.head()?.peel_to_commit()?;

    let branch_name = format!("resync/{}", head.id());

    let spec = format!("{}:{}", branch_name, file.display());

    let branch_oid = match repo.find_branch(&branch_name, BranchType::Local) {
        Ok(branch) => {
           match branch.get().peel_to_commit() {
                Ok(commit) => {
                    Some(commit.id())
                },
                _ => None
            }
        },
        Err(_) => None // maybe this isn't good
    };


    let mut blame_opts = BlameOptions::new();

    let blame = repo.blame_file(file, Some(&mut blame_opts))?;
    let object = repo.revparse_single(&spec[..])?;
    let blob = repo.find_blob(object.id())?;

    let reader = BufReader::new(blob.content());
    for (i, _) in reader.lines().enumerate() {
        if let Some(hunk) = blame.get_line(i + 1) {
            let commit_id = format!("{}", hunk.final_commit_id());
            let time = hunk.final_signature().when().seconds();
            lines.insert(
                i.try_into().unwrap(),
                LineInfo {
                    time: time.try_into().unwrap(),
                    commit: commit_id
                }
            );
        }
    }


    return Ok(lines);
}

/// pass in repo for later
pub fn get_commit_diff(repo: &Repository, old: &Oid) -> Result<usize, Error> {
    let master_commit = repo.head()?.peel_to_commit()?;

    let mut revwalk = repo.revwalk()?;

    revwalk.set_sorting(git2::Sort::TIME)?;

    revwalk.hide(*old)?;
    revwalk.push(master_commit.id())?;

    let count = revwalk.count();

    Ok(count)

}
