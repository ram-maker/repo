use clap::{Arg, Command as ClapCommand}; // Rename `clap::Command` to `ClapCommand`
use std::fs;
use std::path::Path;
use std::process::{Command, exit}; // Keep `std::process::Command` as `Command`
use dirs::home_dir;

fn main() {
    let matches = ClapCommand::new("repo") // Use `ClapCommand` here
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A CLI tool for automating workflows")
        .subcommand(
            ClapCommand::new("init") // Use `ClapCommand` here
                .about("Initializes the repo directory at ~/repo"),
        )
        .subcommand(
            ClapCommand::new("add") // Use `ClapCommand` here
                .about("Adds a new directory inside ~/repo or a specified parent directory")
                .arg(
                    Arg::new("name")
                        .value_name("NAME")
                        .help("Name of the new directory to add")
                        .required(true),
                )
                .arg(
                    Arg::new("parent")
                        .short('p')
                        .long("parent")
                        .value_name("PARENT")
                        .help("Parent directory inside ~/repo where the new directory should be created"),
                ),
        )
        .subcommand(
            ClapCommand::new("list") // Use `ClapCommand` here
                .about("Lists all subdirectories inside ~/repo or a specified subdirectory")
                .arg(
                    Arg::new("subdir")
                        .value_name("SUBDIR")
                        .help("Subdirectory inside ~/repo to list"),
                ),
        )
        .subcommand(
            ClapCommand::new("home") // Use `ClapCommand` here
                .about("Starts a new shell with the working directory set to ~/repo"),
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        init_repo();
    } else if let Some(matches) = matches.subcommand_matches("add") {
        let dir_name = matches.get_one::<String>("name").unwrap();
        let parent_dir = matches.get_one::<String>("parent").map(|s| s.as_str());
        add_directory(dir_name, parent_dir);
    } else if let Some(matches) = matches.subcommand_matches("list") {
        let subdir = matches.get_one::<String>("subdir").map(|s| s.as_str());
        list_directories(subdir);
    } else if let Some(_) = matches.subcommand_matches("home") {
        repo_home();
    } else {
        println!("No command provided. Use 'repo init', 'repo add', 'repo list', or 'repo home'.");
    }
}

fn init_repo() {
    let repo_dir = home_dir().unwrap().join("repo");

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

    // Create default subdirectories
    let subdirs = ["client", "test", "practice"];
    for subdir in subdirs.iter() {
        let subdir_path = repo_dir.join(subdir);
        match fs::create_dir(&subdir_path) {
            Ok(_) => println!("Created subdirectory: {:?}", subdir_path),
            Err(e) => {
                eprintln!("Failed to create subdirectory {:?}: {}", subdir_path, e);
                exit(1);
            }
        }
    }
}

fn add_directory(dir_name: &str, parent_dir: Option<&str>) {
    let repo_dir = home_dir().unwrap().join("repo");

    // Check if the repo directory exists
    if !repo_dir.exists() {
        eprintln!("Repo directory does not exist. Run 'repo init' first.");
        exit(1);
    }

    // Determine the parent directory
    let parent_dir_path = match parent_dir {
        Some(parent) => repo_dir.join(parent),
        None => repo_dir.clone(),
    };

    // Check if the parent directory exists
    if !parent_dir_path.exists() {
        eprintln!("Parent directory does not exist: {:?}", parent_dir_path);
        exit(1);
    }

    // Get the full path to the new directory
    let new_dir_path = parent_dir_path.join(dir_name);

    // Create the new directory
    match fs::create_dir(&new_dir_path) {
        Ok(_) => println!("Created directory: {:?}", new_dir_path),
        Err(e) => {
            eprintln!("Failed to create directory {:?}: {}", new_dir_path, e);
            exit(1);
        }
    }
}

fn list_directories(subdir: Option<&str>) {
    let repo_dir = home_dir().unwrap().join("repo");

    // Check if the repo directory exists
    if !repo_dir.exists() {
        eprintln!("Repo directory does not exist. Run 'repo init' first.");
        exit(1);
    }

    // Determine the directory to list
    let target_dir = match subdir {
        Some(subdir) => repo_dir.join(subdir),
        None => repo_dir.clone(),
    };

    // Check if the target directory exists
    if !target_dir.exists() {
        eprintln!("Directory does not exist: {:?}", target_dir);
        exit(1);
    }

    // Read the contents of the target directory
    match fs::read_dir(&target_dir) {
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
            eprintln!("Failed to read directory {:?}: {}", target_dir, e);
            exit(1);
        }
    }
}

fn repo_home() {
    let repo_dir = home_dir().unwrap().join("repo");

    // Check if the repo directory exists
    if !repo_dir.exists() {
        eprintln!("Repo directory does not exist. Run 'repo init' first.");
        exit(1);
    }

    // Start a new shell with the working directory set to ~/repo
    let status = Command::new("bash")
        .current_dir(&repo_dir) // Set the working directory to ~/repo
        .status() // Start the shell
        .expect("Failed to start shell");

    // Check the exit status of the shell
    if !status.success() {
        eprintln!("Shell exited with an error.");
        exit(1);
    }
}