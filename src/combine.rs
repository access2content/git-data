use crate::git_commands::{get_commits, get_files};
use crate::models::{Commit, FileChanges, Files};

pub fn combine_commit_data() -> Result<Vec<Commit>, Box<dyn std::error::Error>> {
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
