# 第 3 章：组件与 Props

> Dioxus 客户端课程第 3 章
> 学习目标：掌握组件的定义、Props、children、组件复用

## 📂 示例代码
```bash
cd ch03-components/components
dx serve --platform web       # 浏览器访问 http://127.0.0.1:8080
```

---

## 3.1 组件的本质

Dioxus 组件 = **返回 Element 的函数** + `#[component]` 宏

```rust
#[component]
fn Header() -> Element {
    rsx! { h1 { "标题" } }
}
```

类比 React：

| Dioxus | React |
|--------|-------|
| `#[component] fn Header() -> Element` | `function Header() { return <h1>...</h1> }` |
| 在 RSX 里用 `Header {}` | `<Header />` |
| 组件名用大驼峰（PascalCase） | 同 |

### `#[component]` 宏的作用

`#[component]` 宏自动做了：
1. 生成 `HeaderProps` 结构体
2. 实现 `Properties` trait
3. 让你能在 RSX 里用 `Header {}` 调用

---

## 3.2 Props（属性）⭐

### 基本 Props

```rust
#[component]
fn Greeting(name: String, age: i32) -> Element {
    rsx! {
        h3 { "你好，{name}！" }
        p { "年龄：{age}" }
    }
}

// 使用（在父组件的 rsx! 里）
rsx! {
    Greeting { name: "Alice".to_string(), age: 25 }
    Greeting { name: "Bob".to_string(), age: 30 }
}
```

类比 React：
```jsx
// React
function Greeting({ name, age }) { ... }
<Greeting name="Alice" age={25} />
```

### 可选 Props：`Option<T>`

```rust
#[component]
fn UserCard(name: String, email: Option<String>) -> Element {
    rsx! {
        h3 { "👤 {name}" }
        {match email {
            Some(addr) => rsx! { p { "📧 {addr}" } },
            None => rsx! { p { "📧 无邮箱" } },
        }}
    }
}

// 使用：email 可传可不传
rsx! {
    UserCard { name: "张三".to_string(), email: Some("...".to_string()) }
    UserCard { name: "李四".to_string() }   // 不传 email
}
```

### 默认值：用 `Option<T>` + 内部处理 ⭐

Dioxus 0.7 的 `#[default]` 只支持字面量（数字、bool）。对于字符串等默认值，推荐用 `Option<T>` + `unwrap_or_else`：

```rust
#[component]
fn Badge(text: String, color: Option<String>) -> Element {
    let c = color.unwrap_or_else(|| "blue".to_string());   // 默认 "blue"
    rsx! { span { style: "background: {c};", "{text}" } }
}

// 使用
rsx! {
    Badge { text: "Rust".to_string() }                      // color 用默认值 blue
    Badge { text: "Dioxus".to_string(), color: Some("red".to_string()) }
}
```

---

## 3.3 children（子内容）⭐

类比 React 的 `props.children`——让组件能包裹子内容。

```rust
#[component]
fn Card(title: String, children: Element) -> Element {
    rsx! {
        div { class: "card",
            h3 { "{title}" }
            div { {children} }    // ⭐ 渲染子内容
        }
    }
}

// 使用
rsx! {
    Card { title: "关于我".to_string(),
        p { "我是一个 Rust 开发者" }
        button { "关注" }
    }
}
```

`children: Element` 是特殊参数——父组件放在 `Card { ... }` 里的内容会作为 `children` 传入。

---

## 3.4 组件复用

### 用循环生成多个组件

```rust
#[component]
fn ProductList() -> Element {
    let products = vec![
        Product { name: "书".to_string(), price: 99.0 },
        Product { name: "键盘".to_string(), price: 599.0 },
    ];

    rsx! {
        for p in products {
            ProductItem { name: p.name, price: p.price }
        }
    }
}

#[component]
fn ProductItem(name: String, price: f64) -> Element {
    rsx! {
        div {
            h4 { "{name}" }
            p { "¥{price}" }
        }
    }
}
```

> 💡 不需要 React 的 `key` 属性（Dioxus 自动处理）。

---

## 📋 Dioxus 组件 vs React 组件

| 概念 | Dioxus | React |
|------|--------|-------|
| 定义 | `#[component] fn Foo() -> Element` | `function Foo() { return <.../> }` |
| 使用 | `Foo { ... }` | `<Foo ... />` |
| Props | `fn Foo(name: String)` | `function Foo({ name })` |
| 可选 | `Option<String>` | `name?: string` |
| 默认值 | `Option` + `unwrap_or_else` | `name = "default"` |
| children | `children: Element` | `props.children` |
| 循环列表 | `for x in list { Foo {} }` | `{list.map(x => <Foo/>)}` |
| key | 不需要 | 需要 |

---

## ⭐ 常见踩坑

### 1. Props 值的类型必须匹配

```rust
#[component]
fn Greeting(name: String, age: i32) -> Element { ... }

// ✅ 类型匹配
Greeting { name: "Alice".to_string(), age: 25 }

// ❌ name 必须是 String，不能是 &str
Greeting { name: "Alice", age: 25 }
```

### 2. 默认值用 Option（不是 #[default]）

```rust
// ❌ 错误：#[default] 只支持字面量
fn Badge(text: String, #[default = "blue".to_string()] color: String)

// ✅ 正确：用 Option + unwrap_or_else
fn Badge(text: String, color: Option<String>) -> Element {
    let c = color.unwrap_or_else(|| "blue".to_string());
    ...
}
```

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

（暂无提问）

---

## ✅ 第 3 章 小结

学完本章你应该掌握：
1. ✅ 用 `#[component]` 定义组件
2. ✅ 用 Props 传数据（必填、可选、默认值）
3. ✅ 用 `children: Element` 包裹子内容
4. ✅ 用循环复用组件
5. ✅ 理解 Dioxus 组件和 React 组件的对应关系

---

## 📂 本章练习目录

- `components/src/main.rs` —— 7 种组件用法演示
- `components/assets/main.css` —— 样式
