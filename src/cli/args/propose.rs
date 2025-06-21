use clap::{Arg, ArgAction, ArgMatches};

const PROPOSE_LONG: &str = "propose";

pub fn propose_arg() -> Arg {
    Arg::new(PROPOSE_LONG)
        .long(PROPOSE_LONG)
        .action(ArgAction::SetTrue)
        .help("propose the new branch")
}

pub fn read_propose(matches: &ArgMatches) -> bool {
    matches.get_flag(PROPOSE_LONG)
}
