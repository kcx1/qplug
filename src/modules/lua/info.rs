use mlua::{Lua, LuaSerdeExt, Result, Table, UserData};

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::cli::subcommands::build::VersionType;

use super::parser::{self, name_table};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginInfo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "BuildVersion")]
    pub build_version: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Author")]
    pub author: String,
    #[serde(rename = "Description")]
    pub description: String,
}

impl UserData for PluginInfo {}

impl IntoIterator for PluginInfo {
    type Item = (String, String);
    type IntoIter = std::array::IntoIter<(String, String), 6>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([
            ("Name".to_string(), self.name),
            ("Version".to_string(), self.version),
            ("BuildVersion".to_string(), self.build_version),
            ("Id".to_string(), self.id),
            ("Author".to_string(), self.author),
            ("Description".to_string(), self.description),
        ])
    }
}

impl PluginInfo {
    pub fn get_struct(path: &PathBuf, lua: &Lua) -> Result<PluginInfo> {
        let globals = lua.globals();
        let info_file = fs::read_to_string(path)?;
        lua.load(info_file.to_string()).exec()?;

        let info: PluginInfo = lua.from_value(globals.get("PluginInfo")?).unwrap();
        Ok(info)
    }

    pub fn get_table(self, lua: &Lua) -> Table {
        let table = lua.create_table().unwrap();
        for pairs in self.into_iter() {
            let (k, v) = pairs;
            table.set(k, v).unwrap();
        }
        table
    }

    pub fn update(mut self, increment: VersionType) -> Result<PluginInfo> {
        // self.id = Uuid::new_v4().to_string();
        self.build_version = PluginInfo::update_build_version(self.build_version, increment);

        Ok(self)
    }

    pub fn write(self, path: PathBuf, lua: &Lua) -> Result<()> {
        let table = parser::serialize_table(lua, &self.get_table(lua));
        let contents = name_table("PluginInfo", &table);

        Ok(fs::write(path, contents)?)
    }

    pub fn update_build_version(version: String, increment: VersionType) -> String {
        let mut ver: Vec<&str> = version.split('.').collect();

        match increment {
            VersionType::Dev => {
                let update = ver[3].parse::<i32>().unwrap_or(0) + 1;
                let update_string = &update.to_string();
                ver[3] = update_string;
                ver.join(".").to_string()
            }
            VersionType::Patch => {
                let update = ver[2].parse::<i32>().unwrap_or(0) + 1;
                let update_string = &update.to_string();
                ver[2] = update_string;
                ver[3] = "0";
                ver.join(".").to_string()
            }
            VersionType::Minor => {
                let update = ver[1].parse::<i32>().unwrap_or(0) + 1;
                let update_string = &update.to_string();
                ver[1] = update_string;
                ver[2] = "0";
                ver[3] = "0";
                ver.join(".").to_string()
            }
            VersionType::Major => {
                let update = ver[0].parse::<i32>().unwrap_or(0) + 1;
                let update_string = &update.to_string();
                ver[0] = update_string;
                ver[1] = "0";
                ver[2] = "0";
                ver[3] = "0";
                ver.join(".").to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_get_info() {
        let lua = Lua::new();
        PluginInfo::get_struct(&Path::new("./Api/plugin_src/info.lua").to_path_buf(), &lua)
            .unwrap();
    }
    #[test]
    fn test_update_major_build_number() {
        let updated = PluginInfo::update_build_version("1.0.0.0".to_string(), VersionType::Major);
        let control = "2.0.0.0".to_string();
        assert_eq!(updated, control);
    }
    #[test]
    fn test_update_minor_build_number() {
        let updated = PluginInfo::update_build_version("1.0.0.0".to_string(), VersionType::Minor);
        let control = "1.1.0.0".to_string();
        assert_eq!(updated, control);
    }
    #[test]
    fn test_update_patch_build_number() {
        let updated = PluginInfo::update_build_version("1.0.0.0".to_string(), VersionType::Patch);
        let control = "1.0.1.0".to_string();
        assert_eq!(updated, control);
    }
    #[test]
    fn test_update_patch_build_number_with_big_number() {
        let updated = PluginInfo::update_build_version("1.0.999.0".to_string(), VersionType::Patch);
        let control = "1.0.1000.0".to_string();
        assert_eq!(updated, control);
    }
    #[test]
    fn test_update_dev_build_number() {
        let updated = PluginInfo::update_build_version("1.0.0.0".to_string(), VersionType::Dev);
        let control = "1.0.0.1".to_string();
        assert_eq!(updated, control);
    }
    #[test]
    fn test_update_info() {
        let lua = Lua::new();
        let info =
            PluginInfo::get_struct(&Path::new("./Api/plugin_src/info.lua").to_path_buf(), &lua)
                .unwrap();
        let updated = info.clone().update(VersionType::Patch).unwrap();

        assert_ne!(&updated.build_version, &info.build_version);
        //Updated this to remain the same once created.
        assert_eq!(&updated.id, &info.id);
    }
    #[test]
    fn test_write_info() {
        let lua = Lua::new();
        let info_path = Path::new("./Api/plugin_src/info.lua");
        let info = PluginInfo::get_struct(&info_path.to_path_buf(), &lua).unwrap();
        info.clone()
            .write(
                Path::new("./Api/plugin_src/test_info.lua").to_path_buf(),
                &lua,
            )
            .unwrap();
    }
}
