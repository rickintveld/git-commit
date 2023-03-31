use colored::Colorize;
use inquire::{MultiSelect, Text};
use std::{env, fs, process::Command, str};

fn main() {
    let has_git: bool = is_git_repository();

    if !has_git {
        println!("{}", "This is not a git repository!".red().italic());
        std::process::exit(0);
    }

    let ansi_art = "
       __________  __  _____  _____________
      / ____/ __ \\/  |/  /  |/  /  _/_  __/
     / /   / / / / /|_/ / /|_/ // /  / /   
    / /___/ /_/ / /  / / /  / // /  / /    
    \\____/\\____/_/  /_/_/  /_/___/ /_/         
    "
    .green();

    println!("{}", ansi_art);

    stage_files();
    commit();
}

fn is_git_repository() -> bool {
    let mut has_git = false;
    let paths = fs::read_dir(env::current_dir().unwrap()).unwrap();

    for path in paths {
        let dir_entry = path.unwrap();
        if dir_entry.path().is_file() {
            continue;
        }

        if dir_entry.file_name() == ".git" {
            has_git = true;
        }
    }

    has_git
}

fn stage_files() {
    let output = Command::new("git")
        .current_dir(env::current_dir().unwrap())
        .arg("diff")
        .arg("--name-only")
        .output()
        .unwrap();

    let mut stdout = String::new();

    stdout.push_str(match str::from_utf8(&output.stdout) {
        Ok(files) => files,
        Err(_) => panic!("No changed git files found!"),
    });

    let options: Vec<&str> = stdout.split("\n").collect();

    let selection = MultiSelect::new("Select the files you want to commit:", options).prompt();

    let files = selection.unwrap();

    for file in files {
        Command::new("git")
            .current_dir(env::current_dir().unwrap())
            .args(["add", file])
            .output()
            .expect(format!("Failed to stage file: {}", file.red()).as_str());
    }
}

fn commit() {
    let message: String = commit_message();

    Command::new("git")
        .current_dir(env::current_dir().unwrap())
        .args(["commit", "-m", message.as_str()])
        .output()
        .expect(format!("{}", "Failed to commit these files".red()).as_str());
}

fn commit_message() -> String {
    let message_input = Text::new("Enter commit message").prompt();
    let message = message_input.unwrap();

    message
}
