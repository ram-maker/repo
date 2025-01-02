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
                .about("Initializes the repo directory at ~/repo"),
        )
        .subcommand(
            Command::new("add")
                .about("Adds a subdirectory inside ~/repo")
                .arg(
                    Arg::new("name")
                        .value_name("NAME")
                        .help("Name of the new subdirectory to add")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("Lists all subdirectories inside ~/repo"),
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        init_repo();
    } else if let Some(matches) = matches.subcommand_matches("add") {
        let subdir_name = matches.get_one::<String>("name").unwrap();
        add_subdirectory(subdir_name);
    } else if let Some(_) = matches.subcommand_matches("list") {
        list_subdirectories();
    } else {
        println!("No command provided. Use 'repo init', 'repo add', or 'repo list'.");
    }
}

fn init_repo() {
    let repo_dir = dirs::home_dir().unwrap().join("repo");

    // Check if the repo directory already exists
    if repo_dir.exists() {
        println!("Repo directory already exists at: {:?}", repo_dir);
        exit(0);
    }

    // Create the repo directory
    match fs::create_dir(&repo_dir) {
        Ok(_) => println!("Repo directory created at: {:?}", repo_dir),
        Err(e) => {
            eprintln!("Failed to create repo directory: {}", e);
            exit(1);
        }
    }
}

fn add_subdirectory(subdir_name: &str) {
    let repo_dir = dirs::home_dir().unwrap().join("repo");

    // Check if the repo directory exists
    if !repo_dir.exists() {
        eprintln!("Repo directory does not exist. Run 'repo init' first.");
        exit(1);
    }

    // Get the full path to the new subdirectory
    let new_subdir_path = repo_dir.join(subdir_name);

    // Create the new subdirectory
    match fs::create_dir(&new_subdir_path) {
        Ok(_) => println!("Created subdirectory: {:?}", new_subdir_path),
        Err(e) => {
            eprintln!("Failed to create subdirectory {:?}: {}", new_subdir_path, e);
            exit(1);
        }
    }
}

fn list_subdirectories() {
    let repo_dir = dirs::home_dir().unwrap().join("repo");

    // Check if the repo directory exists
    if !repo_dir.exists() {
        eprintln!("Repo directory does not exist. Run 'repo init' first.");
        exit(1);
    }

    // Read the contents of the repo directory
    match fs::read_dir(&repo_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        println!("{}", path.file_name().unwrap().to_string_lossy());
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read repo directory: {}", e);
            exit(1);
        }
    }
}