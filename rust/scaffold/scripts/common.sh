#!/bin/bash

# Scaffold 脚本通用函数库
# 提供：颜色定义、日志函数、常用工具函数

# ============================================
# 颜色定义
# ============================================
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly CYAN='\033[0;36m'
readonly NC='\033[0m' # No Color

# ============================================
# 日志函数
# ============================================

# 打印普通信息
info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

# 打印成功信息
success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# 打印警告信息
warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# 打印错误信息
error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 打印调试信息
debug() {
    echo -e "${CYAN}[DEBUG]${NC} $1"
}

# ============================================
# 工具函数
# ============================================

# 获取脚本所在目录的父目录（项目根目录）
# 用法: cd_root
cd_root() {
    local script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    cd "$script_dir/.."
}

# 获取项目根目录路径
# 用法: root_path=$(get_root_path)
get_root_path() {
    local script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    echo "$script_dir/.."
}

# 检查命令是否存在
# 用法: if command_exists cargo; then ...
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# 检查文件是否存在
# 用法: if file_exists "Cargo.toml"; then ...
file_exists() {
    [ -f "$1" ]
}

# 检查目录是否存在
# 用法: if dir_exists "target"; then ...
dir_exists() {
    [ -d "$1" ]
}

# 确认提示
# 用法: if confirm "是否继续?"; then ...
confirm() {
    local prompt="$1"
    local default="${2:-N}"

    if [ "$default" = "Y" ]; then
        prompt="$prompt [Y/n]: "
    else
        prompt="$prompt [y/N]: "
    fi

    read -p "$prompt" -n 1 -r response
    echo
    response=${response:-$default}

    [[ $response =~ ^[Yy]$ ]]
}

# 打印分隔线
# 用法: separator "="
separator() {
    local char="${1:-=}"
    local width=50
    printf "${char}%.0s" $(seq 1 $width)
    echo
}

# 打印标题
# 用法: print_title "开始构建"
print_title() {
    echo ""
    separator "="
    info "===== $1 ====="
    separator "="
    echo ""
}

# 检查 Rust 项目
# 用法: check_rust_project
check_rust_project() {
    if ! file_exists "Cargo.toml"; then
        error "当前目录不是 Rust 项目（未找到 Cargo.toml）"
        exit 1
    fi
}

# 获取二进制文件路径
# 用法: binary_path=$(get_binary_path "release")
get_binary_path() {
    local build_type="${1:-debug}"
    if [ "$build_type" = "release" ]; then
        echo "target/release/scaffold"
    else
        echo "target/debug/scaffold"
    fi
}

# 构建项目
# 用法: build_project "release"
build_project() {
    local build_type="${1:-debug}"
    local build_args=""

    if [ "$build_type" = "release" ]; then
        build_args="--release"
    fi

    info "构建项目 ($build_type)..."
    cargo build $build_args
}

# 清理构建目录
# 用法: clean_build
clean_build() {
    info "清理构建目录..."
    cargo clean
}
