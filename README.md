# 🧹 Sweep

**Sweep is not like a traditional Discord bot**. It understands complex requests and is able to act directly with the Discord API using tools. **No hardcoded commands**.

![Demo GIF](.github/general_showcase.gif)

## ✨ Features

- Tool calling to **interact directly with the Discord API**
- A **per-channel context** so messages don't get mixed up
- Uses an **OpenAI-compatible endpoint**, so **both selfhosted and non-selfhosted backends are supported**
- **Versatile use cases**. You are not tied to any hardcoded logic

## 🛡️ Safety

Sweep implements safety measures to prevent unprivileged users from manipulating it into performing unauthorized actions. Note that LLMs can still be tricked.

- **Approval system:** Users must explicitly grant permission via an embed button before Sweep can act on their behalf.  
  ![Demo GIF](.github/approval_showcase.gif)

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
- There is **no explicit requirement for a parameter count**, but be aware that smaller models are more likely to mess up requests.

## 🔬 Tested With

There's a discussion category purely for model ratings. [Check it out!](https://github.com/nghuy0901/BotAgent/discussions/categories/model-discussions)

## ⚡ Quickstart

First, clone the repository:

```bash
git clone https://github.com/nghuy0901/BotAgent.git
```

Then, create a `sweep.toml` and configure Sweep. You can find all available options and their defaults [here](sweep.default.toml).

Then, configure your Discord bot token by setting the `SWEEP_DISCORD_TOKEN` environment variable (recommended, .env supported) or by setting the `discord.token` variable in the `sweep.toml`.

Lastly, run Sweep:

```bash
cargo run --release
```

## ⚙️ Configuration

### 💫 OpenAI Endpoint Configuration

Sweep uses [async-openai](https://github.com/64bit/async-openai) for connecting to the OpenAI-compatible endpoint. You can configure it in your `sweep.toml` configuration file. Check Sweep's [default config file](sweep.default.toml) for available options.

The `llm.endpoint` option **MUST BE SET**!

### 🧹 Sweep

You can configure Sweep via a `sweep.toml` file. All variables and defaults can be checked [here](sweep.default.toml).

Sweep also supports .env files. You can create a `.env` file and override any `sweep.toml` configuration, like this:
```ini
SWEEP_DISCORD_TOKEN=your_token_here
```

## 🗺️ Roadmap

You can check existing feature requests [here](https://github.com/nghuy0901/BotAgent/issues). You can also submit new feature requests.

## 💣 Common Errors

- `Unexpected Endpoint`: Make sure that your OpenAI endpoint variable does not have a trailing `/`

## 🤝 Contributing

Pull requests and issues are very welcome! This applies to bug fixes, bug reports, feature requests and basically everything!

## 📄 License

This project is licensed under the **AGPL-3.0**. This means that if you modify Sweep and run it as a service, you must publish your modifications under the same license.

See [LICENSE](./LICENSE) for details.
