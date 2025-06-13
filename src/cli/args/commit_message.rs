use clap::{Arg, ArgMatches};

use crate::git::git_domain;

const COMMIT_MESSAGE_LONG: &str = "message";

pub fn commit_message_arg(help: &'static str) -> Arg {
    Arg::new(COMMIT_MESSAGE_LONG)
        .short('m')
        .long(COMMIT_MESSAGE_LONG)
        .help(help)
}

pub fn read_commit_message(matches: &ArgMatches) -> Option<git_domain::CommitMessage> {
    let commit_message_option = matches.get_one::<String>(COMMIT_MESSAGE_LONG);
    commit_message_option.map(|x| x.into())
}
