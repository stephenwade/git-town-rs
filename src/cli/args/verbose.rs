use clap::{Arg, ArgAction, ArgMatches};

use crate::config::config_domain;

const VERBOSE_LONG: &str = "verbose";

pub fn verbose_arg() -> Arg {
    Arg::new(VERBOSE_LONG)
        .short('v')
        .long(VERBOSE_LONG)
        .action(ArgAction::SetTrue)
        .help("display all Git commands run under the hood")
}

pub fn read_verbose(matches: &ArgMatches) -> config_domain::Verbose {
    matches.get_flag(VERBOSE_LONG).into()
}
