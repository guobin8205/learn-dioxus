# 第 10 章：Markdown 编辑器（上）—— UI 搭建

> Dioxus 客户端课程第 10 章（实战项目第 1 部分）
> 学习目标：搭建 Markdown 编辑器的 UI 和实时预览

## 📂 示例代码
```bash
cd ch10-markdown-editor/markdown_editor
dx serve --platform web       # 浏览器访问 http://127.0.0.1:8080
```

---

## 10.1 项目概述

### Markdown 编辑器的功能
- 📝 左侧编辑 Markdown 文本
- 👁️ 右侧实时预览渲染结果
- 🌙 暗色/亮色主题切换
- 📊 字数统计

### 用到的技术（综合前面 9 章）
| 章节 | 技术点 | 应用 |
|------|--------|------|
| Ch2 | RSX 语法 | UI 结构 |
| Ch3 | 组件 | App 组件 |
| Ch4 | use_signal | 编辑器内容、主题状态 |
| Ch5 | oninput | 实时输入响应 |
| Ch8 | 样式 | 双栏布局、Markdown 样式 |
| Ch9 | inline style | 保证布局生效 |

---

## 10.2 ⭐ Markdown 解析：pulldown-cmark

pulldown-cmark 是 Rust 最成熟的 Markdown 解析库：

```toml
[dependencies]
pulldown-cmark = "0.12"
```

### 解析函数

```rust
use pulldown_cmark::{Parser, Options, html};

fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);         // 启用表格
    options.insert(Options::ENABLE_STRIKETHROUGH);  // 启用删除线
    options.insert(Options::ENABLE_FOOTNOTES);      // 启用脚注

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);      // 转成 HTML 字符串
    html_output
}
```

输入 `"# 标题"` → 输出 `"<h1>标题</h1>"`

---

## 10.3 ⭐ 实时预览（核心机制）

### 编辑器 + 预览的数据流

```
用户输入文字
    ↓ oninput 事件
content.set(e.value())       ← 更新 Signal
    ↓ 自动重新渲染
render_markdown(&content())  ← 重新解析 Markdown
    ↓ HTML 字符串
dangerous_inner_html         ← 插入到预览区
```

### 代码

```rust
let mut content = use_signal(|| "# Hello".to_string());
let markdown_html = render_markdown(&content());   // 派生值

rsx! {
    textarea {
        value: "{content}",
        oninput: move |e| content.set(e.value()),   // 输入时更新
    }

    div {
        // ⭐ dangerous_inner_html：把 HTML 字符串作为内容插入
        // （普通文本插值会转义 < > 等，这里需要原生 HTML）
        dangerous_inner_html: "{markdown_html}",
    }
}
```

### ⭐ `dangerous_inner_html` 的作用

```rust
// ❌ 普通文本：HTML 标签会被转义（显示为纯文本）
div { "{markdown_html}" }
// 显示：<h1>标题</h1>（标签可见）

// ✅ dangerous_inner_html：HTML 被渲染
div { dangerous_inner_html: "{markdown_html}" }
// 显示：大字标题（标签生效）
```

> ⚠️ 名字带 "dangerous" 是因为它有 XSS 风险——不要用于不可信的用户输入。这里 Markdown 解析器已经处理了安全性，所以可以用。

---

## 10.4 ⭐ 双栏布局

### 用 Flexbox 实现左右分栏

```rust
rsx! {
    div {
        style: "display: flex; height: calc(100vh - 53px);",  // ⭐ flex 容器

        // 左栏（编辑器）
        div {
            style: "flex: 1; border-right: 1px solid #ddd;",   // 各占一半
            textarea { ... }
        }

        // 右栏（预览）
        div {
            style: "flex: 1;",                                   // 各占一半
            div { dangerous_inner_html: "{html}" }
        }
    }
}
```

### ⭐ 经验：关键布局用 inline style

吸取第 8 章的教训——**Grid/Flex 布局用 inline style 最可靠**，外部 CSS 有时不加载。

---

## 10.5 主题切换

```rust
let mut dark_mode = use_signal(|| false);
let theme_class = if dark_mode() { "dark" } else { "light" };

rsx! {
    div {
        class: "app {theme_class}",
        // CSS 里 .app.dark 和 .app.light 有不同背景色
    }
    button {
        onclick: move |_| dark_mode.toggle(),
        if dark_mode() { "☀️" } else { "🌙" }
    }
}
```

---

## 10.6 Markdown 样式渲染

Markdown 转成 HTML 后，需要 CSS 让它好看。关键是**所有 Markdown 样式都要限定在 `.preview-content` 内**：

```css
/* ⭐ 只影响预览区，不影响编辑器 */
.preview-content h1 { font-size: 2em; border-bottom: 2px solid #e0e0e0; }
.preview-content code { background: rgba(175,47,47,0.1); color: #b7410e; }
.preview-content pre { background: #f6f8fa; padding: 16px; border-radius: 6px; }
.preview-content blockquote { border-left: 4px solid #b7410e; }
.preview-content table { border-collapse: collapse; width: 100%; }
```

---

## 📋 当前功能清单

| 功能 | 状态 | 章节 |
|------|------|------|
| 双栏布局（编辑+预览） | ✅ | Ch10 |
| 实时预览 | ✅ | Ch10 |
| Markdown 解析 | ✅ | Ch10 |
| 代码块、表格、引用 | ✅ | Ch10 |
| 暗色/亮色主题 | ✅ | Ch10 |
| 字数统计 | ✅ | Ch10 |
| 本地存储（刷新不丢） | ⏳ | Ch11 |
| 导入 .md 文件 | ⏳ | Ch11 |
| 导出 .md 文件 | ⏳ | Ch11 |
| 打包发布 | ⏳ | Ch12 |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：为什么用 `dangerous_inner_html` 而不是普通文本插值？

**A：** Markdown 解析后得到的是 HTML 字符串（如 `<h1>标题</h1>`），需要被浏览器当作 HTML 渲染。

- 普通文本插值 `{html}` → 标签被转义，显示为纯文本 `<h1>标题</h1>`
- `dangerous_inner_html: "{html}"` → 标签生效，显示为大标题

> ⚠️ 注意：`dangerous_inner_html` 有 XSS 风险，只用于可信内容（pulldown-cmark 已处理安全性）。

### Q2：为什么布局用 inline style 而不是外部 CSS？

**A：** 第 8 章遇到过——外部 CSS 在 Dioxus Web 模式偶尔不加载。关键布局（Flex/Grid）用 inline style 保证一定生效。非关键样式（颜色、字体）可以用外部 CSS。

---

## ✅ 第 10 章 小结

学完本章你应该掌握：
1. ✅ 用 pulldown-cmark 解析 Markdown
2. ✅ 用 `dangerous_inner_html` 渲染 HTML
3. ✅ 实现「编辑→预览」实时联动
4. ✅ 用 Flexbox 实现双栏布局
5. ✅ 实现主题切换

---

## 📂 本章练习目录

- `markdown_editor/src/main.rs` —— 编辑器主代码
- `markdown_editor/assets/main.css` —— 样式（含 Markdown 渲染样式）
