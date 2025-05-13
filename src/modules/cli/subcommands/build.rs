use anyhow::Context;
use clap::ValueEnum;
use mlua::{Lua, UserData};
use std::path::PathBuf;

use crate::globals::INIT_LUA;
use crate::lua::info::PluginInfo;
use crate::modules::user::UserEnv;
use crate::{files::find_project_dir, lua::parser::merge_lua_files};

use super::copy::copy_to_plugin_directory;

//TODO: Refactor this to a more central location.
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "lower")]
pub enum VersionType {
    Dev,
    Patch,
    Minor,
    Major,
}
impl UserData for VersionType {}

pub fn build(
    version: VersionType,
    info_path: PathBuf,
    user_env: UserEnv,
    copy_path: Option<&PathBuf>,
    build_only: bool,
) -> anyhow::Result<()> {
    //TODO: Have user_env.config.build_tool return a Result. Use unwrap_or_else. This would mean
    //that we could make the default build tool a local function instead of public. This would
    //leverage better locality of behavior, but would work better if there are custom error handling.
    let build_tool = &user_env.config.build_tool;
    if build_only {
        build_tool(None);
        return Ok(());
    }

    update_version(version, info_path, user_env.lua)?;
    build_tool(None);
    copy_to_plugin_directory(user_env.config, copy_path).context("Could not copy plugin")?;
    Ok(())
}

pub fn default_build_tool() -> anyhow::Result<()> {
    let marker = find_project_dir(None);
    match marker {
        Some(marker) => {
            let root_path = marker;
            let plugin_path = root_path.join("plugin_src");
            match merge_lua_files(
                root_path,
                plugin_path,
                Some(INIT_LUA.clone().context("Failed to load init.lua")?),
            ) {
                Ok(_) => Ok(println!("Plugin updated successfully.")),
                Err(e) => Err(anyhow::anyhow!(format!("Failed to update plugin: {}", e))),
            }
        }

        None => Err(anyhow::anyhow!(format!(
            "No plugin found. Please create a plugin first or navigate to a plugin directory."
        ))),
    }
}

fn update_version(version: VersionType, info_path: PathBuf, lua: &Lua) -> anyhow::Result<()> {
    // dbg!(&info_path);
    let mut info = PluginInfo::from_file(&info_path, lua).context("Error getting plugin info.")?;
    info = info.update_version(version)?;
    info.write_to_file(info_path, lua)
        .expect("Error writing plugin info.");
    Ok(())
}
