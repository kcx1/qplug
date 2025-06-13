use std::path::PathBuf;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QsysCore {
    pub username: Option<String>,
    pub hostname: String,
    pub password: Option<String>,
    pub components: Vec<Component>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Component {
    pub name: String,
    pub script: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    pub core: QsysCore,
}

impl Target {
    pub fn from_config(config: &PathBuf) -> anyhow::Result<Target> {
        Ok(
            serde_json::from_reader(std::fs::File::open(config).context(format!("{:?}", config))?)
                .context("Serialization of Target failed")?,
        )
    }
}
