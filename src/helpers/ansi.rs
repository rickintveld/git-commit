use colored::Colorize;

pub fn header() {
    let ansi_art = "
       __________  __  _____  _____________
      / ____/ __ \\/  |/  /  |/  /  _/_  __/
     / /   / / / / /|_/ / /|_/ // /  / /   
    / /___/ /_/ / /  / / /  / // /  / /    
    \\____/\\____/_/  /_/_/  /_/___/ /_/         
    "
    .green();

    println!("{}", ansi_art);
}