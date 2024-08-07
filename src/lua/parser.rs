use std::{fs, path::Path};

use mlua::{Lua, Table, Value};
use regex::Regex;

pub fn name_table(table_name: &str, table: &str) -> String {
    format!("{} = {}", table_name, table).to_string()
}

pub fn serialize_table(lua: &Lua, table: &Table) -> String {
    fn serialize_value(lua: &Lua, value: &Value) -> String {
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

pub fn merge_lua_files() -> std::io::Result<()> {
    let skeleton_path = "path/to/your/skeleton.lua";
    let output_path = "path/to/your/output.lua";
    let module_dir = "path/to/your/modules";

    // Read the skeleton Lua file
    let skeleton_content = fs::read_to_string(skeleton_path)?;

    // Regex to match require statements (assumes simple pattern like require('module'))
    let re = Regex::new(r#"require\(['"]([^'"]+)['"]\)"#).unwrap();

    // Collect all unique module names from the require statements
    let mut modules = std::collections::HashSet::new();
    for cap in re.captures_iter(&skeleton_content) {
        modules.insert(cap[1].to_string());
    }

    // Read each module's content and replace the require statements
    let mut result_content = skeleton_content.clone();
    for module in modules {
        let module_path = Path::new(module_dir).join(format!("{}.lua", module));
        if module_path.exists() {
            let module_content = fs::read_to_string(module_path)?;
            result_content =
                result_content.replace(&format!(r#"require('{}')"#, module), &module_content);
        } else {
            eprintln!("Warning: Module file for '{}' not found", module);
        }
    }

    // Write the result to a new file
    fs::write(output_path, result_content)?;

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

        let expected = r#"my_table = {key1 = "value1", key2 = 42, key3 = true}"#;
        assert_eq!(&serialized, expected);
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
