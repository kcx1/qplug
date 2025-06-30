use clap::ValueEnum;
use self_update::backends::github;
use self_update::cargo_crate_version;

#[derive(ValueEnum, Clone, Debug)]
pub enum Updatable {
    SelfUpdate,
    Encryption,
    LegacyBuild,
    Definitions,
    All,
}

pub fn update(updatable: &Updatable, version: &Option<&str>) -> anyhow::Result<()> {
    match updatable {
        Updatable::SelfUpdate => self_update(version),
        //TODO: Update all of the update functions; Should just be a git pull, but need to check if
        //actually installed first
        Updatable::Definitions => {
            todo!("Create an update function for definitions")
        }
        Updatable::Encryption => todo!(),
        Updatable::LegacyBuild => todo!(),
        Updatable::All => todo!(),
    }?;
    Ok(())
}

pub fn self_update(version: &Option<&str>) -> anyhow::Result<()> {
    // Create a builder for the update
    let mut status = github::UpdateBuilder::new();

    // Configure the update
    let config = status
        .repo_owner("kcx1")
        .repo_name("qplug")
        .bin_name("qplug")
        .show_download_progress(true)
        .show_output(false)
        .current_version(cargo_crate_version!());

    // Apply specific version if specified
    if let Some(version) = version {
        config.target_version_tag(version);
    }

    // Check and Build config - Then update if successful
    let result = config.build()?.update()?;

    // Check the result status.
    match result {
        self_update::Status::UpToDate(version) => {
            println!("Q-Plug updated to: `{version}`");
        }
        self_update::Status::Updated(version) => {
            println!("\nQ-Plug is already at the latest version {version}!\nYou can specify a different version with `qplug update -v <version>`");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn get_releases() -> anyhow::Result<()> {
        let releases = self_update::backends::github::ReleaseList::configure()
            .repo_owner("kcx1")
            .repo_name("qplug")
            .build()?
            .fetch()
            .expect("\nMake sure that you are connected to the internet!\n\n");
        println!("found releases:");
        println!("{:#?}\n", releases);

        Ok(())
    }
}
