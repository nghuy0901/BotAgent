# 🧹 Sweep

**Sweep is not like a traditional Discord bot**. It understands complex requests and is able to act directly with the Discord API using tools. **No hardcoded commands**.

*TODO: Add demo gif*

## ✨ Features

- Tool calling to **interact directly with the Discord API**
- A **per-channel context** so messages don't get mixed up
- Uses an **OpenAI-compatible endpoint**, so **both selfhosted and non-selfhosted backends are supported**
- **Versatile use cases**. You are not tied to any hardcoded logic

## 🙋 Permission System

> [!IMPORTANT]
> Sweep implements safety measures to reduce the risk of abuse, but there is still a chance for abuse. Unprivileged users may be able to execute harmless actions (sending messages, reactions, ...). A solution to prevent this is being worked on.

LLMs can be tricked. Sweep implements the following features to reduce the risk of abuse (unprivileged users altering your server against your will):
- An approval system: Users need to grant permission to Sweep by interacting with a button on an embed

*TODO: add demo gif*

## 🌐 OpenAI-Compatible Endpoint

Sweep supports any OpenAI-compatible endpoint. Tools that offer such endpoints are:
- [llama.cpp](https://github.com/ggml-org/llama.cpp)
- [Ollama](https://ollama.com/)
- [LM Studio](https://lmstudio.ai/)
- [vLLM](https://vllm.ai/)
- Many cloud providers

## ⚓ Requirements

Rust requirements:
- **Sweep is always developed on the latest Rust version. Backwards-compatibility is not guaranteed.**

LLM requirements:
- **Tool calling support**
- There is **no explicit requirement for a parameter count**, but be aware that smaller models are more likely to mess up requests. See the [Tested With Section](#🔬-tested-with) for more information.

## ⚡ Quickstart

First, clone the repository:

```bash
git clone https://github.com/nghuy0901/BotAgent.git
```

Then, configure Sweep via your `.env` file:

```ini
DISCORD_TOKEN=your_discord_bot_token
MODEL=your_model
OPENAI_BASE_URL=openai_base_url
```

Lastly, run Sweep:

```bash
cargo run --release
```

## 🔬 Tested With

I've tested Sweep with a series of models. I ran all of them via the llama.cpp-server binary. My personal rating:
1. **Qwen3.6-35B-A3B** `Q4_K_M + Reasoning`: It works really well and has a great understanding of what the tools do. It's my favorite
2. **Qwen3.5-9B** `Q4_K_M + Reasoning`: It works well. Sometimes it double-sends messages, but that is fine

I recommend you to test what model works best for you.

## ⚙️ Configuration

### 💫 OpenAI Endpoint Configuration

Sweep uses [async-openai](https://github.com/64bit/async-openai) for connecting to the OpenAI-compatible endpoint. You can configure the used endpoint with environment variables. The most important ones are:
- `OPENAI_API_KEY`: Your API key (if needed)
- `OPENAI_BASE_URL`: The base url of the endpoint (default: `https://api.openai.com/v1`)
- [See more environment variables here](https://github.com/64bit/async-openai/tree/main#usage)

### 🧹 Sweep

The binary expects the following environment variables to be present:
- `DISCORD_TOKEN`: The Discord bot token for logging into the Discord user
- `MODEL`: The model that is used for inference

You may use a `.env` file.

## 🗺️ Roadmap

- Skill system: To give Sweep a better understanding of tasks like server design
- Persona system: To give Sweep its own, server-specific identity
- Vision support for viewing images
- Whitelist system + unrestricted session (permission system)
- Support for regular file attachments

## 🤝 Contributing

Pull requests and issues are very welcome! This applies to bug fixes, bug reports, feature requests and basically everything!

## 📄 License

This project is licensed under the **AGPL-3.0**. This means that if you modify Sweep and run it as a service, you must publish your modifications under the same license.  

See [LICENSE](./LICENSE) for details.
