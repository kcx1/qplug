use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::exit,
};

use git2::Repository;
use mlua::Lua;
use uuid::Uuid;

use crate::{
    assets::{DEFINITIONS_DIR, INFO_LUA},
    config::{Config, Template},
    files::{copy_dir, create_marker_file},
    lua::info::PluginInfo,
};

const PLUGIN_ROOT: &str = "plugin_src";

pub fn create_plugin(name: &String, no_git: &bool, lua: &Lua, config: &Config) {
    // Setup plugin path
    let root_path = Path::new(name);
    let plugin_path = root_path.join(PLUGIN_ROOT);

    // Fail if the plugin already exists.
    if Path::exists(root_path) || Path::exists(&plugin_path) {
        eprint!("The plugin already exists.");
        exit(1);
    }
    // Create plugin directories
    fs::create_dir_all(&plugin_path).expect("Directory creation failed.");

    // fetch the template based on the user's config. Default to internal template if none set.
    fetch_template(&plugin_path, &config.template);

    // Init git repo
    if !no_git {
        init_git(root_path);
        println!("Git initialized");
    }
    println!("New plugin created: {}", name);

    // If name was set as a path, use the last part
    let plugin_name: &String = &name
        .split('/')
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .to_owned()
        .to_string();

    // Write the info.lua file
    let info = get_user_info(plugin_name, None, config);
    info.write_to_file(INFO_LUA.clone().unwrap(), lua)
        .expect("Failed to write info.lua");

    create_marker_file(root_path);
    add_lua_defs(root_path);
}

//TODO: Cleanup signature - Returns not currently being used.
pub fn fetch_template(path: &Path, template: &Template) -> PathBuf {
    // path = path to plugin dir
    // let url = "https://github.com/qsys-plugins/BasePlugin";
    match template {
        Template::Url(s) => match Repository::clone(s, path) {
            Ok(repo) => repo.path().to_path_buf(),
            Err(e) => panic!("Failed to clone: {}", e),
        },
        Template::FileSystem(_) => {
            copy_dir(template, path).expect("Failed to copy user template");
            path.to_path_buf()
        }
        Template::InMemoryDir(_) => {
            copy_dir(template, path).expect("Failed to copy built-in template");
            path.to_path_buf()
        }
    }
}

fn get_user_info(name: &String, existing_info: Option<PluginInfo>, config: &Config) -> PluginInfo {
    match existing_info {
        Some(config) => config,
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
                        .expect("Oops, Could not read your name.");
                    author
                }
            };

            // Description
            io::stdout().flush().unwrap();
            let mut description = String::new();
            println!("Enter a description for your plugin: ");
            io::stdin()
                .read_line(&mut description)
                .expect("Oops, Could not read description.");

            PluginInfo {
                name: name.to_string(),
                version: "0.0.0.0".to_string(),
                build_version: "0.0.0.0".to_string(),
                id: Uuid::new_v4().to_string(),
                author: author.trim().to_string(),
                description: description.trim().to_string(),
            }
        }
    }
}

pub fn add_lua_defs(root_path: &Path) {
    // Add Lua Defs
    let defs_path = root_path.join("definitions");
    fs::create_dir(&defs_path).expect("Directory creation failed.");
    copy_dir(&Template::InMemoryDir(&DEFINITIONS_DIR), &defs_path)
        .expect("Failed to copy definitions.");
}

pub fn init_git(path: &Path) -> Repository {
    match Repository::init(path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to initialize git repo: {}", e),
    }
}
