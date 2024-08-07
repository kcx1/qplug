use clap::{arg, Arg, ArgAction, Command};
use std::io::{self};

use qplug::cli::subcommands::new::create_plugin;

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
                ),
        )
        .subcommand(Command::new("test").arg(Arg::new("Test")))
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let name = sub_matches.get_one::<String>("str").unwrap();
            let no_git = sub_matches.get_one::<bool>("Enable Git").unwrap();
            create_plugin(name, no_git);
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
