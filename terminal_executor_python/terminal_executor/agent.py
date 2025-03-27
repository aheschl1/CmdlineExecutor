from typing import List, Tuple
from langchain.tools import BaseTool
from langchain_ollama.chat_models import ChatOllama
from langgraph.graph import StateGraph
from langchain_core.messages import SystemMessage, HumanMessage, AIMessage, AnyMessage, ToolMessage
from langchain_core.messages.tool import ToolCall
from terminal_executor.state import FeedbackStructure, State
from rich.console import Console
from rich.markdown import Markdown
from terminal_executor.spinner import Spinner
from langgraph.types import Command
from langchain.callbacks.streaming_stdout import StreamingStdOutCallbackHandler
from langchain.callbacks.manager import CallbackManager

class CmdlineAgent:
    """
    A command-line agent for handling interactive AI-based tasks.
    This agent leverages LangChain and LangGraph to process user inputs,
    call an AI model, and execute tools dynamically based on responses.
    """
    def __init__(
        self,
        model_name: str,
        system_prompt: str,
        tools: List[Tuple[BaseTool, callable]],
        context: int = 16000,
        temperature: float = 0.1,
        endpoint: str = "localhost:11434",
    ):
        """
        Initializes the command-line agent.
        
        Args:
            model_name (str): The name of the AI model to use.
            system_prompt (str): The system prompt used for guiding the AI model.
            tools (List[Tuple[BaseTool, callable]]): A list of tools with their corresponding functions.
            context (int): The context length for the model.
            temperature (float): The temperature parameter for response variability.
            endpoint (str): The base URL of the AI model service.
        """
        self.model_name = model_name
        self.system_prompt = system_prompt
        self.tools = tools
        self.console = Console()
        self.tool_name_to_function = {tool[0]["function"]["name"]: tool[1] for tool in tools}

        # Initializing AI model with provided settings
        self.model = ChatOllama(
            model=model_name,
            num_ctx=context,
            temperature=temperature,
            base_url=endpoint,
            extract_reasoning=False
        ).bind_tools(tools=[tool[0] for tool in tools])
    
    
    def prepare_graph(self):
        """
        Prepares the execution graph for managing conversational state transitions.
        
        Returns:
            StateGraph: The configured execution graph.
        """
        graph = StateGraph(State)
        graph.add_node("start", self._start)
        graph.add_node("call_model", self._call_model)
        graph.add_node("run_tools", self._run_tools)
        graph.add_node("display_message", self._display_message)
        graph.add_node("user_input", self._user_input)
        
        # Defining the flow of execution
        graph.add_edge("start", "user_input")
        graph.add_edge("user_input", "call_model")
        graph.add_conditional_edges("call_model", self._output_routing_function, {"run_tools": "run_tools", "display_message": "display_message"})
        graph.add_edge("run_tools", "call_model")
        graph.add_edge("display_message", "user_input")
        graph.set_entry_point("start")
        
        return graph
    
    
    def _display_message(self, state: State):
        """
        Displays the AI-generated response in markdown format.
        
        Args:
            state (State): The current state containing the latest AI message.
        
        Returns:
            State: The updated state after displaying the message.
        """
        content = state.messages[-1].content
        content = "# Response\n" + content
        content = Markdown(content)
        self.console.print(content)
        return state
    
    def _run_tools(self, state: State):
        """
        Executes tool calls based on the AI-generated response.
        
        Args:
            state (State): The current state containing tool calls.
        
        Returns:
            State: The updated state after executing tools.
        """
        tool_message = state.messages[-1]
        messages = []
        
        for tool_call in tool_message.tool_calls:
            if tool_call["name"] not in self.tool_name_to_function:
                messages.append(SystemMessage(content=f"Tool '{tool_call['name']}' not found. Try a different tool."))
                continue
            func = self.tool_name_to_function[tool_call["name"]]
            try:
                output = func(**tool_call["args"])
                messages.append(ToolMessage(content=output, tool_call_id=tool_call["id"]))
            except Exception as e:
                print("Model error:", e)
                messages.append(SystemMessage(content=f"Error: {e}"))
                messages.append(SystemMessage(content="FAILURE. Look at the error message above, and decide if you can fix it."))
        
        return State(
            messages=state.messages + messages,
            query=state.query
        )
    
    def _user_input(self, state: State):
        """
        Captures user input from the command line.
        
        Args:
            state (State): The current conversation state.
        
        Returns:
            State: The updated state with new user input.
        """
        inpoo = input("Query: ")
        return State(
            messages=state.messages + [HumanMessage(content=inpoo)],
            query=inpoo
        )
        
    def _output_routing_function(self, state: State) -> str:
        """
        Determines the next action based on the AI response.
        
        Args:
            state (State): The current conversation state.
        
        Returns:
            str: The next state transition ('run_tools' or 'display_message').
        """
        recent_message: AIMessage = state.messages[-1]
        if len(recent_message.tool_calls) > 0:
            return "run_tools"
        else:
            return "display_message"

    async def _call_model(self, state: State):
        """
        Calls the AI model to generate a response based on the current conversation state.
        
        Args:
            state (State): The current conversation state.
        
        Returns:
            State: The updated state containing the AI-generated message.
        """
        with Spinner():
            output: AIMessage = await self.model.ainvoke(state.messages)
        
        return State(
            messages=state.messages + [output],
            query=state.query
        )
    
    def _start(self, _: State):
        """
        Initializes the conversation with the system prompt.
        
        Args:
            _: State (unused initial state parameter).
        
        Returns:
            State: The initial state containing the system prompt.
        """
        print(f"Model: {self.model_name}")
        return State(
            messages=[SystemMessage(content=self.system_prompt)],
            query=None
        )
