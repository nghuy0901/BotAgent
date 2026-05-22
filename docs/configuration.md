# ⚙️ Configuring Sweep

Sweep can be configured through a `sweep.toml` file, which is read when running the binary, or through environment variables prefixed with `SWEEP__` and using `__` for splitting keys from field names (`llm.api_key` would become `SWEEP__LLM__API_KEY`).

> [!IMPORTANT]
> Fields marked with `[sensitive]` contain sensitive data. These should be set via environment variables. The environment variable name is shown next to the normal field name for such fields.

Every heading in this guide gives an overview of a top-level key. Example: A heading has the top-level key `[testing]` and documents a field named `allow_testing` of type `bool`. To configure that field, you'd write this into your `sweep.toml`:

```toml
# ONLY AN EXAMPLE!

[testing]
allow_testing = true # This would enable the test mode
```

## 🛡️ Approval Configuration `[approval]`

| Field             | Type                                      | Default Value | Description                                                                               |
| ----------------- | ----------------------------------------- | ------------- | ----------------------------------------------------------------------------------------- |
| `timeout`         | `unsigned 64-bit integer`                 | `30`          | The period of time (in seconds) in which an approval has to be answered before timing out |
| `skip_completion` | `["approved" \| "denied" \| "timeout"][]` | `[]`          | A list of approval outcomes that shouldn't trigger the LLM endpoint                       |

## 🤖 Bot Configuration `[bot]`

| Field         | Type                      | Default Value | Description                                                                                                                              |
| ------------- | ------------------------- | ------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `debounce_ms` | `unsigned 64-bit integer` | `1000`        | How long the bot collects events to batch them together                                                                                  |
| `max_turns`   | `unsigned size`           | `10`          | The number of consecutive tool call turns a model can take before being interrupted. Setting this to 0 will disable tools **everywhere** |
