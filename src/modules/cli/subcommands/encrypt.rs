use mlua::IntoLua;

use crate::config::UserEnv;

// #[cfg("windows")]
pub fn install_encryption() {
    todo!("Implement Installation of Encryption")
}

pub fn default_encryption_tool() {
    todo!("Implement the default_encryption_tool")
}

pub fn encrypt(user_env: UserEnv, tool_args: Option<Vec<String>>) -> anyhow::Result<()> {
    let encryption_tool = &user_env.config.encryption_tool;
    println!("{:?}", tool_args);
    let args = tool_args.into_lua(user_env.lua)?;
    encryption_tool(Some(args));
    Ok(())
}
