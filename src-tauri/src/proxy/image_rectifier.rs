//! 图片整流器
//!
//! 在请求发送前自动检测 messages 中的 image 类型块，将 base64 图片数据替换为
//! 固定的文本提示，引导模型调用指定的 skill 处理图片。
//!
//! 这解决了不支持多模态/视觉理解的模型遇到 image 块时报错或行为异常的问题。

use serde_json::Value;

/// 图片整流结果
#[derive(Debug, Clone, Default)]
pub struct ImageRectifyResult {
    /// 是否应用了整流
    pub applied: bool,
    /// 移除的 image 块数量
    pub removed_image_blocks: usize,
    /// 移除的图片数据总字符数（base64 data 长度）
    pub removed_data_chars: usize,
}

/// 对请求体中的图片块进行整流
///
/// 遍历 messages[*].content，将 `type: "image"` 的块替换为文本提示。
///
/// 同时提取紧跟在图片后面的图片缓存引用块中的路径，拼入提示文本中供 skill 使用。
///
/// 注意：该函数会原地修改 body 对象
pub fn rectify_images(body: &mut Value, skill_name: &str, rectifier_enabled: bool) -> ImageRectifyResult {
    let mut result = ImageRectifyResult::default();

    if !rectifier_enabled {
        return result;
    }

    let messages = match body.get_mut("messages").and_then(|m| m.as_array_mut()) {
        Some(m) => m,
        None => return result,
    };

    // 先收集所有消息中需要替换的块索引，避免双重可变借用冲突
    // (msg_idx, block_idx, cache_path_to_remove)
    let mut patches: Vec<(usize, usize, Option<String>)> = Vec::new();

    for (msg_idx, msg) in messages.iter().enumerate() {
        let content = match msg.get("content").and_then(|c| c.as_array()) {
            Some(c) => c,
            None => continue,
        };

        for (block_idx, block) in content.iter().enumerate() {
            let block_type = block.get("type").and_then(|t| t.as_str());

            if block_type == Some("image") {
                // 检查下一个块是否为图片缓存引用，提取路径
                let cache_path = content.get(block_idx + 1).and_then(|next| {
                    let text = next.get("text").and_then(|t| t.as_str()).unwrap_or("");
                    if text.starts_with("[Image: source:") {
                        // 从 "[Image: source: /path/to/file.png]" 中提取路径
                        text.strip_prefix("[Image: source:")
                            .and_then(|s| s.strip_suffix(']'))
                            .map(|s| s.trim().to_string())
                    } else {
                        None
                    }
                });
                patches.push((msg_idx, block_idx, cache_path));
            }
        }
    }

    // 应用收集到的替换
    for (msg_idx, block_idx, cache_path) in patches.into_iter().rev() {
        let content = messages[msg_idx]
            .get_mut("content")
            .and_then(|c| c.as_array_mut())
            .unwrap();

        let block = &mut content[block_idx];

        // 统计移除的 base64 data 长度
        if let Some(data_len) = block
            .get("source")
            .and_then(|s| s.get("data"))
            .and_then(|d| d.as_str())
            .map(|d| d.len())
        {
            result.removed_data_chars += data_len;
        }

        // 替换为文本提示（包含缓存路径供 skill 使用）
        let replacement_text = if let Some(ref path) = cache_path {
            format!(
                "[Image: 此处有一张图片，请使用 `{}` skill 读取并分析这张图片。图片路径: {}]",
                skill_name, path
            )
        } else {
            format!(
                "[Image: 此处有一张图片，请使用 `{}` skill 读取并分析这张图片]",
                skill_name
            )
        };

        *block = serde_json::json!({
            "type": "text",
            "text": replacement_text
        });

        result.removed_image_blocks += 1;
        result.applied = true;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_rectify_single_image_block() {
        let mut body = json!({
            "model": "claude-opus-4-7",
            "messages": [{
                "role": "user",
                "content": [
                    { "type": "text", "text": "看一下这个" },
                    {
                        "type": "image",
                        "source": {
                            "type": "base64",
                            "media_type": "image/png",
                            "data": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg=="
                        }
                    }
                ]
            }]
        });

        let result = rectify_images(&mut body, "image-analyzer", true);

        assert!(result.applied);
        assert_eq!(result.removed_image_blocks, 1);
        assert_eq!(result.removed_data_chars, 96);

        let content = body["messages"][0]["content"].as_array().unwrap();
        assert_eq!(content.len(), 2);
        assert_eq!(content[0]["type"], "text");
        assert_eq!(content[1]["type"], "text");
        assert!(content[1]["text"].as_str().unwrap().contains("image-analyzer"));
    }

    #[test]
    fn test_rectify_image_with_cache_ref_keeps_path() {
        let mut body = json!({
            "model": "claude-opus-4-7",
            "messages": [{
                "role": "user",
                "content": [
                    { "type": "text", "text": "看一下" },
                    {
                        "type": "image",
                        "source": {
                            "type": "base64",
                            "media_type": "image/png",
                            "data": "abc"
                        }
                    },
                    {
                        "type": "text",
                        "text": "[Image: source: /Users/zs/.claude/image-cache/xxx/7.png]",
                        "cache_control": { "type": "ephemeral" }
                    }
                ]
            }]
        });

        let result = rectify_images(&mut body, "my-skill", true);

        assert!(result.applied);
        assert_eq!(result.removed_image_blocks, 1);
        assert_eq!(result.removed_data_chars, 3);

        let content = body["messages"][0]["content"].as_array().unwrap();
        // image 被替换，cache_ref 保留不动
        assert_eq!(content.len(), 3);
        assert_eq!(content[0]["type"], "text");
        assert_eq!(content[1]["type"], "text");
        let text = content[1]["text"].as_str().unwrap();
        assert!(text.contains("my-skill"));
        assert!(text.contains("/Users/zs/.claude/image-cache/xxx/7.png"));
        // cache_ref 块保持不变
        assert_eq!(content[2]["type"], "text");
        assert!(content[2]["text"].as_str().unwrap().starts_with("[Image: source:"));
    }

    #[test]
    fn test_rectify_multiple_image_blocks() {
        let mut body = json!({
            "model": "claude-opus-4-7",
            "messages": [{
                "role": "user",
                "content": [
                    {
                        "type": "image",
                        "source": { "type": "base64", "media_type": "image/png", "data": "abc" }
                    },
                    { "type": "text", "text": "中间文字" },
                    {
                        "type": "image",
                        "source": { "type": "base64", "media_type": "image/jpeg", "data": "def" }
                    }
                ]
            }]
        });

        let result = rectify_images(&mut body, "my-skill", true);

        assert!(result.applied);
        assert_eq!(result.removed_image_blocks, 2);
        assert_eq!(result.removed_data_chars, 6);

        let content = body["messages"][0]["content"].as_array().unwrap();
        assert_eq!(content.len(), 3);
        assert_eq!(content[0]["type"], "text");
        assert!(content[0]["text"].as_str().unwrap().contains("my-skill"));
        assert_eq!(content[1]["type"], "text");
        assert_eq!(content[1]["text"], "中间文字");
        assert_eq!(content[2]["type"], "text");
        assert!(content[2]["text"].as_str().unwrap().contains("my-skill"));
    }

    #[test]
    fn test_no_change_when_no_images() {
        let mut body = json!({
            "model": "claude-opus-4-7",
            "messages": [{
                "role": "user",
                "content": [
                    { "type": "text", "text": "hello" }
                ]
            }]
        });

        let result = rectify_images(&mut body, "irrelevant", true);
        assert!(!result.applied);
    }

    #[test]
    fn test_no_change_when_disabled() {
        let mut body = json!({
            "model": "claude-opus-4-7",
            "messages": [{
                "role": "user",
                "content": [
                    {
                        "type": "image",
                        "source": { "type": "base64", "media_type": "image/png", "data": "abc" }
                    }
                ]
            }]
        });

        let result = rectify_images(&mut body, "irrelevant", false);
        assert!(!result.applied);
        // 验证原始 image block 未被修改
        let content = body["messages"][0]["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "image");
    }

    #[test]
    fn test_rectify_image_in_multiple_messages() {
        let mut body = json!({
            "model": "claude-opus-4-7",
            "messages": [
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "image",
                            "source": { "type": "base64", "media_type": "image/png", "data": "img1" }
                        }
                    ]
                },
                {
                    "role": "user",
                    "content": [
                        { "type": "text", "text": "后续文字" },
                        {
                            "type": "image",
                            "source": { "type": "base64", "media_type": "image/jpeg", "data": "img2" }
                        }
                    ]
                }
            ]
        });

        let result = rectify_images(&mut body, "img-skill", true);

        assert!(result.applied);
        assert_eq!(result.removed_image_blocks, 2);
        assert_eq!(result.removed_data_chars, 8);
    }
}
