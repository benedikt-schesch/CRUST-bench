
use crate::hashing::generate_hash;
use crate::hashing::generate_random_data;
use crate::hashing::RANDOM_SIZE;
use crate::logger::logger;
use crate::logger::LOGGING_TAG;
use std::{
collections::HashMap,
env, fs,
io::{Read, Write},
path::{Path, PathBuf},
};
pub const MAX_UNAME: usize = 50;
pub const MAX_MAIL: usize = 100;
pub const CONFIG_FILE: &str = "/config";
pub const REPOSITORY_FILE: &str = "/repository";
pub const LOG_FILE: &str = "/log";
pub const MAX_REPOSITORY_FILE_INIT: usize = 1024;
pub const MAX_CWD: usize = 4096;
pub const MAX_REP_NAME: usize = 80;
#[derive(Clone)]
pub struct RepoValue {
pub raw_path: String,
pub hash: String,
}
#[derive(Clone)]
pub struct Author {
pub username: String,
pub mail: String,
}
pub const TRACKED_DIR: &str = "tracked";
pub const HASH_LEN: usize = 2 * 32 + 1;
#[derive(Clone)]
pub struct Commit {
pub hash: String,
pub author: Author,
pub parent_hash: String,
}
#[derive(Clone)]
pub struct Repository {
pub name: String,
pub author: Author,
pub dir: String,
pub last_commit: Option<Commit>,
}
fn create_hash() -> String {
let mut random_data = [0u8; RANDOM_SIZE];
generate_random_data(&mut random_data);
let mut hash = [0u8; 32];
generate_hash(&random_data, &mut hash);
let mut s = String::with_capacity(64);
for b in hash {
s.push_str(&format!("{b:02x}"));
}
s
}
pub fn init_repo(author: &Author) -> Result<Repository, String> {
let dir = env::current_dir()
.map_err(|_| "[init_repo] Can not get the current working directory".to_string())?
.to_string_lossy()
.to_string();
let first_commit = Commit {
hash: "0".to_string(),
parent_hash: "-".to_string(),
author: author.clone(),
};
let repo = Repository {
name: "foo".to_string(),
author: author.clone(),
dir: dir.clone(),
last_commit: Some(first_commit.clone()),
};
let dotdir_path = format!("{}/.vsync", repo.dir);
fs::create_dir(&dotdir_path)
.map_err(|_| "[init_repo] Can not create .vsync dir".to_string())?;
write_repository_file(&repo)
.map_err(|_| "[init_repo] Can not create repository file".to_string())?;
let mut map = HashMap::new();
make_init_map_repo(&repo, &mut map, &repo.dir)
.map_err(|_| "[init_repo] can not create the hashmap for the first commit".to_string())?;
create_commit(&repo, &first_commit, &map)
.map_err(|_| "[init_repo] can not create the first commit".to_string())?;
Ok(repo)
}
pub fn write_repository_file(repo: &Repository) -> Result<(), String> {
let repo_file_path = format!("{}/.vsync/repository", repo.dir);
let mut repo_file = fs::File::create(&repo_file_path)
.map_err(|_| " [write_repository_file] Can not open the repository file".to_string())?;
let last_hash = repo
.last_commit
.as_ref()
.map(|c| c.hash.clone())
.unwrap_or_default();
let content = format!(
"{}\n{}\n{}\n{} {}",
repo.dir, last_hash, repo.name, repo.author.username, repo.author.mail
);
repo_file
.write_all(content.as_bytes())
.map_err(|_| " [write_repository_file] Can not open the repository file".to_string())
}
pub fn load_repository() -> Repository {
let dir = env::current_dir()
.unwrap_or_else(|_| PathBuf::from("."))
.to_string_lossy()
.to_string();
let repository_file_path = format!("{}/.vsync/repository", dir);
let repository_text = fs::read_to_string(&repository_file_path)
.unwrap_or_else(|_| panic!("[load_repository] Can not load the repository file"));
let mut repository_lines = repository_text.lines();
let _repo_dir_line = repository_lines.next().unwrap_or("");
let last_commit_hash = repository_lines.next().unwrap_or("").to_string();
let repo_name = repository_lines.next().unwrap_or("").to_string();
let repo_author_line = repository_lines.next().unwrap_or("");
let last_commit_path = format!("{}/.vsync/{}/commit", dir, last_commit_hash);
let last_commit_text = fs::read_to_string(&last_commit_path)
.unwrap_or_else(|_| panic!("[load_repository] Can not load the last commit"));
let mut commit_lines = last_commit_text.lines();
let parent_hash = commit_lines.next().unwrap_or("").to_string();
let last_commit_author_line = commit_lines.next().unwrap_or("").to_string();
let mut split_last = last_commit_author_line.splitn(2, ' ');
let last_user = split_last.next().unwrap_or("").to_string();
let last_mail = split_last
.next()
.unwrap_or_else(|| panic!("[load_repository] Can not load the author of last commit"))
.to_string();
let last_commit = Commit {
hash: last_commit_hash,
author: Author {
username: last_user,
mail: last_mail,
},
parent_hash,
};
let mut split_repo = repo_author_line.splitn(2, ' ');
let repo_user = split_repo.next().unwrap_or("").to_string();
let repo_mail = split_repo
.next()
.unwrap_or_else(|| panic!("[load_repository] Can not load the author of repository"))
.to_string();
Repository {
name: repo_name,
author: Author {
username: repo_user,
mail: repo_mail,
},
dir,
last_commit: Some(last_commit),
}
}
pub fn load_author() -> Author {
if let Ok(vsync_config_path) = env::var("VSYNC_CONFIG_PATH") {
if let Ok(text) = fs::read_to_string(vsync_config_path) {
if let Some(line) = text.lines().next() {
let mut parts = line.splitn(2, ' ');
let username = parts.next().unwrap_or("").to_string();
if let Some(mail) = parts.next() {
return Author {
username,
mail: mail.to_string(),
};
}
}
}
}
logger(
LOGGING_TAG::ERROR_TAG,
"[load_author] The global config file does not existed",
);
panic!("[load_author] The global config file does not existed");
}
pub fn make_init_map_repo(
_repo: &Repository,
map: &mut HashMap<String, RepoValue>,
path: &str,
) -> Result<(), String> {
let dir = fs::read_dir(path)
.map_err(|_| "[make_init_map_repo] Can not open the init commit dir".to_string())?;
for entry in dir {
let entry =
entry.map_err(|_| "[make_init_map_repo] Can not open the init commit dir".to_string())?;
let name = entry.file_name().to_string_lossy().to_string();
if name != "." && name != ".." && name != ".vsync" {
let fullpath = entry.path();
if fullpath.is_dir() {
make_init_map_repo(_repo, map, &fullpath.to_string_lossy())?;
} else {
let hash_string = create_hash();
map.insert(
fullpath.to_string_lossy().to_string(),
RepoValue {
raw_path: fullpath.to_string_lossy().to_string(),
hash: hash_string,
},
);
}
}
}
Ok(())
}
pub fn add_changes(repo: &Repository, files: &Vec<&Path>) -> Result<(), String> {
let mut track_dir_path = format!("{}/.vsync", repo.dir);
if !Path::new(&track_dir_path).is_dir() {
logger(LOGGING_TAG::ERROR_TAG, "[add_changes] Can't find .vsync");
return Err("fail".to_string());
}
track_dir_path.push_str("/tracked");
if !Path::new(&track_dir_path).is_dir() {
fs::create_dir(&track_dir_path)
.map_err(|_| "Can not create a tracking dir".to_string())?;
}
let mut map = HashMap::new();
let track_file_path = format!("{}/track", track_dir_path);
if Path::new(&track_file_path).exists() {
populate_hashmap_from_file(&mut map, &track_dir_path, &track_file_path)?;
}
for trav in files {
if !trav.exists() {
logger(
LOGGING_TAG::ERROR_TAG,
"Can not adding changes that not existed... check the added files",
);
return Err("fail".to_string());
}
let path_str = trav.to_string_lossy().to_string();
if let Some(existing) = map.get(&path_str).cloned() {
let old_file_path = format!("{}/{}", track_dir_path, existing.hash);
fs::remove_file(&old_file_path)
.map_err(|_| "[add_changes] Can not adding changes".to_string())?;
map.remove(&path_str);
}
let hash_string = create_hash();
map.insert(
path_str.clone(),
RepoValue {
raw_path: path_str,
hash: hash_string,
},
);
}
if make_changes(repo, &mut map).is_err() {
logger(LOGGING_TAG::ERROR_TAG, "[add_changes] Can not persist changes");
return Err("fail".to_string());
}
Ok(())
}
pub fn make_changes(repo: &Repository, map: &mut HashMap<String, RepoValue>) -> Result<(), String> {
let track_dir_path = format!("{}/.vsync/tracked", repo.dir);
let track_file_path = format!("{}/track", track_dir_path);
let mut track_file = fs::File::create(&track_file_path)
.map_err(|_| "[make_changes] Can not open track file".to_string())?;
track_file
.write_all(b"-\n-\n")
.map_err(|_| "[make_changes] Can not open track file".to_string())?;
for (path, current) in map.iter() {
let destination_path = format!("{}/{}", track_dir_path, current.hash);
if current.raw_path != destination_path {
let mut changes_file = fs::File::open(&current.raw_path)
.map_err(|_| "[make_changes] Can not open changes file".to_string())?;
let mut destination_file = fs::File::create(&destination_path)
.map_err(|_| "[make_changes] Can not open destination file".to_string())?;
let mut buffer = [0u8; 1024];
loop {
let bytes_read = changes_file
.read(&mut buffer)
.map_err(|_| "[make_changes] Can not open changes file".to_string())?;
if bytes_read == 0 {
break;
}
destination_file
.write_all(&buffer[..bytes_read])
.map_err(|_| "[make_changes] Can not open destination file".to_string())?;
}
}
let line = format!("{path} {}\n", current.hash);
track_file
.write_all(line.as_bytes())
.map_err(|_| "[make_changes] Can not open track file".to_string())?;
}
Ok(())
}
pub fn create_commit(
repo: &Repository,
commit: &Commit,
map: &HashMap<String, RepoValue>,
) -> Result<(), String> {
let commit_dir = format!("{}/.vsync/{}", repo.dir, commit.hash);
fs::create_dir(&commit_dir)
.map_err(|_| "[create_commit] Can not create the commit dir".to_string())?;
let commit_file_path = format!("{}/commit", commit_dir);
let mut commit_file = fs::File::create(&commit_file_path)
.map_err(|_| "[create_commit] Can not create the commit file".to_string())?;
let header = format!(
"{}\n{} {}\n",
commit.parent_hash, commit.author.username, commit.author.mail
);
commit_file
.write_all(header.as_bytes())
.map_err(|_| "[create_commit] Can not create the commit file".to_string())?;
for (path, current) in map.iter() {
let mut source_file = fs::File::open(&current.raw_path)
.map_err(|_| "[create_commit] can not open the src file".to_string())?;
let destination_path = format!("{}/{}", commit_dir, current.hash);
let mut destination_file = fs::File::create(&destination_path)
.map_err(|_| "[create_commit] can not open the destination file".to_string())?;
let mut buffer = [0u8; 1024];
loop {
let bytes_read = source_file
.read(&mut buffer)
.map_err(|_| "[create_commit] can not open the src file".to_string())?;
if bytes_read == 0 {
break;
}
destination_file
.write_all(&buffer[..bytes_read])
.map_err(|_| "[create_commit] can not open the destination file".to_string())?;
}
let line = format!("{path} {}\n", current.hash);
commit_file
.write_all(line.as_bytes())
.map_err(|_| "[create_commit] Can not create the commit file".to_string())?;
}
Ok(())
}
pub fn make_commit(repo: &Repository, author: &Author, _commit: &Commit) -> Result<(), String> {
let last_commit = repo
.last_commit
.clone()
.ok_or_else(|| "[make_commit] missing last commit".to_string())?;
let commit = Commit {
hash: create_hash(),
parent_hash: last_commit.hash.clone(),
author: author.clone(),
};
let mut changes_map = HashMap::new();
let track_dir_path = format!("{}/.vsync/tracked", repo.dir);
let track_file_path = format!("{}/track", track_dir_path);
if Path::new(&track_file_path).exists() {
populate_hashmap_from_file(&mut changes_map, &track_dir_path, &track_file_path)
.map_err(|_| "[make_commit] Can not load changes to the map".to_string())?;
}
let last_commit_dir_path = format!("{}/.vsync/{}", repo.dir, last_commit.hash);
let last_commit_file_path = format!("{}/commit", last_commit_dir_path);
let mut last_commit_map = HashMap::new();
populate_hashmap_from_file(
&mut last_commit_map,
&last_commit_dir_path,
&last_commit_file_path,
)
.map_err(|_| "[make_commit] can not load commit file to the map".to_string())?;
for (path, value) in changes_map.iter() {
last_commit_map.remove(path);
last_commit_map.insert(path.clone(), value.clone());
}
if create_commit(repo, &commit, &last_commit_map).is_err() {
logger(LOGGING_TAG::ERROR_TAG, "[make_commit] Can not create the commit");
return Err("fail".to_string());
}
let mut updated_repo = repo.clone();
updated_repo.last_commit = Some(commit);
if write_repository_file(&updated_repo).is_err() {
logger(
LOGGING_TAG::ERROR_TAG,
"[make_commit] Can not update the repository file",
);
return Err("fail".to_string());
}
if delete_tracked_dir(repo).is_err() {
logger(
LOGGING_TAG::ERROR_TAG,
"[make_commit] Can not delete old tracked files",
);
return Err("fail".to_string());
}
Ok(())
}
pub fn delete_tracked_dir(repo: &Repository) -> Result<(), String> {
let tracked_dir_path = format!("{}/.vsync/tracked", repo.dir);
let tracked_path = Path::new(&tracked_dir_path);
if !tracked_path.is_dir() {
return Ok(());
}
for entry in fs::read_dir(tracked_path).map_err(|_| "[delete_tracked_dir]".to_string())? {
let entry = entry.map_err(|_| "[delete_tracked_dir]".to_string())?;
let path = entry.path();
if path.is_file() {
fs::remove_file(&path).map_err(|_| "[delete_tracked_dir]".to_string())?;
}
}
fs::remove_dir(tracked_path).map_err(|_| "[delete_tracked_dir]".to_string())?;
Ok(())
}
pub fn rollback(repo: &Repository, commit_hash: &str) -> Result<(), String> {
let commit_dir_path = format!("{}/.vsync/{}", repo.dir, commit_hash);
if !Path::new(&commit_dir_path).is_dir() {
logger(LOGGING_TAG::ERROR_TAG, "[rollback] Commit not found");
return Err("fail".to_string());
}
let mut commit_map = HashMap::new();
let commit_file_path = format!("{}/commit", commit_dir_path);
if !Path::new(&commit_file_path).exists() {
logger(LOGGING_TAG::ERROR_TAG, "[rollback] Can not found the commit file");
return Err("fail".to_string());
}
populate_hashmap_from_file(&mut commit_map, &commit_dir_path, &commit_file_path)
.map_err(|_| "[rollback] Can not populate the hashmap from the file".to_string())?;
reset_repo_dir(&repo.dir, &repo.dir)
.map_err(|_| "[rollback] Can not delete files from the repository".to_string())?;
make_rollback_commit(&mut commit_map)
.map_err(|_| "[rollback] Can not rollback files from the commit".to_string())?;
let commit = load_commit(repo, commit_hash)
.map_err(|_| "[rollback] Can not load the commit".to_string())?;
let mut updated_repo = repo.clone();
updated_repo.last_commit = Some(commit);
if write_repository_file(&updated_repo).is_err() {
logger(
LOGGING_TAG::ERROR_TAG,
"[rollback] Can not update the repository file",
);
return Err("fail".to_string());
}
Ok(())
}
pub fn reset_repo_dir(path: &str, root_path: &str) -> Result<(), String> {
let dir = fs::read_dir(path).map_err(|_| "fail".to_string())?;
for entry in dir {
let entry = entry.map_err(|_| "fail".to_string())?;
let name = entry.file_name().to_string_lossy().to_string();
if name != "." && name != ".." && name != ".vsync" {
let full_path = entry.path();
if full_path.is_dir() {
reset_repo_dir(&full_path.to_string_lossy(), root_path)?;
} else {
fs::remove_file(&full_path).map_err(|_| "fail".to_string())?;
}
}
}
if path != root_path {
fs::remove_dir(path).map_err(|_| "fail".to_string())?;
}
Ok(())
}
fn create_directories(path: &str) -> Result<(), String> {
if path.is_empty() {
return Ok(());
}
if let Some(parent) = Path::new(path).parent() {
if !parent.as_os_str().is_empty() {
fs::create_dir_all(parent).map_err(|_| "fail".to_string())?;
}
}
Ok(())
}
pub fn make_rollback_commit(map: &mut HashMap<String, RepoValue>) -> Result<(), String> {
let entries: Vec<(String, RepoValue)> =
map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
for (path, current) in entries {
create_directories(&path).map_err(|_| {
logger(
LOGGING_TAG::ERROR_TAG,
"[make_rollback_commit] can not create directories for copying",
);
"fail".to_string()
})?;
let mut src_file = fs::File::open(&current.raw_path).map_err(|_| {
logger(
LOGGING_TAG::ERROR_TAG,
"[make_rollback_commit] can not open the raw file",
);
"fail".to_string()
})?;
create_directories(&path).map_err(|_| {
logger(
LOGGING_TAG::ERROR_TAG,
"[make_rollback_commit] can not creating dirs for dest file",
);
"fail".to_string()
})?;
let mut dest_file = fs::File::create(&path).map_err(|_| {
logger(
LOGGING_TAG::ERROR_TAG,
"[make_rollback_commit] can not open the destination file",
);
"fail".to_string()
})?;
let mut buffer = [0u8; 1024];
loop {
let bytes_read = src_file.read(&mut buffer).map_err(|_| "fail".to_string())?;
if bytes_read == 0 {
break;
}
dest_file
.write_all(&buffer[..bytes_read])
.map_err(|_| "fail".to_string())?;
}
}
Ok(())
}
pub fn load_commit(repo: &Repository, commit_hash: &str) -> Result<Commit, String> {
let commit_file_path = format!("{}/.vsync/{}/commit", repo.dir, commit_hash);
let commit_text = fs::read_to_string(&commit_file_path)
.map_err(|_| "[load_commit] Commit no found".to_string())?;
let mut lines = commit_text.lines();
let parent_hash = lines.next().unwrap_or("").to_string();
let author_line = lines.next().unwrap_or("").to_string();
let mut split = author_line.splitn(2, ' ');
let username = split.next().unwrap_or("").to_string();
let mail = split
.next()
.ok_or_else(|| "[load_commit] Author of commit not found".to_string())?
.to_string();
Ok(Commit {
hash: commit_hash.to_string(),
author: Author { username, mail },
parent_hash,
})
}
fn populate_hashmap_from_file(
map: &mut HashMap<String, RepoValue>,
raw_path: &str,
filename: &str,
) -> Result<(), String> {
let file_text = fs::read_to_string(filename)
.map_err(|_| "Can not populate a hashmap from such as this file".to_string())?;
let mut lines = file_text.lines();
let _ = lines.next();
let _ = lines.next();
for line in lines {
if let Some(space_pos) = line.find(' ') {
let path = line[..space_pos].to_string();
let hash = line[space_pos + 1..].to_string();
let src_file_path = format!("{}/{}", raw_path, hash);
map.insert(
path,
RepoValue {
raw_path: src_file_path,
hash,
},
);
}
}
Ok(())
}
