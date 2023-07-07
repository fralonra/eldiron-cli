use std::process::{Command, Stdio};

use anyhow::Result;

use crate::common::{pipe_exec_err, pipe_exec_output};

pub fn git_clone(url: &str, branch: Option<&str>) -> Result<()> {
    let mut command = Command::new("git");

    command.args(&["clone", "--depth", "1", url]);

    if let Some(branch) = branch {
        command.args(&["-b", branch]);
    }

    pipe_exec_err(command.output()?)
}

pub fn git_get_remote_latest_tag(url: &str) -> Result<String> {
    let mut child = Command::new("git")
        .args(&[
            "ls-remote",
            "--tags",
            "--refs",
            "--sort=version:refname",
            url,
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut child = Command::new("tail")
        .arg("--lines=1")
        .stdin(
            child
                .stdout
                .take()
                .ok_or(anyhow::anyhow!("Error while reading from stdout."))?,
        )
        .stdout(Stdio::piped())
        .spawn()?;

    let child = Command::new("cut")
        .args(&["--delimiter=/", "--fields=3"])
        .stdin(
            child
                .stdout
                .take()
                .ok_or(anyhow::anyhow!("Error while reading from stdout."))?,
        )
        .stdout(Stdio::piped())
        .spawn()?;

    pipe_exec_output(child.wait_with_output()?)
}

pub fn parse_clone_dir_from_url(url: &str) -> Result<String> {
    let mut child = Command::new("git")
        .args(&["ls-remote", "--get-url", url])
        .stdout(Stdio::piped())
        .spawn()?;

    let child = Command::new("xargs")
        .args(&["basename", "-s", ".git"])
        .stdin(
            child
                .stdout
                .take()
                .ok_or(anyhow::anyhow!("Error while reading from stdout."))?,
        )
        .stdout(Stdio::piped())
        .spawn()?;

    pipe_exec_output(child.wait_with_output()?)
}
