use clap::ValueEnum;
use mlua::{Lua, UserData};
use std::path::PathBuf;

use crate::lua::info::PluginInfo;

use super::compile::compile;
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

pub fn build(version: VersionType, info_path: PathBuf, lua: &Lua) {
    update_version(version, info_path, lua);
    compile();
    copy_to_plugin_directory().expect("Could not copy plugin");
}

fn update_version(version: VersionType, info_path: PathBuf, lua: &Lua) {
    let mut info = PluginInfo::from_file(&info_path, lua).expect("Error getting plugin info.");
    info = info.update_version(version).expect("Update failed.");
    info.write_to_file(info_path, lua)
        .expect("Error writing plugin info.");
}
