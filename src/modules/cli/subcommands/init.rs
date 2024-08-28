use std::fs;

use crate::{config::Config, files::create_marker_file};

use super::new::{add_lua_defs, fetch_template, init_git};

pub fn init_plugin(no_template: bool, no_git: bool, no_defs: bool, config: &Config) {
    let pwd =
        std::env::current_dir().expect("Unable to get current dir. Please check permissions.");
    let plugin_path = pwd.join("plugin_src");

    create_marker_file(pwd.as_path());

    if !no_template {
        fs::create_dir_all(&plugin_path).expect("Directory creation failed.");
        fetch_template(plugin_path.as_path(), &config.template);
    }
    if !no_git {
        init_git(pwd.as_path());
    }
    if !no_defs {
        add_lua_defs(pwd.as_path())
    }
}
