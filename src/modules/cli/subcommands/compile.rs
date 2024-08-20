use std::env;

use crate::{files::find_marker_file, lua::parser::merge_lua_files};

pub fn compile() {
    let current_path = env::current_dir()
        .expect("Unable to get current directory. Please check your permissions.");
    let marker = find_marker_file(current_path.as_path());
    if marker.is_some() {
        let root_path = marker.unwrap();
        let plugin_path = root_path.join("plugin_src");
        match merge_lua_files(root_path, plugin_path) {
            Ok(_) => println!("Plugin updated successfully."),
            Err(e) => println!("Failed to update plugin: {}", e),
        }
    } else {
        println!(
            "No plugin found. Please create a plugin first or navigate to a plugin directory."
        );
    }
}
