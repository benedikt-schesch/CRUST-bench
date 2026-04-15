// Generated Rust Code
use VaultSync::logger::logger;
use VaultSync::logger::LOGGING_TAG;
use VaultSync::vm::add_changes;
use VaultSync::vm::init_repo;
use VaultSync::vm::load_author;
use VaultSync::vm::load_repository;
use VaultSync::vm::make_commit;
use VaultSync::vm::rollback;
use VaultSync::vm::Commit;
use std::env;
use std::path::Path;

pub const ARG_INIT: &str = "init";
pub const ARG_HELP: &str = "--help";
pub const ARG_HELP_SC: &str = "-h";
pub const ARG_ADD_CHANGES: &str = "add";
pub const ARG_COMMIT: &str = "commit";
pub const ARG_ROLLBACK: &str = "rollback";

pub fn main() {
let args: Vec<String> = env::args().collect();
run_cli(args.len() as i32, &args);
}

fn run_cli(argc: i32, argv: &[String]) {
if argc < 2 {
invalid_args();
return;
} else if argc < 3 {
let cmd = &argv[1];
let init_action = cmd == ARG_INIT;
let commit_action = cmd == ARG_COMMIT;
let help_action = cmd == ARG_HELP || cmd == ARG_HELP_SC;

if init_action {
init_repository();
} else if commit_action {
commit_changes();
} else if help_action {
getting_help();
} else {
invalid_args();
}
return;
} else if argc < 4 && argv[1] == ARG_ROLLBACK {
rollback_changes(&argv[2]);
return;
}

if argv[1] == ARG_ADD_CHANGES {
let refs: Vec<&str> = argv.iter().map(|s| s.as_str()).collect();
track_changes(argc, &refs);
return;
}

invalid_args();
}

pub fn getting_help() {
println!("Usage: vaultsync [OPTIONS] COMMAND\n");
println!("Options:");
println!("  -h, --help     Show help message and exit\n");
println!("Commands:");
println!("  init           Initialize a repository");
println!("  commit         Make a commit");
println!("  add [files]    Add files to be tracked in the next commit\n");
println!("For more information, see 'man vaultsync'");
}

fn invalid_args() {
logger(LOGGING_TAG::INFO_TAG, "Invalid arguments, check --help");
}

pub fn init_repository() {
let author = match std::panic::catch_unwind(load_author) {
Ok(a) => a,
Err(_) => {
logger(
LOGGING_TAG::ERROR_TAG,
"Can not getting the author, check the default config at ~/.vsync_rc",
);
return;
}
};

if init_repo(&author).is_ok() {
logger(LOGGING_TAG::INFO_TAG, "Initialization done");
}
}

pub fn track_changes(n: i32, files: &Vec<&str>) {
let mut paths = Vec::new();
let mut i = 2usize;
while i < n as usize && i < files.len() {
paths.push(Path::new(files[i]));
i += 1;
}

let repo = match std::panic::catch_unwind(load_repository) {
Ok(r) => r,
Err(_) => {
logger(LOGGING_TAG::ERROR_TAG, "Can not getting the repo info");
return;
}
};

if add_changes(&repo, &paths).is_ok() {
logger(
LOGGING_TAG::INFO_TAG,
"Files has been tracked successfully",
);
}
}

pub fn commit_changes() {
let author = match std::panic::catch_unwind(load_author) {
Ok(a) => a,
Err(_) => {
logger(
LOGGING_TAG::ERROR_TAG,
"Can not getting the author, check the default config at ~/.vsync_rc",
);
return;
}
};

let repo = match std::panic::catch_unwind(load_repository) {
Ok(r) => r,
Err(_) => {
logger(LOGGING_TAG::ERROR_TAG, "Can not getting the repo info");
return;
}
};

let new_commit = Commit {
hash: String::new(),
author: author.clone(),
parent_hash: String::new(),
};

if make_commit(&repo, &author, &new_commit).is_ok() {
logger(
LOGGING_TAG::INFO_TAG,
"The commit has been done successfully",
);
}
}

pub fn rollback_changes(hash: &str) {
let repo = match std::panic::catch_unwind(load_repository) {
Ok(r) => r,
Err(_) => {
logger(LOGGING_TAG::ERROR_TAG, "Can not getting the repo info");
return;
}
};

if rollback(&repo, hash).is_ok() {
logger(
LOGGING_TAG::INFO_TAG,
"The rollback has been successfully done",
);
}
}
