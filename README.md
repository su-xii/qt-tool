
## 什么是 QT Tool？

QT Tool 是一个用 Rust 编写的切图工具，主要用于处理从设计平台下载的图片资源。它能够：

- 自动解压 ZIP 格式的切图文件
- 根据配置规则将文件分发到指定目录
- 支持多种输出格式（如 Android 的 mdpi/hdpi/xhdpi、iOS 的 @1x/@2x/@3x、Web 的不同尺寸等）
- 提供 HTTP 服务接口，方便与其他工具集成
- 配合浏览器插件实现自动化工作流


## 核心功能

### 1. 命令行模式
直接通过命令行处理 ZIP 文件，适合集成到构建脚本中。

### 2. 服务器模式
启动 HTTP 服务，提供 REST API，支持远程调用和浏览器插件集成。

### 3. 配置驱动
通过 `config.toml` 文件定义解压规则，支持多种匹配模式：
- 目录映射模式：将 ZIP 中的目录映射到输出目录
- 同级通配模式：使用通配符匹配同级文件
- 多倍图模式：支持 @2x、@3x 等倍图匹配

### 4. 浏览器插件
为 CodeSign 平台提供浏览器扩展，自动下载切图并调用 QT Tool 处理。

## 使用场景

- **移动应用开发**：自动处理 Android/iOS 多尺寸图标和切图
- **Web 前端开发**：生成不同分辨率版本的图片资源
- **设计协作**：配合 CodeSign 平台实现设计到代码的自动化流程


## 快速体验

```bash
# 克隆项目
git clone https://gitee.com/su_xii/qt-tool.git
cd qt-tool

# 运行服务
cargo run -- --run

# 或使用命令行模式
qt ./input.zip ok.png -p ./test -o "[v1,v2,v3]"
```

开始使用 [快速开始](qt.sucii.cn/docs/guide/) 文档了解更多详情。
插件使用 [浏览器插件](plug/README.md)