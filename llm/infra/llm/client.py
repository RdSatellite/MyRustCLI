from langchain_openai import ChatOpenAI

def create_chat(
    model: str,
    api_key: str,
    base_url: str,
    temp: float = 0.3,
) -> ChatOpenAI:
    chat = ChatOpenAI(
        model=model,
        api_key=api_key,
        base_url=base_url,
        temperature=temp,
        streaming=True
    )

    return chat
