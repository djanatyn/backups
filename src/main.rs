use std::io;

mod args;
mod bucket;
mod manifest;
mod repository;
mod systemd;

use crate::args::{Action, Args, Init};
use crate::bucket::Bucket;
use crate::manifest::Manifest;
use crate::repository::Repository;
use crate::systemd::SystemdCreds;

/// Initialize a borg repository and run the first backup!
fn init(args: Init) -> io::Result<()> {
    // create repository + run first backup
    let repo = Repository::new(args.name, args.path);
    repo.backup("initial backup");

    // sync with backblaze
    let bucket = Bucket::new(&repo);
    bucket.sync();

    // add credentials to systemd
    let creds = SystemdCreds::new(&repo);

    // record what happened
    let manifest = Manifest {
        repo,
        bucket,
        creds,
    };
    manifest.save();

    Ok(())
}

/// Parse arguments and run.
fn main() -> io::Result<()> {
    use clap::Parser;
    let subcmd = Args::parse();

    match subcmd.action {
        Action::Init(args) => init(args),
    }
}
