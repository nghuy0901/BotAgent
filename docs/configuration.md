# ⚙️ Configuring Sweep

Sweep can be configured through a `sweep.toml` file, which is read when running the binary, or through environment variables prefixed with `SWEEP__` and using `__` for splitting keys from field names (`llm.api_key` would become `SWEEP__LLM__API_KEY`).

> [!IMPORTANT]
> Fields marked with `[sensitive]` contain sensitive data. These should be set via environment variables. The environment variable name is shown next to the normal field name for such fields.

> [!NOTE]
> **Fields with no default value are required to be set by the user**.

Every heading in this guide gives an overview of a top-level key. Example: A heading has the top-level key `[testing]` and documents a field named `allow_testing` of type `bool`. To configure that field, you'd write this into your `sweep.toml`:

```toml
# ONLY AN EXAMPLE!

[testing]
allow_testing = true # This would enable the test mode
```

## 🛡️ Approval Configuration `[approval]`

| Field             | Type                                          | Default Value | Description                                                                               |
| ----------------- | --------------------------------------------- | ------------- | ----------------------------------------------------------------------------------------- |
| `timeout`         | `unsigned 64-bit integer`                     | `30`          | The period of time (in seconds) in which an approval has to be answered before timing out |
| `skip_completion` | list of `"approved" \| "denied" \| "timeout"` | `[]`          | A list of approval outcomes that shouldn't trigger the LLM endpoint                       |

## 🤖 Bot Configuration `[bot]`

| Field         | Type                      | Default Value | Description                                                                                                                              |
| ------------- | ------------------------- | ------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `debounce_ms` | `unsigned 64-bit integer` | `1000`        | How long the bot collects events to batch them together                                                                                  |
| `max_turns`   | `unsigned size`           | `10`          | The number of consecutive tool call turns a model can take before being interrupted. Setting this to 0 will disable tools **everywhere** |

## 💬 Channel Configuration `[channel]`

> [!NOTE]
> Setting both `channel.blacklist` and `channel.whitelist` will produce a warning. The blacklist would be ignored in such case.

| Field       | Type                              | Default Value | Description                                                                   |
| ----------- | --------------------------------- | ------------- | ----------------------------------------------------------------------------- |
| `blacklist` | list of `unsigned 64-bit integer` | `[]`          | A list of channel IDs that Sweep **will ignore**                              |
| `whitelist` | list of `unsigned 64-bit integer` | `[]`          | A list of channel IDs to restrict Sweep's usage to **only those channel IDs** |

### 📝 Channel-specific Configuration `[[channel.override]]`

A channel override can be used for configuring Sweep's behavior in special channels. Overrides also take priority over a normal whitelist/blacklist.

| Field               | Type                         | Default Value | Description                                                                                                  |
| ------------------- | ---------------------------- | ------------- | ------------------------------------------------------------------------------------------------------------ |
| `id`                | `unsigned 64-bit integer`    | **None**      | The ID of the channel                                                                                        |
| `enable`            | `bool`                       | **None**      | Whether Sweep can be used in this channel                                                                    |
| `disable_all_tools` | `bool`                       | `false`       | Whether all tools should be disabled in this channel. Takes priority over `enable_tools` and `disable_tools` |
| `enable_tools`      | list of `internal tool name` | `[]`          | If set, all tools will be disabled, except these                                                             |
| `disable_tools`     | list of `internal tool name` | `[]`          | If set, all tools will be enabled, except these                                                              |

#### Example

```toml
[[channel.override]]
id = 1234
enable = true
enable_tools = ["create_category"]

[[channel.override]]
id = 5678
enable = true
disable_all_tools = true

[[channel.override]]
id = 9101
enable = true
disable_tools = ["get_local_time"]
```

## 🔷 Discord Configuration `[discord]`

| Field                                             | Type     | Default Value | Description           |
| ------------------------------------------------- | -------- | ------------- | --------------------- |
| **[sensitive]** `token` (`SWEEP__DISCORD__TOKEN`) | `string` | **None**      | The Discord bot token |

## 🌐 LLM Endpoint Configuration `[llm]`

| Field                                             | Type     | Default Value | Description                                                                                   |
| ------------------------------------------------- | -------- | ------------- | --------------------------------------------------------------------------------------------- |
| `endpoint`                                        | `string` | **None**      | The base URL of your OpenAI-compatible endpoint. **Make sure it doesn't have a trailing `/`** |
| `model`                                           | `string` | **None**      | The model you are using. You may use `""` for local backends that only have one loaded model  |
| **[sensitive]** `api_key` (`SWEEP__LLM__API_KEY`) | `string` | `""`          | Your API-key for your provider                                                                |
| `project_id`                                      | `string` | `""`          | The project ID of your application                                                            |
| `org_id`                                          | `string` | `""`          | The organization ID of your application                                                       |

## 🛠️ Tool Configuration `[tools]`

| Field     | Type                         | Default Value | Description                       |
| --------- | ---------------------------- | ------------- | --------------------------------- |
| `disable` | list of `internal tool name` | `[]`          | A list of globally disabled tools |

## 👥 User Configuration `[users]`

> [!NOTE]
> Setting both `users.blacklist` and `users.whitelist` will produce a warning. The blacklist would be ignored in such case.

| Field       | Type                              | Default Value | Description                                               |
| ----------- | --------------------------------- | ------------- | --------------------------------------------------------- |
| `blacklist` | list of `unsigned 64-bit integer` | `[]`          | A list of user IDs that Sweep **will ignore**             |
| `whitelist` | list of `unsigned 64-bit integer` | `[]`          | A list of user IDs of people who are allowed to use Sweep |
