# 第 11 章：Markdown 编辑器（中）—— 功能完善

> Dioxus 客户端课程第 11 章（实战项目第 2 部分）
> 学习目标：给编辑器加本地存储、文件导入/导出

## 📂 示例代码
```bash
cd ch11-markdown-editor-2/markdown_editor
dx serve --platform web       # 浏览器访问 http://127.0.0.1:8080
```

---

## 11.1 本章新增功能

| 功能 | 说明 |
|------|------|
| 💾 自动保存 | 输入时自动存到 localStorage（刷新不丢） |
| 💾 手动保存 | 点击保存按钮 |
| 📤 导出 | 下载为 `.md` 文件 |
| 📊 统计 | 字数 + 行数显示 |

---

## 11.2 ⭐ 本地存储（localStorage）

### Web 模式：用 JavaScript 的 localStorage

Dioxus Web 模式下可以用 `dioxus::document::eval()` 执行 JavaScript：

```rust
fn save_to_storage(content: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        let js = format!(
            "localStorage.setItem('key', {})",
            serde_json::to_string(content).unwrap()   // 转成 JS 字符串字面量
        );
        let _ = dioxus::document::eval(&js);
    }
}
```

### 加载时恢复：use_effect

```rust
use_effect(move || {
    #[cfg(target_arch = "wasm32")]
    {
        let js = "localStorage.getItem('key')";
        if let Ok(result) = dioxus::document::eval(js).join() {
            if let Some(saved) = result.as_string() {
                content.set(saved);    // 恢复内容
            }
        }
    }
});
```

> `use_effect` 在组件渲染后执行一次——适合做初始化（加载存储）。

### 桌面模式：用文件系统

```rust
#[cfg(not(target_arch = "wasm32"))]
fn save_to_storage(content: &str) {
    std::fs::write("backup.md", content);
}
```

这就是第 9 章学的平台适配——同一接口，不同实现。

---

## 11.3 ⭐ 自动保存

在 `oninput` 里自动保存：

```rust
textarea {
    oninput: move |e| {
        content.set(e.value());
        save_to_storage(&e.value());   // ⭐ 每次输入都保存
    },
}
```

> 简单粗暴：每次按键都存。实际项目可以加防抖（debounce）优化。

---

## 11.4 ⭐ 导出文件（下载 .md）

### Web 模式：用 JavaScript 创建下载

```rust
onclick: move |_| {
    let js = format!(
        r#"var b=new Blob([{content}],{{type:'text/markdown'}});
        var u=URL.createObjectURL(b);
        var a=document.createElement('a');
        a.href=u; a.download='document.md'; a.click();
        URL.revokeObjectURL(u);"#,
        serde_json::to_string(&content()).unwrap()
    );
    let _ = dioxus::document::eval(&js);
}
```

### 桌面模式：直接写文件

```rust
#[cfg(not(target_arch = "wasm32"))]
{
    std::fs::write("export.md", &content());
}
```

---

## 11.5 ⭐ dioxus::document::eval 的用法

这是 Dioxus Web 模式调用 JavaScript 的方式：

```rust
// 执行 JS（无返回值）
dioxus::document::eval("alert('Hello!')");

// 执行 JS 并获取返回值
let result = dioxus::document::eval("1 + 1").join();
if let Ok(val) = result {
    println!("{}", val.as_number());   // Some(2.0)
}

// 读取字符串
let result = dioxus::document::eval("localStorage.getItem('key')").join();
if let Ok(val) = result {
    if let Some(s) = val.as_string() {
        println!("{}", s);
    }
}
```

### 用途
- 访问浏览器 API（localStorage、fetch、Clipboard）
- 操作 DOM（当 Dioxus API 不够用时）
- 执行 JS 库的功能

> ⚠️ 桌面模式没有 `eval`——用 `#[cfg]` 隔离平台特定代码。

---

## 11.6 统计信息

```rust
let char_count = content().chars().count();    // 字数
let line_count = content().lines().count();    // 行数

rsx! {
    span { "字数：{char_count} · 行：{line_count}" }
}
```

派生值——每次内容变化自动更新。

---

## 📋 当前功能清单

| 功能 | 状态 |
|------|------|
| 双栏布局（编辑+预览） | ✅ |
| 实时预览 | ✅ |
| Markdown 解析 | ✅ |
| 暗色/亮色主题 | ✅ |
| 字数统计 | ✅ |
| **自动保存（localStorage）** | ✅ 新增 |
| **手动保存** | ✅ 新增 |
| **导出 .md 文件** | ✅ 新增 |
| **行数统计** | ✅ 新增 |
| 打包发布 | ⏳ Ch12 |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：dioxus::document::eval 是什么？

**A：** 这是 Dioxus 在 Web 模式下执行 JavaScript 的接口。当你需要访问浏览器 API（localStorage、fetch、DOM 操作）时，用它执行 JS 代码。

```rust
// 执行 JS
dioxus::document::eval("alert('hello')");

// 执行并获取返回值
let result = dioxus::document::eval("1+1").join();
```

> ⚠️ 只在 Web 模式可用，桌面模式用 `#[cfg]` 隔离。

### Q2：为什么用 serde_json::to_string 包裹内容？

**A：** 把 Rust 字符串转成 JavaScript 的字符串字面量（带引号和转义）。如果直接插入 content，特殊字符（引号、换行）会破坏 JS 语法。

```rust
// ❌ 危险：内容里的引号会破坏 JS
format!("localStorage.setItem('k', '{}')", content)   // content 含 ' 会出错

// ✅ 安全：serde_json 转义特殊字符
format!("localStorage.setItem('k', {})", serde_json::to_string(&content).unwrap())
```

---

## ✅ 第 11 章 小结

学完本章你应该掌握：
1. ✅ 用 localStorage 持久化数据（Web 模式）
2. ✅ 用 use_effect 在启动时恢复数据
3. ✅ 用 dioxus::document::eval 调用 JavaScript
4. ✅ 实现文件导出（下载 .md）
5. ✅ 自动保存机制

---

## 📂 本章练习目录

- `markdown_editor/src/main.rs` —— 增强版编辑器
- `markdown_editor/assets/main.css` —— 样式（同第 10 章）
