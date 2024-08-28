use directories;

use crate::files::find_marker_file;

#[cfg(windows)]
pub fn copy_to_plugin_directory() -> Result<u64, std::io::Error> {
    let user_dir = directories::UserDirs::new().expect("Unable to locate user dir.");
    let docs = user_dir.document_dir().expect("Unable to locate docs dir.");
    let plugin_dir = docs.join("QSC/Q-Sys Designer/Plugins");
    let marker_file = find_marker_file(None).expect("You might not be in a plugin directory.");

    std::fs::copy(
        marker_file.join(format!("{:?}.lua", marker_file.file_name())),
        plugin_dir,
    )
}

#[cfg(not(windows))]
pub fn copy_to_plugin_directory() {
    //no-op
}
