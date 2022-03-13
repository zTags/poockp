use crate::misc::{question, Config};

use std::env::current_dir;
use std::fs::{create_dir, create_dir_all, File};
use std::io::{Result as Res, Write};
use std::path::PathBuf;
use std::process::exit;

use serde_yaml::to_string;

pub fn new(name: &str) -> Res<()> {
    let mut path = current_dir()?;
    path.push(name);

    // ask user questions
    let name = question(
        "What do you want to call this project",
        path.file_name().unwrap().to_str().unwrap(),
        path.file_name().unwrap().to_str().unwrap(),
    );
    let version = question("What version is your project", "0.1.0", "0.1.0");
    let is_executable = question("Is your project an executable or a library?", "E/l", "e");
    let entry = question(
        "Where is the entry point of your application",
        "src/main.spwn",
        "src/main.spwn",
    );

    // generate config
    let cfg = Config {
        name,
        version,
        is_executable: &is_executable[..1] != "l",
        entry: entry.clone(),
        dependencies: Vec::default(),
    };

    println!("{}", to_string(&cfg).unwrap());
    let ans = question("Is this correct?", "Y/n", "y").to_ascii_lowercase();
    if &ans[..1] == "n" {
        exit(0);
    }
    // create files
    create_dir(&path)?;
    path.push("poockp.yml");
    let mut cfg_file = File::create(&path).expect("PooCKP project already exists");
    cfg_file.write_all(to_string(&cfg).unwrap().as_bytes())?;

    // TODO: holy shit this sucks
    path.pop();
    let mut p = path.clone().to_str().unwrap().to_string();
    p.push('/');
    p.push_str(entry.as_str());
    #[cfg(target_os = "windows")]
    let p_ = p.replacen('/', "\\", usize::MAX);
    let pa = PathBuf::from(p_);
    let mut pat = pa.clone();
    pat.pop();
    create_dir_all(pat)?;
    let mut f = File::create(pa)?;
    f.write_all(b"$.print(\"Hello, world!\")")?;

    Ok(())
}
