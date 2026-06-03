---
name: image-analysis
description: 图片分析与识别，可分析本地图片和网络图片。适用于 OCR、物体识别、场景理解等。当用户发送图片或要求分析图片时必须使用此技能。

---

# 图片分析与识别

## 用法

```bash
# 分析剪贴板图片（不传 --image 时自动读取系统剪贴板）
scripts/run.sh scripts/vision.py analyze --prompt "描述" --fallback

# 指定图片
scripts/run.sh scripts/vision.py analyze --image <路径|URL> --prompt "描述" --fallback

# 指定模型
scripts/run.sh scripts/vision.py analyze --image <路径> --prompt "描述" --model <provider名>

# 其他参数
--image ... --image ...    # 多图对比
--thinking                 # 思考模式（需 provider 支持）
--json                     # JSON 输出
--show-usage               # 显示 token 用量
```

## 关键规则

- **优先使用 `--fallback`**：按 config.json 顺序依次尝试所有 provider，失败自动切换下一个，第一个成功即返回。结果标注 `[provider名]`。
- 本地图片自动转 Base64，支持 jpg/png/gif/webp/bmp
- `--thinking` 参数需要 provider 支持，不支持时自动忽略并给出警告

