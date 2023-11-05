use std::io::{self, Read};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    // Spawn the `git log` command
    let mut child = Command::new("git")
        .arg("log")
        .stdout(Stdio::piped()) // Ensure we capture the standard output
        .spawn()?;

    // Capture the output
    if let Some(mut stdout) = child.stdout.take() {
        let mut output = String::new();
        stdout.read_to_string(&mut output)?;

        println!("`Data from Git: `\n{}", output);
    }

    // Wait for the command to finish
    let status = child.wait()?;
    println!("Command completed with status: {}", status);

    Ok(())
}
