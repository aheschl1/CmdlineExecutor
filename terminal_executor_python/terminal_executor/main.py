import os
from langchain.agents import initialize_agent, Tool
from langchain.prompts import PromptTemplate
from terminal_executor.agent import CmdlineAgent
from terminal_executor.state import State
from terminal_executor.tools import get_tools
import click
import os
import asyncio

def get_system_prompt():
    return f"""
    You are a command line executor agent. Your purpose is to execute commands on a linux machine, in order to help the user with their tasks.
    You can also chat with the user to provide additional information when they ask. You are running locally, so privacy is not an issue.
    Do anything in your power to execute tasks.
    RULES:
    1. Do not make up information, run commands to get the information, or tell the user you do not know.
    2. If needed, run multiple commands one after the other to get information. You do not need to ask the user for permission.
        2.1 For example, you may look at the output of ls on the home dir, then run ls on the home/Document dir, without asking the user.
        2.2 Another example is running multiple ls commands to identify files, then running cat when you have found the file.
    Here are some important machine details:
    OS: {os.uname().sysname}
    KERNEL: {os.uname().release}
    UPTIME: {os.popen("uptime").read()}
    HOSTNAME: {os.uname().nodename}
    USERS: {os.popen("whoami").read()}
    """


@click.command()
@click.option('--model_name', "-m", default="gemma12b_commandline_exec:latest", help='The name of the AI model to use.')
@click.option("-endpoint", "-e", default="10.8.0.1:11434", help="The base URL of the Ollama service.")
@click.option("-tools", "-t", default=os.environ.get("TERMINAL_EXECUTOR_TOOLS", os.path.expanduser("~/terminal_executor_tools.json")), help="The path to the JSON schema containing tool definitions.")
def main(model_name, endpoint, tools):
    async def async_main():
        agent = CmdlineAgent(
            model_name=model_name,
            system_prompt=get_system_prompt(),
            endpoint=endpoint,
            tools=get_tools(tools, command_timeout=60),
            temperature=0
        )
        await agent.prepare_graph().compile().ainvoke(State(messages=[], query=None), config={
            "recursion_limit": 100
        })
    asyncio.run(async_main())
if __name__ == "__main__":
    main()