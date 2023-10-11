use colored::Colorize;
use inquire::Confirm;

use crate::helpers;

pub fn inquire() {
    let mut iterator: u32 = 0;

    loop {
        if 0 < iterator {
            println!("{}", "> Would you like to commit some more?".green());
        }

        helpers::git::stage_files();
        helpers::git::commit();

        let confirm_new_commit = Confirm::new("Do you want to commit some more?")
            .with_default(false)
            .prompt();

        match confirm_new_commit {
            Ok(true) => println!("{}", "> Ok great!".green()),
            Ok(false) => break,
            Err(_) => {
                println!("{}", "> Error, try again later.".red());
                break;
            }
        }

        iterator += 1;
    }

    helpers::git::push();
}