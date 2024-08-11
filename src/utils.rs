use std::env;
use std::path::PathBuf;
use std::process::Command;

pub fn find_relative_dir(pwd: PathBuf) -> Result<(), ()> {
    let is_git = Command::new("sh")
        .current_dir(pwd)
        .arg("-c")
        .arg("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .unwrap();

    if is_git.stderr.is_empty() {
        Ok(())
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_git() {
        let pwd = env::current_dir().expect("Could not get current directory.");
        assert_eq!(Ok(()), find_relative_dir(pwd));
        assert_eq!(Err(()), find_relative_dir(PathBuf::from("..")));
    }
}
