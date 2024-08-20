use std::{
    fs,
    path::{Path, PathBuf},
};

use mlua::{Lua, Table, Value};
use regex::Regex;
use crate::assets::INIT_LUA;

pub fn name_table(table_name: &str, table: &str) -> String {
    format!("{} = {}", table_name, table).to_string()
}

pub fn serialize_value(lua: &Lua, value: &Value) -> String {
    match value {
        Value::Nil => "nil".to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!(r#""{}""#, s.to_str().unwrap_or("")),
        Value::Table(t) => serialize_table(lua, t),
        _ => "unsupported".to_string(), // Handle more types if needed
    }
}

pub fn serialize_table(lua: &Lua, table: &Table) -> String {
    let mut result = String::new();
    result.push('{');
    table
        .clone()
        .pairs::<String, Value>()
        .flatten()
        .for_each(|(key, value)| {
            let serialized_value = serialize_value(lua, &value);
            result.push_str(&format!("{} = {}, ", key, serialized_value));
        });

    result.push('}');
    result = result.replace(", }", "}");
    result
}

pub fn find_lua_requirements() {}

pub fn merge_lua_files(root_path: PathBuf, plugin_path: PathBuf) -> std::io::Result<()> {
    let init_file = INIT_LUA.clone().unwrap();
    let qplug_file = root_path.join("qplug.lua");

    // Read the skeleton Lua file
    let mut init_content = fs::read_to_string(init_file)?;

    // Regex to match require statements (assumes simple pattern like require('module'))
    let re = Regex::new(r#"require\(['"]([^'"]+)['"]\)"#).unwrap();

    // Collect all unique module names from the require statements
    let mut modules = std::collections::HashSet::new();
    for cap in re.captures_iter(&init_content) {
        modules.insert(cap[1].to_string().replace('.', "/"));
    }

    // Read each module's content and replace the require statements
    let mut result_content = init_content.clone();
    for module in modules {
        let module_path = plugin_path.join(format!("{}.lua", module));
        if module_path.exists() {
            let module_content = fs::read_to_string(module_path)?;
            result_content =
                result_content.replace(&format!(r#"require('{}')"#, module), &module_content);
        } else {
            eprintln!("Warning: Module file for '{}' not found", module);
        }
    }

    // Write the result to a new file
    fs::write(qplug_file, result_content)?;

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////////
/// Tests
//////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {

    use super::*;
    use mlua::{Lua, Table, Value};

    #[test]
    fn test_serialize_simple_table() {
        let lua = Lua::new();
        let table = lua.create_table().unwrap();
        table.set("key1", "value1").unwrap();
        table.set("key2", 42).unwrap();
        table.set("key3", true).unwrap();

        let serialized = name_table("my_table", &serialize_table(&lua, &table));

        lua.load(r#"expected_table = {key1 = "value1", key2 = 42, key3 = true}"#)
            .exec()
            .expect("Control table creation failed");
        lua.load(serialized)
            .exec()
            .expect("Reserialized table creation failed");

        let expected_table: Table = lua.globals().get("expected_table").unwrap();
        let re_serialized: Table = lua.globals().get("my_table").unwrap();

        for pair in expected_table.pairs::<Value, Value>() {
            let (key, value) = pair.unwrap();
            assert_eq!(value, re_serialized.get(key).unwrap());
        }
    }

    #[test]
    fn test_serialize_nested_table() {
        let lua = Lua::new();
        let table = lua.create_table().unwrap();
        let nested_table = lua.create_table().unwrap();
        nested_table.set("nested_key", "nested_value").unwrap();
        table.set("nested_table", nested_table).unwrap();

        let serialized = name_table("my_table", &serialize_table(&lua, &table));

        let expected = r#"my_table = {nested_table = {nested_key = "nested_value"}}"#;
        assert_eq!(&serialized, expected);
    }

    #[test]
    fn test_serialize_empty_table() {
        let lua = Lua::new();
        let table = lua.create_table().unwrap();

        let serialized = name_table("serialized", &serialize_table(&lua, &table));

        assert_eq!(&serialized, "serialized = {}");
    }

    #[test]
    fn test_serialize_table_with_nil() {
        let lua = Lua::new();
        let table = lua.create_table().unwrap();
        table.set("key", Value::Nil).unwrap();

        let serialized = name_table("table_with_nil", &serialize_table(&lua, &table));

        let expected = "table_with_nil = {}";
        assert_eq!(&serialized, expected);
    }

    #[test]
    fn test_serialize_table_with_unsupported_value() {
        let lua = Lua::new();
        let table = lua.create_table().unwrap();
        table
            .set(
                "function_key",
                lua.create_function(|_, _: ()| Ok(())).unwrap(),
            )
            .unwrap();

        let serialized = name_table("table_with_function", &serialize_table(&lua, &table));

        let expected = "table_with_function = {function_key = unsupported}";
        assert_eq!(&serialized, expected);
    }
}
