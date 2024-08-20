use clap::ValueEnum;
use mlua::{Lua, UserData};
use std::path::PathBuf;

use crate::lua::info::PluginInfo;

use super::compile::compile;

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
