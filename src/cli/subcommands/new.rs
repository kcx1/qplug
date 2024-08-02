use std::path::Path;
use std::{fs, ops::Not};

use git2::Repository;

pub fn create_plugin(name: &String, no_git: &bool) {
    // Setup plugin path
    let root_path = Path::new(name);
    let plugin_path = root_path.join("plugin_src");

    if Path::exists(root_path) || Path::exists(&plugin_path) {
        println!("The plugin already exists.");
    } else {
        // Create plugin directories
        fs::create_dir_all(&plugin_path).expect("Directory creation failed.");
        // Download the Base Plugin Example
        fetch_template(&plugin_path);
        // Get rid of unneeded files and directories
        delete_uneeded_files_and_directories(
            &plugin_path,
            vec![
                ".git",
                ".vscode",
                ".gitmodules",
                "PluginCompile",
                "README.md",
            ],
        );

        // Init git repo
        if no_git.not() {
            init_git(root_path);
            println!("Git initialized");
        }
        println!("New plugin created: {}", name);
    }
}

fn fetch_template(path: &Path) -> Repository {
    let url = "https://github.com/qsys-plugins/BasePlugin";
    match Repository::clone(url, path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    }
}

fn delete_uneeded_files_and_directories(path: &Path, dirs: Vec<&str>) {
    for dir in dirs {
        // Remove Directories
        if path.join(dir).is_dir() {
            match fs::remove_dir_all(path.join(dir)) {
                Ok(_) => (),
                Err(e) => panic!("Failed to remove directory: {}. Error: {}", dir, e),
            }
        }
        // Remove Files
        else if path.join(dir).is_file() {
            match fs::remove_file(path.join(dir)) {
                Ok(_) => (),
                Err(e) => panic!("Failed to remove file: {}. Error: {}", dir, e),
            }
        }
    }
}

fn init_git(path: &Path) -> Repository {
    match Repository::init(path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to initialize git repo: {}", e),
    }
}
