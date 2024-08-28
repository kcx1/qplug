use crate::{files::find_marker_file, lua::parser::merge_lua_files};

pub fn compile() {
    let marker = find_marker_file(None);
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
