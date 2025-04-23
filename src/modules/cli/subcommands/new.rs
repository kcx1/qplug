use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use git2::Repository;
use uuid::Uuid;

use crate::{
    assets::DEFINITIONS_DIR,
    config::{Config, Template, UserEnv},
    files::{self, copy_dir, create_marker_file},
    lua::info::PluginInfo,
};

const PLUGIN_ROOT: &str = "plugin_src";

pub fn create_plugin(
    name: Option<&String>,
    no_git: &bool,
    no_template: &bool,
    no_defs: &bool,
    user_env: UserEnv,
) -> anyhow::Result<()> {
    // Check if name was provided - if not set name to parent directory
    let file_name: &String = match name {
        Some(name) => {
            if Path::exists(Path::new(&name))
                || Path::exists(&Path::new(&name).join(PLUGIN_ROOT))
                || Path::exists(Path::new(".qplug"))
            {
                // Fail if the plugin already exists.
                return Err(anyhow::anyhow!("Plugin already exists"));
            }
            name
        }
        None => {
            if Path::exists(&Path::new(".").join(PLUGIN_ROOT)) || Path::exists(Path::new(".qplug"))
            {
                // Fail if the plugin already exists.
                return Err(anyhow::anyhow!("Plugin already exists"));
            }
            &".".to_string()
        }
    };
    // Setup plugin path
    let root_path = Path::new(&file_name);
    let plugin_path = root_path.join(PLUGIN_ROOT);

    // Create plugin directories
    fs::create_dir_all(&plugin_path)
        .context("Failed to create plugin directories. Some may already exist")?;

    // fetch the template based on the user's config. Default to internal template if none set.
    fetch_template(&plugin_path, &user_env.config.template)?;

    if !no_template {
        fs::create_dir_all(&plugin_path)
            .context("Failed to create template. Some of the directories may already exist.")?;
        fetch_template(plugin_path.as_path(), &user_env.config.template)?;
        println!("Template initialized");
    }

    if !no_defs {
        add_lua_defs(root_path).context("Failed to create lua definitions.")?;
        println!("Definitions initialized");
    }

    // Init git repo
    if !no_git {
        init_git(root_path).context("Failed to init git repo")?;
        println!("Git initialized");
    }

    // If name was set as a path, use the last part
    let plugin_name = &file_name
        .split('/')
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .to_owned()
        .to_string();

    // Print Creation Confirmation
    if plugin_name == "." {
        println!(
            "New plugin created: {:?}",
            std::env::current_dir()?.file_name().unwrap()
        );
    } else {
        println!("New plugin created: {}", plugin_name);
    }

    // Write the info.lua file
    let info_lua_file = files::find_file_recursively(&plugin_path, "info.lua");
    let info = get_user_info(plugin_name, None, user_env.config)?;
    info.write_to_file(info_lua_file.unwrap(), user_env.lua)
        .context("Failed to write info.lua")?;

    create_marker_file(root_path)?;
    Ok(())
}

pub fn fetch_template(plugin_dir: &Path, template: &Template) -> anyhow::Result<PathBuf> {
    // let url = "https://github.com/qsys-plugins/BasePlugin";
    match template {
        Template::Url(s) => Ok(Repository::clone(s, plugin_dir)
            .context("Failed to clone")?
            .path()
            .to_path_buf()),
        Template::FileSystem(_) => {
            copy_dir(template, plugin_dir)
                .with_context(|| format!("Failed to copy {:?} to {:?}.", template, plugin_dir))?;
            Ok(plugin_dir.to_path_buf())
        }
        Template::InMemoryDir(_) => {
            copy_dir(template, plugin_dir).context("Failed to copy built-in template")?;
            Ok(plugin_dir.to_path_buf())
        }
    }
}

fn get_user_info(
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

pub fn add_lua_defs(root_path: &Path) -> anyhow::Result<()> {
    // Add Lua Defs
    let defs_path = root_path.join("definitions");
    fs::create_dir(&defs_path).context("Directory creation failed.")?;
    copy_dir(&Template::InMemoryDir(&DEFINITIONS_DIR), &defs_path)
        .with_context(|| format!("Failed to copy {:?} to {:?}", DEFINITIONS_DIR, &defs_path))?;
    Ok(())
}

pub fn init_git(path: &Path) -> anyhow::Result<Repository> {
    Repository::init(path).context("Failed to initialize local git repo")
}
