# Command Line LLM Executor

This repo provides an Ollama wrapper for running informational tools in the command line using an LLM.

Tools are defined in json, and are compiled down using a macro, and passed through to the ollama-rs api.

# Setup

You must modify ```terminal_executor/src/tools.rs``` to point to your tools file.

# Ollama
Ensure that you have ollama setup and running