
use crate::vm::{Author, Commit, Repository};
use std::cell::RefCell;
fn main() {
let author = Author {
username: "testuser".to_string(),
mail: "test@example.com".to_string(),
};
let repo = Repository {
name: "test".to_string(),
author: author.clone(),
last_commit: RefCell::new(None),
dir: "/tmp".to_string(),
path: "/tmp/repo".to_string(),
staged_changes: RefCell::new(Vec::new()),
};
let mut repo = Repository {
name: "test2".to_string(),
author: author.clone(),
last_commit: RefCell::new(Some(Commit {
author: author.clone(),
hash: "0".to_string(),
parent_hash: "-".to_string(),
message: "Initial commit".to_string(),
timestamp: 0,
})),
dir: "/tmp".to_string(),
path: "/tmp/repo2".to_string(),
staged_changes: RefCell::new(Vec::new()),
};
let mut commit = Commit {
hash: "abc123".to_string(),
parent_hash: "def456".to_string(),
author: author.clone(),
message: "Second commit".to_string(),
timestamp: 1234567890,
};
}
