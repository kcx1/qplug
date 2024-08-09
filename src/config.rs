use directories::BaseDirs;
use mlua::{
    Lua,
    Value::{self},
};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize, Debug, Clone)]
pub struct Config<'lua> {
    pub build_tool: Value<'lua>,        // default to built-in
    pub external_template: Value<'lua>, // can be path or url - default to built-in template
}

fn return_config(config_file: PathBuf) -> Option<PathBuf> {
    if config_file.exists() {
        return Some(config_file);
    }
    None
}

fn find_config_file() -> Option<PathBuf> {
    // Check in XDG config directories (Linux, macOS)
    let base_dirs = BaseDirs::new().expect("No User Directory found");
    let mut config_file = base_dirs.config_dir().join("qplug/qplug.lua"); // ~/.config on Linux/macOS, AppData/Roaming on Windows
    match return_config(config_file) {
        Some(config) => Some(config),
        None => {
            config_file = base_dirs.home_dir().join(".qplug.lua");
            return_config(config_file)
        }
    }
}

pub fn get_config(lua: &Lua) -> Config<'_> {
    match find_config_file() {
        Some(path) => {
            todo!("Deserialize Config from {:?}", path);
        }
        None => {
            let lua_config = lua.create_table().expect("Table creation failed");
            lua_config.set("external_template", "Built-in").unwrap();
            lua_config.set("build_tool", Value::Nil).unwrap();

            let external_template = lua_config.get("external_template").unwrap_or(Value::Nil);
            let build_tool = lua_config.get("build_tool").unwrap_or(Value::Nil);

            Config {
                external_template,
                build_tool,
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempdir::TempDir;

    // Test the `find_config_file` function with no config file present.
    #[test]
    fn test_find_config_file_none() {
        let result = find_config_file();
        assert!(result.is_none());
    }

    fn get_dummy_config() -> String {
        r#"
                return {
                    -- Set to a string if you want an external template. Can be a url or a path.
                    external_template = nil,

                    -- Which build tool to use. This can be a string or a function.
                    build_tool = function()
                        local cmd = ".\\plugincompile|PLUGCC.exe . .\\plugin.lua"
                        os.execute(cmd)
                    end,
                }
           "#
        .to_string()
    }

    // Test the `find_config_file` function with a config file in the XDG config directory.
    #[test]
    fn test_find_config_file_in_xdg_config() {
        let temp_dir = TempDir::new("test").unwrap();
        let config_dir = temp_dir.path().join("config/qplug");
        let config_file = config_dir.join("qplug.lua");

        // Create the directory and file
        fs::create_dir_all(&config_dir).unwrap();
        fs::write(&config_file, get_dummy_config()).unwrap();

        // Mock the BaseDirs::config_dir() to return our temp_dir's config path
        let result = return_config(config_file.clone());
        assert_eq!(result, Some(config_file.clone()));

        // TempDir automatically cleans up when it goes out of scope
    }

    // Test the `find_config_file` function with a config file in the home directory.
    #[test]
    fn test_find_config_file_in_home_dir() {
        let temp_dir = TempDir::new("test").unwrap();
        let home_dir = temp_dir.path();
        let config_file = home_dir.join(".qplug.lua");

        // Create the file
        fs::write(&config_file, get_dummy_config()).unwrap();

        // Mock the BaseDirs::home_dir() to return our temp_dir's home path
        let result = return_config(config_file.clone());
        assert_eq!(result, Some(config_file.clone()));

        // TempDir automatically cleans up when it goes out of scope
    }

    // Test the `get_config` function when no config file is found (default values).
    #[test]
    fn test_get_config_default() {
        let lua = Lua::new();
        let config = get_config(&lua);

        assert_eq!(config.build_tool, Value::Nil);
        assert_eq!(
            config.external_template,
            Value::String(lua.create_string("Built-in").unwrap())
        );
    }
}
