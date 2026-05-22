
## 📖 About

**Sweep is not like a traditional Discord bot.** It understands complex requests and interacts directly with the Discord API using tools. **No hardcoded commands.**

Sweep bridges the gap between natural language and API actions, allowing you to build a bot that adapts to your needs via prompts rather than code.

## ✨ Features

- `🛠️` **Tool Calling**: Sweep interacts directly with the Discord API via structured tool calls
- `🧵` **Per-Channel Context**: Messages stay isolated per channel to prevent cross-talk
- `🔌` **OpenAI-Compatible**: Works with self-hosted or cloud LLM backends
- `🧠` **Zero Hardcoded Commands**: Behavior emerges from prompts and tools, not regex or slash commands
- `🛡️` **Approval System**: Every action requires explicit user consent via embed buttons
- `👂` **Wake On Mention**: Sweep only responds on mentions *(configurable)*

## 🛡️ Safety

Sweep implements strict safety measures to prevent unauthorized actions.

- **Explicit Approval**: Users must click an embed button before any server-altering tool executes
- **Permission Checks**: Tools respect Discord role/channel permissions

## 🌐 LLM Compatibility

Sweep supports any OpenAI-compatible endpoint.

> [!IMPORTANT]
> Tool calling support is mandatory. Smaller models (<8B) often fail on complex tool schemas. Check our [Model Discussions](https://github.com/nghuy0901/BotAgent/discussions/categories/model-discussions) for ratings.

> [!TIP]
> If your backend supports it, enable the `preserve_thinking` flag. This helps to reduce incomplete responses, but may increase context usage.

### 🖥️ Local Backends

- [llama.cpp](https://github.com/ggml-org/llama.cpp)
- [Ollama](https://ollama.com/)
- [LM Studio](https://lmstudio.ai/)
- [vLLM](https://vllm.ai/)

### ☁️ Cloud Providers

> [!NOTE]
> Some providers listed here include affiliate links. Using them helps support Sweep's development at no extra cost.

- PayPerQ
    - [Official Link](https://ppq.ai/)
    - [Affiliate Link](https://ppq.ai/invite/01486515)

- [OpenRouter](https://openrouter.ai/)
- [Groq](https://groq.com/)
- [Together AI](https://www.together.ai/)
- _... any other OpenAI-compatible provider_

## ⚡ Quickstart

1. Clone Sweep's repository:

```bash
git clone https://github.com/nghuy0901/BotAgent.git
cd BotAgent
```

2. Create a `sweep.toml` file and [configure Sweep](docs/configuration.md).

3. Set your token:

```bash
export SWEEP__DISCORD__TOKEN="your_token" # You can also use a .env file!
```

4. Run Sweep:

```bash
cargo run --release
```

📖 Full config reference: [Configuration Guide](docs/configuration.md).

## ⚙️ Configuration

- `sweep.toml`: Primary config file. All options are documented in the [configuration guide](docs/configuration.md).
- `.env` support: Override any config value via environment variables. `llm.api_key` would become `SWEEP__LLM__API_KEY`
- `llm.endpoint` (in the config file) is **required**. Make sure that it does not have a trailing `/`.

## 🤝 Contributing

PRs, issues and feature requests are very welcome! This includes:

- Bug fixes & improvements
- New tools or LLM integrations
- Documentation

See [CONTRIBUTING.md](.github/CONTRIBUTING.md) for the guidelines.

## 📄 License

This project is licensed under the **AGPL-3.0**. This means that if you modify Sweep and run it as a service, you must publish your modifications under the same license.

See [LICENSE](LICENSE) for details.
