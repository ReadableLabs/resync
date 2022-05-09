use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::Path;
use git2::{Repository, BlameOptions, Error, BranchType};

pub fn get_line_info(path: &Path, file: &Path) -> Result<HashMap<usize, u64>, Error> { // TODO: make blame oldest and newest commit be equivalent to head id passed in
    let mut lines: HashMap<usize, u64> = HashMap::new();

    let repo = Repository::open(path)?;
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
    blame_opts.oldest_commit(head.id()).newest_commit(*branch_oid.as_ref().unwrap_or(&head.id()));

    let blame = repo.blame_file(file, Some(&mut blame_opts))?;
    let object = repo.revparse_single(&spec[..])?;
    let blob = repo.find_blob(object.id())?;

    let reader = BufReader::new(blob.content());
    for (i, _) in reader.lines().enumerate() {
        if let Some(hunk) = blame.get_line(i + 1) {
            let time = hunk.final_signature().when().seconds();
            lines.insert(i.try_into().unwrap(), time.try_into().unwrap());
            // could use softmax
        }
    }


    return Ok(lines);
}
