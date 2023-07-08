use std::path::PathBuf;

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

use crate::{
    common::{has_installed, remove_dir_if_exist},
    eldiron::{
        server::{build_server, setup_server_service_systemd, ELDIRON_BIN_NAME_SERVER},
        ELDIRON_GIT_URL, ELDIRON_INSTALL_BIN_DIR,
    },
    git::{git_clone, git_get_remote_latest_tag, parse_clone_dir_from_url},
};

enum SourceSelectItem {
    Default,
    Custom,
}

const SOURCE_SELECT_ITEM_LIST: [SourceSelectItem; 2] =
    [SourceSelectItem::Default, SourceSelectItem::Custom];

impl SourceSelectItem {
    fn labels() -> Vec<&'static str> {
        let len = SOURCE_SELECT_ITEM_LIST.len();

        let mut res = Vec::with_capacity(len);

        for i in 0..len {
            res.push(Self::to_label(&SOURCE_SELECT_ITEM_LIST[i]));
        }

        res
    }

    fn to_label(&self) -> &'static str {
        match self {
            Self::Default => "Default (Latest release)",
            Self::Custom => "Custom",
        }
    }
}

enum SourceCustomSelectItem {
    Git,
    Local,
}

const SOURCE_CUSTOM_SELECT_ITEM_LIST: [SourceCustomSelectItem; 2] =
    [SourceCustomSelectItem::Git, SourceCustomSelectItem::Local];

impl SourceCustomSelectItem {
    fn labels() -> Vec<&'static str> {
        let len = SOURCE_CUSTOM_SELECT_ITEM_LIST.len();

        let mut res = Vec::with_capacity(len);

        for i in 0..len {
            res.push(Self::to_label(&SOURCE_CUSTOM_SELECT_ITEM_LIST[i]));
        }

        res
    }

    fn to_label(&self) -> &'static str {
        match self {
            Self::Git => "Git repository",
            Self::Local => "Local directory",
        }
    }
}

enum SourceType {
    CustomGit { url: String, branch: Option<String> },
    CustomLocal(String),
    Latest,
}

pub fn server_setup() -> Result<()> {
    println!("Setting up Eldiron server.");

    if [ELDIRON_INSTALL_BIN_DIR, ELDIRON_BIN_NAME_SERVER]
        .iter()
        .collect::<PathBuf>()
        .as_path()
        .exists()
    {
        if !Confirm::new()
            .with_prompt("Eldiron server already installed on this machine. Re-install?")
            .interact()?
        {
            println!("Abort.");
            return Ok(());
        }
    }

    let root = match select_source()? {
        SourceType::CustomGit { url, branch } => setup_root_custom_git(&url, branch.as_deref()),
        SourceType::CustomLocal(path) => setup_root_custom_local(path),
        SourceType::Latest => setup_root_latest(),
    }?;

    build_server(root)?;

    if has_installed("systemctl") {
        if Confirm::new()
            .with_prompt("Setup sysmtemd service for Eldiron server?")
            .default(true)
            .interact()?
        {
            setup_server_service_systemd()?;
        }
    }

    Ok(())
}

fn select_source() -> Result<SourceType> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select where is the source code of Eldiron:")
        .items(&SourceSelectItem::labels())
        .default(0)
        .interact_opt()?;

    match selection {
        Some(index) => match SOURCE_SELECT_ITEM_LIST[index] {
            SourceSelectItem::Default => Ok(SourceType::Latest),
            SourceSelectItem::Custom => select_source_custom(),
        },
        None => {
            println!("No selection. Use default option instead.");

            Ok(SourceType::Latest)
        }
    }
}

fn select_source_custom() -> Result<SourceType> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select custom Eldiron source:")
        .items(&SourceCustomSelectItem::labels())
        .default(0)
        .interact_opt()?;

    match selection {
        Some(index) => match SOURCE_CUSTOM_SELECT_ITEM_LIST[index] {
            SourceCustomSelectItem::Git => select_source_custom_git(),
            SourceCustomSelectItem::Local => select_source_custom_local(),
        },
        None => {
            println!("No selection. Use default option instead.");

            Ok(SourceType::Latest)
        }
    }
}

fn select_source_custom_git() -> Result<SourceType> {
    let url = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Git URL (Use default repo if omitted):")
        .default(ELDIRON_GIT_URL.to_owned())
        .interact_text()?;

    let branch = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Branch (eg. v0.1.0, master. Use default branch if omitted):")
        .allow_empty(true)
        .interact_text()?;

    let branch = if branch.is_empty() {
        None
    } else {
        Some(branch)
    };

    Ok(SourceType::CustomGit { url, branch })
}

fn select_source_custom_local() -> Result<SourceType> {
    let path = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Directory:")
        .default(".".to_owned())
        .interact_text()?;

    Ok(SourceType::CustomLocal(path))
}

fn setup_root_custom_git(url: &str, branch: Option<&str>) -> Result<String> {
    println!("Setup Eldiron source code from Git repository: {}", url);

    if let Some(branch) = branch {
        println!("Branch: {}", branch);
    }

    let root = parse_clone_dir_from_url(&url)?;

    if remove_dir_if_exist(&root)? {
        println!("Removing directory: {}", root);
    }

    git_clone(&url, branch)?;

    Ok(root)
}

fn setup_root_custom_local(path: String) -> Result<String> {
    println!("Setup Eldiron source code from local directory: {}", path);

    Ok(path)
}

fn setup_root_latest() -> Result<String> {
    let latest_tag = git_get_remote_latest_tag(ELDIRON_GIT_URL)?;

    println!(
        "Setup Eldiron source code from latest release: {}",
        latest_tag
    );

    let latest_tag = if latest_tag.is_empty() {
        None
    } else {
        Some(latest_tag.as_str())
    };

    let root = parse_clone_dir_from_url(ELDIRON_GIT_URL)?;

    if remove_dir_if_exist(&root)? {
        println!("Removing directory: {}", root);
    }

    git_clone(ELDIRON_GIT_URL, latest_tag)?;

    Ok(root)
}
