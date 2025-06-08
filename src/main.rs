use std::process::{Command, Stdio};

use clap::{ArgAction, Command as ClapCommand, arg, command, value_parser};

fn main() {
    // Add a really basic "git-town-rs hack" command.

    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            ClapCommand::new("hack")
                .about("Create a new feature branch off the main branch")
                .arg(arg!(<branch>)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("hack", sub_matches)) => {
            let branch_name = sub_matches
                .get_one::<String>("branch")
                .expect("argument is required");

            let _ = Command::new("git")
                .args(["switch", "-c", branch_name])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
        _ => {
            unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`")
        }
    }
}
