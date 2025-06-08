use std::process::Command;

fn main() {
    // First step: Figure out how to call another command.

    // I want to run `git --version` and output the result.

    let git_version_output = Command::new("git").arg("--version").output().unwrap();

    print!("{}", str::from_utf8(&git_version_output.stdout).unwrap())
}
