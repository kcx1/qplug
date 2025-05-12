use anyhow::Context;
use clap::ValueEnum;
use directories::BaseDirs;
use git2::Repository;

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
    let base_dirs =
        BaseDirs::new().expect("Oops! No valid home diretory could be found on this system");
    let defs_path = base_dirs.config_dir().join("qplug/"); // ~/.config on Linux/macOS, AppData/Roaming on Windows
    let defs = Repository::clone(&url, defs_path)
        .context(format!("Failed to clone template repo: {url}"))?
        .path()
        .to_path_buf();

    println!("Definitions have been installed to: {:?}", defs);

    Ok(())
}
