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

# Adding a tool

To add a tool, you must modify the tools.json file in the terminal_executor directory.