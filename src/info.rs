use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str;
use git2::{Repository, BlameOptions, Error, BranchType, Oid};

pub struct LineInfo {
    pub time: u64,
    pub commit: String,
}

pub fn get_line_info(repo: &Repository, file: &Path) -> Result<HashMap<usize, LineInfo>, Error> { // TODO: make blame oldest and newest commit be equivalent to head id passed in
    let mut lines: HashMap<usize, LineInfo> = HashMap::new();

    // let repo = Repository::open(path)?;
    let head = repo.head()?.peel_to_commit()?;
    // println!("{}", head.id());

    let branch_name = format!("resync/{}", head.id());
    // let branch_name = format!("{}", head.id());

    // let branch_name = format!("resync/{}", head.id()); // use head, not this
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
    // blame_opts.newest_commit(*branch_oid.as_ref().unwrap_or(&head.id()));
    // blame_opts.oldest_commit(head.id()).newest_commit(*branch_oid.as_ref().unwrap_or(&head.id()));

    let blame = repo.blame_file(file, Some(&mut blame_opts))?;
    let object = repo.revparse_single(&spec[..])?;
    let blob = repo.find_blob(object.id())?;

    let reader = BufReader::new(blob.content());
    for (i, _) in reader.lines().enumerate() {
        if let Some(hunk) = blame.get_line(i + 1) {
            // println!("{} - {}", i + 1, hunk.final_signature().when().seconds());
            let commit_id = format!("{}", hunk.final_commit_id());
            // println!("{} - {}", i + 1, commit_id);
            let time = hunk.final_signature().when().seconds();
            lines.insert(
                i.try_into().unwrap(),
                LineInfo {
                    time: time.try_into().unwrap(),
                    commit: commit_id
                }
            );
            // lines.insert(i.try_into().unwrap(), time.try_into().unwrap());
            // could use softmax
        }
    }


    return Ok(lines);
}

/// pass in repo for later
pub fn get_commit_diff(repo: &Repository, new: &str, old: &str) -> Result<usize, Error> {
    // let repo = Repository::open(path)?;

    let mut revwalk = repo.revwalk()?;

    revwalk.set_sorting(git2::Sort::TIME)?;

    revwalk.hide(Oid::from_str(old).unwrap())?;
    revwalk.push(Oid::from_str(new).unwrap())?;
    // let id = repo.revparse_single(old)?.id();
    // revwalk.push(id)?;

    let count = revwalk.count();

    // for id in revwalk {
    //     let id = id?;
    //     println!("{}", id);
    // }

    Ok(count)

}
