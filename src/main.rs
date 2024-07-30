use std::path::Path;
use std::{fs, ops::Not};

use clap::{arg, Arg, ArgAction, Command};
use git2::Repository;

fn cli() -> Command {
    Command::new("qplug")
        .about("Q-Sys plugin Development tool.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new")
                .about("Create a new plugin template.")
                .arg(arg!(<str> "Name of the plugin."))
                .arg_required_else_help(true)
                .arg(
                    Arg::new("Enable Git")
                        .long("no-git")
                        .default_value("false")
                        .action(ArgAction::SetTrue),
                ),
        )
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

fn create_plugin(name: &String, no_git: &bool) {
    // Setup plugin path
    let root_path = Path::new(name);
    let plugin_path = root_path.join("plugin_src");

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
    }
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let name = sub_matches.get_one::<String>("str").unwrap();
            let no_git = sub_matches.get_one::<bool>("Enable Git").unwrap();
            create_plugin(name, no_git);
            println!(
                "New plugin created: {}",
                sub_matches.get_one::<String>("str").unwrap()
            );
        }
        _ => unreachable!(),
    }
}
