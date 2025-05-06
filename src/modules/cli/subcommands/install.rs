use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "lower")]
pub enum Installables {
    Definitions,
    Encryption,
    LegacyBuild,
}

impl std::fmt::Display for Installables {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            Installables::Definitions => println!("{:?}", Installables::Definitions),
            Installables::Encryption => println!("{:?}", Installables::Encryption),
            Installables::LegacyBuild => println!("{:?}", Installables::LegacyBuild),
        };
        Ok(())
    }
}

pub fn install(thing: Option<&Installables>) {
    match thing {
        Some(thing) => println!("{thing}"),
        None => println!("Nothing"),
    }
}
