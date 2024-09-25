use clap::ValueEnum;
use mlua::{Lua, UserData};
use std::path::PathBuf;

use crate::config::UserEnv;
use crate::lua::info::PluginInfo;

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

pub fn build(version: VersionType, info_path: PathBuf, user_env: UserEnv) {
    update_version(version, info_path, user_env.lua);
    (user_env.config.build_tool)();
    copy_to_plugin_directory().expect("Could not copy plugin");
}

fn update_version(version: VersionType, info_path: PathBuf, lua: &Lua) {
    let mut info = PluginInfo::from_file(&info_path, lua).expect("Error getting plugin info.");
    info = info.update_version(version).expect("Update failed.");
    info.write_to_file(info_path, lua)
        .expect("Error writing plugin info.");
}
