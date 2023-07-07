use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Result;

use crate::common::pipe_exec_err;

pub mod server;

pub const ELDIRON_GIT_URL: &'static str = "https://github.com/markusmoenig/Eldiron.git";
pub const ELDIRON_INSTALL_DIR: &'static str = "/usr/local/bin/eldiron";
pub const ELDIRON_INSTALL_BIN_DIR: &'static str = "/usr/local/bin/eldiron/bin";

pub fn install_eldiron<P>(root: P) -> Result<()>
where
    P: AsRef<Path>,
{
    println!("Installing Eldiron at: {}", ELDIRON_INSTALL_DIR);

    pipe_exec_err(
        Command::new("mkdir")
            .args(&["-p", ELDIRON_INSTALL_DIR])
            .output()?,
    )?;

    pipe_exec_err(
        Command::new("mkdir")
            .args(&["-p", ELDIRON_INSTALL_BIN_DIR])
            .output()?,
    )?;

    copy_game_resources(root)
}

fn copy_game_resources<P>(root: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let root = root.as_ref().to_string_lossy().to_string();

    pipe_exec_err(
        Command::new("cp")
            .args(&[
                "-r",
                &[root.as_str(), "assets"]
                    .iter()
                    .collect::<PathBuf>()
                    .to_string_lossy()
                    .to_string(),
                ELDIRON_INSTALL_DIR,
            ])
            .output()?,
    )?;

    pipe_exec_err(
        Command::new("cp")
            .args(&[
                "-r",
                &[root.as_str(), "game"]
                    .iter()
                    .collect::<PathBuf>()
                    .to_string_lossy()
                    .to_string(),
                ELDIRON_INSTALL_DIR,
            ])
            .output()?,
    )
}
