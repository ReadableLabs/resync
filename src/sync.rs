/*
 * Copyright 2022 Tim Kellogg
 * 
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * 
 *     http://www.apache.org/licenses/LICENSE-2.0
 * 
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * This code was taken from "dura" - a background process that watches your Git repos and commits
 * your uncommited changes without impacting HEAD the current branch, or the Git index.
 *
 * https://github.com/tkellogg/dura
 * Commit: 4948b965e81aa8cbba02737dca41f218b5b22956
 *
 * Changes
 * - Removed output
 * - Changed occurences of "dura" to "resync"
 * - Changed "capture" function name to "sync"
 * - Removed function "is_repo", "get_git_author", and "get_git_email"
 * - Renamed from "snapshot.rs" to "sync.rs"
 * - Changed return type from CaptureStatus to boolean
 * - Removed config using
*/

use git2::{BranchType, DiffOptions, Error, IndexAddOption, Repository, Signature};
use std::path::Path;

pub fn sync(path: &Path) -> Result<String, Error> {
    let repo = Repository::open(path)?;
    let head = repo.head()?.peel_to_commit()?;
    let message = "resync auto-sync";

    // status check
    if repo.statuses(None)?.is_empty() {
        return Ok(head.id().to_string());
    }

    let branch_name = format!("resync/{}", head.id());
    let branch_commit = match repo.find_branch(&branch_name, BranchType::Local) {
        Ok(mut branch) => {
            match branch.get().peel_to_commit() {
                Ok(commit) if commit.id() != head.id() => Some(commit),
                _ => {
                    branch.delete()?;
                    None
                }
            }
        }
        Err(_) => None,
    };
    let parent_commit = branch_commit.as_ref().unwrap_or(&head);

    // tree
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;

    let dirty_diff = repo.diff_tree_to_index(
        Some(&parent_commit.tree()?),
        Some(&index),
        Some(DiffOptions::new().include_untracked(true)),
    )?;
    if dirty_diff.deltas().len() == 0 {
        return Ok(head.id().to_string());
    }

    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;
    if repo.find_branch(&branch_name, BranchType::Local).is_err() {
        repo.branch(branch_name.as_str(), &head, false)?;
    }

    let committer = Signature::now("resync", "support@readable.so")?;
    let oid = repo.commit(
        Some(&format!("refs/heads/{}", &branch_name)),
        &committer,
        &committer,
        message,
        &tree,
        &[parent_commit],
    )?;

    Ok(oid.to_string())
}
