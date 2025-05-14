use std::path::PathBuf;

use include_dir::{include_dir, Dir};

use crate::files::find_file_recursively;

/// Builtin Templates
pub static TEMPLATE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/assets/pluginframework");
/// Builtin Definitions
pub static DEFINITIONS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/assets/definitions");
/// Q-plug directory relative to the config directory: Should be used with
/// ```rust
/// use qplug::files::user_config;
/// use qplug::globals::QPLUG_DIR;
///
/// user_config().join(QPLUG_DIR);
/// ```
pub static QPLUG_DIR: &'static str = "qplug/";
/// Qplug configuration file name.
pub static QPLUG_CONFIG: &'static str = "qplug.lua";

//INFO: This can recurse into the parent dirctory and modify the template `info.lua`. Test from
//outside this project.
pub static INFO_LUA: std::sync::LazyLock<Option<PathBuf>> = std::sync::LazyLock::new(|| {
    find_file_recursively(
        std::env::current_dir()
            .expect("Unable to get current dir")
            .as_path(),
        "info.lua",
    )
});

/// The entrypoint for a Q-Sys plugin
pub static INIT_LUA: std::sync::LazyLock<Option<PathBuf>> = std::sync::LazyLock::new(|| {
    find_file_recursively(
        std::env::current_dir()
            .expect("Unable to get current dir")
            .as_path(),
        "init.lua",
    )
});
