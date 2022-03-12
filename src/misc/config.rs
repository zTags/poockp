use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub entry: String,
    pub is_executable: bool,
}
