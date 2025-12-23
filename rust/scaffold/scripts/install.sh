#!/bin/bash

# Scaffold 安装脚本
# 一键安装 scaffold 到系统

set -e

# 引入通用函数
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

cd_root

INSTALL_DIR="$HOME/.cargo/bin"
INSTALLED_PATH="$INSTALL_DIR/scaffold"

print_title "开始安装 Scaffold"

# 检查安装目录
if ! dir_exists "$INSTALL_DIR"; then
    info "创建安装目录: $INSTALL_DIR"
    mkdir -p "$INSTALL_DIR"
fi

# 构建 release 版本
info "正在构建 release 版本..."
cargo build --release --quiet

# 复制到安装目录
info "安装到 $INSTALL_DIR"
cp target/release/scaffold "$INSTALLED_PATH"
chmod +x "$INSTALLED_PATH"

success "安装完成!"
echo ""

# 检查 PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    warn "检测到 $INSTALL_DIR 不在 PATH 中"
    echo ""
    echo "请运行以下命令添加到 PATH："
    echo ""
    echo "  echo 'export PATH=\"\$HOME/.cargo/bin:\$PATH\"' >> ~/.zshrc"
    echo "  source ~/.zshrc"
    echo ""
else
    success "现在可以直接使用: scaffold --help"
fi
