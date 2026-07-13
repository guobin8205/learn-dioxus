# 第 7 章：路由（Router）

> Dioxus 客户端课程第 7 章
> 学习目标：掌握多页面导航（路由、链接、参数、编程式导航）

## 📂 示例代码
```bash
cd ch07-router/router_demo
dx serve --platform web       # 浏览器访问 http://127.0.0.1:8080
```

---

## 7.1 路由的核心概念

Dioxus Router 用 **enum + `#[derive(Routable)]`** 定义路由——**类型安全**，编译器保证所有路由都被处理。

类比 React Router：
- React: `<Routes><Route path="/" element={<Home/>}/></Routes>`
- Dioxus: `enum Route { #[route("/")] Home {} }`

---

## 7.2 ⭐ 定义路由

### 第 1 步：启用 router feature

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web", "router"] }
#                                        ↑ 加上 router
```

### 第 2 步：定义 Route enum

```rust
#[derive(Clone, Routable, PartialEq, Debug)]
enum Route {
    #[route("/")]
    Home {},                          // 首页

    #[route("/about")]
    About {},                         // 关于页

    #[route("/user/:id")]            // ⭐ 带参数（:id）
    User { id: i32 },

    #[route("/:route")]              // 404（匹配任意路径）
    NotFound { route: String },
}
```

**关键点**：
- `#[route("路径")]` 标注每个路由的 URL
- `:id` 表示动态参数（从 URL 提取）
- enum 变体的字段 = URL 参数
- 最后一个 `#[route("/:route")]` 是 404 兜底

### 第 3 步：用 Router 组件渲染

```rust
#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}    // ⭐ 根据 URL 自动渲染对应页面
    }
}
```

`Router` 会读取浏览器 URL，匹配 Route enum 的变体，渲染对应组件。

---

## 7.3 页面组件

每个路由变体对应一个同名组件（首字母小写）：

```rust
// 对应 Route::Home {}
#[component]
fn Home() -> Element {
    rsx! { h1 { "首页" } }
}

// 对应 Route::User { id }
#[component]
fn User(id: i32) -> Element {     // ⭐ 参数自动从 URL 传入
    rsx! { p { "用户 {id}" } }
}

// 对应 Route::NotFound { route }
#[component]
fn NotFound(route: String) -> Element {
    rsx! { h1 { "404: /{route}" } }
}
```

> 命名规则：`Route::Home {}` → 组件 `fn Home()`，`Route::User { id }` → 组件 `fn User(id: i32)`

---

## 7.4 ⭐ 导航方式

### 方式 1：Link 组件（声明式导航）

```rust
use dioxus::prelude::*;

rsx! {
    // ⭐ Link 跳转（类比 React Router 的 <Link>）
    Link { to: Route::Home {}, "首页" }
    Link { to: Route::About {}, "关于" }
    Link { to: Route::User { id: 42 }, "用户 42" }
}
```

`Link` 渲染成 `<a>` 标签，点击后 URL 变化，Router 自动渲染新页面。

### 方式 2：use_navigator（编程式导航）

```rust
// 在事件处理里用代码跳转（如表单提交后）
button {
    onclick: move |_| {
        let nav = navigator();              // ⭐ 获取导航器
        nav.push(Route::About {});          // 跳转
    },
    "跳转到关于"
}
```

| 方式 | 适用 | 类比 React |
|------|------|-----------|
| `Link { to: ... }` | 普通链接 | `<Link to="/about">` |
| `navigator().push(...)` | 代码控制（表单/逻辑后） | `navigate("/about")` |

---

## 7.5 带参数的路由

```rust
#[derive(Routable)]
enum Route {
    #[route("/user/:id")]
    User { id: i32 },           // ⭐ :id 从 URL 提取，传入 User 组件
}

// URL: /user/42 → User 组件收到 id=42
#[component]
fn User(id: i32) -> Element {
    rsx! { p { "用户 ID: {id}" } }
}

// 在链接里传参数
Link { to: Route::User { id: 42 }, "查看用户 42" }
```

---

## 7.6 404 页面

```rust
#[derive(Routable)]
enum Route {
    #[route("/")]
    Home {},

    // ⭐ 最后一个：匹配任意未定义路径
    #[route("/:route")]
    NotFound { route: String },
}

// 访问 /anything → NotFound 组件，route = "anything"
```

---

## ⭐ Dioxus Router vs React Router

| 概念 | Dioxus | React Router |
|------|--------|-------------|
| 定义路由 | `enum Route` + `#[derive(Routable)]` | `<Routes><Route>` |
| 类型安全 | ✅ 编译器保证 | ❌ 字符串路径 |
| 链接 | `Link { to: Route::Home {} }` | `<Link to="/">` |
| 编程导航 | `navigator().push(...)` | `navigate("/")` |
| 参数 | `User { id: i32 }` | `useParams()` |
| 404 | `#[route("/:route")]` | `<Route path="*">` |

### Dioxus 的优势

- **类型安全**：路由路径是 enum 变体，拼错编译器会报错
- **参数类型自动推断**：`:id` 自动变成 `i32`（或你指定的类型）
- **穷尽检查**：所有路由都在 enum 里，不会遗漏

---

## 📋 路由速查

| 操作 | 写法 |
|------|------|
| 启用路由 | `features = ["router"]` |
| 定义路由 | `#[derive(Routable)] enum Route` |
| 渲染路由 | `Router::<Route> {}` |
| 链接 | `Link { to: Route::Xxx {}, "文字" }` |
| 编程导航 | `navigator().push(Route::Xxx {})` |
| 带参数 | `#[route("/user/:id")] User { id: i32 }` |
| 404 | `#[route("/:route")] NotFound { route: String }` |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

（暂无提问）

---

## ✅ 第 7 章 小结

学完本章你应该掌握：
1. ✅ 用 `#[derive(Routable)] enum Route` 定义路由
2. ✅ 用 `Router::<Route> {}` 渲染
3. ✅ 用 `Link` 和 `navigator()` 导航
4. ✅ 用 `:id` 定义带参数的路由
5. ✅ 配置 404 页面

---

## 📂 本章练习目录

- `router_demo/src/main.rs` —— 4 个页面 + 参数路由 + 404
- `router_demo/assets/main.css` —— 样式
