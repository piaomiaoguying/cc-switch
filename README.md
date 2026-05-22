# CC Switch（增强分支）

**[CC Switch](https://github.com/farion1231/cc-switch) 的增强分支 — 为 AI CLI 工具新增图片整流器和多 Provider 视觉分析技能。**

[English](README_ZH.md) | 中文 | [日本語](README_JA.md)

---

## 为什么需要本项目？

**AI CLI 工具 + 第三方模型 = 图片理解真空地带。** 这是 Claude Code、OpenCode 等工具搭配 DeepSeek 等不支持多模态的模型时，最让人头疼的问题。

### 传统方案的死结

常见的做法是在全局 `CLAUDE.md` 中写规则，强制要求使用某个 Skill 或 MCP 去处理图片。但这里有一个绕不开的坑：

**Claude Code 在系统层面硬编码了指令 — 粘贴图片时必定调用 `read` 工具，直接把图片发给模型。** 这个系统指令的优先级高于 `CLAUDE.md`，所以你的规则根本拦不住它。

结果是什么？Ctrl+V 粘贴一张截图，模型收到 base64 图片数据，直接报错或胡言乱语。**整段对话就此腐烂，无法继续。**

![关闭图片整流器时，Claude Code + DeepSeek 对话直接腐烂](assets/screenshots/claudecode+DeepSeek关闭图片整流器运行示意图.png)

更糟的是，Debug 时我们几乎不会把截图先保存到本地再传 — 直接截图 → 粘贴到剪贴板 → Ctrl+V 才是最自然的工作流。传统方案只适用于"先保存为文件，再给路径"的场景，跟实际使用习惯完全错位。

### 本项目的解法

**图片整流器 + 图片分析技能，两层防线。**

| 场景 | 传统方案 | 本项目 |
|------|---------|--------|
| Ctrl+V 粘贴剪贴板图片到 Claude Code | ❌ 系统硬编码 `read`，直接发给模型，对话烂掉 | ✅ 整流器在代理层拦截，替换为文本提示，引导模型调用 Skill |
| 给定图片文件路径 | ⚠️ `CLAUDE.md` 规则勉强可用，但不同 CLI 行为不一致 | ✅ 整流器统一拦截，不依赖 MD 文件规则 |
| OpenCode 粘贴剪贴板图片 | ❌ OpenCode 不生成临时文件，路径都拿不到 | ✅ Skill 脚本自动检测：有路径读文件，没路径直接从剪贴板读取 |
| 多 Provider 切换 | ❌ 单个模型挂了就挂了 | ✅ Fallback 机制，30+ Provider 自动切换 |

> **关闭整流器：** Claude Code + DeepSeek，Ctrl+V 粘贴图片后对话直接腐烂

![claudecode+DeepSeek关闭图片整流器运行示意图](assets/screenshots/claudecode+DeepSeek关闭图片整流器运行示意图.png)

> **启用整流器：** Claude Code + DeepSeek，图片被正常拦截并引导调用 Skill

![claudecode+DeepSeek启用图片整流器运行示意图](assets/screenshots/claudecode+DeepSeek启用图片整流器运行示意图.png)

> **OpenCode + DeepSeek：** Skill 检测到无临时文件，自动从剪贴板读取

![opencode+DeepSeek运行示意图](assets/screenshots/opencode+DeepSeek运行示意图.png)

### 两层防线的分工

1. **图片整流器（代理层）** — 在请求发出前拦截。检测 messages 中的 base64 图片块，移除原始数据，替换为文本提示，引导模型调用 Skill。**这一步在模型看到请求之前就完成了。**

2. **图片分析技能（CLI 工具）** — 被模型调用后，智能判断图片来源：传了本地路径就读文件，没传路径就从系统剪贴板直接读取。**兼容 Claude Code（粘贴生成临时文件）和 OpenCode（粘贴不生成临时文件）两种行为。**

---

## 原项目简介

[CC Switch](https://github.com/farion1231/cc-switch) 是一款管理 AI CLI 工具（Claude Code、Codex、Gemini CLI、OpenCode、OpenClaw、Hermes）的桌面应用，提供供应商管理、代理/故障转移、MCP/Skills 管理和用量统计等功能。

本分支在此基础上新增了针对**图片输入**处理的功能，解决大量第三方 API 供应商不支持多模态（视觉理解）的问题。

---

## 本分支新增功能

### 1. 图片整流器

**解决的问题：** 大量第三方 API 供应商（中转站、非官方接口）不支持多模态输入。当请求的 messages 数组中出现 `type: "image"` 的 base64 图片块时，这些供应商会报错或行为异常。

**解决方案：** 图片整流器是代理层的拦截器，在**请求转发之前**运行。它扫描 `messages[*].content`，检测 `type: "image"` 块，移除 base64 数据（通常数 MB），替换为文本提示，引导模型调用指定 skill 来处理图片。

**关键特性：**
- 自动检测并替换 messages 中的 `type: "image"` 块
- 存在图片缓存引用时自动提取文件路径拼入提示
- 可配置调用的 skill 名称（默认：`image-analysis`）
- 与下方的图片分析技能无缝配合

**配置方式：** 代理设置 → 高级整流器 → 图片整流器。打开开关，可按需修改 skill 名称。

![图片整流器开关](assets/screenshots/图片整流器开关示意图.png)

---

### 2. 图片分析技能

一个独立的 Python 命令行图片视觉识别工具，兼容任何能执行 Shell 命令的 AI CLI。

**核心能力：**
- 支持本地图片（jpg/png/gif/webp/bmp）、网络图片 URL、系统剪贴板（macOS AppleScript）
- 多图对比（多次 `--image`）
- 思考模式（`--thinking`）
- JSON 输出（`--json`）
- Token 用量显示（`--show-usage`）

**预置 30+ AI Provider：**

| 平台 | 代表模型 |
|------|---------|
| 火山引擎 | 豆包 Seed 2.0 Pro/Lite/Mini、Vision 250815 |
| 硅基流动 | Qwen3.6-35B-A3B、Qwen3.6-27B |
| 阿里百炼 | Qwen3.6 Plus/Flash、Qwen3.5 Omni、Kimi K2.6、MiniMax M2.5 |
| 智谱 | GLM-4.6V-Flash |
| 商汤 | SenseNova-6.7-Flash-Lite |

**Fallback 机制：** `--fallback` 参数按配置顺序依次尝试所有 provider，失败自动切换。第一个成功的结果标注 `[provider名]` 后返回。

**快速上手：**

```bash
# 复制配置模板并填入 API key
cp skills/image-analysis/scripts/config.example.json skills/image-analysis/scripts/config.json

# 单图分析（fallback 模式）
python skills/image-analysis/scripts/vision.py analyze \
  --image path/to/image.png \
  --prompt "详细描述这张图片" \
  --fallback

# 剪贴板图片分析（macOS）
python skills/image-analysis/scripts/vision.py analyze \
  --prompt "这张图片里有什么文字？" \
  --json --fallback
```

**与图片整流器的联动：** 当某模型不支持图片时，整流器会将图片块替换为提示词如 `"请使用 'image-analysis' skill 读取并分析这张图片。图片路径: /path/to/file.png"`，模型随后调用此 CLI 工具完成实际视觉分析。

---

### 3. dev.sh — 开发启动脚本

封装 `pnpm tauri dev` / `pnpm tauri build` 的便捷脚本：

```bash
./dev.sh         # Debug 模式（默认）
./dev.sh debug   # Debug 模式（含请求体打印）
./dev.sh release # Release 模式
./dev.sh build   # Release 编译
```

预置 `CARGO_HTTP_PROXY`，方便国内网络环境下编译 Rust 依赖。

---

## 安装

从源码编译：

```bash
git clone https://github.com/piaomiaoguying/cc-switch.git
cd cc-switch
./dev.sh build
```

---

## 文档

- [用户手册](docs/user-manual/README.md)
- [原 CC Switch 仓库](https://github.com/farion1231/cc-switch)

---

## 许可证

MIT — 与原项目一致。
