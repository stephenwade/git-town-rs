use clap::{Arg, ArgAction, ArgMatches};

use crate::config_domain;

const BEAM_LONG: &str = "beam";

pub fn beam_arg() -> Arg {
    Arg::new(BEAM_LONG)
        .short('b')
        .long(BEAM_LONG)
        .action(ArgAction::SetTrue)
        .help("Beam some commits from this branch to the new branch")
}

pub fn read_beam(matches: &ArgMatches) -> config_domain::Beam {
    matches.get_flag(BEAM_LONG).into()
}
