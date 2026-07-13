# 第 2 章：RSX 语法全解

> Dioxus 客户端课程第 2 章
> 学习目标：掌握 rsx! 宏的完整语法（类 JSX 但有 Rust 特色）

## 📂 示例代码
```bash
cd ch02-rsx/rsx_syntax
dx serve --platform web       # 浏览器访问 http://127.0.0.1:8080
```

---

## RSX 是什么？

`rsx!` 是 Dioxus 的 UI 声明宏，语法类似 JSX，但编译成 Rust 代码。

```rust
rsx! {
    div { class: "container",
        h1 { "Hello" }
        p { "世界" }
    }
}
```

### RSX vs JSX 对比

| JSX | RSX | 说明 |
|-----|-----|------|
| `<div>` | `div { ... }` | 标签 |
| `className="x"` | `class: "x"` | 属性 |
| `{expr}` | `{expr}` | 插入表达式 |
| `style={{color: 'red'}}` | `style: "color: red;"` | 样式 |
| `{cond && <X/>}` | `if cond { X {} }` | 条件渲染 |
| `{list.map(...)}` | `for x in list { ... }` | 列表渲染 |

---

## 2.1 基本结构

```rust
rsx! {
    div {                    // 标签
        h1 { "标题" }        // 子元素 + 文本
        p { "段落" }
    }
}
```

- 标签不用尖括号 `<div>`，用 `div { ... }`
- 文本直接写字符串
- 嵌套用 `{ }` 包裹子元素

---

## 2.2 属性

属性用 `属性名: 值` 的语法：

```rust
rsx! {
    div {
        class: "container",              // class（相当于 className）
        id: "my-div",                    // id
        style: "color: red;",            // style
        onclick: move |_| { ... },       // 事件（下一章详讲）
    }
}
```

> ⚠️ 注意：属性值末尾用逗号 `,` 分隔（Rust 语法），不是 JSX 的空格。

---

## 2.3 插入 Rust 表达式 {expr} ⭐

文本里的 `{变量}` 会插入变量值：

```rust
let name = "Alice";
rsx! {
    p { "你好，{name}" }              // 你好，Alice
    p { "长度：{name.len()}" }        // 长度：5
    p { "计算：{1 + 2}" }             // 计算：3
}
```

### ⚠️ 重要规则：复杂表达式提取到变量

RSX 的字符串插值 `{...}` 不能嵌套 `{}`。复杂表达式要**先提取到变量**：

```rust
// ❌ 错误：嵌套大括号有歧义
rsx! { p { "{name.to_uppercase()}" } }   // 编译错误！

// ✅ 正确：先提取到变量
let upper = name.to_uppercase();
rsx! { p { "大写：{upper}" } }
```

> 💡 这和 JSX 不同——JSX 的 `{expr}` 可以是任意表达式，RSX 的字符串插值更严格。

---

## 2.4 条件渲染

### 方式 1：if 表达式（返回 rsx!）

```rust
{if is_logged_in {
    rsx! { p { "已登录" } }
} else {
    rsx! { p { "未登录" } }
}}
```

### 方式 2：if 语句（推荐，更简洁）

```rust
if is_logged_in {
    p { "欢迎回来！" }
}
// 不满足就什么都不渲染
```

> 💡 类比 React 的 `{cond && <Component/>}`。

---

## 2.5 列表渲染（for 循环）⭐

RSX 用 Rust 的 `for` 循环遍历：

```rust
let items = vec!["苹果", "香蕉", "橙子"];

rsx! {
    ul {
        for item in &items {
            li { "{item}" }
        }
    }
}
```

带索引：

```rust
rsx! {
    ol {
        for (i, item) in items.iter().enumerate() {
            li { "[{i}] {item}" }
        }
    }
}
```

> 💡 类比 React 的 `{items.map((item, i) => <li key={i}>{item}</li>)}`。
> RSX 不需要 `key`（Dioxus 自动处理）。

---

## 2.6 动态属性

属性值也可以用变量插入：

```rust
let is_active = true;
let box_class = if is_active { "box active" } else { "box" };

rsx! {
    div {
        class: "{box_class}",       // 动态 class
    }
}
```

---

## 2.7 事件处理（预览，第 5 章详讲）

```rust
rsx! {
    button {
        onclick: move |_| println!("点击了！"),
        "点我"
    }
}
```

---

## 2.8 Fragment（多个根元素）

RSX 可以直接写多个根元素，不需要 React 的 `<> </>`：

```rust
rsx! {
    h1 { "标题" }
    p { "段落" }
    button { "按钮" }
}
```

---

## 2.9 特殊字符

要显示特殊字符（如 `<div>` 字符串），用 `{}` 包裹 Rust 表达式：

```rust
rsx! {
    {rsx! { p { {"<div> 这是字符串"} } }}
    {rsx! { p { {"引号 \" 也能显示"} } }}
}
```

---

## ⭐ RSX 的关键规则（踩坑总结）

### 规则 1：变量声明必须在 rsx! 之外

```rust
fn App() -> Element {
    // ✅ 所有变量先在这里声明
    let name = "Alice";
    let items = vec![1, 2, 3];

    rsx! {
        // ❌ 不能在 rsx! 里写 let
        // let x = 5;
        p { "{name}" }
    }
}
```

### 规则 2：字符串插值不能嵌套大括号

```rust
// ❌ 错误
"{name.to_uppercase()}"

// ✅ 先提取到变量
let upper = name.to_uppercase();
"{upper}"
```

### 规则 3：属性之间用逗号分隔

```rust
// ✅ 逗号分隔
div {
    class: "a",
    id: "b",
}

// ❌ 不要用分号或空格
```

---

## 📋 RSX vs JSX 速查

| 功能 | JSX | RSX |
|------|-----|-----|
| 标签 | `<div>` | `div { }` |
| 属性 | `class="x"` | `class: "x"` |
| 插入变量 | `{name}` | `{name}` |
| 条件 | `{cond && <X/>}` | `if cond { X {} }` |
| 列表 | `{arr.map(x => ...)}` | `for x in arr { ... }` |
| 事件 | `onClick={fn}` | `onclick: fn` |
| Fragment | `<>...</>` | 直接写多个 |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：为什么 RSX 的 `{name.method()}` 会报错？

**A：** RSX 的字符串插值 `{...}` 不能嵌套大括号（编译器无法解析）。

```rust
// ❌ 错误：{name.to_uppercase()} 嵌套了大括号
rsx! { p { "{name.to_uppercase()}" } }

// ✅ 正确：先提取到变量
let upper = name.to_uppercase();
rsx! { p { "{upper}" } }
```

这是 RSX 和 JSX 的主要区别——JSX 的 `{expr}` 可以是任意表达式，RSX 的字符串插值更严格。

### Q2：为什么 `let` 不能写在 rsx! 里？

**A：** `rsx!` 是宏，展开后生成的是表达式（返回 Element），不是语句块。所有变量声明必须在宏外部。

---

## ✅ 第 2 章 小结

学完本章你应该掌握：
1. ✅ RSX 的基本语法（标签、属性、文本）
2. ✅ 插入 Rust 表达式 `{expr}`
3. ✅ 条件渲染（if）
4. ✅ 列表渲染（for 循环）
5. ✅ 动态属性
6. ✅ 知道 RSX 和 JSX 的关键差异（不能嵌套大括号、变量声明位置）

---

## 📂 本章练习目录

- `rsx_syntax/src/main.rs` —— RSX 语法全演示
- `rsx_syntax/assets/main.css` —— 样式
