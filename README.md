# CC Switch (Enhanced Fork)

**An enhanced fork of [CC Switch](https://github.com/farion1231/cc-switch) — adding image rectification and a multi-provider vision analysis skill for AI CLI tools.**

[English](README.md) | [中文](README_ZH.md) | [日本語](README_JA.md)

---

## About the Original Project

[CC Switch](https://github.com/farion1231/cc-switch) is a desktop app for managing AI CLI tools (Claude Code, Codex, Gemini CLI, OpenCode, OpenClaw, Hermes). It provides a visual interface for provider management, proxy/failover, MCP/Skills management, and usage tracking.

This fork adds features specifically for handling **image inputs** in AI CLI workflows — solving the problem where many third-party API providers don't support multimodal (vision) capabilities.

---

## What This Fork Adds

### 1. Image Rectifier

**Problem:** Many third-party API providers (relays, non-official endpoints) do not support multimodal input. When a `type: "image"` block containing base64 data appears in the messages array, these providers return errors or behave unexpectedly.

**Solution:** The image rectifier is a proxy-layer interceptor that runs **before requests are forwarded**. It scans `messages[*].content`, detects `type: "image"` blocks, strips the base64 data (often megabytes), and replaces them with a text prompt instructing the model to invoke a designated skill to handle the image.

**Key features:**
- Automatic detection and replacement of `type: "image"` blocks in messages
- Preserves image cache file paths when available (extracts from `[Image: source: ...]` blocks)
- Configurable skill name (default: `image-analysis`)
- Designed to work seamlessly with the image analysis skill below

**Configuration:** Proxy Settings → Advanced Rectifier → Image Rectifier. Toggle the switch and optionally change the skill name.

---

### 2. Image Analysis Skill

A standalone Python CLI tool for image vision recognition, compatible with any AI CLI that can execute shell commands.

**Capabilities:**
- Analyze local images (jpg/png/gif/webp/bmp), network image URLs, or clipboard images (macOS)
- Multi-image comparison via repeated `--image` flags
- Thinking mode (`--thinking`) for extended reasoning
- JSON output (`--json`) for programmatic use
- Token usage display (`--show-usage`)

**30+ pre-configured AI providers:**

| Platform | Representative Models |
|----------|----------------------|
| Volcengine (Ark/ModelArk) | Doubao Seed 2.0 Pro/Lite/Mini, Vision 250815 |
| SiliconFlow | Qwen3.6-35B-A3B, Qwen3.6-27B |
| Alibaba Bailian (DashScope) | Qwen3.6 Plus/Flash, Qwen3.5 Omni, Omni Flash, Kimi K2.6, MiniMax M2.5 |
| Zhipu | GLM-4.6V-Flash |
| SenseNova | SenseNova-6.7-Flash-Lite |

**Fallback mechanism:** The `--fallback` flag tries all configured providers in order, automatically switching on failure. The first successful result is returned with a `[provider-name]` label.

**Quick start:**

```bash
# Copy config template and add your API keys
cp skills/image-analysis/scripts/config.example.json skills/image-analysis/scripts/config.json

# Single image analysis with fallback
python skills/image-analysis/scripts/vision.py analyze \
  --image path/to/image.png \
  --prompt "Describe this image in detail" \
  --fallback

# Clipboard image (macOS)
python skills/image-analysis/scripts/vision.py analyze \
  --prompt "What text is in this image?" \
  --json --fallback
```

**How it works with the Image Rectifier:** When a model doesn't support images, the rectifier replaces the image block with a prompt like `"Please use the 'image-analysis' skill to read and analyze this image. Image path: /path/to/file.png"`. The model then calls this CLI tool to perform the actual vision analysis.

---

### 3. dev.sh — Development Helper Script

A convenience script that wraps `pnpm tauri dev` / `pnpm tauri build` with pre-configured settings:

```bash
./dev.sh         # Debug mode (default)
./dev.sh debug   # Debug mode with request body dump
./dev.sh release # Release mode
./dev.sh build   # Release build
```

Includes pre-set `CARGO_HTTP_PROXY` for developers behind firewalls.

---

## Installation

Download the latest release from the [original CC Switch releases page](https://github.com/farion1231/cc-switch/releases), or build from source:

```bash
git clone https://github.com/piaomiaoguying/cc-switch.git
cd cc-switch
./dev.sh build
```

---

## Documentation

- [User Manual](docs/user-manual/README.md)
- [Original CC Switch Repository](https://github.com/farion1231/cc-switch)

---

## License

MIT — same as the original project.
