# CC Switch（增强分支）

**[CC Switch](https://github.com/farion1231/cc-switch) 的增强分支 — 为 AI CLI 工具新增图片整流器和多 Provider 视觉分析技能。**

[English](README_ZH.md) | 中文 | [日本語](README_JA.md)

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

从[原 CC Switch 的 Releases 页面](https://github.com/farion1231/cc-switch/releases)下载最新版本，或从源码编译：

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
