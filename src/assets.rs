use std::path::PathBuf;

use include_dir::{include_dir, Dir};

use crate::files::find_file_recursively;

pub static TEMPLATE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/assets/pluginframework");
pub static DEFINITIONS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/assets/definitions");

//FIXME: This can recurse into the parent directory when creating a plugin, and grab the wrong
//info.lua file. If only used outside of the 'new' function - it's okay.
pub static INFO_LUA: std::sync::LazyLock<Option<PathBuf>> = std::sync::LazyLock::new(|| {
    find_file_recursively(
        std::env::current_dir()
            .expect("Unable to get current dir")
            .as_path(),
        "info.lua",
    )
});

pub static INIT_LUA: std::sync::LazyLock<Option<PathBuf>> = std::sync::LazyLock::new(|| {
    find_file_recursively(
        std::env::current_dir()
            .expect("Unable to get current dir")
            .as_path(),
        "init.lua",
    )
});
