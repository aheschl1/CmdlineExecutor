use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;
use std::fs;
use syn::{parse_macro_input, LitStr};
use std::env;

#[derive(Deserialize)]
struct CmdlineCommand{
    name: String,
    description: String,
    params: Vec<CmdlineParam>
}

#[derive(Deserialize)]
struct CmdlineParam{
    name: String,
    description: String,
    required: bool,
    positional: bool,
    flag: bool,
    prefix: String
}

/// Generate a set of functions from a JSON file that can be used to execute commands in a tmux session
#[proc_macro]
pub fn generate_ollamars_cmdline_tool_functions(_input: TokenStream) -> TokenStream {
    // Read and parse the JSON file
    // let file_path = parse_macro_input!(input as LitStr).value();
    let use_name = env::var("USER").unwrap_or("failure".to_string());
    let file_path = env::var("TERMINAL_EXECUTOR_TOOLS").unwrap_or(format!("/home/{use_name}/terminal_executor_tools.json"));
    let json_content = fs::read_to_string(&file_path).expect(format!("Failed to read {file_path}").as_str());
    let commands: Vec<CmdlineCommand> = serde_json::from_str(&json_content).expect("Invalid JSON format");

    // Generate function definitions
    let mut generated_code = quote! {};
    let mut function_objects = quote! {};
    let mut function_return_types = quote! {};

    for (i, cmd) in commands.iter().enumerate() {
        let doc_string = format!("{}\n\nParameters:\n{}",cmd.description, cmd.params.iter().map(|param| {
                format!("{}: {}{}", param.name, param.description, if param.required { " (required)" } else { "" })
            }).collect::<Vec<String>>().join("\n"));
        let func_name = syn::Ident::new(&cmd.name, proc_macro2::Span::call_site());
        // we need to accept args for all arguments which are not flags
        // if optional, we need to accept Option<T> instead of T
        let func_args = cmd.params.iter().map(|param| {
            let arg_name = syn::Ident::new(&param.name, proc_macro2::Span::call_site());
            // let arg_type = if param.required {
            //     quote! { String }
            // } else {
            //     quote! { Option<String> }
            // };
            // let arg_type = quote! { String };
            let arg_type = if param.flag {
                quote! { bool }
            } else {
                quote! { String }
            };
            quote! { #arg_name: #arg_type }
        });
        let arg_name_strs = cmd.params.iter().map(|param| {
            param.name.clone()
        }).collect::<Vec<String>>();
        // we need a list of "positiona", "flag", "normal" for each argument
        let func_arg_types = cmd.params.iter().map(|param| {
            if param.positional {
                "positional"
            } else if param.flag {
                "flag"
            } else {
                "normal"
            }
        }).collect::<Vec<&str>>();

        let func_arg_prefixes = cmd.params.iter().map(|param| {
            param.prefix.as_str()
        }).collect::<Vec<&str>>();

        let func_name_str = cmd.name.clone();

        let func_args_vec: Vec<_> = cmd.params.iter().map(|param| {
            let arg_name = syn::Ident::new(&param.name, proc_macro2::Span::call_site());
            // if param.required {
            //     // Required argument is just a String
            //     quote! { #arg_name.to_string() }
            // } else {
            //     // Optional argument is Option<String>
            //     quote! { #arg_name.unwrap_or("".to_string()) }
            // }
            quote! { #arg_name.to_string() }
        }).collect();

        let func_args_vec = quote! { vec![#(#func_args_vec),*] };

        let body = quote!{
            let session_name = #func_name_str;
            let args_names: Vec<&str> = vec![#(#arg_name_strs),*]; // Expand args into a Vec
            let arg_types: Vec<&str> = vec![#(#func_arg_types),*]; // Expand arg_types into a Vec
            let arg_prefixes: Vec<&str> = vec![#(#func_arg_prefixes),*];
            let mut command_to_run: String = format!("{} ", &session_name);

            for (i, a) in #func_args_vec.iter().enumerate(){
                let arg_name = args_names[i];
                let arg_type = arg_types[i];
                let arg_prefix = arg_prefixes[i];
                if arg_type == "flag" && (String::from(a)=="false"){continue;}
                if a.len() == 0 {continue;}
                else{
                    if arg_type == "flag"{
                        command_to_run.push_str(&format!("{}{} ", arg_prefix, arg_name));
                    }else if arg_type == "positional"{
                        command_to_run.push_str(&format!("{} ", a));
                    }else{
                        // -name val
                        command_to_run.push_str(&format!("{}{} {} ", arg_prefix, arg_name, a)); 
                    }
                }
            };

            let status = Command::new("tmux")
                .arg("new-session")
                .arg("-d")  // Start detached
                .arg("-s")
                .arg(session_name)
                .arg("bash")
                .arg("-c")
                .arg(format!("{}; exec bash", command_to_run)) // Ensure command keeps shell open
                .status();

            println!("Running: {}", command_to_run);
    
            match status {
                Ok(_) => {        
                    // Wait a bit before capturing output to allow command to generate output
                    std::thread::sleep(Duration::from_secs(1));
        
                    let capture_output = Command::new("tmux")
                        .arg("capture-pane")
                        .arg("-p") // Get content of the tmux pane
                        .arg("-t")
                        .arg(session_name)
                        .stdout(Stdio::piped())
                        .spawn();
        
                    match capture_output {
                        Ok(mut child) => {
                            let stdout = child.stdout.take().expect("Failed to capture stdout");
                            let reader = BufReader::new(stdout);
        
                            let start_time = Instant::now();
                            let timeout = Duration::new(40, 0); // 8 seconds
                            let mut output = String::new();
                            let mut did_timeout = false;
        
                            for line in reader.lines() {
                                if let Err(_) = line {
                                    output.push_str("<system>launch success, but failed to read line from stdout. </system>\n");
                                } else {
                                    let line = line.unwrap();
                                    output.push_str(&line);
                                    output.push('\n');
                                }
        
                                if start_time.elapsed() > timeout {
                                    did_timeout = true;
                                    output.push_str("<system>launch success, but timed out stdout/stderr. </system>\n");
                                    break;
                                }
                            }
                            
                            if !did_timeout{
                                let _ = Command::new("tmux")
                                    .arg("kill-session")
                                    .arg("-t")
                                    .arg(session_name)
                                    .status();
                            } else{
                                output.push_str("<system> command is still running. </system>\n");
                            }
                            return Ok(format!("<system>command: {} output: {} </system>", command_to_run, output));
                        }
                        Err(e) => return Ok(format!("<system>command launched, but failed to capture stdout.</system>")),
                    }
                }
                Err(e) => return Ok(format!("<system>failed to launch tmux session: {:?}</system>", e)),
            }

        };
        let func_args_tokens: proc_macro2::TokenStream = quote! { #( #func_args ),* };
        generated_code = quote! {
            #generated_code
            #[doc=#doc_string]
            #[ollama_rs::function]
            pub async fn #func_name(#func_args_tokens) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
                #body
            }
        };
        function_objects = quote! {
            #func_name,
            #function_objects
        };

        if commands.len() > 1 && i > 0{    
            function_return_types = quote! {
                (#func_name, #function_return_types)
            };
        }else{
            function_return_types = quote! {
                #func_name
            };
        }
        
    }

    generated_code = quote! {
        pub fn get_functions() -> #function_return_types{
            return tool_group![#function_objects];
        }
        // let tools = tool_group!(vec![#function_objects]);
        #generated_code
    };
    generated_code.into()
}
