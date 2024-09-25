use clap::ValueEnum;

use crate::{config::find_config_file, files::find_marker_file};

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "lower")]
pub enum CheckOption {
    Version,
    Qplug,
    Config,
}

pub fn check(check_option: CheckOption) {
    match check_option {
        CheckOption::Version => println!("Qplug version: {}", env!("CARGO_PKG_VERSION")),
        CheckOption::Qplug => {
            let marker_file = find_marker_file(None);
            match marker_file {
                Some(f) => println!("Qplug plugin found! {:?}", f),
                None => println!("Not a Qplug plugin. You may want to try `qplug init` or navigating to a qplug directory."),
            }
        }
        CheckOption::Config => match find_config_file() {
            Some(f) => println!("Config file found! {:?}", f),
            None => println!("No config file found. You may want to try `qplug new`"),
        },
    }
}
