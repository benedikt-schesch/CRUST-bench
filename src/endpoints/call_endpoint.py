import endpoints.openrouter as openrouter
# uncomment the next line if you want to use gemini
# import endpoints.gemini as gemini
import multiprocessing
import json
import sys


def _get_claude():
    import endpoints.claude as claude
    return claude

def _get_claude37():
    import endpoints.claude37 as claude37
    return claude37

def _get_gpt():
    import endpoints.gpt as gpt
    return gpt

def _get_qwq():
    import endpoints.qwq as qwq
    return qwq

def _get_vllm():
    import endpoints.vllm_client as vclient
    return vclient


def get_result(messages, lock, model, config):
    if model == "claude":
        return _get_claude().get_result(messages, lock, config)
    elif model == "claude37":
        return _get_claude37().get_result(messages, lock, config)
    elif model == "gpt-4o" or model == "o1-mini" or model == "o1":
        return _get_gpt().get_result(messages, lock, config)
    elif (
        model == "QwQ-32B-Preview"
        or model == "Qwen2.5-Coder-32B"
        or model == "Virtuoso-Medium-v2"
        or model == "Meta-Llama-3-8B"
    ):
        m = _get_vllm().VLLMServer(model)
        return m.get_result(messages, lock, config)
    elif model == "gemini":
        return gemini.get_result(messages, lock, config)
    elif model == "openrouter":
        return openrouter.get_result(messages, lock, config)
    else:
        raise ValueError("Invalid model")


def get_result_n(messages, lock, model, config, n):
    if model == "gpt-4o" or model == "o1-mini" or model == "o1":
        return _get_gpt().get_result_n(messages, lock, config, n)
    elif model == "claude":
        return _get_claude().get_result_n(messages, lock, config, n)
    elif model == "openrouter":
        return openrouter.get_result_n(messages, lock, config, n)
    else:
        raise ValueError("Invalid model")


if __name__ == "__main__":
    print(
        get_result(
            [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": "What is the weather in San Francisco?"},
            ],
            multiprocessing.Lock(),
            "claude",
            None,
        )
    )
    print(
        get_result(
            [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": "What is the weather in San Francisco?"},
            ],
            multiprocessing.Lock(),
            "gpt",
            None,
        )
    )
