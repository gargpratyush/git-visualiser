use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use serde::{Serialize, Deserialize};
use crate::models::CommitInfo;

#[derive(Serialize, Deserialize)]
struct CacheEntry<T> {
    data: T,
    timestamp: SystemTime,
}

pub struct Cache {
    commits: HashMap<String, CacheEntry<Vec<CommitInfo>>>,
    authors: HashMap<PathBuf, CacheEntry<Vec<String>>>,
    branches: HashMap<PathBuf, CacheEntry<Vec<String>>>,
    cache_duration: Duration,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            commits: HashMap::new(),
            authors: HashMap::new(),
            branches: HashMap::new(),
            cache_duration: Duration::from_secs(300), // 5 minutes default
        }
    }

    pub fn get_commits(&self, branch: &str) -> Option<&Vec<CommitInfo>> {
        self.commits.get(branch).and_then(|entry| {
            if entry.timestamp.elapsed().unwrap_or(Duration::MAX) < self.cache_duration {
                Some(&entry.data)
            } else {
                None
            }
        })
    }

    pub fn set_commits(&mut self, branch: String, commits: Vec<CommitInfo>) {
        self.commits.insert(
            branch,
            CacheEntry {
                data: commits,
                timestamp: SystemTime::now(),
            },
        );
    }

    pub fn get_authors(&self, repo_path: &PathBuf) -> Option<&Vec<String>> {
        self.authors.get(repo_path).and_then(|entry| {
            if entry.timestamp.elapsed().unwrap_or(Duration::MAX) < self.cache_duration {
                Some(&entry.data)
            } else {
                None
            }
        })
    }

    pub fn set_authors(&mut self, repo_path: PathBuf, authors: Vec<String>) {
        self.authors.insert(
            repo_path,
            CacheEntry {
                data: authors,
                timestamp: SystemTime::now(),
            },
        );
    }

    pub fn get_branches(&self, repo_path: &PathBuf) -> Option<&Vec<String>> {
        self.branches.get(repo_path).and_then(|entry| {
            if entry.timestamp.elapsed().unwrap_or(Duration::MAX) < self.cache_duration {
                Some(&entry.data)
            } else {
                None
            }
        })
    }

    pub fn set_branches(&mut self, repo_path: PathBuf, branches: Vec<String>) {
        self.branches.insert(
            repo_path,
            CacheEntry {
                data: branches,
                timestamp: SystemTime::now(),
            },
        );
    }

    pub fn clear(&mut self) {
        self.commits.clear();
        self.authors.clear();
        self.branches.clear();
    }

    pub fn set_cache_duration(&mut self, duration: Duration) {
        self.cache_duration = duration;
    }
} 