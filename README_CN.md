# DLUT 自动登录 - Rust 版本

大连理工大学（DLUT）校园网自动登录工具的 Rust 实现。

## 功能特性

- 支持登录凭证的 DES/3DES 加密
- 自动检测网络状态
- 适用于嵌入式系统（如 OpenWrt）的轻量级 CLI
- 同步 HTTP 请求（最小化二进制体积）
- 自动重试机制，适合无人值守运行

## 构建

```bash
cargo build --release
# 可选：去除符号表以减小二进制文件大小
strip target/release/dlut_auto_login_rs
```

## 使用方法

### 基本用法（单次登录）：
```bash
./dlut_auto_login_rs -u <username> -p <password>
```

### 获取当前网络状态：
```bash
./dlut_auto_login_rs --info
```

### 强制登录（跳过在线状态检查）：
```bash
./dlut_auto_login_rs -u <username> -p <password> --force
```

### 守护进程模式（循环重试直到成功）：
```bash
./dlut_auto_login_rs -u <username> -p <password> --daemon
```

### 指定 IP 地址：
```bash
./dlut_auto_login_rs -u <username> -p <password> -i <ip_address>
```

### 获取帮助：
```bash
./dlut_auto_login_rs --help
```

## 命令行选项

- `-u, --username <USERNAME>`: 用户名
- `-p, --password <PASSWORD>`: 密码
- `-i, --ip <IP>`: IPV4 地址（可选，未提供时将自动检测）
- `--info`: 获取 drcom 信息并退出
- `--force`: 登录前跳过状态检查
- `--daemon`: 守护进程模式（持续重试直到成功）

## 项目结构

- `src/main.rs` - 入口点和 CLI 处理
- `src/des_crypto.rs` - DES 加密/解密实现
- `src/drcom.rs` - DrCOM 协议处理（网络状态检查）
- `src/login.rs` - 登录逻辑和表单处理

## 依赖项

- `ureq` - 极简同步 HTTP 客户端
- `des` - DES 加密
- `hex` - Hex 编码/解码
- `serde` - 序列化框架
- `serde_json` - JSON 解析
- `argh` - 极简命令行参数解析

## 注意事项

- 本工具针对嵌入式系统（如 OpenWrt）进行了优化
- 移除了交互模式，以便于作为守护进程/脚本使用
- 工具在尝试登录前会检查是否已经在线
- 如果登录失败，将无限重试直到成功（适用于启动脚本）

## 许可证

与原始 Python 项目相同
