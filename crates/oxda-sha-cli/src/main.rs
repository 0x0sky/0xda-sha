// © 2026 aiaiaiai · aiaiaiai.org

//! Process shell for `0xda-sha`.

#![forbid(unsafe_code)]

use std::env;
use std::process::ExitCode as ProcessExitCode;

use oxda_sha_cli::{execute, parse, ExitCode, SystemGitResolver};

fn main() -> ProcessExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let git = SystemGitResolver;

    match parse(&args).and_then(|command| execute(&command, &git)) {
        Ok(output) => {
            print!("{output}");
            ProcessExitCode::from(ExitCode::Success as u8)
        }
        Err(error) => {
            eprintln!("{error}");
            ProcessExitCode::from(error.exit_code() as u8)
        }
    }
}
