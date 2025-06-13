use clap::{Arg, ArgAction, ArgMatches};

use crate::config::config_domain;

const COMMIT_LONG: &str = "commit";

pub fn commit_arg() -> Arg {
    Arg::new(COMMIT_LONG)
        .short('c')
        .long(COMMIT_LONG)
        .action(ArgAction::SetTrue)
        .help("commit the stashed changes into the new branch")
}

pub fn read_commit(matches: &ArgMatches) -> config_domain::Commit {
    matches.get_flag(COMMIT_LONG).into()
}
