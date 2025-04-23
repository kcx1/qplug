use mlua::{
    Lua,
    Value::{self},
};

use crate::files::find_project_dir;

pub fn load_api(lua: &Lua) -> anyhow::Result<()> {
    let local_find = lua.create_function(|_, _: Value| {
        // Unwrapping here inside of the closure. Might be a better way.
        let binding = find_project_dir(None).unwrap();
        let result: Option<&str> = binding.to_str();
        match result {
            Some(path) => Ok(path.to_owned()),
            _ => panic!("Invalid path"),
        }
    });
    Ok(lua.globals().set("find_project_dir", local_find?)?)
}
