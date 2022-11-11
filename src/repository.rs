use duct::cmd;
use std::path::PathBuf;

const PASSWORD_MANAGER_PREFIX: &str = "backups";
const PASSWORD_LENGTH: i64 = 32;
const BACKUP_REPO_DIRECTORY: &str = "/archive"; // my NAS!

/// An initialized borg backup repository.
#[derive(Debug)]
pub struct Repository {
    /// What is the name of the repository?
    pub repo_name: String,
    /// Where is it stored?
    pub repo_path: PathBuf,
    /// What files does this repository backup?
    pub backup_target: PathBuf,
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

        cmd!("borg", "init", "--encryption", "repokey", dbg!(&repo_path))
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
            "--json",
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
