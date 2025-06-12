use crate::cmd::cmd_helpers;

pub fn root_cmd() -> clap::Command {
    let root_desc = "Branching and workflow support for Git";
    let root_help = "
Git Town helps create, sync, and ship changes
efficiently and with minimal merge conflicts.";

    clap::Command::new("Git Town")
        .version(clap::crate_version!())
        .about(root_desc)
        .long_about(cmd_helpers::long(root_desc, root_help))
        .subcommand_required(true)
        .arg_required_else_help(true)
}
