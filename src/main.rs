///! djanatyn's backup automation:
///! - generate password for backup,
///! - create borg backup repository,
///! - store borg key in password manager,
///! - generate systemd-creds,
///! - run initial backup,
///! - create backblaze-b2 bucket,
///! - upload borg repository to b2 bucket
///!
use clap::{Parser, Subcommand};
use duct::cmd;
use std::io;
use std::path::PathBuf;

const PASSWORD_LENGTH: i64 = 32;
const BACKUP_REPO_DIRECTORY: &str = "/archive/"; // my NAS!
const PASSWORD_MANAGER_PREFIX: &str = "backups";

/// Command line arguments.
#[derive(Parser, Debug)]
struct Args {
    /// Action subcommand.
    #[command(subcommand)]
    action: Action,
}

/// What command?
#[derive(Subcommand, Debug)]
enum Action {
    /// Initialize a database.
    Init(Init),
}

/// Initialize a backup.
#[derive(Parser, Debug)]
struct Init {
    /// Name of backup.
    #[arg(short, long)]
    name: String,

    /// Path to directory to back up.
    #[arg(short, long)]
    path: PathBuf,
}

/// Generate password, add to password manager.
///
/// Returns name of generated password.
fn generate_password(name: &String) -> String {
    let path = format!("{PASSWORD_MANAGER_PREFIX}/{name}/password");

    cmd!(
        "pwgen",
        "-1",
        "--symbols",
        "--secure",
        PASSWORD_LENGTH.to_string()
    )
    .pipe(cmd!("pass", "insert", "--multiline", &path))
    .run()
    .expect("failed to run");

    path
}

/// Initialize a borg repository in `BACKUP_REPO_DIRECTORY`.
///
/// Returns path to initialized repository.
///
/// The "password" argument is a reference to the entry in a password manager,
/// not a literal secret.
fn borg_init_repo(repo_name: &String, password: &String) -> PathBuf {
    let pass_command = format!("pass show {password}");
    let backup_directory = format!("{BACKUP_REPO_DIRECTORY}/{repo_name}");

    cmd!("borg", "init", "--encryption", "repokey", &backup_directory)
        .env("BORG_PASSCOMMAND", pass_command)
        .run()
        .expect("failed to init borg repo");

    PathBuf::from(backup_directory)
}

/// Run a borg backup.
fn borg_run_backup(
    backup_name: &String,
    borg_repo_path: &PathBuf,
    files_to_backup: &PathBuf,
    password: &String,
) {
    let pass_command = format!("pass show {password}");
    let archive = format!("{0}::{backup_name}", borg_repo_path.display());

    cmd!(
        "borg",
        "create",
        "--verbose",
        "--stats",
        archive,
        files_to_backup
    )
    .env("BORG_PASSCOMMAND", pass_command)
    .run()
    .expect("failed to init borg repo");

    // TODO: store borg key
}

/// Initialize a backup!
fn init(args: Init) -> io::Result<()> {
    let pass = generate_password(&args.name);
    let repo = borg_init_repo(&args.name, &pass);
    borg_run_backup(&"initial backup".to_string(), &repo, &args.path, &pass);

    // TODO: store borg key in password manager,
    // TODO: generate systemd-creds,
    // TODO: create backblaze-b2 bucket,
    // TODO: upload borg repository to b2 bucket

    todo!();
}

/// Parse arguments and run.
fn main() -> io::Result<()> {
    let subcmd = Args::parse();

    match subcmd.action {
        Action::Init(args) => init(args),
    }
}
