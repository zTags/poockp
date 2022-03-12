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
    Install { name: String },

    #[clap(about = "Removes a PCKP/PooCKP package")]
    Remove { name: String },

    #[clap(about = "Initializes a new PooCKP project")]
    Init {},

    #[clap(about = "Creates a new PooCKP project")]
    New { name: String },
}

fn main() {
    let args = Poockp::parse();

    match &args.subcommands {
        Subcommands::Install { name } => println!("installing {name}"),
        Subcommands::Remove { name } => println!("removing {name}"),
        Subcommands::New { name } => subcommands::new(name.as_str()).unwrap(),
        Subcommands::Init {} => subcommands::init().unwrap(),
    }
}
