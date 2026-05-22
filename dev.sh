#!/bin/bash
set -e

# Rust 工具链
export PATH="$HOME/.cargo/bin:$PATH"

cd "$(dirname "$0")"

# 仅为 cargo 编译设置代理（国内网络需要），不影响应用运行时
export CARGO_HTTP_PROXY=http://127.0.0.1:7890
export CARGO_HTTPS_PROXY=http://127.0.0.1:7890

echo "=== CC Switch 开发模式启动 ==="
pnpm dev
