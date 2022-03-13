pub mod misc;
pub mod subcommands;

use clap::{Parser, Subcommand};

// HEHEHEHA
#[macro_export]
macro_rules! reachable {
    () => {
        println!("Checkpoint reached!");
    };
}

#[derive(Parser, Debug)]
struct Poockp {
    #[clap(subcommand)]
    subcommands: Subcommands,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    #[clap(about = "Installs a PCKP/PooCKP package")]
    Add { names: Vec<String> },

    #[clap(about = "Removes a PCKP/PooCKP package")]
    Remove { names: Vec<String> },

    #[clap(about = "Initializes a new PooCKP project")]
    Init {},

    #[clap(about = "Creates a new PooCKP project")]
    New { name: String },
}

fn main() {
    let args = Poockp::parse();

    match &args.subcommands {
        Subcommands::Add { names } => subcommands::add(names.to_vec()), // TODO: why is this like this lmao
        Subcommands::Remove { names } => subcommands::remove(names.to_vec()),
        Subcommands::New { name } => subcommands::new(name.as_str()).unwrap(),
        Subcommands::Init {} => subcommands::init().unwrap(),
    }
}
