use clap::{Arg, ArgAction, ArgMatches};

const BEAM_LONG: &str = "beam";

pub fn beam_arg() -> Arg {
    Arg::new(BEAM_LONG)
        .short('b')
        .long(BEAM_LONG)
        .action(ArgAction::SetTrue)
        .help("beam some commits from this branch to the new branch")
}

pub fn read_beam(matches: &ArgMatches) -> bool {
    matches.get_flag(BEAM_LONG)
}
