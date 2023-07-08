mod restart;
mod setup;
mod shutdown;
mod start;

use anyhow::Result;
use clap::Subcommand;

use self::{
    restart::server_restart, setup::server_setup, shutdown::server_shutdown, start::server_start,
};

#[derive(Debug, Subcommand)]
pub enum ServerCommand {
    /// Restart server service under systemd.
    Restart,
    /// Setup a new server.
    Setup,
    /// Shutdown server service under systemd.
    Shutdown,
    /// Start server service under systemd.
    Start,
}

pub fn handle_command_server(command: ServerCommand) -> Result<()> {
    match command {
        ServerCommand::Restart => server_restart(),
        ServerCommand::Setup => server_setup(),
        ServerCommand::Shutdown => server_shutdown(),
        ServerCommand::Start => server_start(),
    }
}
