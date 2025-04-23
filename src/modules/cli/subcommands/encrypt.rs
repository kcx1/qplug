use crate::config::UserEnv;

// #[cfg("windows")]
pub fn install_encryption() {
    todo!("Implement Installation of Encryption")
}

pub fn default_encryption_tool() {
    todo!("Implement the default_encryption_tool")
}

pub fn encrypt(user_env: UserEnv) -> anyhow::Result<()> {
    let encryption_tool = &user_env.config.encryption_tool;
    encryption_tool();
    Ok(())
}
