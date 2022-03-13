use std::path::PathBuf;

pub fn refresh(path: PathBuf, dependencies: Vec<String>) {
    println!("calculating dependencies for {:?}", dependencies);
}