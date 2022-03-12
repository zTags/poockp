use crate::misc::{question, Config};

use std::io::{Result as Res, Write};
use std::env::current_dir;
use std::process::exit;
use std::fs::File;

use serde_yaml::to_string;

pub fn init() -> Res<()> {
    let cwd = current_dir()?;
    let name = question("What do you want to call this project", cwd.file_name().unwrap().to_str().unwrap(), cwd.file_name().unwrap().to_str().unwrap());
    let version = question("What version is your project", "0.1.0", "0.1.0");
    let is_executable = question("Is your project an executable or a library?", "E/l", "e");
    let entry = question("Where is the entry point of your application", "src/main.spwn", "src/main.spwn");
    let cfg = Config { name, 
        version, 
        is_executable: &is_executable[..1] != "l",
        entry };

    println!("{}", to_string(&cfg).unwrap());
    let ans = question("Is this correct?", "Y/n", "y").to_ascii_lowercase();
    if &ans[..1] == "n" { exit(0); }
    let mut cfg_path = cwd.clone();
    cfg_path.push("poockp.yml");
    let mut cfg_file = File::create(cfg_path).expect("PooCKP project already exists");
    cfg_file.write_all(to_string(&cfg).unwrap().as_bytes())?;

    Ok(())
}