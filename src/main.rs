mod client;
mod user;

use std::{
    env,
    io::{self, Write}
};

use client::{ OpenAIClient };
use user::User;

#[tokio::main]
async fn main() {
    let mut user = match User::load_or_init() {
        Some(existing_user) => existing_user,
        None => {
            let name = prompt_input("Hi, what's your name?");
            User::new(name)
        }
    };

    let base_url = env::var("BASE_URL")
        .expect("Environment variable BASE_URL must be set");
    let api_key = env::var("API_KEY")
        .expect("Environment variable API_KEY must be set");
    let client = OpenAIClient::new(base_url, api_key);

    run_cli_loop(&mut user, &client).await;
}

async fn run_cli_loop(
    user: &mut User,
    client: &OpenAIClient,
) {
    loop {
        println!("\nCommands: profile | rename | exit");
        let cmd = prompt_input(">");

        match cmd.as_str() {
            "profile" => {
                println!("Your name: {}", user.name());
            }

            "rename" => {
                let new_name = prompt_input("Enter new name:");
                user.rename(&new_name);
                println!("Name updated!");
            }

            // "chat" => {
            //     chat_loop(user, client).await;
            // }

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

// async fn chat_loop(
//     user: &user,
//     client: &OpenAIClient,
// ) {

// }

fn prompt_input(message: &str) -> String {
    if message == ">" {
        print!(">");
    } else {
        println!("{message}");
    }
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}
