mod restart;
mod setup;
mod shutdown;

use anyhow::Result;
use clap::Subcommand;

use self::{restart::server_restart, setup::server_setup, shutdown::server_shutdown};

#[derive(Debug, Subcommand)]
pub enum ServerCommand {
    Restart,
    Setup,
    Shutdown,
}

pub fn handle_command_server(command: ServerCommand) -> Result<()> {
    match command {
        ServerCommand::Restart => server_restart(),
        ServerCommand::Setup => server_setup(),
        ServerCommand::Shutdown => server_shutdown(),
    }
}
