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

    load_api(&lua_env).expect("Failto load API");

    let user_config = UserConfig::new(&lua_env).expect("Failed to create User Config Instance");
    let config = Config::from_user_config(&user_config).expect("Failde to load User config");

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
            cli::subcommands::new::create_plugin(name, no_git, no_template, no_defs, env)
                .expect("Failed to create plugin project");
        }
        Some(("build", sub_matches)) => {
            let version = sub_matches
                .get_one::<cli::subcommands::build::VersionType>("Increment Build Version")
                .unwrap();
            let build_only = sub_matches.get_flag("Build Only");
            let copy_path: Option<&String> = sub_matches.get_one("Path to copy");
            cli::subcommands::build::build(
                version.to_owned(),
                INFO_LUA.clone().unwrap(),
                env,
                copy_path,
                build_only,
            )
            .expect("Failed to build plugin project");
        }
        Some(("update", sub_matches)) => {
            let version: Option<&str> = sub_matches.get_one("Version").map(|x: &String| x.as_str());

            cli::subcommands::update::update(&version).expect("Could not update Q-Plug");
        }
        Some(("copy", sub_matches)) => {
            let default_dir = user_config.plugin_dir.to_string().unwrap();
            let copy_path: Option<&String> =
                sub_matches.get_one("Copy Path").or(Some(&default_dir));
            cli::subcommands::copy::copy_to_plugin_directory(env.config, copy_path)
                .expect("Could not copy plugin");
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
        Some(("encrypt", sub_matches)) => match sub_matches.subcommand() {
            Some(("install", _encrypt_matches)) => cli::subcommands::encrypt::install_encryption(),
            _ => cli::subcommands::encrypt::encrypt(env).expect("Failed to encrypt plugin"),
        },
        _ => unreachable!(),
    }
}
