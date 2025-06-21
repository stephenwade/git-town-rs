use clap::{Arg, ArgAction, ArgMatches};

const DRY_RUN_LONG: &str = "dry-run";

pub fn dry_run_arg() -> Arg {
    Arg::new(DRY_RUN_LONG)
        .long(DRY_RUN_LONG)
        .action(ArgAction::SetTrue)
        .help("print but do not run the Git commands")
}

pub fn read_dry_run(matches: &ArgMatches) -> bool {
    matches.get_flag(DRY_RUN_LONG)
}
