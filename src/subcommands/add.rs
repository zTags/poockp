use crate::misc::{load_config, refresh};

use serde_yaml::to_string;

use std::fs::write;

pub fn add(pkgs: Vec<String>) {
    let (mut cfg, path) = load_config().expect("failed to locate config");
    for pkg in pkgs {
        if cfg.dependencies.contains(&pkg) {
            continue;
        }
        cfg.dependencies.push(pkg);
    }
    write(&path, to_string(&cfg).expect("failed to deserialize cfg")).expect("failed to write poockp.yml");
    refresh(path, cfg.dependencies);
}