use std::process;

use anyhow::Result;
use clap::{Parser, Subcommand};
use eldiron_cli::{
    commands::server::{handle_command_server, ServerCommand},
    common::{print_err, welcome},
};

#[derive(Debug, Parser)]
#[command(name = "eldiron")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Server {
        #[command(subcommand)]
        command: ServerCommand,
    },
}

fn run_cli() -> Result<()> {
    let args = Cli::parse();

    welcome();

    match args.command {
        Command::Server { command } => handle_command_server(command),
    }
}

fn main() {
    if let Err(err) = run_cli() {
        print_err(err);

        process::exit(1);
    }
}
