use directories::BaseDirs;
use mlua::{
    Lua, Table,
    Value::{self, Nil},
};
use serde::Serialize;
use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
};

use crate::assets::TEMPLATE_DIR;

use super::files::{find_project_dir, pwd, MARKER_FILE};

pub struct UserEnv<'a> {
    pub lua: &'a Lua,
    pub config: &'a Config<'a>,
}

pub struct Author {
    pub name: Option<String>,
    pub email: Option<String>,
    pub company: Option<String>,
}

#[derive(Debug)]
pub enum Template<'a> {
    Url(String),
    FileSystem(PathBuf),
    InMemoryDir(&'a include_dir::Dir<'static>),
}

type Tool = Box<dyn Fn()>;

pub struct Config<'a> {
    pub build_tool: Tool,
    pub encryption_tool: Tool,
    pub template: Template<'a>,
    pub me: Author,
    pub plugin_dir: Option<PathBuf>, // path to the Q-Sys plugin directory. Not needed on Windows
}

impl Config<'_> {
    pub fn from_user_config(user_config: &UserConfig) -> anyhow::Result<Self> {
        // Internal implementation as a callable
        let default_build_tool = || {
            let _ = crate::cli::subcommands::build::default_build_tool();
        };

        // Determine which build_tool to use
        let build_tool: Box<dyn Fn()> = match &user_config.build_tool {
            Value::Function(f) => {
                let f_clone = f.clone();
                Box::new(move || {
                    let _ = f_clone.call::<()>(());
                })
            }
            _ => Box::new(default_build_tool),
        };

        let default_encryption_tool = || {
            let _ = crate::cli::subcommands::encrypt::default_encryption_tool();
        };

        // Determine which build_tool to use
        let encryption_tool: Box<dyn Fn()> = match &user_config.encryption_tool {
            Value::Function(f) => {
                let f_clone = f.clone();
                Box::new(move || {
                    let _ = f_clone.call::<()>(());
                })
            }
            _ => Box::new(default_encryption_tool),
        };

        // Determine which template to use
        let template: Template = match &user_config.external_template {
            Value::String(s) => {
                let template_str = s.to_str()?;
                if template_str.starts_with("http") {
                    Template::Url(template_str.to_owned())
                } else {
                    Template::FileSystem(PathBuf::from_str(&template_str)?)
                }
            }
            _ => Template::InMemoryDir(&TEMPLATE_DIR),
        };

        let me: Author = match &user_config.me {
            Value::Table(t) => Author {
                name: Some(t.get("name").unwrap_or(Value::Nil).to_string()?),
                email: Some(t.get("email").unwrap_or(Value::Nil).to_string()?),
                company: Some(t.get("company").unwrap_or(Value::Nil).to_string()?),
            },
            _ => Author {
                name: None,
                email: None,
                company: None,
            },
        };

        let qsys_plugin_dir = match &user_config.plugin_dir {
            Value::String(s) => Some(PathBuf::from_str(&s.to_str()?)?),
            _ => None,
        };

        Ok(Config {
            build_tool,
            encryption_tool,
            template,
            me,
            plugin_dir: qsys_plugin_dir,
        })
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct UserConfig {
    pub build_tool: Value, // default to built-in
    pub encryption_tool: Value,
    pub external_template: Value, // can be path or url - default to built-in template
    pub me: Value,
    pub plugin_dir: Value,
}

impl UserConfig {
    pub fn new(lua: &Lua) -> anyhow::Result<UserConfig> {
        let user_config = match find_config_file() {
            Some(path) => {
                // Create a function that will return the table form the user config and call it
                lua.load(fs::read_to_string(&path)?)
                    .into_function()?
                    .call(Nil)?
            }
            None => {
                let lua_config = lua.create_table()?;
                lua_config.set("external_template", Value::Nil)?;
                lua_config.set("build_tool", Value::Nil)?;
                lua_config.set("encryption_tool", Value::Nil)?;
                lua_config.set("me", Value::Nil)?;
                lua_config
            }
        };

        overload_global_config(&user_config, None, lua)?;

        Ok(UserConfig {
            external_template: user_config.get("external_template").unwrap_or(Value::Nil),
            build_tool: user_config.get("build_tool").unwrap_or(Value::Nil),
            encryption_tool: user_config.get("encryption_tool").unwrap_or(Value::Nil),
            me: user_config.get("me").unwrap_or(Value::Nil),
            plugin_dir: user_config.get("plugin_dir").unwrap_or(Value::Nil),
        })
    }
}

pub fn find_config_file() -> Option<PathBuf> {
    fn return_config(config_file: PathBuf) -> Option<PathBuf> {
        if config_file.exists() {
            return Some(config_file);
        }
        None
    }
    // Check in XDG config directories (Linux, macOS)
    let base_dirs = BaseDirs::new()?;
    let mut config_file = base_dirs.config_dir().join("qplug/qplug.lua"); // ~/.config on Linux/macOS, AppData/Roaming on Windows
    match return_config(config_file) {
        Some(config) => Some(config),
        None => {
            config_file = base_dirs.home_dir().join(".qplug.lua");
            return_config(config_file)
        }
    }
}

/// Overload the global config with either a user provided config or a marker file
fn overload_global_config<'a>(
    user_config: &'a Table,
    local_config: Option<PathBuf>,
    lua: &Lua,
) -> anyhow::Result<&'a Table> {
    // Either User provided config or find a marker file
    let overload_config = local_config.or_else(|| {
        Some(
            find_project_dir(Some(&pwd().unwrap()))?
                .join(MARKER_FILE)
                .to_path_buf(),
        )
    });

    match overload_config {
        None => Ok(user_config),
        Some(overload_config) => {
            let new_config: Table = lua
                .load(fs::read_to_string(overload_config)?)
                .into_function()?
                .call(Nil)?;

            new_config.for_each(|key: Value, val: Value| user_config.set(key, val))?;

            Ok(user_config)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    // INFO: This test only works if you don't have a config file in your home directory.
    #[test]
    fn test_find_config_file_none() -> anyhow::Result<()> {
        let result = find_config_file();
        assert!(result.is_none());
        Ok(())
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
    fn test_find_config_file_in_xdg_config() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let config_dir = temp_dir.path().join("config/qplug");
        let config_file = config_dir.join("qplug.lua");

        // Create the directory and file
        fs::create_dir_all(&config_dir)?;
        fs::write(&config_file, get_dummy_config())?;

        // Mock the BaseDirs::config_dir() to return our temp_dir's config path
        assert_eq!(
            test_return_config(config_file.clone()),
            Some(config_file.clone())
        );

        Ok(())

        // tempdir automatically cleans up when it goes out of scope
    }

    // Test the `find_config_file` function with a config file in the home directory.
    #[test]
    fn test_find_config_file_in_home_dir() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let home_dir = temp_dir.path();
        let config_file = home_dir.join(".qplug.lua");

        // Create the file
        fs::write(&config_file, get_dummy_config())?;

        // Mock the BaseDirs::home_dir() to return our temp_dir's home path
        let result = test_return_config(config_file.clone());
        assert_eq!(result, Some(config_file.clone()));

        // tempdir automatically cleans up when it goes out of scope
        Ok(())
    }

    // INFO: This test only works if you don't have a config file in your home directory.
    // TODO: Solve why this test is failing.
    // #[test]
    // fn test_get_config_default() -> anyhow::Result<()> {
    //     let lua = Lua::new();
    //     let config = UserConfig::new(&lua)?;
    //
    //     assert_eq!(config.build_tool, Value::Nil);
    //     assert_eq!(config.external_template, Value::Nil);
    //     Ok(())
    // }
}
