from langchain_core.utils.function_calling import convert_to_openai_tool
import json

def build_function(tool: dict, command_timeout: int = 20):
    """
    Given a tool JSON schema, dynamically build a function that can be called.
    
    Args:
        tool (dict): The JSON schema defining the command-line tool.
        command_timeout (int, optional): The timeout for the command execution in seconds. Defaults to 20.
    
    Returns:
        function: A dynamically created function that executes the command.
    """
    name = tool["name"]
    args = []
    prefixes = [a['prefix'] for a in tool["params"]]
    types = []
    arg_names = [a['name'] for a in tool["params"]]
    
    for param in tool["params"]:
        if param["flag"]:
            types.append("FLAG")
        elif param["positional"]:
            types.append("POSITIONAL")
        else:
            types.append("OPTIONAL")
            
        if param["required"]:
            args.append(f"{param['name']}: {'str' if not param['flag'] else 'bool'}")
        else:
            args.append(f"{param['name']}: {'str' if not param['flag'] else 'bool'} = {'False' if param['flag'] else {''}}")
    
    body = f"""
    import subprocess
    args = []
    for argname, argtype, pre, arg in zip({arg_names}, {types}, {prefixes}, [{', '.join([f'{param["name"]}' for param in tool["params"]])}]):
        if arg is not False and len(str(arg)) > 0:
            if argtype == "FLAG":
                args.append(pre + argname)
            elif argtype == "POSITIONAL":
                args.append(arg)
            else:
                args.append(pre + argname + ' ' + arg)
    print("Running: {name}  " + ' '.join(args))  
    result = subprocess.run(["{name}  " + ' '.join(args)], shell=True, capture_output=True, timeout={command_timeout})
    return result.stdout.decode('utf-8')
    """
    
    f_string = f"def {name}({', '.join(args)}):\n    {body}"
    context = {}
    exec(f_string, context)

    return context[name]
    

def convert_to_proper_schema(tool: dict):
    """
    Convert a command JSON schema to an OpenAI tool schema.
    
    Args:
        tool (dict): The JSON schema defining the command-line tool.
    
    Returns:
        dict: The OpenAI-compatible function schema.
    """
    output = {
        "type": "function",
        "function": {
            "name": tool["name"],
            "description": tool["description"],
            "parameters": {
                "type": "object",
                "properties": {},
                "required": [],
                "additionalProperties": False
            }
        }
    }
    for param in tool["params"]:
        output["function"]["parameters"]["properties"][param["name"]] = {
            "type": "string" if not param["flag"] else "boolean",
            "description": param["description"]
        }
        if param["required"]:
            output["function"]["parameters"]["required"].append(param["name"])
    return output
    

def get_tools(json_path: str, command_timeout: int = 20):
    """
    Load and parse command definitions from a JSON file.
    
    Args:
        json_path (str): The file path to the JSON schema containing tool definitions.
        command_timeout (int, optional): The timeout for each command execution. Defaults to 20 seconds.
    
    Returns:
        list: A list of tuples, each containing an OpenAI tool schema and the corresponding function.
    """
    with open(json_path, 'r') as f:
        tools = json.load(f)
    return [(convert_to_proper_schema(tool), build_function(tool, command_timeout)) for tool in tools]

if __name__ == "__main__":
    tools = get_tools("/home/andrewheschl/Documents/CmdlineExecutor/terminal_executor/tools/tools.json")
    for tool in tools[0:1]:
        print(tool[1]("~", True, False))
