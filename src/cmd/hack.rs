use crate::{cli::args, cmd::cmd_helpers, execute};
use clap::{Arg, ArgMatches};
use std::error::Error;

pub const HACK_NAME: &str = "hack";

const HACK_DESC: &str = "Create a new feature branch off the main branch";
const HACK_HELP: &str = r#"
Consider this stack:

main
 \
  branch-1
   \
*   branch-2

We are on the "branch-2" branch. After running "git hack branch-3", our
workspace contains these branches:

main
 \
  branch-1
   \
    branch-2
 \
* branch-3

The new branch "feature-2"
is a child of the main branch.

If there are no uncommitted changes,
it also syncs all affected branches."#;

pub fn hack_cmd() -> clap::Command {
    clap::Command::new(HACK_NAME)
        .about(HACK_DESC)
        .long_about(cmd_helpers::long(HACK_DESC, HACK_HELP))
        .arg(Arg::new("branch").num_args(0..))
        .arg(args::beam::beam_arg())
        .arg(args::commit::commit_arg())
        .arg(args::commit_message::commit_message_arg(
            "the commit message",
        ))
        .arg(args::detached::detached_arg())
        .arg(args::dry_run::dry_run_arg())
        .arg(args::propose::propose_arg())
        .arg(args::prototype::prototype_arg())
        .arg(args::verbose::verbose_arg())
}

pub fn execute_hack(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let branches: Vec<_> = matches
        .get_many::<String>("branch")
        .unwrap_or_default()
        .collect();
    let beam = args::beam::read_beam(matches);
    let commit = args::commit::read_commit(matches);
    let commit_message = args::commit_message::read_commit_message(matches);
    let detached = args::detached::read_detached(matches);
    let dry_run = args::dry_run::read_dry_run(matches);
    let propose = args::propose::read_propose(matches);
    let prototype = args::prototype::read_prototype(matches);
    let verbose = args::verbose::read_verbose(matches);

    let repo = execute::open_repo::open_repo(execute::open_repo::OpenRepoArgs {
        dry_run,
        print_branch_names: true,
        print_commands: true,
        validate_git_repo: true,
        validate_is_online: false,
        verbose,
    })?;

    Ok(())
}
