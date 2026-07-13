# 第 9 章：平台适配

> Dioxus 客户端课程第 9 章
> 学习目标：掌握一套代码适配多平台（Web/桌面）的技巧

## 📂 示例代码
```bash
cd ch09-platform/platform_demo

# Web 模式（编译成 WASM）
dx serve --platform web

# 桌面模式（编译成原生应用）
dx serve --platform desktop
```

> ⭐ 同一份代码，切换 `--platform` 就能跑不同平台！

---

## 9.1 ⭐ 编译期平台检测：`#[cfg(target_arch = "...")]`

Rust 的条件编译，根据**编译目标**选择不同代码：

```rust
// wasm32 = Web 模式（编译成 WebAssembly）
#[cfg(target_arch = "wasm32")]
fn do_something() { /* Web 版逻辑 */ }

// 非 wasm32 = 桌面模式（原生二进制）
#[cfg(not(target_arch = "wasm32"))]
fn do_something() { /* 桌面版逻辑 */ }
```

### 工作原理

```
dx serve --platform web
  → cargo build --target wasm32-unknown-unknown
  → target_arch = "wasm32" 成立 → 编译 Web 代码

dx serve --platform desktop
  → cargo build（当前平台，如 x86_64）
  → target_arch = "wasm32" 不成立 → 编译桌面代码
```

### 实际用法

```rust
fn read_file() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        // Web：不能直接读文件（浏览器沙箱）
        "用 fetch API 或用户上传".to_string()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // 桌面：可以直接读文件
        std::fs::read_to_string("data.txt").unwrap_or_default()
    }
}
```

> ⭐ `#[cfg]` 是**编译期**的——不匹配的分支根本不会被编译，零运行时开销。

---

## 9.2 平台特定依赖

在 `Cargo.toml` 里为不同平台配置不同依赖：

```toml
# 桌面端独有依赖
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
reqwest = "0.12"          # HTTP 客户端（桌面用）
rusqlite = "0.31"         # SQLite（桌面用）

# Web 端独有依赖
[target.'cfg(target_arch = "wasm32")'.dependencies]
web-sys = "0.3"           # 浏览器 API
```

这样 Web 编译时不会下载桌面依赖，反之亦然。

---

## 9.3 ⭐ 条件渲染（不同平台不同 UI）

```rust
fn platform_ui() -> Element {
    #[cfg(target_arch = "wasm32")]
    {
        rsx! {
            // Web 版：显示分享按钮
            button { "分享到社交媒体" }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        rsx! {
            // 桌面版：显示文件操作
            button { "打开本地文件" }
        }
    }
}
```

> 同一个函数，根据平台编译出不同的 UI。

---

## 9.4 平台能力对比 ⭐

| 功能 | Web（WASM） | 桌面（原生） |
|------|------------|------------|
| 文件读写 | ❌ 受限（用上传/fetch） | ✅ 完全支持 |
| 网络请求 | ✅ fetch API | ✅ reqwest |
| 系统通知 | ⚠️ 需权限 | ✅ 原生通知 |
| 数据库 | ⚠️ IndexedDB | ✅ SQLite |
| 窗口控制 | ❌ 浏览器管理 | ✅ 完全控制 |
| 多线程 | ⚠️ Web Worker | ✅ 原生线程 |
| Rust 生态 | ⚠️ 部分（需支持 WASM） | ✅ 全部 |

### 关键差异

- **Web**：受浏览器沙箱限制，不能直接访问文件系统/数据库
- **桌面**：完整的 Rust 生态，可访问系统资源

---

## 9.5 平台适配最佳实践 ⭐

### 1. 业务逻辑写成平台无关

```rust
// ✅ 80% 的代码应该是平台无关的
fn calculate(data: &Data) -> Result {
    // 纯计算逻辑，不涉及平台 API
}
```

### 2. 平台特定代码用 cfg 隔离

```rust
// ✅ 只有涉及平台差异的部分用 cfg
fn save_data(data: &Data) {
    #[cfg(target_arch = "wasm32")]
    save_to_localstorage(data);     // Web 用 localStorage

    #[cfg(not(target_arch = "wasm32"))]
    save_to_file(data);             // 桌面用文件
}
```

### 3. 用 trait 抽象平台差异

```rust
trait Storage {
    fn save(&self, key: &str, value: &str);
    fn load(&self, key: &str) -> Option<String>;
}

// Web 实现
#[cfg(target_arch = "wasm32")]
struct WebStorage;
#[cfg(target_arch = "wasm32")]
impl Storage for WebStorage { /* localStorage */ }

// 桌面实现
#[cfg(not(target_arch = "wasm32"))]
struct FileStorage;
#[cfg(not(target_arch = "wasm32"))]
impl Storage for FileStorage { /* 文件 */ }
```

### 4. 桌面端可以访问完整 Rust 生态

桌面模式就是普通 Rust 程序，所有 crate 都能用：
- 数据库：`rusqlite`、`sqlx`
- 网络：`reqwest`、`tokio`
- 文件：`std::fs`、`notify`（文件监听）
- 系统：`sysinfo`、`open`（打开浏览器/文件）

---

## ⭐ 常见的 cfg 模式

```rust
// 只在 Web 编译
#[cfg(target_arch = "wasm32")]

// 只在桌面编译
#[cfg(not(target_arch = "wasm32"))]

// 只在 Windows 编译
#[cfg(target_os = "windows")]

// 只在 macOS 编译
#[cfg(target_os = "macos")]

// 只在 debug 模式
#[cfg(debug_assertions)]
```

---

## 📋 多端开发流程

```
开发阶段：
  1. 用 dx serve --platform web 快速开发（热重载快）
  2. 大部分代码平台无关
  3. 遇到平台差异，用 #[cfg] 隔离

测试阶段：
  1. dx serve --platform web 测试 Web 版
  2. dx serve --platform desktop 测试桌面版

发布阶段：
  1. dx bundle --platform web → 部署到服务器
  2. dx bundle --platform desktop → 生成 .exe/.dmg
```

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

（暂无提问）

---

## ✅ 第 9 章 小结

学完本章你应该掌握：
1. ✅ 用 `#[cfg(target_arch = "wasm32")]` 检测平台
2. ✅ 用条件编译写平台特定代码
3. ✅ 在 Cargo.toml 配置平台特定依赖
4. ✅ 理解 Web 和桌面的能力差异
5. ✅ 用 trait 抽象平台差异

---

## 📂 本章练习目录

- `platform_demo/src/main.rs` —— 平台检测、条件渲染、能力对比
- `platform_demo/assets/main.css` —— 样式
