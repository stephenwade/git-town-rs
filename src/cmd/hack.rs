use std::process::{Command, Stdio};

use clap::{Arg, ArgMatches};

use crate::cli::args;
use crate::cmd::cmd_helpers;

pub const HACK_NAME: &str = "hack";

pub fn hack_cmd() -> clap::Command {
    let hack_desc = "Create a new feature branch off the main branch";
    let hack_help = r#"
Consider this stack:

main
 \
  branch-1
   \
*   branch-2

We are on the "branch-2" branch. After running "git hack branch-3", our
workspace contains these branches:

main
 \
  branch-1
   \
    branch-2
 \
* branch-3

The new branch "feature-2"
is a child of the main branch.

If there are no uncommitted changes,
it also syncs all affected branches."#;

    clap::Command::new(HACK_NAME)
        .about(hack_desc)
        .long_about(cmd_helpers::long(hack_desc, hack_help))
        .arg(Arg::new("branch").num_args(0..))
        .arg(args::beam::beam_arg())
}

pub fn execute_hack(matches: &ArgMatches) {
    let branches: Vec<_> = matches
        .get_many::<String>("branch")
        .unwrap_or_default()
        .collect();
    let beam = args::beam::read_beam(matches);
    println!("branches: {:?}", branches);
    println!("beam: {:?}", &beam);

    // _ = Command::new("git")
    //     .args(["switch", "-c", branch])
    //     .stdout(Stdio::null())
    //     .stderr(Stdio::null())
    //     .status();
}
