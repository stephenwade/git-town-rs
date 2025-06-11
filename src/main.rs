use clap::{Args, Parser, Subcommand};
use std::process::{Command, Stdio};

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

#[derive(Args)]
struct HackArgs {
    /// The name of the new feature branch to create
    branch: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Hack(args) => {
            let _ = Command::new("git")
                .args(["switch", "-c", &args.branch])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
    }
}
