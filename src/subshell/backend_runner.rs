use core::str;
use std::{
    io::{self, Write},
    thread,
};

use super::CONCURRENT_GIT_RETRIES;
use crate::{messages, misc::Counter, subshell::CONCURRENT_GIT_RETRY_DELAY};
use anstream::println;
use duct::cmd;
use owo_colors::OwoColorize;

/// Executes backend shell commands without output to the CLI.
pub struct BackendRunner {
    pub commands_counter: Counter,
    /// If set, runs the commands in the given directory.
    /// If not set, runs the commands in the current working directory.
    pub dir: Option<String>,
    /// Whether to print the executed commands to the CLI
    pub verbose: bool,
}

impl BackendRunner {
    pub fn query(
        &mut self,
        executable: &str,
        args: &[&str],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.execute(&[], executable, args)
    }

    fn execute(
        &mut self,
        envs: &[(&str, &str)],
        executable: &str,
        args: &[&str],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.commands_counter.increment();
        if self.verbose {
            print_header(envs, executable, args);
        }
        let mut sub_process = cmd(executable, args).stderr_to_stdout().unchecked();
        for (k, v) in envs {
            sub_process = sub_process.env(k, v);
        }
        if let Some(dir) = &self.dir {
            sub_process = sub_process.dir(dir);
        }
        let mut concurrent_git_retries_left = CONCURRENT_GIT_RETRIES;
        let mut result: Result<(Vec<u8>, bool), Box<dyn std::error::Error>>;
        loop {
            let output_result = sub_process.run();
            match output_result {
                Ok(output) => {
                    let success = output.status.success();
                    result = Ok((output.stdout.clone(), success));
                    if success || !contains_concurrent_git_access(&output.stdout) {
                        break;
                    }
                    concurrent_git_retries_left -= 1;
                    if concurrent_git_retries_left == 0 {
                        break;
                    }
                    println!("{}", messages::GIT_ANOTHER_PROCESS_IS_RUNNING_RETRY);
                    thread::sleep(CONCURRENT_GIT_RETRY_DELAY);
                }
                Err(e) => {
                    result = Err(e.into());
                    break;
                }
            }
        }
        if self.verbose {
            if let Ok((ref output_bytes, _)) = result {
                let mut output_with_nulls_replaced = Vec::<u8>::with_capacity(output_bytes.len());
                for b in output_bytes.iter() {
                    if *b == 0 {
                        output_with_nulls_replaced.push(b'\n');
                        output_with_nulls_replaced.push(b'\n');
                    } else {
                        output_with_nulls_replaced.push(*b);
                    }
                }
                io::stdout().write_all(&output_with_nulls_replaced)?;
            }
        }
        match result {
            Ok((stdout, success)) => {
                if success {
                    Ok(stdout)
                } else {
                    Err("command failed".into())
                }
            }
            Err(err) => Err(err),
        }
    }
}

// func (self BackendRunner) QueryTrim(executable string, args ...string) (string, error) {
// 	output, err := self.execute([]string{}, executable, args...)
// 	return strings.TrimSpace(stripansi.Strip(output)), err
// }

// func (self BackendRunner) Run(executable string, args ...string) error {
// 	_, err := self.execute([]string{}, executable, args...)
// 	return err
// }

// func (self BackendRunner) RunWithEnv(env []string, executable string, args ...string) error {
// 	_, err := self.execute(env, executable, args...)
// 	return err
// }

fn contains_concurrent_git_access(bytes: &Vec<u8>) -> bool {
    let text = String::from_utf8_lossy(bytes);
    text.contains("fatal: Unable to create '") && text.contains("index.lock': File exists.")
}

fn print_header(envs: &[(&str, &str)], cmd: &str, args: &[&str]) {
    let quoted: Vec<String> = args
        .iter()
        .map(|x| {
            if x.is_empty() || x.contains(' ') {
                format!(r#""{}""#, x)
            } else {
                x.to_string()
            }
        })
        .collect();
    let mut text = "\n(verbose) ".to_owned();
    for (k, v) in envs {
        text.push_str(&format!("{}={} ", k, v));
    }
    text.push_str(&format!("{} {}", cmd, quoted.join(" ")));
    println!("{}", text.bold());
}
