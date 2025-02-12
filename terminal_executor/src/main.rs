use clap::command;
use ollama_rs::generation::tools::ToolGroup;
use ollama_rs::history::ChatHistory;
use ollama_rs_tool_macro::generate_ollamars_cmdline_tool_functions;
use ollama_rs::coordinator::Coordinator;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::options::GenerationOptions;
use serde::Serialize;
use termimad::gray;
use termimad::MadSkin;
use tokio;
use ollama_rs::{tool_group, Ollama};
use whoami::fallible;
use std::process::Command;
use std::time::Instant;
use std::time::Duration;
use std::process::Stdio;
use std::io::BufReader;
use std::io::BufRead;
use clap::Parser;
use sysinfo::System;
use uname::uname;
use whoami;

const DEFAULT_MODEL: &str = "cmdline_executor_llama3b:latest";
// building the tool calls
generate_ollamars_cmdline_tool_functions!("/home/andrew/Documents/CmdlineLLMWorkspace/CmdlineExecutor/terminal_executor/tools/tools.json");

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
}

#[derive(Serialize)]
struct SystemInfo {
    os: String,
    kernel: String,
    uptime: String,
    hostname: String,
    cpu: String,
    memory: String,
    user: String,
}

impl SystemInfo {
    pub fn new() -> Self {
        let sys = System::new_all();
        let uname_info = uname().unwrap();
        
        let os = System::long_os_version().unwrap_or_else(|| "Unknown OS".to_string());
        let kernel = uname_info.release;
        let uptime = format!("{} seconds", sysinfo::System::uptime());
        let hostname = fallible::hostname().unwrap_or("unknown".to_string());
        let cpu = sys
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());
        let memory = format!("{:.2} GB", sys.total_memory() as f64 / (1024.0 * 1024.0));
        let user = whoami::username();

        SystemInfo {
            os,
            kernel,
            uptime,
            hostname,
            cpu,
            memory,
            user,
        }
    }
}

/// Perform a single chat reply
async fn do_single_chat_reply<T: ChatHistory, V: ToolGroup>(single_chat: String, coordinator: &mut Coordinator<T, V>) -> Result<(), ollama_rs::error::OllamaError> {
    let resp = coordinator
        .chat(vec![ChatMessage::user(single_chat)])
        .await?;
    println!("{}", &resp.message.content);
    Ok(())
}

/// Perform chat mode
/// This function will loop until the user types "exit"
async fn do_chat_mode<T: ChatHistory, V: ToolGroup>(coordinator: &mut Coordinator<T, V>, skin: &MadSkin) -> Result<(), ollama_rs::error::OllamaError> {
    while let Some(line) = BufReader::new(std::io::stdin()).lines().next() {
        let line = line.unwrap();
        if line == "exit" {
            break;
        }
        let resp = coordinator
            .chat(vec![ChatMessage::user(line)])
            .await?;
        
        skinned_output("---", skin).await;
        skinned_output(&resp.message.content, skin).await;
    }
    Ok(())
}

/// Output a message with a light gray bold font
async fn skinned_output(message: &str, skin: &MadSkin) {
    eprintln!("{}", skin.term_text(message));
}

#[tokio::main]
async fn main() -> Result<(), ollama_rs::error::OllamaError> {
    let args = Args::parse();
    // now we can start the coordinator
    let ollama = Ollama::default();
    let tools = get_functions();
    let history: Vec<ChatMessage> = vec![ChatMessage::system(
        serde_json::to_string(&SystemInfo::new()).unwrap()
    )];
    let mut coordinator = Coordinator::new_with_tools(ollama, String::from(args.model), history, tools)
        .options(GenerationOptions::default()
        .num_ctx(64000))
        .debug(args.debug);

    // setup skin
    let mut skin = MadSkin::default();
    skin.bold.set_fg(gray(20));

    if let Some(single_chat) = args.single_chat {
        do_single_chat_reply(single_chat, &mut coordinator).await?;
        return Ok(());
    }
    // otherwise, we are in chat mode
    do_chat_mode(&mut coordinator, &skin).await?;
    
    Ok(())
}
