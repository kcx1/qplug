use crate::files::copy_dir;
use crate::modules::user::UserEnv;
use git2::Repository;
use std::path::Path;
use std::{fs, path::PathBuf};

use anyhow::Context;

#[derive(Debug)]
pub enum Template<'a> {
    Url(String),
    FileSystem(PathBuf),
    InMemoryDir(&'a include_dir::Dir<'static>),
}

pub fn create_template(plugin_path: &PathBuf, user_env: &UserEnv) -> anyhow::Result<()> {
    fs::create_dir_all(&plugin_path)
        .context("Failed to create template. Some of the directories may already exist.")?;
    fetch_template(plugin_path.as_path(), &user_env.config.template)?;
    println!("Template initialized");
    Ok(())
}

fn fetch_template(plugin_dir: &Path, template: &Template) -> anyhow::Result<PathBuf> {
    // let url = "https://github.com/qsys-plugins/BasePlugin";
    match template {
        Template::Url(s) => Ok(Repository::clone(s, plugin_dir)
            .context(format!("Failed to clone template repo: {s}"))?
            .path()
            .to_path_buf()),
        Template::FileSystem(_) => {
            copy_dir(template, plugin_dir)
                .with_context(|| format!("Failed to copy {:?} to {:?}.", template, plugin_dir))?;
            Ok(plugin_dir.to_path_buf())
        }
        Template::InMemoryDir(_) => {
            copy_dir(template, plugin_dir).context("Failed to copy built-in template")?;
            Ok(plugin_dir.to_path_buf())
        }
    }
}
