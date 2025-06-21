use clap::{Arg, ArgAction, ArgMatches};

const DETACHED_LONG: &str = "detached";

pub fn detached_arg() -> Arg {
    Arg::new(DETACHED_LONG)
        .short('d')
        .long(DETACHED_LONG)
        .action(ArgAction::SetTrue)
        .help("don't update the perennial root branch")
}

pub fn read_detached(matches: &ArgMatches) -> bool {
    matches.get_flag(DETACHED_LONG)
}
