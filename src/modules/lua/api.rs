use mlua::{
    Lua,
    Value::{self},
};

use crate::files::find_project_dir;

pub fn load_api(lua: &Lua) {
    let local_find = lua.create_function(|_, _: Value| {
        let binding = find_project_dir(None).unwrap();
        let result: Option<&str> = binding.to_str();
        match result {
            Some(path) => Ok(path.to_owned()),
            _ => panic!("Invalid path"),
        }
    });

    lua.globals()
        .set("find_project_dir", local_find.unwrap())
        .unwrap();
}
