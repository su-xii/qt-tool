#!/bin/bash

# 获取脚本所在目录，确保相对路径配置正确
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR" || exit

# 执行启动命令
qt --run -c config.toml

# 捕获退出状态码
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
    echo ""
    echo "程序退出，错误代码: $EXIT_CODE"
fi