use anyhow::Result;

use crate::eldiron::server::restart_server_service_systemd;

pub fn server_restart() -> Result<()> {
    println!("Restarting Eldiron server.");

    restart_server_service_systemd()
}
