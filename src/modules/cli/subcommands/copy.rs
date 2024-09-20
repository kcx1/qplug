#![allow(unused_imports)]
use directories;

use crate::files::find_marker_file;

#[cfg(windows)]
pub fn copy_to_plugin_directory() -> Result<u64, std::io::Error> {
    let user_dir = directories::UserDirs::new().expect("Unable to locate user dir.");
    let docs = user_dir.document_dir().expect("Unable to locate docs dir.");
    let plugin_dir = docs.join("QSC").join("Q-Sys Designer").join("Plugins");
    let marker_file = find_marker_file(None).expect("You might not be in a plugin directory.");
    let file_name = marker_file
        .file_name()
        .expect("Compiled qplug file not found. Please build or compile it first.")
        .to_string_lossy();

    let source_file = marker_file.join(format!("{file_name}.qplug"));
    let destination = plugin_dir.join(format!("{file_name}.qplug"));

    println!(
        "Copying from {} to {}",
        source_file.display(),
        destination.display()
    );

    std::fs::copy(source_file, destination)
}

#[cfg(not(windows))]
pub fn copy_to_plugin_directory() -> Result<u64, std::io::Error> {
    println!("Not on windows, not copying to plugin directory");
    Ok(0)
}
