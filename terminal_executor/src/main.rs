use std::env;
use ollama_rs_tool_macro::generate_ollamars_cmdline_tool_functions;
use ollama_rs::coordinator::Coordinator;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::options::GenerationOptions;
use termimad::gray;
use termimad::MadSkin;
use tokio;
use ollama_rs::{tool_group, Ollama};
use std::process::Command;
use std::time::Instant;
use std::time::Duration;
use std::process::Stdio;
use std::io::BufReader;
use std::io::BufRead;

generate_ollamars_cmdline_tool_functions!("/home/andrew/Documents/CmdlineLLMWorkspace/CmdlineExecutor/terminal_executor/tools/tools.json");

fn get_model_name() -> String{
    match env::var("CMDLINE_LLM_MODEL") {
        Ok(val) => val,
        Err(_) => "cmdline_executor_llama3b:latest".to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), ollama_rs::error::OllamaError> {
    let model = get_model_name();
    let ollama = Ollama::default();


    let tools = get_functions();
    let history: Vec<ChatMessage> = vec![];
    let mut coordinator = Coordinator::new_with_tools(ollama, model, history, tools)
        .options(GenerationOptions::default().num_ctx(64000)).debug(true);

    
    // start with the default skin
    let mut skin = MadSkin::default();
    // let's decide bold is in light gray
    skin.bold.set_fg(gray(20));
    // collect inputs and send
    while let Some(line) = BufReader::new(std::io::stdin()).lines().next() {
        let line = line.unwrap();
        if line == "exit" {
            break;
        }
        let resp = coordinator
            .chat(vec![ChatMessage::user(line)])
            .await?;
        
        eprintln!("{}", skin.term_text("---"));
        eprintln!("{}", skin.term_text(&resp.message.content));
    }
    Ok(())
}
