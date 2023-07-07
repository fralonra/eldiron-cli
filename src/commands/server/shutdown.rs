use anyhow::Result;

use crate::eldiron::server::shutdown_server_service_systemd;

pub fn server_shutdown() -> Result<()> {
    println!("Shutting down Eldiron server.");

    shutdown_server_service_systemd()
}
