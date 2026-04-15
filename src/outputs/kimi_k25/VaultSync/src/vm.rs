//! VM module for VaultSync

use std::cell::RefCell;
use std::path::Path;

/// Represents an author of a commit
#[derive(Debug, Clone)]
pub struct Author {
pub username: String,
pub mail: String,
}

/// Represents a commit in the repository
#[derive(Debug, Clone)]
pub struct Commit {
pub hash: String,
pub parent_hash: String,
pub author: Author,
pub message: String,
pub timestamp: u64,
}

/// Repository structure for managing version control operations
pub struct Repository {
pub name: String,
pub author: Author,
pub last_commit: RefCell<Option<Commit>>,
pub dir: String,
pub path: String,
pub staged_changes: RefCell<Vec<String>>,
}

/// Initializes a new repository with the given author
pub fn init_repo(author: &Author) -> Result<Repository, String> {
Ok(Repository {
name: "default".to_string(),
author: author.clone(),
last_commit: RefCell::new(None),
dir: "/tmp".to_string(),
path: "/tmp".to_string(),
staged_changes: RefCell::new(Vec::new()),
})
}

/// Adds changes to the repository staging area
pub fn add_changes(repo: &Repository, changes: &[&Path]) -> Result<(), String> {
let mut staged = repo.staged_changes.borrow_mut();
for path in changes {
staged.push(path.to_string_lossy().to_string());
}
Ok(())
}

/// Creates a new commit with the given author and commit information
pub fn make_commit(repo: &Repository, author: &Author, commit: &Commit) -> Result<Commit, String> {
let new_commit = Commit {
hash: commit.hash.clone(),
parent_hash: commit.parent_hash.clone(),
author: author.clone(),
message: commit.message.clone(),
timestamp: commit.timestamp,
};

// Update the last commit in the repository
*repo.last_commit.borrow_mut() = Some(new_commit.clone());

// Clear staged changes after commit
repo.staged_changes.borrow_mut().clear();

Ok(new_commit)
}
