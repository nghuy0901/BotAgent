# 🏆 Model Ranking

This file is used to rate how well Sweep integrates with certain large language models.

| Model             | Parameters             | Quant    | Context | Tool Calling | Understanding | Notes                                                                          |
| ----------------- | ---------------------- | -------- | ------- | ------------ | ------------- | ------------------------------------------------------------------------------ |
| `Qwen3.6-35B-A3B` | `35B (MoE, 3B active)` | `Q4_K_M` | `32768` | Great        | Great         | One of the best local models for Sweep.                                        |
| `Qwen3.5-9B`      | `9B`                   | `Q4_K_M` | `32768` | Good         | Great         | It understands requests well, but has problems with multi-step tool calling. ` |

## 🤝 Contributing

If you want to add a ranking, just create a new issue and call it `rating: {MODEL_NAME}` with the following content:

```
Model Name:
Parameters:
Quant:
Context:

Ratings:
- Tool Calling:
- Understanding:

Notes:
```
