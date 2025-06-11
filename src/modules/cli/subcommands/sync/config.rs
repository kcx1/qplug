use std::{io::stdin, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QsysCore {
    pub username: Option<String>,
    pub hostname: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    // id: String,
    // r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    pub core: QsysCore,
    pub component: Component,
}

impl Target {
    // TODO:
    // The problem wiht this funciton is that it is an all or nothing match. If a config is found
    // EVERTHING is in the config. Otherwise the user is PROMPTED for everything. This needs to
    // blend the approach and fill in whatever is missing.
    pub fn new(config: Option<PathBuf>) -> anyhow::Result<Target> {
        match config {
            Some(config) => Ok(serde_json::from_reader(&mut std::fs::File::open(config)?)?),
            None => {
                // TODO: Create function to fill out the config.
                let mut name = String::new();
                stdin().read_line(&mut name)?;
                Ok(Target {
                    core: QsysCore {
                        hostname: String::from("0.0.0.0"),
                        username: None,
                        password: None,
                    },
                    component: Component { name },
                })
            }
        }
    }

    pub fn from_config(config: &PathBuf) -> anyhow::Result<Target> {
        Ok(serde_json::from_reader(std::fs::File::open(config)?)?)
    }
}
