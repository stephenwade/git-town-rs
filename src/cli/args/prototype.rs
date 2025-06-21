use clap::{Arg, ArgAction, ArgMatches};

const PROTOTYPE_LONG: &str = "prototype";

pub fn prototype_arg() -> Arg {
    Arg::new(PROTOTYPE_LONG)
        .short('p')
        .long(PROTOTYPE_LONG)
        .action(ArgAction::SetTrue)
        .help("create a prototype branch")
}

pub fn read_prototype(matches: &ArgMatches) -> bool {
    matches.get_flag(PROTOTYPE_LONG)
}
