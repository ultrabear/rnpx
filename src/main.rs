use core::error;
use fs_err as fs;
use std::{
    env,
    io::{self, Write},
    process::ExitCode,
};

use clap::Parser;
use npm_escape::escape_script;
use runner::exec;

mod npm_escape;
mod package;
mod runner;

/// A basic program to run package.json scripts without pnpm or npm startup overhead
/// no workspace support (yet, maybe ever idk)
#[derive(clap::Parser)]
#[clap(version)]
struct Args {
    /// The program or package.json defined script that rnpx will run
    program: String,

    /// Args to pass to the running program
    args: Vec<String>,
}

fn inner() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    let package_json = package::parse()?;

    let cwd = fs::canonicalize(env::current_dir()?)?;

    match package_json.scripts.get(&args.program) {
        Some(script) => {
            let run = escape_script(script, &args.args);

            _ = writeln!(io::stdout(), "");
            _ = writeln!(
                io::stdout(),
                "> {}@{} {} {}",
                package_json.name,
                package_json.version,
                args.program,
                cwd.display()
            );
            _ = writeln!(io::stdout(), "> {run}");
            _ = writeln!(io::stdout(), "");

            exec("sh", vec!["-c".into(), run])?;
        }
        None => {
            exec(&args.program, args.args)?;
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    match inner() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            _ = writeln!(io::stderr(), "\x1b[91mERROR\x1b[0m: {e}");
            ExitCode::FAILURE
        }
    }
}
