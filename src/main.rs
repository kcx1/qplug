use clap::{arg, Arg, ArgAction, Command};
use std::io::{self};

use qplug::cli::subcommands::new::create_plugin;

enum Repo {
    Base,
    Example,
    Custom,
}

impl ToString for Repo {
    fn to_string(&self) -> String {
        match self {
            Repo::Base => String::from("https://github.com/qsysdev/qplug-base"),
            Repo::Example => String::from("https://github.com/qsysdev/qplug-example"),
            Repo::Custom => {
                let mut custom = String::new();
                io::stdin().read_line(&mut custom).unwrap();
                custom
            }
        }
    }
}

fn cli() -> Command {
    Command::new("qplug")
        .about("Q-Sys plugin Development tool.")
        .subcommand_required(true)
        .arg_required_else_help(true)
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
        .subcommand(Command::new("test").arg(Arg::new("Test")))
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
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "lower")]
enum VersionType {
    Dev,
    Patch,
    Minor,
    Major,
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let name = sub_matches.get_one::<String>("str").unwrap();
            let no_git = sub_matches.get_one::<bool>("Enable Git").unwrap();
            create_plugin(name, no_git);
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
        }
        Some(("test", _)) => {
            let mut result = String::new();
            println!("This is a test. Please input anything.");
            io::stdin().read_line(&mut result).unwrap();
            println!("Here is the test result: {}", result.trim());
        }
        _ => unreachable!(),
    }
}
