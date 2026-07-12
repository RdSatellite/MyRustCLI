"""Simple chatting feature"""

from typing import List, Literal

from fastapi import HTTPException, Request, Depends
from fastapi.routing import APIRouter
from pydantic import BaseModel, Field, ConfigDict

from langchain_core.messages import (
    AIMessage,
    HumanMessage,
    SystemMessage,
)
from langchain_openai import ChatOpenAI

dialogue_router: APIRouter = APIRouter(prefix="/dialogue")

# --- System prompt --- #

SYSTEM_PROMPT: str = (
    "You are an AI chatting with a user in a terminal.\n"
    "\n"
    "Keep responses concise.\n"
    "Reply in one or two short sentences.\n"
    "Do not use Markdown.\n"
    "Do not use bullet lists unless requested.\n"
    "Be friendly and conversational.\n"
)

# --- Dependency --- #

def get_llm(request: Request) -> ChatOpenAI:
    """Extracts the LLM client from the application state."""
    return request.app.state.llm

# --- Schema --- #

Role = Literal[
    "system", 
    "human", "ai", 
    "user", "assistant"
]


class DialogueMessage(BaseModel):
    role: Role
    content: str

    @classmethod
    def system(cls, content: str) -> "DialogueMessage":
        return cls(role="system", content=content)

    @classmethod
    def user(cls, content: str) -> "DialogueMessage":
        return cls(role="user", content=content)

    @classmethod
    def assistant(cls, content: str) -> "DialogueMessage":
        return cls(role="assistant", content=content)


class DialogueRequest(BaseModel):
    model_config = ConfigDict(extra="forbid")

    model: str = Field(default="deepseek-v4-pro", description="Model name")
    messages: list[DialogueMessage] = Field(default_factory=list)


class DialogueResponse(BaseModel):
    message: DialogueMessage


def to_langchain_message(msg: DialogueMessage):
    match msg.role:
        case "system":
            return SystemMessage(content=msg.content)
        case "user":
            return HumanMessage(content=msg.content)
        case "assistant":
            return AIMessage(content=msg.content)
        case _:
            raise ValueError(f"Unknown role: {msg.role}")

# --- Handlers --- #

@dialogue_router.post(
    "",
    response_model=DialogueResponse
)
def dialogue(
    req: DialogueRequest,
    llm: ChatOpenAI = Depends(get_llm),
):
    messages = [SystemMessage(content=SYSTEM_PROMPT)]
    messages.extend(to_langchain_message(m) for m in req.messages)

    if req.model == "" or req.model == "default":
        model = llm.model

    try: 
        ai_msg = llm.invoke(
            messages,
            model=model,
        )
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
    
    return DialogueResponse(
        message=DialogueMessage.assistant(ai_msg.text)
    )
