use clap::Args;
use std::process::{Command, Stdio};

#[derive(Args)]
pub struct HackArgs {
    /// The name of the new feature branch to create
    branch: String,
}

pub fn execute_hack(args: HackArgs) {
    _ = Command::new("git")
        .args(["switch", "-c", &args.branch])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}
