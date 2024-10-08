use clap::Command;
use clap_complete::{generate, Shell};
use mlua::Lua;
use qplug::assets::INFO_LUA;
use qplug::cli;
use qplug::config::{Config, UserConfig, UserEnv};
use qplug::lua::api::load_api;
use std::io::{self};

fn create_lua_env() -> Lua {
    Lua::new()
}

const APP_NAME: &str = "qplug";

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let lua_env = create_lua_env();

    load_api(&lua_env);

    let user_config = UserConfig::new(&lua_env);
    let config = Config::from_user_config(&user_config);

    let env = UserEnv {
        lua: &lua_env,
        config: &config,
    };

    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let name = sub_matches.get_one::<String>("Name");
            let no_git = sub_matches.get_one::<bool>("Disable Git").unwrap();
            let no_defs = sub_matches
                .get_one::<bool>("Disable Lua Definitions")
                .unwrap();
            let no_template = sub_matches
                .get_one::<bool>("Disable Template Creation")
                .unwrap();
            cli::subcommands::new::create_plugin(name, no_git, no_template, no_defs, env);
        }
        Some(("build", sub_matches)) => {
            //TODO: Look into allowing builds for custom flat qplug files. (no info.lua file)
            let version = sub_matches
                .get_one::<cli::subcommands::build::VersionType>("Increment Build Version")
                .unwrap();
            cli::subcommands::build::build(version.to_owned(), INFO_LUA.clone().unwrap(), env)
        }
        Some(("update", sub_matches)) => {
            let version: Option<&str> = sub_matches.get_one("Version").map(|x: &String| x.as_str());

            cli::subcommands::update::update(&version).expect("Could not update Q-Plug");
        }
        Some(("copy", _sub_matches)) => {
            cli::subcommands::copy::copy_to_plugin_directory().expect("Could not copy plugin");
        }
        Some(("compile", _sub_matches)) => {
            cli::subcommands::compile::compile();
        }
        Some(("check", sub_matches)) => {
            let check_option = sub_matches
                .get_one::<cli::subcommands::check::CheckOption>("Check Option")
                .unwrap();
            cli::subcommands::check::check(check_option.to_owned());
        }
        Some(("completions", sub_matches)) => {
            let shell = sub_matches.get_one::<Shell>("shell").unwrap();
            let mut app = Command::new(APP_NAME);
            generate(*shell, &mut app, APP_NAME, &mut io::stdout());
        }
        _ => unreachable!(),
    }
}
