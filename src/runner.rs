use std::{io, os::unix::process::CommandExt, process::Command};

use fs_err as fs;

pub enum Never {}

pub fn exec(program: &str, args: Vec<String>) -> Result<Never, io::Error> {
    let mut c = Command::new(program);

    c.args(args);

    let path = std::env::var("PATH").unwrap_or_default();

    // inject node modules bins into path
    // SAFETY: this is a singlethreaded oneshot program, we need to inject path to get executables
    // to resolve
    unsafe {
        std::env::set_var(
            "PATH",
            format!(
                "{}:{path}",
                fs::canonicalize("node_modules/.bin")?.display()
            ),
        );
    }

    Err(c.exec())
}
