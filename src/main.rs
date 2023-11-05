mod combine;
mod git_commands;
mod models;

fn main() {
    match combine::combine_commit_data() {
        Ok(commits) => {
            for commit in commits {
                println!("{:?}", commit);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
