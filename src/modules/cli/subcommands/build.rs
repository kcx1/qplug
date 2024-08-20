use clap::ValueEnum;
use mlua::{Lua, UserData};
use std::{env, path::PathBuf};

use crate::{
    files::find_marker_file,
    lua::{info::PluginInfo, parser::merge_lua_files},
};

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

// Build steps:
//     - Get info
//     - Update info.version based on increment
//     - Compile plugin using config.

//TODO: Add compile command
pub fn compile() {
    let current_path = env::current_dir()
        .expect("Unable to get current directory. Please check your permissions.");
    let marker = find_marker_file(current_path.as_path());
    if marker.is_some() {
        let root_path = marker.unwrap();
        let plugin_path = root_path.join("plugin_src");
        match merge_lua_files(root_path, plugin_path) {
            Ok(_) => println!("Plugin updated successfully."),
            Err(e) => println!("Failed to update plugin: {}", e),
        }
    } else {
        println!(
            "No plugin found. Please create a plugin first or navigate to a plugin directory."
        );
    }
}

//TODO: Add update command to only update version without compiling
pub fn update_version(version: VersionType, info_path: PathBuf, lua: &Lua) {
    let mut info = PluginInfo::get_struct(&info_path, lua).expect("Error getting plugin info.");
    info = info.update(version).expect("Update failed.");
    info.write(info_path, lua)
        .expect("Error writing plugin info.");
}

pub fn build(version: VersionType, info_path: PathBuf, lua: &Lua) {
    update_version(version, info_path, lua);
    compile();
}
