mod cmd;

use clap::{Parser, Subcommand};
use cmd::HackArgs;

use crate::cmd::execute_hack;

#[derive(Parser)]
#[command(subcommand_required = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new feature branch off the main branch
    Hack(HackArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hack(args) => execute_hack(args),
    }
}
