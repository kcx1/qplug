use clap::ValueEnum;
use mlua::{Lua, UserData};
use std::path::PathBuf;

use crate::config::UserEnv;
use crate::lua::info::PluginInfo;
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
    build_path: Option<&String>,
    copy_path: Option<&String>,
    build_only: bool,
) {
    //TODO: Have user_env.config.build_tool return a Result. Use unwrap_or_else. This would mean
    //that we could make the default build tool a local function instead of public. This would
    //leverage better locality of behavior, but would work better if there are custom error handling.
    let build_tool = &user_env.config.build_tool;
    if build_only {
        return build_tool(build_path);
    }

    let path = build_path.map_or(info_path, PathBuf::from);
    update_version(version, path, user_env.lua);
    build_tool(build_path);
    copy_to_plugin_directory(user_env.config, copy_path).expect("Could not copy plugin");
}

pub fn default_build_tool(build_file: Option<&String>) {
    let marker = find_project_dir(None);
    if marker.is_some() {
        let root_path = marker.unwrap();
        let plugin_path = root_path.join("plugin_src");
        match merge_lua_files(root_path, plugin_path, build_file.map(PathBuf::from)) {
            Ok(_) => println!("Plugin updated successfully."),
            Err(e) => println!("Failed to update plugin: {}", e),
        }
    } else {
        println!(
            "No plugin found. Please create a plugin first or navigate to a plugin directory."
        );
    }
}

fn update_version(version: VersionType, info_path: PathBuf, lua: &Lua) {
    let mut info = PluginInfo::from_file(&info_path, lua).expect("Error getting plugin info.");
    info = info.update_version(version).expect("Update failed.");
    info.write_to_file(info_path, lua)
        .expect("Error writing plugin info.");
}
