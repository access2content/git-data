use serde::{Deserialize, Serialize};

//  Structs to store the output data
#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub path: String,
    pub addition: u32,
    pub deletion: u32,
}

#[derive(Debug)]
pub struct FileChanges {
    pub hash: String,
    pub files: Vec<Files>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub hash: String,
    pub date: String,
    pub email: String,
    pub files: Vec<Files>,
}
