
use std::cell::RefCell;
use std::path::Path;
#[derive(Debug, Clone)]
pub struct Author {
pub username: String,
pub mail: String,
}
#[derive(Debug, Clone)]
pub struct Commit {
pub hash: String,
pub parent_hash: String,
pub author: Author,
pub message: String,
pub timestamp: u64,
}
pub struct Repository {
pub name: String,
pub author: Author,
pub last_commit: RefCell<Option<Commit>>,
pub dir: String,
pub path: String,
pub staged_changes: RefCell<Vec<String>>,
}
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
pub fn add_changes(repo: &Repository, changes: &[&Path]) -> Result<(), String> {
let mut staged = repo.staged_changes.borrow_mut();
for path in changes {
staged.push(path.to_string_lossy().to_string());
}
Ok(())
}
pub fn make_commit(repo: &Repository, author: &Author, commit: &Commit) -> Result<Commit, String> {
let new_commit = Commit {
hash: commit.hash.clone(),
parent_hash: commit.parent_hash.clone(),
author: author.clone(),
message: commit.message.clone(),
timestamp: commit.timestamp,
};
*repo.last_commit.borrow_mut() = Some(new_commit.clone());
repo.staged_changes.borrow_mut().clear();
Ok(new_commit)
}
