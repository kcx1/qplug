use std::path::PathBuf;

use crate::config::Config;
use directories;

use crate::files::find_project_dir;

#[allow(dead_code)]
struct FileInfo {
    name: String,
    path: PathBuf,
}

fn get_compiled_file() -> FileInfo {
    let marker_file = find_project_dir(None).expect("You might not be in a plugin directory.");
    let file_name = marker_file
        .file_name()
        .expect("Compiled qplug file not found. Please build or compile it first.")
        .to_string_lossy();

    FileInfo {
        name: file_name.to_string(),
        path: marker_file.join(format!("{file_name}.qplug")),
    }
}

#[allow(dead_code)]
fn get_qsys_plugin_dir() -> PathBuf {
    let user_dir = directories::UserDirs::new().expect("Unable to locate user dir.");
    let docs = user_dir.document_dir().expect("Unable to locate docs dir.");
    docs.join("QSC").join("Q-Sys Designer").join("Plugins")
}

fn copy_files(source_file: FileInfo, destination: &PathBuf) -> Result<u64, std::io::Error> {
    println!(
        "Copying from {} to {}",
        source_file.path.display(),
        destination.display()
    );
    std::fs::copy(source_file.path, destination)
}

pub fn copy_to_plugin_directory(
    config: &Config,
    copy_path: Option<&String>,
) -> Result<u64, std::io::Error> {
    let source_file = get_compiled_file();
    match copy_path {
        // A path is passed as an argument
        Some(destination) => copy_files(source_file, &PathBuf::from(destination)),
        // No path is passed as an argument
        None => {
            #[cfg(windows)]
            {
                // Try using a path declared in the config. Otherwise, use to the Q-Sys Designer plugin directory
                let destination = match &config.plugin_dir {
                    Some(destination) => destination,
                    None => &get_qsys_plugin_dir().join(format!("{}.qplug", source_file.name)),
                };
                copy_files(source_file, &destination);
            }
            #[cfg(not(windows))]
            {
                // Try useing a path declared in the config. Otherwise, let the user know it won't work.
                match &config.plugin_dir {
                    Some(destination) => copy_files(source_file, destination),
                    _ => {
                        println!("Not on windows , not copying to plugin directory");
                        Ok(0)
                    }
                }
            }
        }
    }
}
