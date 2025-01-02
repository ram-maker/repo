use clap::{Arg, Command};
use std::fs;
use std::path::Path;
use std::process::exit;

fn main() {
    let matches = Command::new("repo")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A CLI tool for automating workflows")
        .subcommand(
            Command::new("init")
                .about("Initializes the repo directory at ~/repo")
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        init_repo();
    } else {
        println!("No command provided. Use 'repo init' to initialize the repo directory.");
    }
}

fn init_repo() {
    let repo_dir = dirs::home_dir().unwrap().join("repo");

    // Check if the directory already exists
    if repo_dir.exists() {
        println!("Repo directory already exists at: {:?}", repo_dir);
        exit(0);
    }

    // Create the directory
    match fs::create_dir(&repo_dir) {
        Ok(_) => println!("Repo directory created at: {:?}", repo_dir),
        Err(e) => {
            eprintln!("Failed to create repo directory: {}", e);
            exit(1);
        }
    }
}