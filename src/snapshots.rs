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
 *
 * Changes
 *
 * - Changed occurences of "dura" to "resync"
 * - Changed "capture" function name to "sync"
 * - Removed function "is_repo"
*/

use git2::{BranchType, DiffOptions, Error, IndexAddOption, Repository, Signature};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct CaptureStatus {
    pub dura_branch: String,
    pub commit_hash: String,
    pub base_hash: String,
}

impl fmt::Display for CaptureStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "resync: {}, commit_hash: {}, base: {}",
            self.dura_branch, self.commit_hash, self.base_hash
        )
    }
}

pub fn sync(path: &Path) -> Result<Option<CaptureStatus>, Error> {
    let repo = Repository::open(path)?;
    let head = repo.head()?.peel_to_commit()?;
    let message = "resync auto-sync";

    // status check
    if repo.statuses(None)?.is_empty() {
        return Ok(None);
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
        return Ok(None);
    }

    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;
    if repo.find_branch(&branch_name, BranchType::Local).is_err() {
        repo.branch(branch_name.as_str(), &head, false)?;
    }

    let committer = Signature::now(&get_git_author(&repo), &get_git_email(&repo))?;
    let oid = repo.commit(
        Some(&format!("refs/heads/{}", &branch_name)),
        &committer,
        &committer,
        message,
        &tree,
        &[parent_commit],
    )?;

    Ok(Some(CaptureStatus {
        dura_branch: branch_name,
        commit_hash: oid.to_string(),
        base_hash: head.id().to_string(),
    }))
}

fn get_git_author(repo: &Repository) -> String {
    let dura_cfg = Config::load();
    if let Some(value) = dura_cfg.commit_author {
        return value;
    }

    if !dura_cfg.commit_exclude_git_config {
        if let Ok(git_cfg) = repo.config() {
            if let Ok(value) = git_cfg.get_string("user.name") {
                return value;
            }
        }
    }

    "dura".to_string()
}

fn get_git_email(repo: &Repository) -> String {
    let dura_cfg = Config::load();
    if let Some(value) = dura_cfg.commit_email {
        return value;
    }

    if !dura_cfg.commit_exclude_git_config {
        if let Ok(git_cfg) = repo.config() {
            if let Ok(value) = git_cfg.get_string("user.email") {
                return value;
            }
        }
    }

    "support@readable.so".to_string()
}
