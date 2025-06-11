use std::path::PathBuf;

pub async fn watch_file(file: &PathBuf) -> anyhow::Result<String> {
    Ok(file.to_str().unwrap().into())
}
