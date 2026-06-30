use std::{
    fs::{self}, 
    io::{self, Write}
};

fn main() {
    let mut name = load_or_init_user();
    run_cli_loop(&mut name);
}


fn load_or_init_user() -> String {
    match fs::read_to_string("names.txt") {
        Ok(name) => {
            let name = name.trim().to_string();
            println!("Welcome back, {name}!");
            name
        }
        Err(_) => {
            println!("Hi, what's your name?");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let name = input.trim().to_string();

            fs::write("names.txt", &name)
                .expect("Failed to write file");

            println!("Hello, {name}!");
            name
        }
    }
}


fn run_cli_loop(name: &mut String) {
    loop {
        println!("\nCommands: profile | rename | exit");
        print!("> ");
        io::stdout().flush().unwrap();

        // Get command
        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");
        let cmd = cmd.trim();

        match cmd {
            "profile" => {
                println!("Your name: {name}");
            }

            "rename" => {
                println!("Enter new name:");

                let mut new_name = String::new();
                io::stdin()
                    .read_line(&mut new_name)
                    .expect("Failed to read line");
                let new_name = new_name.trim().to_string();

                *name = new_name.clone();

                fs::write("names.txt", &name)
                    .expect("Failed to write file");

                println!("Name updated!");
            }

            "exit" => {
                println!("Bye!");
                break;
            }

            _ => {
                println!("Unknown command");
            }
        }
    }
}