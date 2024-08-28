use clap::{arg, value_parser, Arg, ArgAction, Command};
use clap_complete::Shell;
use subcommands::build::VersionType;

pub mod subcommands;

pub fn cli() -> Command {
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
            Command::new("init")
                .about("Initialize the current directory as a plugin.")
                .arg(
                    Arg::new("Disabe Git")
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
        .subcommand(
            Command::new("compile")
                .about("Complie the plugin. Do not increment versioning or copy to plugin folder."),
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
