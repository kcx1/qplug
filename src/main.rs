use anyhow::Context;
use clap::{ArgMatches, Command};
use clap_complete::{generate, Shell};
use mlua::Lua;
use qplug::cli::{self, subcommands};
use qplug::config::Config;
use qplug::globals::INFO_LUA;
use qplug::lua::api::load_api;
use qplug::modules::user::{UserConfig, UserEnv};
use std::io::{self};
use std::path::PathBuf;

fn create_lua_env() -> Lua {
    Lua::new()
}

const APP_NAME: &str = "qplug";

fn main() -> anyhow::Result<()> {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let lua_env = create_lua_env();

    load_api(&lua_env).context("Failto load API")?;

    let user_config = UserConfig::new(&lua_env).context("Failed to create User Config Instance")?;
    let config = Config::from_user_config(user_config).context("Failde to load User config")?;

    let env = UserEnv {
        lua: &lua_env,
        config: &config,
    };

    let matches: ArgMatches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let name = sub_matches.get_one::<String>("Name");
            let no_git = sub_matches.get_one::<bool>("Disable Git").unwrap();
            let no_luarc = sub_matches.get_one::<bool>("Disable luarc").unwrap();
            let no_template = sub_matches
                .get_one::<bool>("Disable Template Creation")
                .unwrap();
            subcommands::new::create_plugin(name, no_git, no_template, no_luarc, env)
                .context("Failed to create plugin project")
        }
        Some(("build", sub_matches)) => {
            let version = sub_matches
                .get_one::<subcommands::build::VersionType>("Increment Build Version")
                .unwrap();
            let build_only = sub_matches.get_flag("Build Only");
            let copy_path: Option<&PathBuf> = sub_matches.get_one("Path to copy");
            subcommands::build::build(
                version.to_owned(),
                INFO_LUA.clone().unwrap(),
                env,
                copy_path,
                build_only,
            )
            .context("Failed to build plugin project")
        }
        Some(("update", sub_matches)) => {
            let version: Option<&str> = sub_matches.get_one("Version").map(|x: &String| x.as_str());

            subcommands::update::update(&version).context("Could not update Q-Plug")
        }
        Some(("copy", sub_matches)) => {
            let default_dir = env.config.plugin_dir.clone().unwrap();
            let copy_path: Option<&PathBuf> =
                sub_matches.get_one("Copy Path").or(Some(&default_dir));
            subcommands::copy::copy_to_plugin_directory(env.config, copy_path)
                .context("Could not copy plugin")?;
            Ok(())
        }
        Some(("check", sub_matches)) => {
            let check_option = sub_matches
                .get_one::<subcommands::check::CheckOption>("Check Option")
                .unwrap();
            subcommands::check::check(check_option.to_owned())
        }
        Some(("sync", sub_matches)) => {
            let script = sub_matches.get_one::<PathBuf>("script").unwrap();
            let config = sub_matches.get_one::<PathBuf>("config").unwrap();
            let is_watched = sub_matches.get_flag("is_watched");
            subcommands::sync::sync(script, config, is_watched)
        }
        Some(("completions", sub_matches)) => {
            let shell = sub_matches.get_one::<Shell>("shell").unwrap();
            let mut app = Command::new(APP_NAME);
            generate(*shell, &mut app, APP_NAME, &mut io::stdout());
            Ok(())
        }
        Some(("install", sub_mathces)) => {
            let thing_to_install =
                sub_mathces.get_one::<subcommands::install::Installables>("install");
            subcommands::install::install(thing_to_install).context("Failed to install.")
        }
        Some(("encrypt", sub_matches)) => {
            let tool_args: Vec<String> = sub_matches
                .get_many::<String>("tool")
                .unwrap()
                .cloned()
                .collect();
            subcommands::encrypt::encrypt(env, Some(tool_args)).context("Failed to encrypt plugin")
        }
        Some((_, _)) => todo!("Some tuple not implemented"),
        None => {
            todo!("Sorry this feature is not implemented")
        }
    }
}
