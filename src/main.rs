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
    subcommands: Subcommands
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    #[clap(about = "Installs a PCKP package")]
    Install {
        name: String
    },

    #[clap(about = "Removes a PCKP package")]
    Remove {
        name: String
    },

    #[clap(about = "Initializes a new Poockp project")]
    Init { },

    #[clap(about = "Creates a new Poockp project")]
    New { name: String }
}

fn main() {
    let args = Poockp::parse();

    match &args.subcommands {
        Subcommands::Install { name } => println!("installing {name}"),
        Subcommands::Remove { name } => println!("removing {name}"),
        Subcommands::New { name } => println!("creating new PooCKP project {name}..."),
        Subcommands::Init { } => subcommands::init().unwrap()
    }
}