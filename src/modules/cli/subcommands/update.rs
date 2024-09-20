use self_update::backends::github;
use self_update::cargo_crate_version;

pub fn update(version: &Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
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
    fn get_releases() -> Result<(), Box<dyn std::error::Error>> {
        let releases = self_update::backends::github::ReleaseList::configure()
            .repo_owner("kcx1")
            .repo_name("qplug")
            .build()?
            .fetch()?;
        println!("found releases:");
        println!("{:#?}\n", releases);

        Ok(())
    }
}
