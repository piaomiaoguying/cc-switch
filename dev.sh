#!/bin/bash
set -e

# Rust 工具链
export PATH="$HOME/.cargo/bin:$PATH"

cd "$(dirname "$0")"

# 仅为 cargo 编译设置代理（国内网络需要），不影响应用运行时
export CARGO_HTTP_PROXY=http://127.0.0.1:7890
export CARGO_HTTPS_PROXY=http://127.0.0.1:7890

MODE="${1:-debug}"

pnpm install --frozen-lockfile

case "$MODE" in
  debug|d)
    echo "=== CC Switch 开发模式启动 (DEBUG) ==="
    echo "  - Rust 编译: debug 模式（含 DUMP 请求体打印）"
    echo "  - 提示: Debug 模式下，代理会在终端打印完整请求体"
    echo ""
    pnpm tauri dev
    ;;
  release|r)
    echo "=== CC Switch 开发模式启动 (RELEASE) ==="
    echo "  - Rust 编译: release 模式（无调试输出）"
    echo ""
    pnpm tauri dev --release
    ;;
  build-debug)
    echo "=== CC Switch 编译 (DEBUG) ==="
    pnpm tauri build --debug
    ;;
  build|b)
    echo "=== CC Switch 编译 (RELEASE) ==="
    pnpm tauri build
    ;;
  *)
    echo "用法: ./dev.sh [debug|d|release|r|build|b|build-debug]"
    echo ""
    echo "  debug, d        开发模式 - Debug 编译（含 DUMP 请求体打印）"
    echo "  release, r      开发模式 - Release 编译（无调试输出）"
    echo "  build, b        正式编译 - Release 模式"
    echo "  build-debug     正式编译 - Debug 模式"
    echo ""
    echo "  默认: debug"
    exit 1
    ;;
esac
