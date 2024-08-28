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

const APP_NAME: &str = "qplug";

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
        Some(("init", sub_matches)) => {
            let no_git = sub_matches.get_one::<bool>("Disable Git").unwrap();
            let no_defs = sub_matches
                .get_one::<bool>("Disable Lua Definitions")
                .unwrap();
            let no_template = sub_matches
                .get_one::<bool>("Disable Template Creation")
                .unwrap();
            cli::subcommands::init::init_plugin(*no_template, *no_git, *no_defs, &config);
        }
        Some(("build", sub_matches)) => {
            //TODO: Look into allowing builds for custom flat qplug files. (no info.lua file)
            let version = sub_matches
                .get_one::<cli::subcommands::build::VersionType>("Increment Build Version")
                .unwrap();
            cli::subcommands::build::build(version.to_owned(), INFO_LUA.clone().unwrap(), &lua_env)
        }
        Some(("compile", _sub_matches)) => {
            cli::subcommands::compile::compile();
        }
        Some(("completions", sub_matches)) => {
            let shell = sub_matches.get_one::<Shell>("shell").unwrap();
            let mut app = Command::new(APP_NAME);
            generate(*shell, &mut app, APP_NAME, &mut io::stdout());
        }
        _ => unreachable!(),
    }
}
