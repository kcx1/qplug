use directories;

use crate::files::find_marker_file;

#[cfg(windows)]
pub fn copy_to_plugin_directory() -> Result<u64, std::io::Error> {
    let user_dir = directories::UserDirs::new().expect("Unable to locate user dir.");
    let docs = user_dir.document_dir().expect("Unable to locate docs dir.");
    let plugin_dir = docs.join("QSC").join("Q-Sys Designer").join("Plugins");
    let marker_file = find_marker_file(None).expect("You might not be in a plugin directory.");

    println!("Copying plugin to {}", plugin_dir.display());

    let source_file = marker_file.join(format!("{:?}.lua", marker_file.file_name()));
    let destination = plugin_dir.join(format!("{:?}.lua", marker_file.file_name()));

    match source_file.exists() {
        true => std::fs::copy(source_file, destination),
        false => {
            println!("Source file not found");
            Ok(0)
        }
    }
}

#[cfg(not(windows))]
pub fn copy_to_plugin_directory() -> Result<u64, std::io::Error> {
    println!("Not on windows, not copying to plugin directory");
    Ok(0)
}
