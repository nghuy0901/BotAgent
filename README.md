# 🧹 Sweep

Sweep is not like a traditional Discord bot. It understands complex requests and is able to act directly with the Discord API using tools. **No hardcoded commands**.

## 📹 Demo Video

*TODO*

## ✨ Features

- Tool calling to **interact directly with the Discord API**
- A **per-channel context** so messages don't get mixed up
- Uses an **OpenAI-compatible endpoint**, so even **non-selfhosted backends are supported**
- **Versatile use cases**. You are not tied to any hardcoded logic

## ⚓ Requirements

Rust requirements:
- ...

LLM requirements:
- Tool calling support
- There is no explicit requirement for a parameter count, but be aware that smaller models are more likely to mess up requests.

## 🌟 Detailed Features List

### ⚡ Event-Driven Inference

*TODO*

### 🙋 Permission System

> [!IMPORTANT]
> Sweep implements safety measures to reduce the risk of abuse, but there is still a chance for abuse. Unprivileged users may be able to execute harmless actions (sending messages, reactions, ...). A solution to prevent this is being worked on.

LLMs can be tricked. Sweep implements the following features to reduce the risk of abuse (unprivileged users altering your server against your will):
- An approval system: Users need to grant permission to Sweep by interacting with a button on an embed

#### 📹 Demo Video

*TODO*

### 📚 Per-Channel Context

*TODO*

### 🛠️ Tool System

*TODO*

### 🌐 OpenAI-Compatible Endpoint

Sweep supports any OpenAI-compatible endpoint. Tools that offer such endpoints are:
- llama.cpp
- ollama
- LM Studio
- vLLM
- Many cloud providers

## 🗺️ Roadmap

- Skill system
- Persona system
- Vision support for viewing images
- Whitelist system + unrestricted session (permission system)
- Support for regular file attachments

## 🔌 Setup

The binary expects an environment variable named `DISCORD_TOKEN` to be present for logging into the Discord bot user. You may use a `.env` file.

## ⚙️ Configuration

### 💫 OpenAI Endpoint Configuration

Sweep uses [async-openai] for connecting to the OpenAI-compatible endpoint. You can configure the used endpoint with environment variables. The most important ones are:
- `OPENAI_API_KEY`: Your API key (if needed)
- `OPENAI_BASE_URL`: The base url of the endpoint (default: `https://api.openai.com/v1`)
- [See more environment variables here](https://github.com/64bit/async-openai/tree/main#usage)

### 🧹 Sweep

*TODO*

## 📄 License

This project is licensed under the **AGPL-3.0**. This means that if you modify Sweep and run it as a service, you must publish your modifications under the same license.  

See [LICENSE](./LICENSE) for details.
