#!/usr/bin/env python3
"""
图片识别与理解命令行工具
"""
import sys
import json
import argparse
import subprocess
import base64

# 设置 stdout 编码为 UTF-8（Windows 兼容）
if sys.platform == 'win32':
    import io
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')

from vision_manager import VisionManager, load_config


def _clipboard_to_data_url():
    """从 macOS 剪贴板提取图片，返回 data URL（不写磁盘）"""
    for type_name, mime in [
        ('PNGf', 'image/png'),
        ('TIFF', 'image/tiff'),
        ('GIFf', 'image/gif'),
    ]:
        r = subprocess.run(
            ['osascript', '-e', f'the clipboard as «class {type_name}»'],
            capture_output=True, text=True
        )
        if r.returncode != 0:
            continue
        raw = r.stdout.strip()
        if not raw:
            continue
        prefix = f'«data {type_name}'
        if not raw.startswith(prefix) or not raw.endswith('»'):
            continue
        hex_part = raw[len(prefix):-1]
        try:
            binary = bytes.fromhex(hex_part)
        except ValueError:
            continue
        b64 = base64.b64encode(binary).decode()
        return f'data:{mime};base64,{b64}'
    raise RuntimeError('剪贴板中无图片数据')


def main():
    parser = argparse.ArgumentParser(description='图片识别与理解')
    subparsers = parser.add_subparsers(dest='command', help='命令')

    # analyze 命令
    analyze_parser = subparsers.add_parser('analyze', help='分析图片')
    analyze_parser.add_argument('--image', action='append', help='图片 URL 或本地路径（可多次指定）')
    analyze_parser.add_argument('--prompt', required=True, help='提示词')
    analyze_parser.add_argument('--model', help='指定模型（从 config.json 的 providers 中选择）')
    analyze_parser.add_argument('--fallback', action='store_true', help='按配置顺序依次尝试所有 provider，失败自动切换下一个')
    analyze_parser.add_argument('--thinking', action='store_true', help='开启思考模式（需 provider 支持）')
    analyze_parser.add_argument('--json', action='store_true', help='JSON 格式输出')
    analyze_parser.add_argument('--show-usage', action='store_true', help='显示 token 使用情况')
    analyze_parser.add_argument('--config', help='配置文件路径')

    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        return

    try:
        # 加载配置
        config = load_config(args.config)
        manager = VisionManager(config)

        if args.command == 'analyze':
            if not args.image:
                try:
                    args.image = [_clipboard_to_data_url()]
                except RuntimeError:
                    print("错误：请至少提供一个图片（--image），或确保系统剪贴板中有图片数据")
                    sys.exit(1)

            model_name = args.model
            if args.fallback:
                if args.model:
                    print("警告：--fallback 模式下忽略 --model 参数")
                result, model_name = manager.analyze_with_fallback(
                    prompt=args.prompt,
                    images=args.image,
                    thinking=args.thinking
                )
            else:
                result = manager.analyze(
                    prompt=args.prompt,
                    images=args.image,
                    model=args.model,
                    thinking=args.thinking
                )

            if args.json:
                output = {"model": model_name, "result": result} if model_name else result
                print(json.dumps(output, ensure_ascii=False, indent=2))
            else:
                print(manager.format_result(result, show_usage=args.show_usage, model_name=model_name))

    except FileNotFoundError as e:
        print(f"错误：{e}")
        print("\n请先创建配置文件 config.json，参考 config.json.example")
        sys.exit(1)
    except ValueError as e:
        print(f"错误：{e}")
        sys.exit(1)
    except Exception as e:
        print(f"发生错误：{e}")
        sys.exit(1)


if __name__ == '__main__':
    main()
