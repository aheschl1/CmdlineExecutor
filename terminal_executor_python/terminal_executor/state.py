from typing import Optional, TypedDict
from pydantic import BaseModel
from langchain_core.messages import SystemMessage, HumanMessage, AIMessage, AnyMessage

class State(BaseModel):
    messages:  list[AnyMessage]
    query: Optional[str]
    
class FeedbackStructure(TypedDict):
    feedback: str
    return_to_user: bool