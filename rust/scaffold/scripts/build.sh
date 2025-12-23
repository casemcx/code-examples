#!/bin/bash

# Scaffold 构建脚本
# 用于构建 Rust workspace 项目

set -e  # 遇到错误立即退出

# 引入通用函数
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

cd_root

# 默认参数
BUILD_TYPE="debug"
VERBOSE=false
CLEAN=false
RUN_AFTER_BUILD=false
RUN_ARGS=""

# 解析命令行参数
while [[ $# -gt 0 ]]; do
    case $1 in
        -r|--release)
            BUILD_TYPE="release"
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -c|--clean)
            CLEAN=true
            shift
            ;;
        --run)
            RUN_AFTER_BUILD=true
            shift
            ;;
        --run-args)
            RUN_ARGS="$2"
            shift 2
            ;;
        -h|--help)
            echo "用法: $0 [选项]"
            echo ""
            echo "选项:"
            echo "  -r, --release          构建发布版本"
            echo "  -v, --verbose          显示详细输出"
            echo "  -c, --clean            构建前清理"
            echo "  --run                  构建后运行程序"
            echo "  --run-args <args>      运行程序时的参数"
            echo "  -h, --help             显示帮助信息"
            echo ""
            echo "示例:"
            echo "  $0                      # 构建调试版本"
            echo "  $0 --release            # 构建发布版本"
            echo "  $0 --release --run      # 构建并运行发布版本"
            echo "  $0 --clean --release    # 清理后构建发布版本"
            exit 0
            ;;
        *)
            error "未知参数: $1"
            echo "使用 -h 或 --help 查看帮助"
            exit 1
            ;;
    esac
done

# 显示构建信息
info "开始构建 Scaffold 项目"
info "构建类型: $BUILD_TYPE"

# 清理构建
if [ "$CLEAN" = true ]; then
    info "清理之前的构建..."
    clean_build
    success "清理完成"
fi

# 构建参数
BUILD_ARGS=()
if [ "$BUILD_TYPE" = "release" ]; then
    BUILD_ARGS+=("--release")
fi
if [ "$VERBOSE" = true ]; then
    BUILD_ARGS+=("--verbose")
fi

# 执行构建
info "执行 cargo build..."
cargo build "${BUILD_ARGS[@]}"

success "构建完成!"

# 获取二进制文件路径
BINARY_PATH=$(get_binary_path "$BUILD_TYPE")
info "二进制文件: $BINARY_PATH"

# 显示文件大小
if file_exists "$BINARY_PATH"; then
    FILE_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
    info "文件大小: $FILE_SIZE"
fi

# 运行程序
if [ "$RUN_AFTER_BUILD" = true ]; then
    info "运行程序..."
    echo ""
    "$BINARY_PATH" $RUN_ARGS
fi
