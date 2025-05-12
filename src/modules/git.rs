use std::path::Path;

use anyhow::Context;
use git2::Repository;

pub fn init_git(path: &Path) -> anyhow::Result<Repository> {
    Repository::init(path).context("Failed to initialize local git repo")
}
