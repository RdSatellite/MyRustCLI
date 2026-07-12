"""Simple chatting feature"""

from typing import List, Literal

from fastapi import HTTPException, Request, Depends
from fastapi.routing import APIRouter
from pydantic import BaseModel, RootModel, Field
from langchain_core.messages import AIMessage
from langchain_openai import ChatOpenAI

from infra.llm import StructuredHelper
from dialogue import DialogueMessage

profile_router: APIRouter = APIRouter(prefix="/profile")


# --- Prompts --- #

SYSTEM_PROMPT: str = """
You are an information extraction engine.

Your task is to identify durable facts about the USER from a conversation.

You will receive:

1. The user's existing profile.
2. The recent conversation.

Your job is to return ONLY NEW information that should be added or updated.

Rules:

- Extract only long-term information.
- Ignore temporary information.
- Ignore the assistant's preferences or opinions.
- Do not infer facts without evidence.
- Do not speculate.
- Do not repeat information already present.
- If a fact becomes more specific, replace it.
- Prefer concise values.

Good examples:

Programming Language -> Rust
Favorite Editor -> Neovim
Occupation -> Backend Engineer
Company -> OpenAI
Hobby -> Photography

Bad examples:

Current mood
Current weather
Today's task
Questions asked in this conversation
Temporary plans
One-off opinions

If nothing new is learned, return an empty profile.
""".strip()


EXTRACT_PROMPT = """
Existing profile:

{profile}

Conversation:

{history}
""".strip()


# --- Dependency --- #

def get_llm(request: Request) -> ChatOpenAI:
    """Extracts the LLM client from the application state."""
    return request.app.state.llm


# --- Schema --- #

class Profile(BaseModel):
    data: dict[str, str] = Field(default_factory=dict)

class ExtractProfileRequest(BaseModel):
    profile: Profile
    history: list[DialogueMessage]

class ExtractProfileResponse(BaseModel):
    new_insight: Profile


# --- Handler --- #

@profile_router.post(
    "/extract",
    response_model=ExtractProfileResponse 
)
async def extract_profile(
    body: ExtractProfileRequest,
    llm: ChatOpenAI = Depends(get_llm)
):  
    class ExtractedProfile(RootModel[dict[str, str]]):
        pass

    prompt = (
        SYSTEM_PROMPT
        + "\n\n"
        + EXTRACT_PROMPT.format(
            profile=body.profile.data,
            history=stringify_history(body.history),
        )
    )

    prompt = StructuredHelper.build_prompt(
        prompt,
        ExtractedProfile,
    )

    raw: AIMessage = await llm.ainvoke(prompt)

    rsp = StructuredHelper.parse(
        raw.text,
        ExtractedProfile,
    )

    return ExtractProfileResponse(
        new_insight=Profile(
            data=rsp.root
        )
    )


def stringify_history(history: list[DialogueMessage]) -> str:
    return "\n".join(
        f"{msg.role.capitalize()}: {msg.content}"
        for msg in history
    )
