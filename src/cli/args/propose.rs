use clap::{Arg, ArgAction, ArgMatches};

use crate::config::config_domain;

const PROPOSE_LONG: &str = "propose";

pub fn propose_arg() -> Arg {
    Arg::new(PROPOSE_LONG)
        .long(PROPOSE_LONG)
        .action(ArgAction::SetTrue)
        .help("propose the new branch")
}

pub fn read_propose(matches: &ArgMatches) -> config_domain::Propose {
    matches.get_flag(PROPOSE_LONG).into()
}
