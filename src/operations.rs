use crate::models::Commit;
use std::collections::HashMap;

pub fn get_committers(commits: &[Commit]) -> HashMap<String, u32> {
    let mut contributor_map = HashMap::new();

    for commit in commits {
        *contributor_map.entry(commit.email.clone()).or_insert(0) += 1;
    }

    contributor_map
}

pub fn map_file_to_committers(commits: &[Commit]) -> HashMap<String, Vec<HashMap<String, u32>>> {
    let mut file_committer_map: HashMap<String, Vec<HashMap<String, u32>>> = HashMap::new();

    for commit in commits {
        for file in &commit.files {
            let file_entry = file_committer_map
                .entry(file.path.clone())
                .or_insert_with(Vec::new);

            // Check if we already have an entry for this committer
            let mut found = false;
            for committer_map in file_entry.iter_mut() {
                if let Some(count) = committer_map.get_mut(&commit.email) {
                    *count += 1;
                    found = true;
                    break;
                }
            }

            // If not found, add a new entry
            if !found {
                let mut new_committer_map = HashMap::new();
                new_committer_map.insert(commit.email.clone(), 1);
                file_entry.push(new_committer_map);
            }
        }
    }

    file_committer_map
}
