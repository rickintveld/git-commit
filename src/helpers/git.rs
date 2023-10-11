use colored::Colorize;
use inquire::{Confirm, MultiSelect, Text};
use std::{
    env, fs,
    io::{Read, Write},
    process::{Command, Stdio},
    str,
};

pub fn is_repository() {
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

    if !has_git {
        println!("{}", "This is not a git repository!".red().italic());
        std::process::exit(0);
    }
}

pub fn stage_files() {
    let mut cmd_git_status = Command::new("git")
        .args(["status", "-s"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut cmd_cut = Command::new("cut")
        .arg("-c4-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(ref mut stdout) = cmd_git_status.stdout {
        if let Some(ref mut stdin) = cmd_cut.stdin {
            let mut buf: Vec<u8> = Vec::new();
            stdout.read_to_end(&mut buf).unwrap();
            stdin.write_all(&buf).unwrap();
        }
    }

    let output = cmd_cut.wait_with_output().unwrap().stdout;

    let mut stdout = String::new();

    stdout.push_str(match str::from_utf8(&output) {
        Ok(files) => files,
        Err(_) => panic!("No changed git files found!"),
    });

    let options: Vec<&str> = stdout.split("\n").collect();
    let selection = MultiSelect::new("Select the files you want to commit: ", options).prompt();
    let files = selection.unwrap();

    for file in files {
        Command::new("git")
            .current_dir(env::current_dir().unwrap())
            .args(["add", file])
            .output()
            .expect(format!("Failed to stage file: {}", file.red()).as_str());
    }
}

pub fn commit() {
    let message: String = commit_message();

    Command::new("git")
        .current_dir(env::current_dir().unwrap())
        .args(["commit", "-m", message.as_str()])
        .output()
        .expect(format!("{}", "Failed to commit these files".red()).as_str());

    println!("{}", "> The selected files are committed".green());
}

fn commit_message() -> String {
    let message_input = Text::new("Enter commit message: ").prompt();
    let message = message_input.unwrap();

    message
}

pub fn push() {
    let confirm_push = Confirm::new("Do you want to push the commits?")
        .with_default(false)
        .prompt();

    match confirm_push {
        Ok(true) => println!("{}", "> Ok great!".green()),
        Ok(false) => {
            println!("{}", "> Ok sure, process will be terminated.".blue());
            std::process::exit(0);
        }
        Err(_) => {
            println!("{}", "> Error, try again later.".red());
            std::process::exit(0);
        }
    }

    let output = Command::new("git")
        .current_dir(env::current_dir().unwrap())
        .arg("push")
        .output()
        .expect(format!("{}", "Failed to push these commits".red()).as_str());

    println!("{}", "> The commits are pushed!".green());

    let stdout: String = String::from_utf8(output.stdout).unwrap();
    println!("{}", stdout);
}