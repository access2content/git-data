use serde::{Deserialize, Serialize};
use std::io::{self, ErrorKind};
use std::process::Command;
use std::str;

fn get_files() -> io::Result<String> {
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

fn get_commits() -> io::Result<String> {
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

//  Structs to store the output data
#[derive(Serialize, Deserialize, Debug)]
struct Files {
    path: String,
    addition: u32,
    deletion: u32,
}

#[derive(Debug)]
struct FileChanges {
    hash: String,
    files: Vec<Files>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Commit {
    hash: String,
    date: String,
    email: String,
    files: Vec<Files>,
}

fn combine_commit_data() -> Result<Vec<Commit>, Box<dyn std::error::Error>> {
    let mut changed_files = Vec::new();
    let mut commit_details = None;

    //  1. Get File changes
    let file_changes = get_files().and_then(|files| {
        for line in files.lines() {
            if line.len() == 40 {
                if let Some(commit) = commit_details.take() {
                    changed_files.push(commit);
                }
                commit_details = Some(FileChanges {
                    hash: line.to_string(),
                    files: vec![],
                });
            } else if let Some(commit) = commit_details.as_mut() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 3 {
                    let file = Files {
                        addition: parts[0].parse().unwrap_or(0),
                        deletion: parts[1].parse().unwrap_or(0),
                        path: parts[2].to_string(),
                    };
                    commit.files.push(file);
                }
            }
        }

        if let Some(commit) = commit_details {
            changed_files.push(commit);
        }

        Ok(changed_files)
    })?;

    //  2. Get Commit details
    let commit_details = get_commits().and_then(|log| {
        let log_lines: Vec<&str> = log.lines().collect();
        let mut log_iter = log_lines.iter();
        let mut commits = Vec::new();

        for file_change in file_changes {
            if let (Some(&hash), Some(&email), Some(&date)) =
                (log_iter.next(), log_iter.next(), log_iter.next())
            {
                if hash == file_change.hash {
                    commits.push(Commit {
                        hash: hash.to_string(),
                        email: email.to_string(),
                        date: date.to_string(),
                        files: file_change.files,
                    });
                }
            }
        }

        Ok(commits)
    })?;

    Ok(commit_details)
}

fn main() {
    match combine_commit_data() {
        Ok(commits) => {
            for commit in commits {
                println!("{:?}", commit);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
