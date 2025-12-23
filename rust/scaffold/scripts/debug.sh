#!/bin/bash

# Scaffold 调试脚本
# 用于调试 Rust workspace 项目

set -e

# 引入通用函数
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

cd_root

# 默认参数
LOG_LEVEL="debug"
BUILD_FIRST=false
USE_RELEASE=false
COMMAND_ARGS=""
USE_LLDB=false

# 解析命令行参数
while [[ $# -gt 0 ]]; do
    case $1 in
        -l|--log-level)
            LOG_LEVEL="$2"
            shift 2
            ;;
        -b|--build)
            BUILD_FIRST=true
            shift
            ;;
        -r|--release)
            USE_RELEASE=true
            shift
            ;;
        --lldb)
            USE_LLDB=true
            shift
            ;;
        -h|--help)
            echo "用法: $0 [选项] [命令参数]"
            echo ""
            echo "选项:"
            echo "  -l, --log-level <level>  设置日志级别 (trace/debug/info/warn/error)"
            echo "                          默认: debug"
            echo "  -b, --build             运行前先构建"
            echo "  -r, --release           使用 release 版本调试"
            echo "  --lldb                  使用 lldb 调试器"
            echo "  -h, --help              显示帮助信息"
            echo ""
            echo "示例:"
            echo "  $0                      # 使用 debug 日志运行"
            echo "  $0 -l trace             # 使用 trace 日志运行"
            echo "  $0 -b                   # 构建后运行"
            echo "  $0 -b -- new --help     # 构建后运行并显示 new 命令帮助"
            echo "  $0 --lldb               # 使用 lldb 调试器启动"
            exit 0
            ;;
        *)
            # 将剩余参数传递给程序
            COMMAND_ARGS="$*"
            break
            ;;
    esac
done

# 确定二进制文件路径
BINARY_PATH=$(get_binary_path "$([ "$USE_RELEASE" = true ] && echo 'release' || echo 'debug')")

# 构建前检查
if [ "$BUILD_FIRST" = true ]; then
    info "构建项目..."
    if [ "$USE_RELEASE" = true ]; then
        cargo build --release
    else
        cargo build
    fi
    success "构建完成"
fi

# 检查二进制文件是否存在
if ! file_exists "$BINARY_PATH"; then
    warn "二进制文件不存在: $BINARY_PATH"
    info "正在构建..."
    if [ "$USE_RELEASE" = true ]; then
        cargo build --release
    else
        cargo build
    fi
fi

# 显示调试信息
info "===== 调试配置 ====="
info "二进制文件: $BINARY_PATH"
info "日志级别: $LOG_LEVEL"
info "工作目录: $(pwd)"
info "==================="
echo ""

# 使用 lldb 调试器
if [ "$USE_LLDB" = true ]; then
    info "使用 lldb 调试器启动..."
    lldb "$BINARY_PATH" -- $COMMAND_ARGS
else
    # 使用 --log-level 参数运行
    info "启动程序 (--log-level=$LOG_LEVEL)..."
    echo ""

    # 设置错误堆栈
    export RUST_BACKTRACE=1

    # 构建命令
    if [ -z "$COMMAND_ARGS" ]; then
        "$BINARY_PATH" --log-level "$LOG_LEVEL"
    else
        "$BINARY_PATH" --log-level "$LOG_LEVEL" $COMMAND_ARGS
    fi
fi
