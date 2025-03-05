use std::process::Command;
use std::time::{Instant, Duration};
use std::process::Stdio;
use ollama_rs::tool_group;
use ollama_rs_tool_macro::generate_ollamars_cmdline_tool_functions;
use std::io::BufReader;
use std::io::BufRead;

// This macro expands to numerous structs and functions which are defined in the json file
// This macro also exposes get_functions() which returns the ollama-rs ToolGroup
generate_ollamars_cmdline_tool_functions!("/home/andrewheschl/Documents/CmdlineExecutor/terminal_executor/tools/tools.json");
