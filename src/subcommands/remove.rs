use crate::misc::{load_config, refresh};

use serde_yaml::to_string;

use std::fs::write;

pub fn remove(pkgs: Vec<String>) {
    let (mut cfg, path) = load_config().expect("failed to locate config");
    for pkg in pkgs {
        if cfg.dependencies.contains(&pkg) {
            let index = cfg.dependencies.iter().position(|x| *x == pkg).unwrap();
            cfg.dependencies.remove(index);
        };
    }
    write(&path, to_string(&cfg).expect("failed to deserialize cfg")).expect("failed to write poockp.yml");
    refresh(path, cfg.dependencies);
}