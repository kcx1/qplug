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
    pub fn from_file(file: &PathBuf, lua: &Lua) -> Result<PluginInfo> {
        let globals = lua.globals();
        let info_file = fs::read_to_string(file)?;
        lua.load(info_file.to_string()).exec()?;

        let info: PluginInfo = lua.from_value(globals.get("PluginInfo")?).unwrap();
        Ok(info)
    }

    pub fn from_table(self, table: Table, lua: &Lua) -> Self {
        let serialized_tbl = parser::serialize_table(lua, &table);
        let globals = lua.globals();
        lua.load(serialized_tbl).exec().unwrap();

        let info: Self = lua
            .from_value(
                globals
                    .get("PluginInfo")
                    .expect("error parsing plugin info"),
            )
            .unwrap();
        info
    }

    pub fn to_lua_table(self, lua: &Lua) -> Table {
        let table = lua.create_table().unwrap();
        for pairs in self.into_iter() {
            let (k, v) = pairs;
            table.set(k, v).unwrap();
        }
        table
    }

    pub fn update_field(&mut self, field: &str, value: String) {
        match field {
            "Name" => self.name = value,
            "Version" => self.version = value,
            "BuildVersion" => self.build_version = value,
            "Id" => self.id = value,
            "Author" => self.author = value,
            "Description" => self.description = value,
            _ => eprintln!("Invalid field: {}", field),
        }
    }

    pub fn get_field(&self, field: &str) -> String {
        match field {
            "Name" => self.name.to_string(),
            "Version" => self.version.to_string(),
            "BuildVersion" => self.build_version.to_string(),
            "Id" => self.id.to_string(),
            "Author" => self.author.to_string(),
            "Description" => self.description.to_string(),
            _ => panic!("Invalid field: {}", field),
        }
    }

    pub fn update_version(mut self, increment: VersionType) -> Result<PluginInfo> {
        // self.id = Uuid::new_v4().to_string();
        self.build_version = PluginInfo::update_build_version(self.build_version, increment);
        self.sync_version_with_build_version();

        Ok(self)
    }

    pub fn write_to_file(self, file: PathBuf, lua: &Lua) -> Result<()> {
        let table = parser::serialize_table(lua, &self.to_lua_table(lua));
        let contents = name_table("PluginInfo", &table);

        Ok(fs::write(file, contents)?)
    }

    fn update_build_version(version: String, increment: VersionType) -> String {
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
    fn sync_version_with_build_version(&mut self) {
        let version_parts: Vec<&str> = self.build_version.split('.').collect();
        if version_parts.len() >= 2 {
            self.version = format!("{}.{}", version_parts[0], version_parts[1]);
        } else {
            self.version = self.build_version.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::assets::INFO_LUA;

    use super::*;

    #[test]
    fn test_get_info() {
        let lua = Lua::new();
        let info_file = INFO_LUA.clone().unwrap();
        PluginInfo::from_file(&info_file, &lua).expect("info file not found");
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
        let info = PluginInfo::from_file(&INFO_LUA.clone().unwrap(), &lua).unwrap();
        let updated = info.clone().update_version(VersionType::Patch).unwrap();

        assert_ne!(&updated.build_version, &info.build_version);
        //Updated this to remain the same once created.
        assert_eq!(&updated.id, &info.id);
    }
    #[test]
    fn test_write_info() {
        let lua = Lua::new();
        let info_path = INFO_LUA.clone().unwrap();
        let info = PluginInfo::from_file(&info_path.to_path_buf(), &lua).unwrap();
        info.clone()
            .write_to_file(
                Path::new("./Api/plugin_src/test_info.lua").to_path_buf(),
                &lua,
            )
            .unwrap();
    }
}
