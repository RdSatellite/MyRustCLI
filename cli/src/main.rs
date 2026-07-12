mod character;
mod dialogue;

use std::{
    env,
    io::{self, Write},
};

use character::CharacterSystem;
use dialogue::DialogueSystem;

#[tokio::main]
async fn main() {
    let backend_url = env::var("LLM_INET")
        .expect("Environment variable LLM_INET must be set");

    let mut dialogue = DialogueSystem::new(backend_url.clone());

    let mut character = match CharacterSystem::load_or_init(backend_url.clone()) {
        Some(existing) => existing,
        None => {
            let name = prompt_input("Hi, what's your name?");
            CharacterSystem::init(backend_url, name)
        }
    };

    run_cli_loop(&mut character, &mut dialogue).await;
}

async fn run_cli_loop(character: &mut CharacterSystem, dialogue: &mut DialogueSystem) {
    loop {
        println!("\nCommands: chat | profile | rename | exit");
        let cmd = prompt_input("> ");

        match cmd.as_str() {
            "chat" => {
                chat_loop(character, dialogue).await;
            }

            "profile" => {
                println!("Your name: {}", character.name());
                println!("Profile:\n{}", character.profile_summary());
            }

            "rename" => {
                let new_name = prompt_input("Enter new name:");
                character.rename(&new_name);
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

async fn chat_loop(character: &mut CharacterSystem, dialogue: &mut DialogueSystem) {
    println!();
    println!("// --- Chat with AI: Nice to see you! --- //");
    println!("Say 'exit' to end the session");

    loop {
        let prompt = prompt_input("> ");

        if prompt.to_lowercase() == "exit" {
            println!("See you later!");

            // Analyze the conversation to update the character profile.
            if let Err(e) = character.analyze_conversation(dialogue.history()).await {
                eprintln!("Warning: profile analysis failed: {e}");
            }

            break;
        }

        if prompt.is_empty() {
            continue;
        }

        print!("\n[{}]\n", String::from("Assistant"));
        print!("Thinking...");
        io::stdout().flush().unwrap();

        match dialogue.send(&prompt).await {
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
