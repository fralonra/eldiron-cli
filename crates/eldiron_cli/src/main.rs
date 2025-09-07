use std::process;

use dialoguer::console::style;
use eldiron_cli::run_cli;

fn main() {
    if let Err(err) = run_cli() {
        println!("{}", style("Eldiron Cli exited with an error").red());
        println!("{}", style(err).red());

        process::exit(1);
    }
}
