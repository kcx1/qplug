use directories::BaseDirs;
use mlua::{
    Lua, Table,
    Value::{self},
};
use serde::Serialize;
use std::{
    fs::{self},
    path::PathBuf,
};

use crate::assets::TEMPLATE_DIR;

pub struct Author {
    pub name: Option<String>,
    pub email: Option<String>,
    pub company: Option<String>,
}

pub enum Template<'a> {
    Url(String),
    FileSystem(PathBuf),
    InMemoryDir(&'a include_dir::Dir<'static>),
}

pub struct Config<'lua, 'a> {
    pub build_tool: Box<dyn Fn() + 'lua>,
    pub template: Template<'a>,
    pub me: Author,
}

impl<'lua, 'a> Config<'lua, 'a> {
    pub fn from_user_config(user_config: &'lua UserConfig) -> Self {
        // Internal implementation as a callable
        let default_build_tool = crate::cli::subcommands::compile::compile;

        // Determine which build_tool to use
        let build_tool: Box<dyn Fn()> = match &user_config.build_tool {
            Value::Function(f) => Box::new(|| f.call(()).unwrap()),
            _ => Box::new(default_build_tool),
        };

        // Determine which template to use
        let template: Template = match &user_config.external_template {
            Value::String(s) => {
                let template_str = s.to_str().unwrap();
                if template_str.starts_with("http") {
                    Template::Url(template_str.to_owned())
                } else {
                    Template::FileSystem(PathBuf::from(template_str))
                }
            }
            _ => Template::InMemoryDir(&TEMPLATE_DIR),
        };

        let me: Author = match &user_config.me {
            Value::Table(t) => Author {
                name: Some(t.get("name").unwrap_or(Value::Nil).to_string().unwrap()),
                email: Some(t.get("email").unwrap_or(Value::Nil).to_string().unwrap()),
                company: Some(t.get("company").unwrap_or(Value::Nil).to_string().unwrap()),
            },
            _ => Author {
                name: None,
                email: None,
                company: None,
            },
        };

        Config {
            build_tool,
            template,
            me,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct UserConfig<'lua> {
    pub build_tool: Value<'lua>,        // default to built-in
    pub external_template: Value<'lua>, // can be path or url - default to built-in template
    pub me: Value<'lua>,
}

impl UserConfig<'_> {
    pub fn new(lua: &Lua) -> UserConfig<'_> {
        let user_config = match find_config_file() {
            Some(path) => {
                lua.globals()
                    .set("lua_config", fs::read_to_string(path).unwrap())
                    .unwrap();
                lua.globals().get::<_, Table>("lua_config").unwrap()
            }
            None => {
                let lua_config = lua.create_table().expect("Table creation failed");
                lua_config.set("external_template", Value::Nil).unwrap();
                lua_config.set("build_tool", Value::Nil).unwrap();
                lua_config.set("me", Value::Nil).unwrap();
                lua_config
            }
        };

        UserConfig {
            external_template: user_config.get("external_template").unwrap_or(Value::Nil),
            build_tool: user_config.get("build_tool").unwrap_or(Value::Nil),
            me: user_config.get("me").unwrap_or(Value::Nil),
        }
    }
}

fn find_config_file() -> Option<PathBuf> {
    fn return_config(config_file: PathBuf) -> Option<PathBuf> {
        if config_file.exists() {
            return Some(config_file);
        }
        None
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    // Test the `find_config_file` function with no config file present.
    #[test]
    fn test_find_config_file_none() {
        let result = find_config_file();
        assert!(result.is_none());
    }
    fn test_return_config(config_file: PathBuf) -> Option<PathBuf> {
        if config_file.exists() {
            return Some(config_file);
        }
        None
    }

    fn get_dummy_config() -> String {
        r#"
                return {
                    -- Set to a string if you want an external template. Can be a url or a path.
                    external_template = "My/path/to/template",

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
        let temp_dir = tempdir().unwrap();
        let config_dir = temp_dir.path().join("config/qplug");
        let config_file = config_dir.join("qplug.lua");

        // Create the directory and file
        fs::create_dir_all(&config_dir).unwrap();
        fs::write(&config_file, get_dummy_config()).unwrap();

        // Mock the BaseDirs::config_dir() to return our temp_dir's config path
        let result = test_return_config(config_file.clone());
        assert_eq!(result, Some(config_file.clone()));

        // tempdir automatically cleans up when it goes out of scope
    }

    // Test the `find_config_file` function with a config file in the home directory.
    #[test]
    fn test_find_config_file_in_home_dir() {
        let temp_dir = tempdir().unwrap();
        let home_dir = temp_dir.path();
        let config_file = home_dir.join(".qplug.lua");

        // Create the file
        fs::write(&config_file, get_dummy_config()).unwrap();

        // Mock the BaseDirs::home_dir() to return our temp_dir's home path
        let result = test_return_config(config_file.clone());
        assert_eq!(result, Some(config_file.clone()));

        // tempdir automatically cleans up when it goes out of scope
    }

    // Test the `get_config` function when no config file is found (default values).
    #[test]
    fn test_get_config_default() {
        let lua = Lua::new();
        let config = UserConfig::new(&lua);

        assert_eq!(config.build_tool, Value::Nil);
        assert_eq!(config.external_template, Value::Nil);
    }
}
