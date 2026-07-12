# main.py
import os

from dotenv import load_dotenv
from fastapi import FastAPI

import infra
from dialogue import dialogue_router
from profile import profile_router

load_dotenv(os.path.join(os.path.dirname(__file__), "..", ".env"))

BASE_URL: str = os.getenv("BASE_URL")
API_KEY: str = os.getenv("API_KEY")
DEFAULT_MODEL: str = os.getenv("MODEL")

app = FastAPI(title="LLM Service", version="0.1.0")

llm = infra.llm.create_chat(
    model=DEFAULT_MODEL,
    api_key=API_KEY,
    base_url=BASE_URL,
)
app.state.llm = llm

# --- Handlers --- #

@app.get("/health")
async def health() -> dict[str, str]:
    return {"status": "ok"}

app.include_router(dialogue_router)
app.include_router(profile_router)

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("main:app", host="127.0.0.1", port=8000, reload=True)
