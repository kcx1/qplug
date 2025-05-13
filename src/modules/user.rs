use std::ops::Index;
use std::path::PathBuf;
use std::{
    fs,
    io::{self, Write},
    ops::IndexMut,
};

use crate::config::Config;
use crate::globals::QPLUG_CONFIG;

use anyhow::Context;
use mlua::Table;
use mlua::{
    Lua,
    Value::{self, Nil},
};
use serde::Serialize;
use uuid::Uuid;

use crate::lua::info::PluginInfo;

use super::config::find_config_file;
use super::files::{find_project_dir, pwd, MARKER_FILE};

pub struct UserEnv<'a> {
    pub lua: &'a Lua,
    pub config: &'a Config<'a>,
}

#[derive(Serialize, Debug, Clone)]
pub struct UserConfig {
    pub build_tool: Value, // default to built-in
    pub encryption_tool: Value,
    pub external_template: Value, // can be path or url - default to built-in template
    pub me: Value,
    pub plugin_dir: Value,
}

impl IndexMut<&'_ str> for UserConfig {
    fn index_mut(&mut self, idx: &str) -> &mut Self::Output {
        match idx {
            "build_tool" => &mut self.build_tool,
            "encryption_tool" => &mut self.encryption_tool,
            "external_template" => &mut self.external_template,
            "me" => &mut self.me,
            "plugin_dir" => &mut self.plugin_dir,
            _ => panic!("Unknown field {}", idx),
        }
    }
}

impl Index<&'_ str> for UserConfig {
    type Output = Value;

    fn index(&self, idx: &str) -> &Value {
        match idx {
            "build_tool" => &self.build_tool,
            "encryption_tool" => &self.encryption_tool,
            "external_template" => &self.external_template,
            "me" => &self.me,
            "plugin_dir" => &self.plugin_dir,
            _ => panic!("Unknown field {}", idx),
        }
    }
}

impl UserConfig {
    pub fn new(lua: &Lua) -> anyhow::Result<UserConfig> {
        let user_config = match find_config_file(QPLUG_CONFIG) {
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

pub fn get_user_info(
    name: &String,
    existing_info: Option<PluginInfo>,
    config: &Config,
) -> anyhow::Result<PluginInfo> {
    match existing_info {
        Some(config) => Ok(config),
        None => {
            // Author Name
            let author = match &config.me.name {
                // Get name from config file
                Some(name) => name.to_owned(),
                // If not set in config file, ask user
                None => {
                    let mut author = String::new();
                    println!("Enter your name: ");
                    io::stdin()
                        .read_line(&mut author)
                        .context("Oops, Could not read your name.")?;
                    author
                }
            };

            // Description
            io::stdout().flush()?;
            let mut description = String::new();
            println!("Enter a description for your plugin: ");
            io::stdin()
                .read_line(&mut description)
                .context("Oops, Could not read description.")?;

            Ok(PluginInfo {
                name: name.to_string(),
                version: "0.0.0.0".to_string(),
                build_version: "0.0.0.0".to_string(),
                id: Uuid::new_v4().to_string(),
                author: author.trim().to_string(),
                description: description.trim().to_string(),
            })
        }
    }
}
