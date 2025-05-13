use crate::modules::template::Template;
use crate::modules::user::UserEnv;
use crate::modules::{git::init_git, template::create_template};
use std::fs;
use std::path::Path;

use anyhow::Context;

use crate::{
    files::{self, copy_dir, create_marker_file},
    globals::DEFINITIONS_DIR,
    modules::user::get_user_info,
};

const PLUGIN_ROOT: &str = "plugin_src";

pub fn create_plugin(
    name: Option<&String>,
    no_git: &bool,
    no_template: &bool,
    local_defs: &bool,
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

    if !no_template {
        create_template(&plugin_path, &user_env)?
    }

    if *local_defs {
        add_lua_defs(root_path).context("Failed to create lua definitions.")?;
        println!("Definitions initialized");
    } else {
        //TODO: Write the luals.json file
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

pub fn add_lua_defs(root_path: &Path) -> anyhow::Result<()> {
    // Add Lua Defs
    let defs_path = root_path.join("definitions");
    fs::create_dir(&defs_path).context("Directory creation failed.")?;
    //TODO: Check to see if the definition folder has been downloaded. If so, prefer that.
    copy_dir(&Template::InMemoryDir(&DEFINITIONS_DIR), &defs_path)
        .with_context(|| format!("Failed to copy {:?} to {:?}", DEFINITIONS_DIR, &defs_path))?;
    Ok(())
}
