use anyhow::Result;
use clap::{Parser, Subcommand};

/// Command-line tool for Eldiron.
#[derive(Debug, Parser)]
#[command(name = "eldiron")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Setup, start, restart, shutdown the Eldiron server.
    Build {
        #[command(subcommand)]
        command: BuildCommand,
    },
}

#[derive(Debug, Subcommand)]
enum BuildCommand {
    /// Restart server service under systemd.
    Build,
}

pub fn run_cli() -> Result<()> {
    let args = Cli::parse();

    println!("Welcome to Eldiron Cli");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!();

    match args.command {
        Command::Build { command } => match command {
            BuildCommand::Build => todo!(),
        },
    }
}
