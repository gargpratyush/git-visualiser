use anyhow::Result;
use git2::{Repository, BranchType};
use chrono::{Local, TimeZone};
use std::path::Path;
use crate::models::CommitInfo;

pub struct GitManager {
    repo: Repository,
}

// added a comment

impl GitManager {
    pub fn new(path: &Path) -> Result<Self> {
        let repo = Repository::open(path)?;
        Ok(GitManager { repo })
    }

    pub fn branch_exists(&self, branch_name: &str) -> bool {
        self.repo.find_branch(branch_name, BranchType::Local).is_ok()
    }

    pub fn get_commits(&self, branch: &str) -> Result<Vec<CommitInfo>> {
        let branch = self.repo.find_branch(branch, BranchType::Local)?;
        let commit = branch.get().peel_to_commit()?;
        
        let mut commits = Vec::new();
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push(commit.id())?;

        for oid in revwalk {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            
            let author = commit.author();
            let name = author.name().unwrap_or("Unknown");
            let email = author.email().unwrap_or("unknown@email.com");
            
            let date = match Local.timestamp_opt(commit.time().seconds(), 0) {
                chrono::LocalResult::Single(dt) => dt,
                chrono::LocalResult::Ambiguous(_, _) => Local::now(),
                chrono::LocalResult::None => Local::now(),
            };
            let date_str = date.format("%Y-%m-%d %H:%M:%S").to_string();

            let diff = if let Ok(parent) = commit.parent(0) {
                let mut diff_opts = git2::DiffOptions::new();
                let diff = self.repo.diff_tree_to_tree(
                    Some(&parent.tree()?),
                    Some(&commit.tree()?),
                    Some(&mut diff_opts),
                )?;
                
                let mut diff_str = String::new();
                diff.print(git2::DiffFormat::Patch, |_, _, line| {
                    diff_str.push_str(&format!("{}\n", String::from_utf8_lossy(line.content())));
                    true
                })?;
                
                Some(diff_str)
            } else {
                None
            };

            commits.push(CommitInfo {
                hash: oid.to_string(),
                message: commit.message().unwrap_or("").to_string(),
                author: format!("{} <{}>", name, email),
                date: date_str,
                diff,
            });
        }

        Ok(commits)
    }

    pub fn get_branches(&self) -> Result<Vec<String>> {
        let mut branches = Vec::new();
        
        for branch in self.repo.branches(Some(git2::BranchType::Local))? {
            let (branch, _) = branch?;
            if let Some(name) = branch.name()? {
                branches.push(name.to_string());
            }
        }
        
        Ok(branches)
    }
} 