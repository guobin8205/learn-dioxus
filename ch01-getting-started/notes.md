# 第 1 章：环境搭建与第一个应用

> Dioxus 跨平台客户端课程第 1 章
> 学习目标：搭建环境、创建第一个 Dioxus 应用、跑通 Web 和桌面

## 📂 示例代码
```bash
cd ch01-getting-started/my_first_app
dx serve --platform web       # Web 模式（浏览器访问 http://127.0.0.1:8080）
dx serve --platform desktop   # 桌面模式（弹出窗口）
```

---

## 1.1 环境准备

### 必装工具

```bash
# 1. Rust 工具链（已在 Rust 基础课程安装）
rustc --version   # 需要 1.96+

# 2. Dioxus CLI（dx）
cargo binstall dioxus-cli
dx --version      # 0.7.x

# 3. wasm32 目标（Web 模式需要）
rustup target add wasm32-unknown-unknown
```

> 💡 `dx serve` 首次运行 Web 模式时会自动安装 wasm32-unknown-unknown，无需手动安装。

---

## 1.2 项目结构（⭐ 核心）

```
my_first_app/
├── Cargo.toml          ← Rust 依赖配置（控制平台）
├── Dioxus.toml         ← Dioxus 专属配置（应用元数据）
├── assets/             ← 静态资源（CSS、图片）
│   └── main.css
└── src/
    └── main.rs         ← 代码入口
```

### `Cargo.toml` —— 通过 feature 切换平台

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
#                                        ↑
#                    改这里切换平台：
#                    "web"      → WebAssembly（浏览器）
#                    "desktop"  → 桌面应用（WebView）
#                    "mobile"   → 移动应用（实验性）
```

> 💡 实际开发中通常用 `dx serve --platform xxx` 切换，dx 会自动处理 feature。

### `Dioxus.toml` —— Dioxus 专属配置

```toml
[application]
name = "My First Dioxus App"
default_platform = "web"      # 默认平台
asset_dir = "assets"

[web.app]
title = "My First Dioxus App"  # 浏览器标签标题

[web.resource.dev]
style = ["assets/main.css"]    # 引入 CSS
```

---

## 1.3 第一个应用代码

```rust
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);      // ⭐ 入口：启动应用
}

fn App() -> Element {          // 组件：返回 Element
    rsx! {                     // ⭐ RSX：声明 UI
        h1 { "Hello, Dioxus!" }
        p { "这是我的第一个 Dioxus 应用 🦀" }
        button { "点我" }
    }
}
```

### 三个核心概念（类比 React）

| Dioxus | React | 说明 |
|--------|-------|------|
| `dioxus::launch(App)` | `createRoot(el).render(<App/>)` | 启动应用 |
| `fn App() -> Element` | `function App()` | 组件是一个函数 |
| `rsx! { h1 {...} }` | JSX `<h1>...</h1>` | 声明 UI 结构 |

> 你有 React 基础，这三个概念很容易对应：launch = render，组件 = 函数组件，rsx! = JSX。

---

## 1.4 运行应用 ⭐

### Web 模式（浏览器）

```bash
dx serve --platform web
# 访问 http://127.0.0.1:8080
```

首次运行会自动：
1. 安装 `wasm32-unknown-unknown`（WebAssembly 编译目标）
2. 安装 `wasm-bindgen-cli`（WASM 绑定工具）
3. 安装 `esbuild`（JS 打包工具）
4. 编译 Rust → WASM

> ⚠️ 首次编译较慢（下载 + WASM 编译），后续增量编译很快。

### 桌面模式（弹出窗口）

```bash
dx serve --platform desktop
```

会用系统 WebView（Windows 用 WebView2，macOS 用 WKWebView）渲染界面。

### 常用 dx 命令

| 命令 | 作用 |
|------|------|
| `dx serve --platform web` | Web 开发模式（热重载） |
| `dx serve --platform desktop` | 桌面开发模式 |
| `dx build --platform web --release` | Web 生产构建 |
| `dx bundle --platform desktop` | 桌面打包（生成 .exe/.dmg） |

### 开发时的热重载

`dx serve` 运行时，修改 `main.rs` 保存，浏览器会**自动刷新**。这就是热重载——和 React 的 HMR 类似。

---

## 1.5 多端架构（⭐ Dioxus 的核心价值）

```
                    你的 Rust 代码（main.rs）
                            │
            ┌───────────────┼───────────────┐
            ↓               ↓               ↓
        dx serve         dx serve        dx serve
        --platform web   --platform      --platform
                        desktop          mobile
            ↓               ↓               ↓
        浏览器           桌面窗口         手机 App
       (WASM)         (WebView)        (原生)
```

**一套 Rust 代码，三个平台运行**——这就是 Dioxus 的核心价值。你只需要改 `--platform` 参数。

---

## 📋 命令速查表

| 命令 | 作用 |
|------|------|
| `cargo binstall dioxus-cli` | 安装 dx CLI |
| `rustup target add wasm32-unknown-unknown` | 安装 Web 目标 |
| `dx serve --platform web` | Web 开发 |
| `dx serve --platform desktop` | 桌面开发 |
| `dx build --platform web --release` | Web 生产构建 |
| `dx bundle --platform desktop` | 桌面打包 |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：dx new 创建项目时报错 "not a terminal"？

**A：** `dx new` 在非 TTY 环境（如某些 IDE 终端、自动化脚本）会报这个错。解决：
1. 在真正的终端（Windows Terminal / PowerShell）运行 `dx new my-app`
2. 或者手动创建项目结构（就像本章做的那样——Cargo.toml + Dioxus.toml + src/main.rs）

---

### Q2：用 `cargo run` 运行报错 panic（exit code 101）？

**A：** Dioxus 项目**不能用 `cargo run` 运行**，必须用 `dx serve`。

#### 原因
```
cargo run          → 编译成 Windows 原生 .exe
features = ["web"] → 代码以为自己是 Web 渲染器
                   → 在原生 .exe 里启动 web 渲染器 → panic
```

`cargo run` 和 Cargo.toml 里硬编码的 `features = ["web"]` 冲突——一个说要原生二进制，一个说自己是 Web 应用。

#### 正确做法
```bash
# Web 模式
dx serve --platform web

# 桌面模式
dx serve --platform desktop
```

#### 为什么 cargo run 不行？
| | `cargo run` | `dx serve` |
|---|------------|-----------|
| 编译目标 | 当前平台原生二进制 | 根据 `--platform` 切换 |
| feature | 用 Cargo.toml 硬编码的 | 自动切换 web/desktop |
| WASM | ❌ | ✅ |
| 热重载 | ❌ | ✅ |

> 💡 记住：**Dioxus 项目永远用 `dx serve` 运行**。`dx` 是 Dioxus 专用的构建工具，它会正确处理平台切换、WASM 编译、资源打包。

---

## ✅ 第 1 章 小结

学完本章你应该掌握：
1. ✅ 安装 Dioxus CLI（dx）
2. ✅ 理解项目结构（Cargo.toml / Dioxus.toml / src / assets）
3. ✅ 写出最简单的 Dioxus 应用（launch + App + rsx!）
4. ✅ 用 `dx serve` 跑通 Web 模式
5. ✅ 理解「一套代码多端运行」的核心价值

---

## 📂 本章练习目录

- `my_first_app/` —— 第一个 Dioxus 应用
