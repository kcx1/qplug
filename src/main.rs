use clap::{arg, value_parser, Arg, ArgAction, Command, ValueEnum};
use clap_complete::{generate, Shell};
use mlua::Lua;
use std::{
    io::{self},
    path::PathBuf,
};

use qplug::{cli::subcommands::new::create_plugin, lua::parser::merge_lua_files};

fn create_lua_env() -> Lua {
    Lua::new()
}

fn cli() -> Command {
    Command::new("qplug")
        .about("Q-Sys plugin Development tool.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // New
        .subcommand(
            Command::new("new")
                .about("Create a new plugin template.")
                .arg(arg!(<str> "Name of the plugin."))
                .arg_required_else_help(true)
                .arg(
                    Arg::new("Enable Git")
                        .long("no-git")
                        .default_value("false")
                        .action(ArgAction::SetTrue),
                )
                .arg(Arg::new("Specify Base Plugin").long("repo")),
        )
        // Build
        .subcommand(
            Command::new("build")
                .about("Build and complie the plugin.")
                .arg(
                    Arg::new("Increment Build Version")
                        // .long("version")
                        // .short('v')
                        .value_parser(value_parser!(VersionType))
                        .default_value("dev")
                        .ignore_case(true),
                ),
        )
        //Generate shell completion
        .subcommand(
            Command::new("completions")
                .hide(true)
                .about("Generate shell completions")
                .arg(
                    Arg::new("shell")
                        .value_parser(value_parser!(Shell))
                        .help("The shell to generate completions for")
                        .required(true),
                ),
        )
}

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "lower")]
enum VersionType {
    Dev,
    Patch,
    Minor,
    Major,
}

fn main() {
    let lua_env = create_lua_env();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let name = sub_matches.get_one::<String>("str").unwrap();
            let no_git = sub_matches.get_one::<bool>("Enable Git").unwrap();
            create_plugin(name, no_git, &lua_env);
        }
        Some(("build", sub_matches)) => {
            let version = sub_matches
                .get_one::<VersionType>("Increment Build Version")
                .unwrap();
            match version {
                VersionType::Major => println!("major"),
                VersionType::Minor => println!("minor"),
                VersionType::Patch => println!("patch"),
                VersionType::Dev => println!("dev"),
            }
            // TODO: Figure out some magic to know where project root is in relationship to user's
            // pwd
            let root_path = PathBuf::from(".");
            let plugin_path = root_path.join("plugin_src");
            merge_lua_files(root_path, plugin_path).unwrap();
        }
        Some(("completions", sub_matches)) => {
            let shell = sub_matches.get_one::<Shell>("shell").unwrap();
            let mut app = Command::new("my_cli_app");
            generate(*shell, &mut app, "my_cli_app", &mut io::stdout());
        }
        _ => unreachable!(),
    }
}
