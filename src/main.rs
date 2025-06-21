mod cli;
mod cmd;
mod execute;
mod git;
mod messages;
mod misc;
mod subshell;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cmd::execute()?;
    Ok(())
}
