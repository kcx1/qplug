use clap::Command;
use clap_complete::{generate, Shell};
use mlua::Lua;
use qplug::assets::INFO_LUA;
use qplug::cli;
use qplug::config::{Config, UserConfig};
use std::io::{self};

fn create_lua_env() -> Lua {
    Lua::new()
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let lua_env = create_lua_env();

    let user_config = UserConfig::new(&lua_env);
    let config = Config::from_user_config(&user_config);

    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let name = sub_matches.get_one::<String>("str").unwrap();
            let no_git = sub_matches.get_one::<bool>("Enable Git").unwrap();
            cli::subcommands::new::create_plugin(name, no_git, &lua_env, &config);
        }
        Some(("build", sub_matches)) => {
            let version = sub_matches
                .get_one::<cli::subcommands::build::VersionType>("Increment Build Version")
                .unwrap();
            todo!("Use build command from config")
            cli::subcommands::build::build(version.to_owned(), INFO_LUA.clone().unwrap(), &lua_env)
        }
        Some(("completions", sub_matches)) => {
            let shell = sub_matches.get_one::<Shell>("shell").unwrap();
            let mut app = Command::new("my_cli_app");
            generate(*shell, &mut app, "my_cli_app", &mut io::stdout());
        }
        _ => unreachable!(),
    }
}
