from pathlib import Path
import os
import json
import multiprocessing
from openai import OpenAI

client = OpenAI(
    base_url="https://openrouter.ai/api/v1",
    api_key=os.environ.get("OPENROUTER_API_KEY", ""),
)

FILE_PATH = Path(__file__)
CACHE_FILE = FILE_PATH.parent / "cache/openrouter_cache.jsonl"
if not CACHE_FILE.parent.exists():
    CACHE_FILE.parent.mkdir(parents=True, exist_ok=True)
CACHE_FLAG = True
if not os.path.exists(CACHE_FILE):
    with open(CACHE_FILE, "w") as file:
        pass
with open(CACHE_FILE, "r", encoding="utf-8") as file:
    data = file.readlines()
CACHE = {json.loads(line)["prompt"]: json.loads(line) for line in data}


def call_openrouter(messages, config, n=1):
    kwargs = {
        "model": config["model"],
        "messages": messages,
        "n": n,
    }
    for key in ("temperature", "max_tokens", "top_p"):
        if key in config:
            kwargs[key] = config[key]
    return client.chat.completions.create(**kwargs)


def get_result(messages, lock, config):
    if config is None:
        raise ValueError("Config is None. Please look into the config.")
    current_prompt = (
        "\n".join([msg["content"] for msg in messages]) + f"\n{json.dumps(config)}"
    )
    if CACHE_FLAG and current_prompt in CACHE:
        return CACHE[current_prompt]
    try:
        response = call_openrouter(messages, config)
    except Exception as e:
        return {"prompt": current_prompt, "response": str(e), "usage": {}}
    data = {
        "prompt": current_prompt,
        "response": response.choices[0].message.content,
        "usage": {
            "completion_tokens": response.usage.completion_tokens,
            "prompt_tokens": response.usage.prompt_tokens,
            "total_tokens": response.usage.total_tokens,
        },
    }
    if CACHE_FLAG:
        with lock:
            with open(CACHE_FILE, "a", encoding="utf-8") as file:
                file.write(json.dumps(data) + "\n")
            CACHE[current_prompt] = data
    return data


def get_result_n(messages, lock, config, n):
    if config is None:
        raise ValueError("Config is None. Please look into the config.")
    current_prompt = (
        "\n".join([msg["content"] for msg in messages])
        + f"\n{json.dumps(config)}"
        + f"\n={n}"
    )
    if CACHE_FLAG and current_prompt in CACHE:
        return CACHE[current_prompt]
    try:
        response = call_openrouter(messages, config, n)
    except Exception as e:
        return {"prompt": current_prompt, "response": str(e), "usage": {}}
    data = {
        "prompt": current_prompt,
        "response": [choice.message.content for choice in response.choices],
        "usage": {
            "completion_tokens": response.usage.completion_tokens,
            "prompt_tokens": response.usage.prompt_tokens,
            "total_tokens": response.usage.total_tokens,
        },
    }
    if CACHE_FLAG:
        with lock:
            with open(CACHE_FILE, "a", encoding="utf-8") as file:
                file.write(json.dumps(data) + "\n")
            CACHE[current_prompt] = data
    return data
