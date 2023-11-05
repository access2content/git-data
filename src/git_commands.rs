use std::io::{self, ErrorKind};
use std::process::Command;
use std::str;

pub fn get_files() -> io::Result<String> {
    let output = Command::new("git")
        .args(["log", "--pretty=format:%H", "--numstat"])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(ErrorKind::Other, "Git command failed"));
    }

    match str::from_utf8(&output.stdout) {
        Ok(output_str) => Ok(output_str.to_string()),
        Err(e) => Err(io::Error::new(ErrorKind::InvalidData, e)),
    }
}

pub fn get_commits() -> io::Result<String> {
    let output = Command::new("git")
        .args(["log", "--pretty=format:%H\n%aE\n%ad"])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(ErrorKind::Other, "Git command failed"));
    }

    match str::from_utf8(&output.stdout) {
        Ok(output_str) => Ok(output_str.to_string()),
        Err(e) => Err(io::Error::new(ErrorKind::InvalidData, e)),
    }
}
