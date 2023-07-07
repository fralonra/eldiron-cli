use anyhow::Result;

use crate::eldiron::server::start_server_service_systemd;

pub fn server_start() -> Result<()> {
    println!("Starting Eldiron server.");

    start_server_service_systemd()
}
