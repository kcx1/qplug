use mlua::{Error, Lua, LuaSerdeExt, Result, UserData, Value};
use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;

use crate::lua::info::{BuildIncrement, PluginInfo};

fn get_info(path: &String) -> Result<PluginInfo> {
    let lua = Lua::new();
    let globals = lua.globals();
    let info_file = fs::read_to_string(path)?;
    lua.load(info_file.to_string()).exec()?;

    let mut info: PluginInfo = lua.from_value(globals.get("PluginInfo")?).unwrap();
    println!("{:?}", info);
    Ok(info)
}

fn update_build_version(version: String, increment: BuildIncrement) -> String {
    let mut ver: Vec<&str> = version.split('.').collect();

    match increment {
        BuildIncrement::Patch => {
            let update = ver[2].parse::<i32>().unwrap_or(0) + 1;
            let update_string = &update.to_string();
            ver[2] = update_string;
            ver.join(".").to_string()
        }
        BuildIncrement::Minor => {
            let update = ver[1].parse::<i32>().unwrap_or(0) + 1;
            let update_string = &update.to_string();
            ver[1] = update_string;
            ver[2] = "0";
            ver.join(".").to_string()
        }
        BuildIncrement::Major => {
            let update = ver[0].parse::<i32>().unwrap_or(0) + 1;
            let update_string = &update.to_string();
            ver[0] = update_string;
            ver[1] = "0";
            ver[2] = "0";
            ver.join(".").to_string()
        }
    }
}

fn update_info(mut info: PluginInfo, increment: BuildIncrement) -> Result<PluginInfo> {
    info.Id = Uuid::new_v4().to_string();
    info.BuildVersion = update_build_version(info.BuildVersion, increment);

    Ok(info)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_info() {
        get_info(&"./Api/plugin_src/info.lua".to_string()).unwrap();
    }
    #[test]
    fn test_update_major_build_number() {
        let updated = update_build_version("1.0.0".to_string(), BuildIncrement::Major);
        let control = "2.0.0".to_string();
        assert_eq!(updated, control);
    }
    #[test]
    fn test_update_minor_build_number() {
        let updated = update_build_version("1.0.0".to_string(), BuildIncrement::Minor);
        let control = "1.1.0".to_string();
        assert_eq!(updated, control);
    }
    #[test]
    fn test_update_patch_build_number() {
        let updated = update_build_version("1.0.0".to_string(), BuildIncrement::Patch);
        let control = "1.0.1".to_string();
        assert_eq!(updated, control);
    }
    #[test]
    fn test_update_patch_build_number_with_big_number() {
        let updated = update_build_version("1.0.999".to_string(), BuildIncrement::Patch);
        let control = "1.0.1000".to_string();
        assert_eq!(updated, control);
    }
    #[test]
    fn test_update_info() {
        let info = get_info(&"./Api/plugin_src/info.lua".to_string()).unwrap();
        let updated = update_info(info.to_owned(), BuildIncrement::Patch).unwrap();
        assert_ne!(&updated.BuildVersion, &info.BuildVersion);
        assert_ne!(&updated.Id, &info.Id);
    }
}
