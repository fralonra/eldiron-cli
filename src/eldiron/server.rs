use std::{
    fs::File,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Result;

use crate::{
    common::pipe_exec_err,
    eldiron::{install_eldiron, ELDIRON_INSTALL_BIN_DIR},
};

pub const ELDIRON_BIN_NAME_SERVER: &'static str = "eldiron-server";

pub fn build_server<P>(root: P) -> Result<()>
where
    P: AsRef<Path>,
{
    println!("Eldiron source code path: {:?}", root.as_ref());

    install_eldiron(&root)?;

    build_server_bin(root)
}

pub fn restart_server_service_systemd() -> Result<()> {
    println!("Restarting Eldiron server systemd service.");

    pipe_exec_err(
        Command::new("systemctl")
            .args(&["restart", ELDIRON_BIN_NAME_SERVER])
            .output()?,
    )
}

pub fn setup_server_service_systemd() -> Result<()> {
    println!("Setting up Eldiron service for systemd.");

    pipe_exec_err(
        Command::new("echo")
            .arg(include_str!("../resources/eldiron-server.service"))
            .stdout(File::create("/etc/systemd/system/eldiron-server.service")?)
            .output()?,
    )?;

    pipe_exec_err(
        Command::new("systemctl")
            .args(&["enable", ELDIRON_BIN_NAME_SERVER])
            .output()?,
    )?;

    println!("Setting up systemd successfully. Run 'eldiron server start' to start the server.");

    Ok(())
}

pub fn shutdown_server_service_systemd() -> Result<()> {
    println!("Shutting down Eldiron server systemd service.");

    pipe_exec_err(
        Command::new("systemctl")
            .args(&["stop", ELDIRON_BIN_NAME_SERVER])
            .output()?,
    )
}

pub fn start_server_service_systemd() -> Result<()> {
    println!("Starting Eldiron server systemd service.");

    pipe_exec_err(
        Command::new("systemctl")
            .args(&["start", ELDIRON_BIN_NAME_SERVER])
            .output()?,
    )
}

fn build_server_bin<P>(root: P) -> Result<()>
where
    P: AsRef<Path>,
{
    println!("Building Eldiron server.");

    pipe_exec_err(
        Command::new("cargo")
            .current_dir(&root)
            .args(&["build", "--release", "-p", "server"])
            .output()?,
    )?;

    pipe_exec_err(
        Command::new("mv")
            .current_dir(root)
            .args(&[
                "-uf",
                "target/release/server",
                &[ELDIRON_INSTALL_BIN_DIR, ELDIRON_BIN_NAME_SERVER]
                    .iter()
                    .collect::<PathBuf>()
                    .to_string_lossy()
                    .to_string(),
            ])
            .output()?,
    )
}
