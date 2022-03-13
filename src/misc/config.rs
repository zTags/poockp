use std::env::current_dir;
use std::fs::read_to_string;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use serde_yaml::from_str;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub entry: String,
    pub is_executable: bool,
    pub dependencies: Vec<String>,
}

pub fn load_config() -> Option<(Config, PathBuf)> {
    let mut path = current_dir().expect("Failed to obtain current working directory").to_str().unwrap().to_string();
    
    loop {
        let mut p = PathBuf::from(path.clone());
        p.push("poockp.yml");
        if p.exists() {
            let cfg_text = read_to_string(&p).expect("failed to read poockp.yml");
            let cfg: Config = from_str(&cfg_text).expect("data deserialization failed");
            return Some((cfg, p));
        } else {
            p.pop();
            p.pop();
            let path_str = p.to_str().unwrap().to_string();
            if &path_str[1..] == ":\\" || path_str == "/" {
                break;
            }
            path = p.to_str().unwrap().to_string();
        }
    }

    None
}