use std::fs;

use serde_json::Value;

use crate::globals::QPLUG_CONFIG;

use super::config::find_config_file;

pub fn update_lsp_config() -> anyhow::Result<()> {
    if let Some(user_config) = find_config_file(QPLUG_CONFIG) {
        let buffered_luarc = fs::read_to_string(user_config)?;
        let luarc: Value = serde_json::from_str(&buffered_luarc)?;
    }

    Ok(())
}
