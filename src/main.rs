mod combine;
mod git_commands;
mod models;
mod operations;

fn main() {
    //  Get all Commits
    let all_commits = combine::combine_commit_data();

    match all_commits {
        Ok(commits) => {
            //  1. Get different committers with commit count
            let committers = operations::get_committers(&commits);
            for (email, count) in committers.clone() {
                println!("{} has {} commits", email, count);
            }

            //  Get counts of a Committer
            match committers.get("access2content@gmail.com") {
                Some(&value) => println!(
                    "The value for '{}' is {}",
                    "access2content@gmail.com", value
                ),
                None => println!("{} is not in the map", "access2content@gmail.com"),
            }

            //  2. Get committers to different files
            let files = operations::map_file_to_committers(&commits);
            for (file, committers_vec) in &files {
                print!("File: {} -> ", file);
                for committer_map in committers_vec {
                    for (email, count) in committer_map {
                        print!("- {}: {}", email, count);
                    }
                }
                println!();
            }

            //  Get list of committers for a file
            match files.get("src/main.rs") {
                Some(value) => {
                    for committer_map in value {
                        for (email, count) in committer_map {
                            print!("- {}: {}", email, count);
                        }
                    }
                }
                None => println!("No committers"),
            }
        }
        Err(e) => eprintln!("Some error{}", e),
    }
}
