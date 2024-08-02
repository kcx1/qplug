use mlua::UserData;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

pub enum BuildIncrement {
    Patch = 2,
    Minor = 1,
    Major = 0,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginInfo {
    pub Name: String,
    pub Version: String,
    pub BuildVersion: String,
    pub Id: String,
    pub Author: String,
    pub Description: String,
}

impl UserData for PluginInfo {}
