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
    /// Initialize a borg backup repository.
    Init(Init),
}

/// Initialize a backup repository.
#[derive(Parser, Debug)]
struct Init {
    /// What should the repository be named?
    #[arg(short, long)]
    name: String,

    /// What files should be backed up?
    #[arg(short, long)]
    path: PathBuf,
}

/// An initialized borg backup repository.
#[derive(Debug)]
struct Repository {
    /// What is the name of the repository?
    repo_name: String,
    /// Where is it stored?
    repo_path: PathBuf,
    /// What files does this repository backup?
    backup_target: PathBuf,
    /// What is the password manager entry to unlock this repository?
    password_entry: String,
}

impl Repository {
    /// Generate password, add entry to password manager.
    /// Returns path to password entry.
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

    /// Save the borg repository key in the password manager.
    fn save_borg_key(&self) {
        let path = format!("{PASSWORD_MANAGER_PREFIX}/{0}/borg-key", self.repo_name);
        cmd!("borg", "key", "export", &self.repo_path)
            .env(
                "BORG_PASSCOMMAND",
                format!("pass show {0}", self.password_entry),
            )
            .pipe(cmd!("pass", "insert", "--multiline", &path))
            .run()
            .expect("failed to export key");
    }

    /// Initialize repository.
    pub fn new(repo_name: String, backup_target: PathBuf) -> Repository {
        let password_entry = Self::generate_password(&repo_name);
        let repo_path = PathBuf::from(format!("{BACKUP_REPO_DIRECTORY}/{repo_name}"));

        cmd!("borg", "init", "--encryption", "repokey", &repo_path)
            .env("BORG_PASSCOMMAND", format!("pass show {password_entry}"))
            .run()
            .expect("failed to init borg repo");

        // repository is initialized
        let repo = Repository {
            repo_name,
            repo_path,
            backup_target,
            password_entry,
        };

        // save the repository key before returning!
        repo.save_borg_key();
        repo
    }

    /// Construct name for archive using repository path.
    pub fn archive_name(&self, name: &str) -> String {
        format!("{0}::{name}", self.repo_path.display())
    }

    /// Run a backup!
    pub fn backup(&self, backup_name: &str) {
        cmd!(
            "borg",
            "create",
            "--verbose",
            "--stats",
            self.archive_name(backup_name),
            &self.backup_target
        )
        .env(
            "BORG_PASSCOMMAND",
            format!("pass show {0}", self.password_entry),
        )
        .run()
        .expect("failed to run backup");
    }
}

/// Initialize a borg repository and run the first backup!
fn init(args: Init) -> io::Result<()> {
    let repo = Repository::new(args.name, args.path);
    repo.backup("initial backup");

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
