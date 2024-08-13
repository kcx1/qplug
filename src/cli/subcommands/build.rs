use clap::ValueEnum;
use mlua::UserData;
use std::env;

use crate::lua::parser::merge_lua_files;

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "lower")]
pub enum VersionType {
    Dev,
    Patch,
    Minor,
    Major,
}
impl UserData for VersionType {}

pub fn build(version: VersionType) {
    match version {
        VersionType::Major => println!("major"),
        VersionType::Minor => println!("minor"),
        VersionType::Patch => println!("patch"),
        VersionType::Dev => println!("dev"),
    }
    // TODO: Figure out some magic to know where project root is in relationship to user's
    // pwd
    let root_path = env::current_dir()
        .expect("Unable to get current directory. Please check your permissions.");
    let plugin_path = root_path.join("plugin_src");
    merge_lua_files(root_path, plugin_path).unwrap();
}
