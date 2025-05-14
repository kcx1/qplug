use anyhow::Context;
use clap::ValueEnum;
use git2::Repository;

use crate::{files::user_config, globals::QPLUG_DIR, modules::lua_lsp::write_lsp_file};

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "lower")]
pub enum Installables {
    Definitions,
    Encryption,
    LegacyBuild,
}

impl std::fmt::Display for Installables {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            Installables::Definitions => println!("{:?}", Installables::Definitions),
            Installables::Encryption => println!("{:?}", Installables::Encryption),
            Installables::LegacyBuild => println!("{:?}", Installables::LegacyBuild),
        };
        Ok(())
    }
}

pub fn install(installable: Option<&Installables>) -> anyhow::Result<()> {
    if let Some(installable) = installable {
        match installable {
            Installables::Definitions => install_definitions()?,
            Installables::Encryption => println!("Encryption"),
            Installables::LegacyBuild => println!("LegacyBuild"),
        };
    };
    match installable {
        Some(thing) => Ok(println!("{thing}")),
        None => Ok(println!("Nothing")),
    }
}

fn install_definitions() -> anyhow::Result<()> {
    let url = "https://github.com/kcx1/Q-Sys-Defs";
    let defs_path = user_config().join(QPLUG_DIR); // ~/.config on Linux/macOS, AppData/Roaming on Windows
    let defs = Repository::clone(&url, defs_path)
        .context(format!("Failed to clone template repo: {url}"))?
        .path()
        .to_path_buf();

    println!("Definitions have been installed to: {:?}", defs);

    write_lsp_file()?;

    Ok(())
}
