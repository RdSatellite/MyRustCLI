mod llm;
mod user;

use std::{
    env, 
    io::{self, Write},
    collections::HashMap
};

use llm::{ LLMClient, OpenAIClient };
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
        println!("\nCommands: chat | profile | rename | exit");
        let cmd = prompt_input("> ");

        match cmd.as_str() {
            "chat" => {
                chat_loop(user, client).await;
            }

            "profile" => {
                println!("Your name: {}", user.name());
            }

            "rename" => {
                let new_name = prompt_input("Enter new name:");
                user.rename(&new_name);
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

async fn chat_loop(
    user: &User,
    client: &OpenAIClient,
) {
    println!();
    println!("// --- Chat with AI: Nice to see you! --- //");
    println!("Say 'exit' to end the session");

    loop {
        let prompt = prompt_input("> ");

        if prompt.to_lowercase() == "exit" {
            println!("See you later!");
            break;
        }

        if prompt.is_empty() {
            continue;
        }

        print!("\n[{}]\n", String::from("Assistant"));
        print!("Thinking...");
        io::stdout().flush().unwrap();

        match client.invoke(&prompt).await {
            Ok(reply) => {
                print!("\r");
                println!("{reply}\n");
            }
            Err(e) => {
                print!("\r");
                println!("Error calling LLM: {e}");
            }
        }
    }
}

fn prompt_input(message: &str) -> String {
    if message == "> " {
        print!("> ");
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
