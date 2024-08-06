use mlua::{Lua, Table, Value};

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
