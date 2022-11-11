use clap::Parser;
use std::io;

mod args;
mod repository;
use crate::args::{Action, Args, Init};
use crate::repository::Repository;

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
