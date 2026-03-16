@echo off
REM 设置编码为 UTF-8 以防配置文件或输出包含中文
chcp 65001 >nul

REM 执行启动命令
qt --run -c config.toml

REM 如果程序崩溃或立即退出，暂停一下以便查看错误信息（可选）
if %errorlevel% neq 0 (
    echo.
    echo 程序退出，错误代码: %errorlevel%
    pause
)