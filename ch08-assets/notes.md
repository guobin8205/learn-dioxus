# 第 8 章：样式与资源

> Dioxus 客户端课程第 8 章
> 学习目标：掌握 CSS 样式、动态样式、图片资源加载、manganis

## 📂 示例代码
```bash
cd ch08-assets/styling
dx serve --platform web       # 浏览器访问 http://127.0.0.1:8080
```

---

## 8.1 三种样式方式

### 1. 内联样式（style 属性）

```rust
rsx! {
    p {
        style: "color: red; font-size: 18px;",
        "红色文字"
    }
}
```

动态样式（插入变量）：
```rust
let size = 18;
rsx! {
    p {
        style: "font-size: {size}px;",
        "字号 {size}"
    }
}
```

### 2. CSS 类（外部样式表）

```rust
// assets/main.css
// .highlight { background: yellow; }

rsx! {
    p { class: "highlight", "高亮文字" }
}
```

在 `Dioxus.toml` 里引入 CSS：
```toml
[web.resource.dev]
style = ["assets/main.css"]
```

### 3. ⭐ manganis（编译期资源引用）

```rust
// 用 asset! 宏引用资源，编译期检查文件存在
static LOGO: manganis::Asset = manganis::asset!("/assets/logo.svg");

rsx! {
    img { src: LOGO, alt: "Logo" }
}
```

> `asset!` 的好处：编译期检查文件是否存在，避免运行时找不到资源。

---

## 8.2 ⭐ 动态样式（根据状态变化）

这是 Dioxus 最强大的样式功能——样式随状态自动更新：

### 动态 class

```rust
let mut dark_mode = use_signal(|| false);
let theme_class = if dark_mode() { "dark" } else { "light" };

rsx! {
    div { class: "app {theme_class}",
        // theme_class 变化时，class 自动更新
    }
}
```

### 动态 style

```rust
let bg = if dark_mode() { "#1a1a2e" } else { "#fafafa" };

rsx! {
    div {
        style: "background: {bg};",
        "背景随主题变化"
    }
}
```

### 条件 class

```rust
let active = font_size() > 18;
let box_class = if active { "box active" } else { "box" };

rsx! {
    div { class: "{box_class}", "条件 class" }
}
```

> ⚠️ 注意：所有变量必须在 `rsx!` 外部声明（第 2 章的规则）。

---

## 8.3 图片与资源

### 方式 1：manganis asset!（推荐）

```rust
static LOGO: manganis::Asset = manganis::asset!("/assets/icons/logo.svg");

rsx! {
    img { src: LOGO, alt: "Logo" }
}
```

**好处**：
- 编译期检查文件存在
- 自动处理路径（Web/桌面不同）
- 支持图片优化（压缩、格式转换）

### 方式 2：直接路径字符串

```rust
rsx! {
    img { src: "/assets/icons/logo.svg", alt: "Logo" }
}
```

简单但不会编译期检查。

### Cargo.toml 依赖

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
manganis = "0.7"    # ⭐ 资源处理
```

---

## 8.4 CSS 组织方式

### 项目结构

```
styling/
├── assets/
│   ├── main.css           ← 全局样式
│   ├── icons/
│   │   └── logo.svg       ← 图片资源
│   └── fonts/             ← 字体（可选）
└── src/
    └── main.rs
```

### 在 Dioxus.toml 引入 CSS

```toml
[web.resource.dev]
style = ["assets/main.css"]    # 可以引入多个
# style = ["assets/main.css", "assets/theme.css"]
```

---

## 8.5 响应式布局

Dioxus 支持标准 CSS，包括媒体查询：

```css
/* Grid 布局 */
.grid-3 { display: grid; grid-template-columns: repeat(3, 1fr); gap: 10px; }

/* 窄屏改成一列 */
@media (max-width: 500px) {
    .grid-3 { grid-template-columns: 1fr; }
}
```

---

## 8.6 ⭐ Tailwind CSS 集成（可选）

Dioxus 支持用 Tailwind 写样式（原子化 CSS）：

### 配置步骤

1. 安装 Tailwind CLI：
```bash
npm install -D tailwindcss
npx tailwindcss init
```

2. 创建 `tailwind.css`：
```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

3. 在 Rust 代码里用 manganis 引用：
```rust
static TAILWIND: manganis::Asset = manganis::asset!("/tailwind.css");
```

4. 使用 Tailwind 类名：
```rust
rsx! {
    div { class: "flex items-center justify-center p-4 bg-blue-500 text-white rounded",
        "Tailwind 样式"
    }
}
```

> 详细步骤参考 [Dioxus 0.7 Tailwind 指南](https://dioxuslabs.com/learn/0.7/guides/utilities/tailwind/)

---

## ⭐ 样式对比：Dioxus vs React

| 概念 | Dioxus | React |
|------|--------|-------|
| 内联样式 | `style: "color: red;"` | `style={{ color: 'red' }}` |
| class | `class: "name"` | `className="name"` |
| CSS 文件 | Dioxus.toml 配置 | import "./style.css" |
| 动态 class | `class: "{var}"` | `className={var}` |
| CSS Modules | 不常用 | 常用 |
| Tailwind | ✅ 支持 | ✅ 支持 |
| 资源引用 | `manganis::asset!` | import |

---

## 📋 样式速查

| 操作 | 写法 |
|------|------|
| 内联样式 | `style: "color: red;"` |
| 动态样式 | `style: "color: {color};"` |
| CSS 类 | `class: "name"` |
| 动态 class | `class: "{theme}"` |
| 引用图片 | `manganis::asset!("/path")` |
| 引入 CSS | `Dioxus.toml` 的 `[web.resource.dev]` |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

（暂无提问）

---

## ✅ 第 8 章 小结

学完本章你应该掌握：
1. ✅ 三种样式方式（内联、CSS 类、manganis）
2. ✅ 动态样式（根据状态变化 class/style）
3. ✅ 用 manganis 引用图片资源
4. ✅ 响应式布局（CSS Grid + 媒体查询）
5. ✅ 了解 Tailwind 集成方式

---

## 📂 本章练习目录

- `styling/src/main.rs` —— 6 种样式演示
- `styling/assets/main.css` —— 样式表
- `styling/assets/icons/rust-logo.svg` —— 图片资源
