
## 📖 About

**Sweep is not like a traditional Discord bot.** It understands complex requests and interacts directly with the Discord API using tools. **No hardcoded commands.**

Sweep bridges the gap between natural language and API actions, allowing you to build a bot that adapts to your needs via prompts rather than code.

## ✨ Features

- **Tool Calling**: Sweep interacts directly with the Discord API via structured tool calls.
- **Per-Channel Context**: Messages stay isolated per channel to prevent cross-talk
- **OpenAI-Compatible**: Works with self-hosted or cloud LLM backends
- **Zero Hardcoded Commands**: Behavior emerges from prompts and tools, not regex or slash commands
- **Approval System**: Every action requires explicit user consent via embed buttons

## 🛡️ Safety

Sweep implements strict safety measures to prevent unauthorized actions.

- **Explicit Approval**: Users must click an embed button before any server-altering tool executes
- **Permission Checks**: Tools respect Discord role/channel permissions

## 🌐 LLM Compatibility

Sweep connects to any OpenAI-compatible endpoint. Backends that offer OpenAI-compatible endpoints are:

- [llama.cpp](https://github.com/ggml-org/llama.cpp)
- [Ollama](https://ollama.com/)
- [LM Studio](https://lmstudio.ai/)
- [vLLM](https://vllm.ai/)
- Many cloud providers

> [!IMPORTANT]
> Tool calling support is mandatory. Smaller models (<8B) may struggle with complex tool schemas. Check our [Model Discussions](https://github.com/nghuy0901/BotAgent/discussions/categories/model-discussions) for ratings.

> [!TIP]
> If you can, enable the `perserve_thinking` flag for your model. This helps to reduce incomplete responses.

## ⚡ Quickstart

1. Clone Sweep's repository:

```bash
git clone https://github.com/nghuy0901/BotAgent.git
cd BotAgent
```

2. Copy and configure `sweep.default.toml`:

```bash
# Make sure to configure your LLM endpoint!

cp sweep.default.toml sweep.toml
```

3. Set your token:

```bash
export SWEEP__DISCORD__TOKEN="your_token" # You can also use a .env file!
```

4. Run Sweep:

```bash
cargo run --release
```

📖 Full config reference: [sweep.default.toml](sweep.default.toml).

## ⚙️ Configuration

- `sweep.toml`: Primary config file. All options are documented in the [default config](sweep.default.toml).
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
