mod helpers;
mod services;

fn main() {
    helpers::git::is_repository();

    helpers::ansi::header();
    
    services::prompt::inquire();
}