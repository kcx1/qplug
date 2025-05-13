use std::{
    fs, io,
    path::{Path, PathBuf},
};

use anyhow::Context;

use super::template::Template;

pub const MARKER_FILE: &str = ".qplug";

pub enum Entry {
    FileSystem(fs::DirEntry),
    InMemoryFile(include_dir::File<'static>),
    InMemoryDir(include_dir::Dir<'static>),
}

impl Entry {
    pub fn path(&self) -> PathBuf {
        match self {
            Entry::FileSystem(entry) => entry.path(),
            Entry::InMemoryFile(file) => PathBuf::from(file.path()),
            Entry::InMemoryDir(dir) => PathBuf::from(dir.path()),
        }
    }

    pub fn is_dir(&self) -> bool {
        match self {
            Entry::FileSystem(entry) => entry.path().is_dir(),
            Entry::InMemoryFile(_) => false,
            Entry::InMemoryDir(_) => true,
        }
    }
}

pub trait Extractable {
    fn extract(&self, dest: &Path) -> anyhow::Result<()>;
}

impl Extractable for include_dir::Dir<'_> {
    fn extract(&self, base_path: &Path) -> anyhow::Result<()> {
        Ok(include_dir::Dir::extract(self, base_path)?)
    }
}

impl Extractable for Path {
    fn extract(&self, dest: &Path) -> anyhow::Result<()> {
        if self.is_dir() {
            //Create the destination directory
            fs::create_dir_all(dest)?;

            for entry in fs::read_dir(self)? {
                let entry = entry?;
                let source_path = entry.path();
                let dest_path = dest.join(entry.file_name());

                if source_path.is_dir() {
                    source_path.extract(&dest_path)?;
                } else {
                    fs::copy(&source_path, &dest_path)?;
                }
            }
        } else if self.is_file() {
            fs::copy(self, dest)?;
        } else {
            return Err(anyhow::anyhow!(format!(
                "Not a file or a directory: {}",
                io::ErrorKind::InvalidInput
            )));
        };
        Ok(())
    }
}

impl Extractable for Template<'_> {
    fn extract(&self, dest: &Path) -> anyhow::Result<()> {
        match self {
            Template::FileSystem(p) => p.extract(dest),
            Template::InMemoryDir(d) => Ok(d.extract(dest)?),
            Template::Url(_) => unimplemented!(),
        }
    }
}

pub fn copy_dir(source: &Template, dest: &Path) -> anyhow::Result<()> {
    // Extract the contents from the source to the destination.
    source.extract(dest)
}

pub fn create_marker_file(root_path: &Path) -> anyhow::Result<()> {
    //TODO: Create some cache of other relevant files here such as path to the init.lua file. And
    //the path to the info.lua file for the project.
    fs::write(root_path.join(MARKER_FILE), "return {}")?;
    Ok(())
}

/// Search the parent directories for the marker file.
/// Starts from the given path.
pub fn find_project_dir(path: Option<&Path>) -> Option<PathBuf> {
    search_parents(MARKER_FILE, path)
    // match search_parents(MARKER_FILE, path) {
    //     Some(path) => Ok(path),
    //     None => Err(anyhow::anyhow!("Could not find marker file")),
    // }
}

/// Searches the parent directories for a file from the starting path.
/// Rturns the directory if the file is found, or None if not.
fn search_parents(file: &str, starting_path: Option<&Path>) -> Option<PathBuf> {
    let mut current_dir = match starting_path {
        Some(path) => path.into(),
        None => pwd().ok()?,
    };

    loop {
        if current_dir.join(file).exists() {
            return Some(current_dir.to_path_buf());
        }

        match current_dir.parent() {
            Some(parent) => current_dir = parent.into(),
            None => break,
        }
    }

    None
}

pub fn find_file_recursively(dir: &Path, file_name: &str) -> Option<PathBuf> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).ok()? {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(found) = find_file_recursively(&path, file_name) {
                    return Some(found);
                }
            } else if path.file_name().and_then(|name| name.to_str()) == Some(file_name) {
                return Some(path);
            }
        }
    }
    None
}

pub fn pwd() -> anyhow::Result<PathBuf> {
    std::env::current_dir()
        .context("Failed to get current directory. Please check your permissions.")
}

#[cfg(test)]
mod tests {
    use crate::globals::TEMPLATE_DIR;

    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_extract_path_to_existing_directory() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let src_dir = temp_dir.path().join("source");
        let dest_dir = temp_dir.path().join("dest");

        // Create source directory and files
        fs::create_dir(&src_dir)?;
        let src_file_path = src_dir.join("file.txt");
        let mut src_file = File::create(&src_file_path)?;
        writeln!(src_file, "Hello, world!")?;

        // Extract to destination directory
        src_dir.extract(&dest_dir)?;

        // Verify the file was copied
        let copied_file_content = fs::read_to_string(dest_dir.join("file.txt"))?;
        assert_eq!(copied_file_content, "Hello, world!\n");
        Ok(())
    }

    #[test]
    fn test_extract_path_to_non_existing_directory() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let src_dir = temp_dir.path().join("source");
        let dest_dir = temp_dir.path().join("non_existing_dest");

        // Create source directory and files
        fs::create_dir(&src_dir)?;
        let src_file_path = src_dir.join("file.txt");
        let mut src_file = File::create(&src_file_path)?;
        writeln!(src_file, "Hello, world!")?;

        // Extract to non-existing destination directory
        src_dir.extract(&dest_dir)?;

        // Verify the file was copied
        let copied_file_content = fs::read_to_string(dest_dir.join("file.txt"))?;
        assert_eq!(copied_file_content, "Hello, world!\n");
        Ok(())
    }

    #[test]
    fn test_extract_file_to_path() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let src_file_path = temp_dir.path().join("file.txt");
        let dest_file_path = temp_dir.path().join("copied_file.txt");

        // Create source file
        let mut src_file = File::create(&src_file_path)?;
        writeln!(src_file, "Hello, world!")?;

        // Extract file to destination path
        src_file_path.extract(&dest_file_path)?;

        // Verify the file was copied
        let copied_file_content = fs::read_to_string(dest_file_path)?;
        assert_eq!(copied_file_content, "Hello, world!\n");
        Ok(())
    }

    #[test]
    fn test_copy_dir_from_in_memory() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let dest_dir = temp_dir.path().join("in_memory_dest");
        std::fs::create_dir_all(&dest_dir)?;

        assert!(dest_dir.exists());

        // Extract in-memory directory to the destination
        copy_dir(&Template::InMemoryDir(&TEMPLATE_DIR), &dest_dir).expect("Copy failed");

        // Verify some expected file or directory exists in the destination
        let expected_file_path = TEMPLATE_DIR.get_entry("init.lua");
        assert!(expected_file_path.is_some());
        Ok(())
    }

    #[test]
    fn test_create_marker_file() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let root_path = temp_dir.path();

        // Create marker file
        create_marker_file(root_path)?;

        // Verify the marker file was created
        let marker_file_path = root_path.join(MARKER_FILE);
        assert!(marker_file_path.exists());

        Ok(())
    }

    #[test]
    fn test_find_marker_file() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let root_path = temp_dir.path();
        let nested_dir = root_path.join("nested");

        // Create nested directory and marker file in root
        fs::create_dir(&nested_dir)?;
        create_marker_file(root_path)?;

        // Find the marker file starting from the nested directory
        let found_marker = find_project_dir(Some(root_path));

        // Verify the correct directory was found
        assert_eq!(found_marker, Some(root_path.to_path_buf()));
        Ok(())
    }

    #[test]
    fn test_find_marker_file_not_found() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let root_path = temp_dir.path();
        let nested_dir = root_path.join("nested");

        // Create nested directory without a marker file
        fs::create_dir(&nested_dir)?;

        // Try to find the marker file starting from the nested directory
        let found_marker = find_project_dir(Some(&nested_dir));

        // Verify that no marker file was found
        assert!(!found_marker.is_some());
        Ok(())
    }
}
