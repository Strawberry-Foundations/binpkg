use serde::{Deserialize, Serialize};
use crate::utils::open_specfile;

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Specfile {
    pub package: SpecfilePackage
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct SpecfilePackage {
    pub name: String,
    pub id: String,
    pub version: String,
    pub architecture: String,
    pub author: String,
    pub description: String,
}

impl Specfile {
    pub fn from_file(path: &str) -> Self {
        let specfile_content = open_specfile(path);
        serde_yaml::from_str(&specfile_content).unwrap()
    }
}