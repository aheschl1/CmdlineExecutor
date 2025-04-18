use clap::command;
use ollama_rs::generation::tools::ToolGroup;
use ollama_rs::history::ChatHistory;
use ollama_rs::coordinator::Coordinator;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::options::GenerationOptions;
use termimad::gray;
use termimad::MadSkin;
use tokio;
use ollama_rs::Ollama;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use clap::Parser;
use url::Url;
mod utils;
mod tools;

const DEFAULT_MODEL: &str = "gemma12b_commandline_exec:latest";
const DEFAULT_CONTEXT: u32 = 16000;

/// Perform a single chat reply
/// This function will chat with the model once and print the response
/// 
/// # Arguments
/// * `single_chat` - The chat message to send
/// * `coordinator` - The coordinator to use
/// * `skin` - The skin to use for output
async fn do_single_reply<T: ChatHistory, V: ToolGroup>(
    single_chat: String, 
    coordinator: &mut Coordinator<T, V>, 
    skin: &MadSkin
) -> Result<(), ollama_rs::error::OllamaError> {
    let resp = coordinator
        .chat(vec![ChatMessage::user(single_chat)])
        .await?;
    skinned_output(&resp.message.content, skin).await;
    Ok(())
}

/// Perform chat mode
/// This function will loop until the user types "exit"
/// 
/// # Arguments
/// * `coordinator` - The coordinator to use
/// * `skin` - The skin to use for output
async fn do_chat_mode<T: ChatHistory, V: ToolGroup>(
    coordinator: &mut Coordinator<T, V>, 
    skin: &MadSkin
) -> Result<(), ollama_rs::error::OllamaError> {
    print!("You: ");
    std::io::stdout().flush().unwrap();

    while let Some(line) = BufReader::new(std::io::stdin()).lines().next() {
        let line = line.unwrap();
        if line == "exit" {
            break;
        }
        let resp = coordinator
            .chat(vec![ChatMessage::user(line)])
            .await;
        if let Err(e) = resp {
            println!("Error: {:?}", e);
            print!("You: ");
            std::io::stdout().flush().unwrap();
            continue;
        }
        skinned_output("---", skin).await;
        skinned_output(&resp.unwrap().message.content, skin).await;
        print!("You: ");
        std::io::stdout().flush().unwrap();
    }
    Ok(())
}

/// Output a message with a skin
async fn skinned_output(message: &str, skin: &MadSkin) {
    eprintln!("{}", skin.term_text(message));
}


#[derive(Parser, Debug)]
#[command(name = "terminalExecutor", about = "A command-line tool for llm interaction with linux machine")]
struct Args {
    /// Single chat message (optional)
    #[arg(short = 's', long = "single_chat")]
    single_chat: Option<String>,
    /// Debug mode (optional flag)
    #[arg(short = 'd', long = "debug")]
    debug: bool,
    /// Model selection (optional)
    #[arg(short = 'm', long = "model", default_value = DEFAULT_MODEL)]
    model: String,
    /// Context size (optional)
    #[arg(short = 'c', long = "context", default_value_t = DEFAULT_CONTEXT)]
    context: u32,
}

fn get_ollama_url() -> String {
    let host = env::var("OLLAMA_HOST").unwrap_or("127.0.0.1".to_string());
    let port: u32 = host.split(":").collect::<Vec<&str>>().pop().unwrap().parse().unwrap_or(11434);
    let url = format!("http://{}:{}", host, port);
    return url;
}

#[tokio::main]
async fn main() -> Result<(), ollama_rs::error::OllamaError> {
    let args = Args::parse();
    // now we can start the coordinator
    let url = Url::parse(get_ollama_url().as_str()).unwrap();
    let ollama = Ollama::from_url(url);
    println!("Using model: {}", args.model);
    let history: Vec<ChatMessage> = vec![
        ChatMessage::system(serde_json::to_string(&utils::SystemInfo::new()).unwrap())
    ];
    let mut coordinator = Coordinator::new_with_tools(ollama, String::from(args.model), history, tools::get_functions())
        .options(GenerationOptions::default()
        .temperature(0.0)
        .num_ctx(args.context as u64))
        .debug(args.debug);

    // setup skin
    let mut skin = MadSkin::default();
    skin.bold.set_fg(gray(20));

    match args.single_chat {
        Some(single_chat) => {
            do_single_reply(single_chat, &mut coordinator, &skin).await?;
        },
        None => {
            do_chat_mode(&mut coordinator, &skin).await?;
        }
    }
    Ok(())
}
