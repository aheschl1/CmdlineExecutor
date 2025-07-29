# Command Line LLM Executor

This repo provides an Ollama wrapper for running informational tools in the command line using an LLM.

Tools are defined in json, and are compiled down using a macro, and passed through to the ollama-rs api.

# Setup

To give a list of tools, you can do one of two things:

1) Set the path to the tool json in the environment variable ```TERMINAL_EXECUTOR_TOOLS```
2) Place the tools json at ```/home/$USER/terminal_executor_tools.json```

Sample tools can be found in ![tools.json](./terminal_executor/tools/tools.json)

Recommended: Do not use tools which can modify the system, as the LLM may hallucinate and cause unintended consequences.

# Ollama

Ensure that you have ollama setup and running

# Model

You can specify any model with the -m flag; however, the recommended model is gemma3, with tool support.

Two modelfiles are provided in the ```models``` directory, which have system prompts defined.

Smaller 1b models are fast, but may hallucinate more. The 1b model is recommended for testing, or use without GPU.

# Running

To chat, use the following command:

```shell
terminalExecutor -m <ollama_model_name> -c <contextt_length> 
```

To run a single commans use the following command:

```shell
terminalExecutor -m <ollama_model_name> -c <contextt_length> -s <query>
``` 

To get full options use the following command:

```shell
terminalExecutor --help
```

# Example of Rust Version

```
(/CmdlineExecutor/terminal_executor) $ terminalExecutor -m PetrosStav/gemma3-tools:12b -c 16000 # or cargo run -- -m PetrosStav/gemma3-tools:12b -c 16000

Using model: PetrosStav/gemma3-tools:12b
You: lmk what is in my home directory
Running: ls /home/andrewheschl -l -a 
――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

Okay, here's a summary of the contents of your home directory, based on the ls output:

•   .stunnel-client.conf: A configuration file for stunnel.
•   .sudo_as_admin_successful: A file related to sudo administration.
•   .swt: A directory, likely related to Eclipse or SWT.
•   .temp.txt.swo and .temp.txt.swp: Temporary swap files, likely from a text editor.
•   terminal_executor_tools.json: A JSON file, likely related to a terminal executor tool.
•   test.hs: A Haskell source file.
•   .todo.txt.swp: Another temporary swap file.
•   UnityProjects: A directory containing Unity projects.
•   Videos: A directory for videos.
•   .vim: A directory for Vim configuration files.
•   .viminfo: A Vim information file.
•   .vimrc: A Vim configuration file.
•   VirtualBox VMs: A directory for VirtualBox virtual machines.
•   .vscode: A directory for Visual Studio Code configuration files.
•   .wget-hsts: A file related to wget's HTTPS settings.
•   .yarn: A directory for Yarn package manager files.
•   .yarnrc: A Yarn configuration file.
•   ystemctl docker: A file, name seems incomplete.
•   .zoom: A directory for Zoom application data.
•   Several hidden directories and files (starting with a .) which are configuration files for various applications and tools.

Do you want me to elaborate on any of these items, or perhaps check the contents of a specific directory?

You: can yoou tell me what aliases exist? 

――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

Running: cat ~/.bashrc 
――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

Okay, based on the output of cat ~/.bashrc, here are the aliases defined in your .bashrc file:

•   server: ssh -p 2222 andrew@<redacted> - This alias likely connects to a server via SSH on port 2222.
•   vpn_connect: sudo wg-quick up wg-client - This alias connects to a VPN using WireGuard.
•   vpn_disconnect: sudo wg-quick down wg-client - This alias disconnects from the VPN using WireGuard.



It also includes some environment variables, including tokens for Alpha Vantage, Finnhub, Qiskit, and Ollama.

You: 
```