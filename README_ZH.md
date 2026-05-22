# CC Switch (Enhanced Fork)

**An enhanced fork of [CC Switch](https://github.com/farion1231/cc-switch) — adding image rectification and a multi-provider vision analysis skill for AI CLI tools.**

English | [中文](README.md)

---

## Why This Project?

**AI CLI tools + third-party models = a vacuum in image understanding.** This is the most frustrating problem when using tools like Claude Code or OpenCode with models (e.g., DeepSeek) that don't support multimodal input.

### The Dead End of Traditional Approaches

The common workaround is to add rules in the global `CLAUDE.md`, forcing the use of a Skill or MCP to handle images. But there's an inescapable pitfall:

**Claude Code hardcodes a system-level instruction — when you paste an image, it ALWAYS calls the `read` tool, sending the image directly to the model.** This system instruction has higher priority than `CLAUDE.md`, so your rules simply can't intercept it.

The result? Ctrl+V paste a screenshot, the model receives raw base64 image data, and immediately errors out or hallucinates. **The entire conversation is corrupted beyond recovery.**

![Claude Code + DeepSeek conversation corrupted when image rectifier is off](assets/screenshots/claudecode+DeepSeek关闭图片整流器运行示意图.png)

Worse still, during debugging sessions we almost never bother saving screenshots to local files first — screenshot → clipboard → Ctrl+V is the natural workflow. Traditional approaches only work for "save to file first, then provide the path" scenarios, completely mismatched with real-world usage habits.

### How This Project Solves It

**Image Rectifier + Image Analysis Skill — a two-layer defense.**

| Scenario | Traditional Approach | This Project |
|----------|---------------------|-------------|
| Ctrl+V paste clipboard image to Claude Code | ❌ System hardcodes `read`, sends directly to model, conversation rots | ✅ Rectifier intercepts at proxy layer, replaces with text prompt, guides model to invoke Skill |
| Providing an image file path | ⚠️ `CLAUDE.md` rules barely work, but behavior varies across CLIs | ✅ Rectifier intercepts uniformly, no dependency on MD file rules |
| OpenCode paste clipboard image | ❌ OpenCode doesn't generate temp files, can't even get a path | ✅ Skill script auto-detects: reads file if path given, reads from clipboard otherwise |
| Multi-provider switching | ❌ Single model goes down, everything breaks | ✅ Fallback mechanism with auto-switching across 30+ providers |

> **Rectifier OFF:** Claude Code + DeepSeek — conversation is corrupted immediately after Ctrl+V pasting an image

![claudecode+DeepSeek关闭图片整流器运行示意图](assets/screenshots/claudecode+DeepSeek关闭图片整流器运行示意图.png)

> **Rectifier ON:** Claude Code + DeepSeek — image is intercepted properly, model is guided to invoke the Skill

![claudecode+DeepSeek启用图片整流器运行示意图](assets/screenshots/claudecode+DeepSeek启用图片整流器运行示意图.png)

> **OpenCode + DeepSeek:** Skill detects no temp file, reads directly from clipboard

![opencode+DeepSeek运行示意图](assets/screenshots/opencode+DeepSeek运行示意图.png)

### How the Two Layers Work

1. **Image Rectifier (Proxy Layer)** — Intercepts requests before they're forwarded. Detects base64 image blocks in `messages`, removes the raw data (often several megabytes), and replaces them with text prompts guiding the model to invoke the designated Skill. **This happens before the model ever sees the request.**

2. **Image Analysis Skill (CLI Tool)** — Once invoked by the model, intelligently determines the image source: reads from a local file if a path is provided, or reads directly from the system clipboard if no path is given. **Compatible with both Claude Code (paste generates temp files) and OpenCode (paste doesn't generate temp files).**

> **Note:** This project has only been thoroughly tested on **Claude Code** and **OpenCode**. Other AI CLI tools (Codex, Gemini CLI, OpenClaw, Hermes, etc.) should work in theory but haven't been verified. Feel free to test and provide feedback.

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
- Preserves image cache file paths when available
- Configurable skill name (default: `image-analysis`)
- Designed to work seamlessly with the image analysis skill below

**Configuration:** Router Settings → Rectifier section → Image Rectifier. Toggle the switch and optionally change the skill name.

![Image Rectifier Switch](assets/screenshots/图片整流器开关示意图.png)

---

### 2. Image Analysis Skill

A standalone Python CLI tool for image vision recognition, compatible with any AI CLI that can execute shell commands.

**Core capabilities:**
- Supports local images (jpg/png/gif/webp/bmp), network image URLs, and system clipboard (macOS AppleScript)
- Multi-image comparison via repeated `--image` flags
- Fallback mechanism: tries all configured providers in order, auto-switching on failure. The first successful result is returned with a `[provider-name]` label

**30+ pre-configured AI providers:**

| Platform | Representative Models |
|----------|----------------------|
| Volcengine | Doubao Seed 2.0 Pro/Lite/Mini, Vision 250815 |
| SiliconFlow | Qwen3.6-35B-A3B, Qwen3.6-27B |
| Alibaba Bailian | Qwen3.6 Plus/Flash, Qwen3.5 Omni, Kimi K2.6, MiniMax M2.5 |
| Zhipu | GLM-4.6V-Flash |
| SenseNova | SenseNova-6.7-Flash-Lite |

**How it works with the Image Rectifier:** When a model doesn't support images, the rectifier replaces image blocks with text prompts guiding the model to invoke the Skill. The model then calls the CLI tool automatically to perform the actual vision analysis. The entire process is transparent to the user — no need to manually run Python scripts.

---

### 3. dev.sh — Development Helper Script

A convenience script that wraps `pnpm tauri dev` / `pnpm tauri build`:

```bash
./dev.sh         # Debug mode (default)
./dev.sh debug   # Debug mode (with request body dump)
./dev.sh release # Release mode
./dev.sh build   # Release build
```

Includes pre-set `CARGO_HTTP_PROXY` for developers behind firewalls.

> **Proxy address note:** The proxy address `http://127.0.0.1:7890` pre-configured on lines 10-11 of `dev.sh` is an example value. Users in mainland China usually need a proxy to pull Rust dependencies. Modify the following two lines in `dev.sh` to match your local proxy's port:
>
> ```bash
> export CARGO_HTTP_PROXY=http://127.0.0.1:7890
> export CARGO_HTTPS_PROXY=http://127.0.0.1:7890
> ```
>
> Replace `7890` with your local proxy's actual port (e.g., Clash default: 7890, V2Ray default: 10809, or custom).

---

## Installation

Build from source:

```bash
git clone https://github.com/piaomiaoguying/cc-switch.git
cd cc-switch
./dev.sh build
```

---

## Setup Guide

### Step 1: Register with an AI Provider and Get an API Key

This Skill comes pre-configured with multiple platform vision models, all offering free quotas. Pick the platforms you prefer:

| Platform | Representative Models | Sign-up URL |
|----------|----------------------|-------------|
| Alibaba Bailian | Qwen3.6 Plus/Flash, Qwen3.5 Omni, etc. | [dashscope.aliyun.com](https://dashscope.aliyun.com) |
| Zhipu | GLM-4.6V-Flash | [open.bigmodel.cn](https://open.bigmodel.cn) |
| SiliconFlow | Qwen3.6-35B-A3B, etc. | [siliconflow.cn](https://siliconflow.cn) |
| Volcengine | Doubao Seed 2.0 Pro/Lite/Mini | [console.volcengine.com](https://console.volcengine.com) |
| SenseNova | SenseNova-6.7-Flash-Lite | [platform.sensenova.cn](https://platform.sensenova.cn) |

After registering, get your API Key and fill it into the Skill configuration file:

```bash
cp skills/image-analysis/scripts/config.example.json skills/image-analysis/scripts/config.json
# Edit config.json and fill in the API keys for each platform
```

### Step 2: Configure the Global CLAUDE.md

Add the following rules to your global `CLAUDE.md` (`~/.claude/CLAUDE.md`) to prohibit direct use of the `read` tool for images:

```markdown
## Image Handling Rules
Do NOT use the `read` tool to read any image files (such as .png, .jpg, .jpeg)
MUST use the image-analysis skill to analyze images
When the model indicates it does not support direct image input, automatically invoke the image-analysis skill to analyze the image
```

### Step 3: Import the Skill into Your AI CLI

It's recommended to manage Skills through CC Switch rather than manually copying folders:

1. Keep the `skills/image-analysis` configuration folder under the CC Switch directory
2. Open CC Switch → **Skills Management**
3. Under Claude Code and OpenCode, toggle the Skill switch to enable `image-analysis`

CC Switch will automatically sync the Skill to each CLI's Skill directory.

### Step 4: Configure CC Switch Routing and the Rectifier

This is the most critical step — three layers of switches need to be turned on:

**① Enable Local Routing**

Go to CC Switch → **Router Settings**:
- Turn on the **Local Routing master switch**
- Turn on the **Claude Code** routing switch (a common oversight: master switch is on but the Claude Code switch is off)

**② Configure the Image Rectifier**

Scroll down in Router Settings to find the **Rectifier** section:
- Turn on the **Rectifier master switch**
- Turn on the **Image Rectifier** switch
- Enter `image-analysis` in the Skill name field

![Image Rectifier Switch](assets/screenshots/图片整流器开关示意图.png)

### Done

Once everything above is configured, existing Claude Code instances should pick up the changes immediately. If it doesn't take effect, simply restart a new Claude Code instance. The model's network requests will now pass through the CC Switch proxy, which will intercept and modify the conversation data:

> Image data (base64) that the model doesn't support is forcibly replaced with text content. The text guides the model to invoke your `image-analysis` Skill to complete the image understanding.

The overall flow from this point on:

```
Ctrl+V paste image → CC Switch proxy intercepts → strips base64, replaces with text prompt
→ Model receives text, invokes image-analysis Skill → Skill reads the image and returns analysis results
```

---

## Documentation

- [Original CC Switch Repository](https://github.com/farion1231/cc-switch)

---

## License

MIT — same as the original project.
