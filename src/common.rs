use std::process::Output;

use anyhow::{bail, Error, Result};
use console::style;
use which::which;

pub fn has_installed(bin_name: &str) -> bool {
    which(bin_name).is_ok()
}

pub fn pipe_exec_err(output: Output) -> Result<()> {
    if output.status.success() {
        Ok(())
    } else {
        bail!(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn pipe_exec_output(output: Output) -> Result<String> {
    if output.status.success() {
        let mut out = String::from_utf8_lossy(&output.stdout).to_string();

        out.retain(|c| c != '\n');

        Ok(out)
    } else {
        bail!(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn print_err(err: Error) {
    println!();
    println!("{}", style("Eldiron Cli exited with an error").red());
    println!("{}", style(err).red());
}

pub fn welcome() {
    println!("Welcome to Eldiron Cli");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!();
}
