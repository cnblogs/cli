# Cnblogs 命令行工具

[![Build / Release](https://github.com/cnblogs/cli/actions/workflows/build-release.yml/badge.svg)](https://github.com/cnblogs/cli/actions/workflows/build-release.yml)
[![Build / Development](https://github.com/cnblogs/cli/actions/workflows/build-dev.yml/badge.svg)](https://github.com/cnblogs/cli/actions/workflows/build-dev.yml)

直接从命令行访问和管理您的博客园内容。

## 功能特性

- 📝 内容管理: 创建、查看和互动博客文章、闪存和评论
- 🔐 安全认证: 基于个人访问令牌 (PAT) 的身份验证
- ⚡ 快速轻量: 使用 Rust 构建，性能优异
- 🔧 跨平台: 支持 Windows、macOS 和 Linux
- 📚 直观界面: 熟悉的 CLI 模式和全面的帮助系统

## 安装方法

### 下载预编译二进制文件（推荐）

从[发布页面](https://github.com/cnblogs/cli/releases)下载适用于您平台的最新版本。

#### 快速安装（macOS/Linux）

```sh
# 下载并安装 cnb
curl -fSL -O https://github.com/cnblogs/cli/releases/download/v$version/cnb-$version-$arch-$os.zip
unzip -d . cnb-$version-$arch-$os.zip
mv ./cnb ~/.local/bin/
```

Windows PowerShell

```powershell
# 下载并解压
Invoke-WebRequest -Uri "https://github.com/cnblogs/cli/releases/latest/download/cnb-x86_64-pc-windows-msvc.zip" -OutFile "cnb.zip"
Expand-Archive -Path "cnb.zip" -DestinationPath "."
# 将 cnb.exe 添加到 PATH 环境变量
```

### 从源码构建

`v0.2.1`之前的版本需要nightly版本，`channel`推荐`2026-01-10`以上。最新版本已切换至`stable`版本（1.95+）。

Cargo安装

```sh
# from repo main
cargo install --git https://github.com/cnblogs/cli.git

# Or local install
# Clone repository
git clone --depth 1 https://github.com/cnblogs/cli.git

cargo install --path ./cli --bin cnb
```

源码编译

```sh
# 克隆仓库
git clone --depth 1 https://github.com/cnblogs/cli.git
cd cli

# 构建发布版本
cargo build --release --bin cnb

# 二进制文件位于 ./target/release/cnb（或 Windows 上的 cnb.exe）
```

## 快速开始

### 1. 获取个人访问令牌

1. 访问[https://account.cnblogs.com/settings/tokens](https://account.cnblogs.com/settings/tokens)
2. 点击"生成新令牌"
3. 复制生成的令牌（以后将无法再次查看）

### 2. 登录

```bash
# 登录（推荐）
cnb user login YOUR_PAT_TOKEN

# 验证登录状态
cnb user status
```

您的令牌安全地存储在 `~/.cnblogs/token`（Windows：`%USERPROFILE%\.cnblogs\token`）。

## 命令参考

### 命令格式

```bash
cnb <命令> <子命令> [选项] [参数]
```

### 命令参考

| 命令     | 描述   | 可用子命令                                       |
|--------|------|---------------------------------------------|
| `user` | 用户模块 | `login`, `logout`, `status`                 |
| `ing`  | 闪存管理 | `create`, `delete`, `list`, `show`, `reply` |
| `post` | 博客文章 | `list`, `show`, `reply`                     |
| `news` | 新闻   | `list`                                      |
| `fav`  | 书签   | `list`                                      |

### 使用示例

以下级几个简单的示例：

```sh
# 查看你的随笔
cnb post list
# 查看随笔内容
cnb post show 114514

# 闪存相关操作
cnb ing list
cnb ing list my --page-index 1 --page-size 10

# 发布闪存
cnb ing create 'Hello world!'
cnb ing create 'Hello world!' --tag lucky

# 发布评论
cnb  ing replay 'Awesome!' --id 114514
```

更多使用信息请通过`cnb --help`或者`cnb help`查询

## 项目结构

```text
cli/
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── README.zh-CN.md
├── rust-fmt.toml
├── rust-toolchain.toml
├── shell.nix
└── src
    ├── api                       # 博客园API交互模块
    │   ├── fav.rs                # 书签API
    │   ├── ing.rs                # 闪存API
    │   ├── mod.rs                # Module exports for API layer
    │   ├── news.rs               # 新闻API
    │   ├── post.rs               # 随笔API
    │   ├── urls.rs               # 生成API的url
    │   └── user.rs               # 用户API
    ├── bin                       # 二进制目录
    │   └── cnb.rs                # cli可执行文件
    ├── commands                  # CLI 命令实现
    │   ├── fav.rs
    │   ├── ing.rs
    │   ├── mod.rs
    │   ├── news.rs
    │   ├── post.rs
    │   └── user.rs
    ├── context                   # 上下文管理
    │   ├── config.rs             # 配置文件
    │   ├── mod.rs                # 模块管理，Context实现
    │   └── output.rs             # 输出管理
    ├── display                   # 显示相关
    │   ├── ing.rs                # 闪存相关`trait`的定义和实现
    │   └── mod.rs
    ├── lib.rs                    # lib
    ├── logic                     # 实现逻辑
    │   ├── fav.rs
    │   ├── ing.rs
    │   ├── mod.rs
    │   ├── news.rs
    │   ├── post.rs
    │   └── user.rs
    ├── models                    # 模型定义和格式化输出，
    │   ├── fav.rs
    │   ├── ing.rs
    │   ├── mod.rs
    │   ├── news.rs
    │   ├── post.rs
    │   └── user.rs
    └── tools                     # 工具模块，定义一些拓展和函数
        ├── http.rs               # reqwest的拓展
        ├── mod.rs
        ├── strings.rs            # String的拓展
        └── timer.rs              # chrono的拓展
```

## 许可证

[MIT](https://raw.githubusercontent.com/cnblogs/cli/main/LICENSE)

## 反馈

反馈我们十分期待你对本项目的看法，欢迎随时留言交流！

[Issues](https://github.com/cnblogs/cli/issues)
