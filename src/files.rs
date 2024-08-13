use std::{
    fs,
    path::{Component, Path, PathBuf},
};

const MARKER_FILE: &str = ".qplug";

pub fn copy_dir(source: &Path, dest: &Path, filter: Option<&str>) {
    // Copy a directory recursively from the source to the dest. Optional filter to remove
    // unwanted parent directory from the destination.

    let entries = fs::read_dir(source).expect("Unable to read directory");

    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_dir() {
            copy_dir(&path, dest.join(&path).as_path(), None);
        } else {
            let file_parts = dest.components().filter(|name| match filter {
                Some(s) => *name != Component::Normal(s.as_ref()),
                None => true,
            });
            let new_dest = PathBuf::from_iter(file_parts);
            fs::create_dir_all(&new_dest).expect("Create dir failed.");
            fs::copy(&path, &new_dest.join(path.file_name().unwrap())).unwrap_or_else(|err| {
                panic!(
                    "Error copying {:?} to {:?}: {:?}",
                    &path,
                    new_dest.join(path.file_name().unwrap()),
                    err
                )
            });
        }
    }
}

pub fn create_marker_file(root_path: &Path) {
    fs::write(root_path.join(MARKER_FILE), "").expect("Failed to write marker file");
}

pub fn find_marker_file(starting_dir: &Path) -> Option<PathBuf> {
    let mut current_dir = starting_dir;

    loop {
        if current_dir.join(MARKER_FILE).exists() {
            return Some(current_dir.to_path_buf());
        }

        match current_dir.parent() {
            Some(parent) => current_dir = parent,
            None => break,
        }
    }

    None
}
