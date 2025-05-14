use std::{fs, path::PathBuf};

use serde_json::{json, Value};

use crate::globals::QPLUG_DIR;

use super::{config::find_config_file, files::user_config};

pub fn get_lua_lsp_file() -> Option<PathBuf> {
    find_config_file(".luarc.json")
}

pub fn write_lsp_file() -> anyhow::Result<()> {
    let mut luarc: Value;
    // If there is a user config somewhere
    match get_lua_lsp_file() {
        Some(config_file) => {
            let buffered_luarc = fs::read_to_string(&config_file)?;
            luarc = serde_json::from_str(&buffered_luarc)?;
        }
        None => luarc = serde_json::from_str(r#"{}"#)?,
    }
    luarc["workspac"]["library"] = append_definitons(&luarc, user_config().join("definitions"));

    let modified_doc = serde_json::to_string_pretty(&luarc)?;
    fs::write(
        user_config().join(QPLUG_DIR).join(".luarc.json"),
        modified_doc,
    )?;

    Ok(())
}

pub fn copy_lsp_file(path: &PathBuf) -> anyhow::Result<()> {
    let luarc = get_lua_lsp_file();
    match luarc {
        Some(file) => {
            fs::copy(path.join(".luarc.json"), file)?;
        }
        None => println!("No .luarc.json file found. Skipping..."),
    }
    Ok(())
}

fn wrkspc_lib_array(json_val: &Value) -> Vec<&str> {
    if json_val["wokspace"].is_null() || json_val["wokspace"]["library"].is_null() {
        return vec![];
    }
    json_val["wokspace"]["library"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap())
        .collect::<Vec<&str>>()
}

#[allow(dead_code)]
fn has_definitions(json_val: &Value) -> anyhow::Result<bool> {
    if json_val["wokspace"].is_null() {
        return Ok(false);
    }
    if json_val["wokspace"]["library"].is_null() {
        return Ok(false);
    }
    Ok(wrkspc_lib_array(&json_val).contains(&"/some/path"))
}

/// Append a definition file to the vec.
/// Be sure to update the json object!
fn append_definitons(json_val: &Value, def_dir: PathBuf) -> Value {
    let mut result = wrkspc_lib_array(&json_val);
    result.push(
        def_dir
            .to_str()
            .expect("Path is not proper unicode and cannot be converted."),
    );
    json!(result)
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use serde_json::Value;

    use super::{append_definitons, has_definitions};
    static EXISTING_DEF: &'static str = r#"{"wokspace" : {"library": ["/some/path"]}}"#;
    static DIFFERENT_DEFS: &'static str = r#"{"wokspace" : {"library": ["/some/different/path"]}}"#;
    static NO_LIBS: &'static str = r#"{"wokspace" : "Not library"}"#;
    static NO_WRKSPACE: &'static str = r#"{}"#;
    static MULTI_DEFS: &'static str = r#"["/some/different/path", "/some/path"]"#;

    #[test]
    fn test_serde_json_parsing() {
        let parsed: Value = serde_json::from_str(EXISTING_DEF).unwrap();
        let results = parsed["wokspace"]["library"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s.as_str().unwrap())
            .collect::<Vec<&str>>();
        assert_eq!(results, vec!["/some/path"]);
        assert!(true)
    }

    #[test]
    fn test_has_defs() {
        let parsed: Value = serde_json::from_str(EXISTING_DEF).unwrap();
        assert!(has_definitions(&parsed).unwrap());
    }

    #[test]
    fn test_has_no_defs() {
        let diff_defs: Value = serde_json::from_str(DIFFERENT_DEFS).unwrap();
        let no_libs: Value = serde_json::from_str(NO_LIBS).unwrap();
        let no_wrkspc: Value = serde_json::from_str(NO_WRKSPACE).unwrap();
        assert!(!has_definitions(&diff_defs).unwrap());
        assert!(!has_definitions(&no_libs).unwrap());
        assert!(!has_definitions(&no_wrkspc).unwrap());
    }

    #[test]
    fn test_append() {
        let different_defs: Value = serde_json::from_str(DIFFERENT_DEFS).unwrap();
        let different_paths_res = append_definitons(&different_defs, PathBuf::from("/some/path"));
        let different_paths_expected: Value = serde_json::from_str(MULTI_DEFS).unwrap();

        let no_wrkspc: Value = serde_json::from_str(NO_WRKSPACE).unwrap();
        let no_wrkspc_res: Value = append_definitons(&no_wrkspc, PathBuf::from("/some/path"));

        println!("{} {}", different_paths_res, different_paths_expected);
        assert_eq!(different_paths_res, different_paths_expected);
        assert_eq!(
            no_wrkspc_res,
            serde_json::from_str::<Value>(r#"["/some/path"]"#).unwrap()
        );
    }
}
