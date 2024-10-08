use clap::{
    builder::styling, crate_authors, crate_version, value_parser, Arg, ArgAction, ColorChoice,
    Command,
};
use clap_complete::Shell;
use subcommands::{build::VersionType, check::CheckOption};

pub mod subcommands;

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::BrightWhite.on_default().bold())
    .error(styling::AnsiColor::BrightRed.on_default().bold())
    .usage(styling::AnsiColor::BrightWhite.on_default().bold())
    .literal(styling::AnsiColor::Cyan.on_default().bold())
    .placeholder(styling::AnsiColor::Magenta.on_default())
    .valid(styling::AnsiColor::Green.on_default())
    .invalid(styling::AnsiColor::Red.on_default());

pub fn cli() -> Command {
    Command::new("qplug")
        .about("Q-Sys plugin Development tool.")
        .color(ColorChoice::Always)
        .styles(STYLES)
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        // New
        .subcommand(
            Command::new("new")
                .about("Create a new plugin template.")
                .arg(Arg::new("Name")
                    .action(ArgAction::Set)
                )
                .arg(
                    Arg::new("Disable Git")
                        .long("no-git")
                        .default_value("false")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("Disable Lua Definitions")
                        .long("no-defs")
                        .default_value("false")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("Disable Template Creation")
                        .long("no-template")
                        .default_value("false")
                        .action(ArgAction::SetTrue),
                ),
        )
        // Build
        .subcommand(
            Command::new("build")
                .about("Build and compile the plugin.")
                .arg(
                    Arg::new("Increment Build Version")
                        // .long("version")
                        // .short('v')
                        .value_parser(value_parser!(VersionType))
                        .default_value("dev")
                        .ignore_case(true),
                ),
        )
        // Update
         .subcommand(
    Command::new("update")
                .about("Update the qplug utility to the latest version.")
                .arg(
                    Arg::new("Version")
                    .long("version")
                    .short('v')
                    .help("Specify the version to update (or roll back) to. If omitted, defaults to the latest version. Pass the current version to force update.")
                    .default_value(None)
                    .action(ArgAction::Set)
                )
        )
        .subcommand(Command::new("copy").about("Copy the plugin to the plugin folder."))
        .subcommand(
            Command::new("compile")
                .about("Compile the plugin. Do not increment versioning or copy to plugin folder."),
        )
        .subcommand(Command::new("check")
            .about("check if current directory is a valid plugin.")
            .arg(Arg::new("Check Option")
                .value_parser(value_parser!(CheckOption))
                .ignore_case(true)
                .default_value("qplug")
            )
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
