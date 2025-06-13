use clap::{Arg, ArgAction, ArgMatches};

use crate::config::config_domain;

const PROTOTYPE_LONG: &str = "prototype";

pub fn prototype_arg() -> Arg {
    Arg::new(PROTOTYPE_LONG)
        .short('p')
        .long(PROTOTYPE_LONG)
        .action(ArgAction::SetTrue)
        .help("create a prototype branch")
}

pub fn read_prototype(matches: &ArgMatches) -> config_domain::Prototype {
    matches.get_flag(PROTOTYPE_LONG).into()
}
