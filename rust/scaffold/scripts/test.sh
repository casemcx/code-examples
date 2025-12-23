#!/bin/bash

# Scaffold 测试脚本
# 用于测试命令功能

set -e

# 引入通用函数
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

cd_root

# 测试项目名称
TEST_PROJECT="test-scaffold-project"

# 解析参数
CLEANUP=true
LOG_LEVEL="info"
BUILD_FIRST=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --no-cleanup)
            CLEANUP=false
            shift
            ;;
        -l|--log-level)
            LOG_LEVEL="$2"
            shift 2
            ;;
        -b|--build)
            BUILD_FIRST=true
            shift
            ;;
        -h|--help)
            echo "用法: $0 [选项]"
            echo ""
            echo "选项:"
            echo "  --no-cleanup            测试后不删除测试项目"
            echo "  -l, --log-level <level> 设置日志级别"
            echo "  -b, --build             测试前先构建"
            echo "  -h, --help              显示帮助信息"
            exit 0
            ;;
        *)
            error "未知参数: $1"
            exit 1
            ;;
    esac
done

# 清理旧的测试项目
cleanup() {
    if [ "$CLEANUP" = true ] && dir_exists "$TEST_PROJECT"; then
        info "清理测试项目..."
        rm -rf "$TEST_PROJECT"
        success "清理完成"
    fi
}

# 设置退出时清理
trap cleanup EXIT

# 确保二进制文件存在
if [ "$BUILD_FIRST" = true ]; then
    info "构建项目..."
    cargo build
    success "构建完成"
fi

info "===== 开始测试 Scaffold ====="
echo ""

# 测试 1: 显示帮助
info "测试 1: 显示帮助信息"
cargo run -- --help | grep -q "log-level" && success "✓ 帮助信息包含 log-level 选项" || error "✗ 帮助信息缺少 log-level 选项"
echo ""

# 测试 2: update 命令
info "测试 2: 运行 update 命令"
cargo run -- -l "$LOG_LEVEL" update
echo ""

# 测试 3: new 命令帮助
info "测试 3: 显示 new 命令帮助"
cargo run -- new --help | grep -q "template" && success "✓ new 命令包含 template 选项" || error "✗ new 命令缺少 template 选项"
echo ""

# 测试 4: 创建新项目
info "测试 4: 创建新项目"
if dir_exists "$TEST_PROJECT"; then
    warn "测试项目已存在，先删除..."
    rm -rf "$TEST_PROJECT"
fi

# 这里需要使用实际的模板名称
TEMPLATE_NAME="nextjs模版"
cargo run -- -l "$LOG_LEVEL" new -t "$TEMPLATE_NAME" -n "$TEST_PROJECT"

if dir_exists "$TEST_PROJECT"; then
    success "✓ 项目创建成功: $TEST_PROJECT"

    # 检查项目内容
    if dir_exists "$TEST_PROJECT/.git"; then
        success "✓ Git 仓库已初始化"
    else
        error "✗ Git 仓库未初始化"
    fi
else
    error "✗ 项目创建失败"
fi
echo ""

# 测试 5: 日志级别测试
info "测试 5: 测试不同日志级别过滤功能"

# 删除之前的测试项目，为日志测试做准备
if dir_exists "$TEST_PROJECT"; then
    rm -rf "$TEST_PROJECT"
fi

# 辅助函数：检查日志级别输出
test_log_level() {
    local level=$1
    local output=$(cargo run --quiet -- -l "$level" new -t "$TEMPLATE_NAME" -n "$TEST_PROJECT" 2>&1)

    # 统计各级别日志出现次数
    local trace_count=$(echo "$output" | grep -c "TRACE:" || true)
    local debug_count=$(echo "$output" | grep -c "DEBUG:" || true)
    local info_count=$(echo "$output" | grep -c "开始创建新项目" || true)
    local warn_count=$(echo "$output" | grep -c "WARN:" || true)
    local error_count=$(echo "$output" | grep -c "ERROR:" || true)

    case $level in
        trace)
            if [ $trace_count -gt 0 ] && [ $debug_count -gt 0 ] && [ $info_count -gt 0 ] && [ $warn_count -gt 0 ] && [ $error_count -gt 0 ]; then
                success "✓ TRACE 级别: 显示所有日志 (5/5)"
                return 0
            else
                error "✗ TRACE 级别: 应该显示所有日志"
                echo "  TRACE:$trace_count DEBUG:$debug_count INFO:$info_count WARN:$warn_count ERROR:$error_count"
                return 1
            fi
            ;;
        debug)
            if [ $trace_count -eq 0 ] && [ $debug_count -gt 0 ] && [ $info_count -gt 0 ] && [ $warn_count -gt 0 ] && [ $error_count -gt 0 ]; then
                success "✓ DEBUG 级别: 显示 DEBUG+ 日志 (4/4)"
                return 0
            else
                error "✗ DEBUG 级别: 应该显示 DEBUG 及以上日志"
                echo "  TRACE:$trace_count DEBUG:$debug_count INFO:$info_count WARN:$warn_count ERROR:$error_count"
                return 1
            fi
            ;;
        info)
            if [ $trace_count -eq 0 ] && [ $debug_count -eq 0 ] && [ $info_count -gt 0 ] && [ $warn_count -gt 0 ] && [ $error_count -gt 0 ]; then
                success "✓ INFO 级别: 显示 INFO+ 日志 (3/3)"
                return 0
            else
                error "✗ INFO 级别: 应该显示 INFO 及以上日志"
                echo "  TRACE:$trace_count DEBUG:$debug_count INFO:$info_count WARN:$warn_count ERROR:$error_count"
                return 1
            fi
            ;;
        warn)
            if [ $trace_count -eq 0 ] && [ $debug_count -eq 0 ] && [ $info_count -eq 0 ] && [ $warn_count -gt 0 ] && [ $error_count -gt 0 ]; then
                success "✓ WARN 级别: 显示 WARN+ 日志 (2/2)"
                return 0
            else
                error "✗ WARN 级别: 应该显示 WARN 及以上日志"
                echo "  TRACE:$trace_count DEBUG:$debug_count INFO:$info_count WARN:$warn_count ERROR:$error_count"
                return 1
            fi
            ;;
        error)
            if [ $trace_count -eq 0 ] && [ $debug_count -eq 0 ] && [ $info_count -eq 0 ] && [ $warn_count -eq 0 ] && [ $error_count -gt 0 ]; then
                success "✓ ERROR 级别: 仅显示 ERROR 日志 (1/1)"
                return 0
            else
                error "✗ ERROR 级别: 应该仅显示 ERROR 日志"
                echo "  TRACE:$trace_count DEBUG:$debug_count INFO:$info_count WARN:$warn_count ERROR:$error_count"
                return 1
            fi
            ;;
    esac
}

# 只测试当前设置的日志级别
test_log_level "$LOG_LEVEL"
echo ""

info "===== 测试完成 ====="

if [ "$CLEANUP" = false ]; then
    info "测试项目保留在: $(pwd)/$TEST_PROJECT"
    info "手动清理: rm -rf $TEST_PROJECT"
fi
