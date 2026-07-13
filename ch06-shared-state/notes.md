# 第 6 章：状态共享（Context）

> Dioxus 客户端课程第 6 章
> 学习目标：掌握 use_context_provider / use_context 实现跨组件状态共享

## 📂 示例代码
```bash
cd ch06-shared-state/shared_state
dx serve --platform web       # 浏览器访问 http://127.0.0.1:8080
```

---

## 6.1 问题：Prop Drilling

当多个组件需要访问同一个状态时，如果用 props 层层传递，叫 **prop drilling**：

```
App (有 state)
  → Middle (传 state，自己不用)
    → Inner (用 state)
```

每加一层中间组件，都要传递 state 和回调函数，**极其痛苦**。

### Context 的解决方案

```
App (use_context_provider 提供 state)
  → Middle (不需要知道 state)
    → Inner (use_context 直接获取)
```

类比 React 的 `createContext` + `useContext`。

---

## 6.2 ⭐ use_context_provider + use_context

### 第 1 步：定义共享状态结构体

```rust
#[derive(Clone, Default)]
struct AppState {
    cart_count: Signal<i32>,       // 注意是 Signal，不是普通值
    user_name: Signal<String>,
    theme: Signal<String>,
}
```

> ⚠️ 必须实现 `Clone`（因为 Context 要在多个组件间共享）。里面用 `Signal<T>` 存状态。

### 第 2 步：在根组件提供状态

```rust
#[component]
fn App() -> Element {
    // ⭐ use_context_provider 提供状态给所有子组件
    use_context_provider(|| {
        let cart_count = use_signal(|| 0);
        let user_name = use_signal(|| "访客".to_string());
        AppState { cart_count, user_name }
    });

    rsx! {
        Navbar {}        // 子组件可以直接用 use_context 获取
        ProductList {}
    }
}
```

### 第 3 步：子组件消费状态

```rust
#[component]
fn Navbar() -> Element {
    // ⭐ use_context 从父组件获取共享状态
    let state = use_context::<AppState>();

    // 读取值
    let count = (state.cart_count)();     // ⭐ 注意两层括号！

    rsx! {
        p { "购物车：{count}" }
    }
}

#[component]
fn ProductList() -> Element {
    // 要修改状态，加 mut
    let mut state = use_context::<AppState>();

    rsx! {
        button {
            onclick: move |_| state.cart_count += 1,   // ⭐ 修改共享状态
            "添加"
        }
    }
}
```

> **关键**：Navbar 和 ProductList 完全独立，但共享同一个 cart_count。点击「添加」，Navbar 自动更新！

---

## 6.3 ⭐ 读取和修改 Context 的语法

### 读取值

```rust
let state = use_context::<AppState>();
let count = (state.cart_count)();    // ⭐ 两层括号！
```

为什么两层括号？
- `state.cart_count` → 取出 Signal
- `(state.cart_count)()` → 调用 Signal 的 `()` 读取值（避免字段访问和方法调用歧义）

### 修改值（需要 `mut`）

```rust
let mut state = use_context::<AppState>();   // ⭐ mut！

state.cart_count += 1;                        // 修改
state.theme.set("dark".to_string());          // set
```

> ⚠️ 修改共享状态的组件，`let mut state` 必须加 `mut`。只读取的可以不加。

---

## 6.4 实战：购物车应用

本章演示了一个完整的购物车应用，三个组件共享状态：

```
                    AppState (cart_count, theme)
                           ↑ 提供
                          App
                    ┌─────┼──────┐
                    ↓     ↓      ↓
                Navbar  Products  ThemeSwitcher
                (读)    (写)      (写)
```

- **Navbar**：显示购物车数量（读 cart_count）
- **ProductGrid**：点击商品添加到购物车（写 cart_count）
- **ThemeSwitcher**：切换主题（写 theme），Navbar 颜色跟着变

**关键**：这三个组件完全独立，不需要互相传递 props。它们通过 Context 共享状态，任何一个修改了状态，其他组件自动更新。

---

## 6.5 对比：Prop Drilling（反面教材）

本章代码里也有一个 Prop Drilling 的对比示例：

```rust
// 不用 Context 的痛苦
#[component]
fn PropDrillingExample() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        MiddleLayer { count, on_increment: move |_| count += 1 }   // 传 count 和回调
    }
}

#[component]
fn MiddleLayer(count: Signal<i32>, on_increment: EventHandler<()>) -> Element {
    rsx! {
        // Middle 自己不用 count，只是传递
        InnerLayer { count, on_increment }   // 继续传
    }
}

#[component]
fn InnerLayer(count: Signal<i32>, on_increment: EventHandler<()>) -> Element {
    rsx! {
        p { "{count}" }
        button { onclick: move |_| on_increment.call(()), "+1" }
    }
}
```

每加一层中间组件，都要加 `count` 和 `on_increment` 两个 props——这就是 prop drilling 的痛苦。

---

## 6.6 ⭐ EventHandler（子→父通信）

在 Prop Drilling 示例里，用 `EventHandler<()>` 让子组件通知父组件：

```rust
#[component]
fn Child(on_click: EventHandler<()>) -> Element {
    rsx! {
        button {
            onclick: move |_| on_click.call(()),   // ⭐ 调用父组件传入的回调
            "点击"
        }
    }
}

// 父组件
Child {
    on_click: move |_| {
        // 子组件点击时执行这里的代码
        count += 1;
    }
}
```

> 类比 React 的 `onClick` props。`EventHandler<T>` 的 T 是事件数据的类型。

---

## ⭐ Context vs Prop Drilling 对比

| | Prop Drilling | Context |
|---|--------------|---------|
| 代码量 | 多（每层都要传递） | 少（直接获取） |
| 可维护性 | 差（改一个要改多层） | 好 |
| 适用场景 | 1-2 层 | 多层或全局 |
| 类比 React | props 层层传 | `useContext` |

### 什么时候用 Context？

| 场景 | 推荐 |
|------|------|
| 父子组件传数据（1-2 层） | Props（简单） |
| 全局状态（用户、主题、购物车） | **Context** ✅ |
| 多层嵌套组件共享状态 | **Context** ✅ |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：为什么读取 Context 是 `(state.field)()` 两层括号？

**A：** 因为 `state.field` 取出的是 `Signal<T>`，需要再调用 `()` 才能拿到值。两层括号避免歧义：
```rust
let count = (state.cart_count)();   // 取 Signal → 调用 () 读值
// 等价于
let signal = state.cart_count;      // Signal<i32>
let count = signal();               // i32
```

### Q2：Context 里的状态都要用 Signal 包装吗？

**A：** 是的。Context 要在多个组件间共享，Signal 是 `Copy` 的（可以自由复制），所以非常适合。如果用普通值，Clone 会产生副本，修改不会同步。

---

### Q3：运行时 panic "The hook list is already borrowed"？

**A：** 这是因为**在 hook 里嵌套调用 hook**，违反了 Hooks 规则。

#### 错误代码
```rust
// ❌ use_signal 在 use_context_provider 的闭包里（嵌套）
use_context_provider(|| {
    let cart_count = use_signal(|| 0);    // ← 违反规则！
    AppState { cart_count }
});
```

#### 正确代码
```rust
// ✅ 用 Signal::new 代替（不是 hook，可以在闭包里用）
use_context_provider(|| AppState {
    cart_count: Signal::new(0),           // ✅ 直接创建
    user_name: Signal::new("访客".to_string()),
});
```

#### 原理
- `use_signal` / `use_context` 等是 **Hook**，必须直接在组件函数体顶层调用
- `Signal::new` 是**普通函数**，可以在任何地方调用（包括闭包内）
- 两者都能创建 Signal，但 `use_signal` 额外注册到组件的 hook 列表（用于响应式更新）

#### Hooks 规则（和 React 一样）
1. **只能在组件顶层直接调用**，不能嵌套在闭包里
2. **不能在 if/for 里**（必须每次渲染按相同顺序）
3. 违反会运行时 panic：`hook list is already borrowed`

---

## ✅ 第 6 章 小结

学完本章你应该掌握：
1. ✅ 用 `use_context_provider` 提供共享状态
2. ✅ 用 `use_context::<T>()` 消费状态
3. ✅ 读取 `(state.field)()` 和修改 `state.field.set()` 语法
4. ✅ 理解 Context 解决了 Prop Drilling 问题
5. ✅ 用 `EventHandler` 实现子→父通信

---

## 📂 本章练习目录

- `shared_state/src/main.rs` —— 购物车 + 主题切换 + Prop Drilling 对比
- `shared_state/assets/main.css` —— 样式
