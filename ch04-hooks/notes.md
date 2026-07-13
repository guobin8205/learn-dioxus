# 第 4 章：Hooks 状态管理

> Dioxus 客户端课程第 4 章
> 学习目标：掌握 use_signal 创建状态，让 UI 响应数据变化

## 📂 示例代码
```bash
cd ch04-hooks/hooks
dx serve --platform web       # 浏览器访问 http://127.0.0.1:8080
```

---

## 什么是 Hooks？

Hooks 让函数组件「记住」状态。状态变化时，UI **自动重新渲染**。

类比 React：
- React: `const [count, setCount] = useState(0)`
- Dioxus: `let mut count = use_signal(|| 0)`

---

## 4.1 use_signal：最基本的状态 ⭐

### 创建状态

```rust
let mut count = use_signal(|| 0);    // 初始值 0
let mut name = use_signal(|| String::from("Alice"));
let mut show = use_signal(|| true);
```

### 读取状态

```rust
// 用 count() 读取当前值（注意有括号！）
rsx! { p { "计数：{count}" } }        // RSX 里直接用
let double = count() * 2;             // Rust 代码里用 count()
```

### ⭐ 修改状态（三种方式）

```rust
let mut count = use_signal(|| 0);

// 方式 1：运算符（最简洁）
count += 1;        // 加
count -= 1;        // 减

// 方式 2：set()（直接赋值）
count.set(0);      // 重置为 0
count.set(100);    // 设为指定值

// 方式 3：write()（更复杂的修改）
count.write().clear();   // 例如对 Vec 清空
```

### 完整计数器示例

```rust
#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        p { "{count}" }
        button { onclick: move |_| count += 1, "+1" }
        button { onclick: move |_| count -= 1, "-1" }
        button { onclick: move |_| count.set(0), "重置" }
    }
}
```

> ⚠️ **关键**：`let mut count` 必须加 `mut`，因为修改时要用 `+=`、`set()` 等。

---

## 4.2 派生值（从状态计算）

不需要额外的 Hook，直接计算：

```rust
let mut count = use_signal(|| 0);

// 派生值（等价于 React 的 useMemo，但更简单）
let double = count() * 2;
let is_even = count() % 2 == 0;
let message = if count() > 10 { "大" } else { "小" };

rsx! {
    p { "{count} × 2 = {double}" }
    p { "{message}" }
}
```

> 💡 每次 count 变化，组件重新渲染，派生值自动更新。

---

## 4.3 各种类型的状态

### String（文本）

```rust
let mut text = use_signal(|| "你好".to_string());

rsx! {
    p { "{text}" }
    input {
        value: "{text}",
        oninput: move |e| text.set(e.value()),   // ⭐ e.value() 获取输入
    }
}
```

### bool（开关）

```rust
let mut show = use_signal(|| false);

rsx! {
    button { onclick: move |_| show.toggle(), "切换" }
    if show() { p { "显示的内容" } }
}
```

> 💡 `show.toggle()` 是 bool 专用的快捷方法（等价于 `show.set(!show())`）。

### Vec（列表）

```rust
let mut items = use_signal(|| vec![1, 2, 3]);

rsx! {
    for item in items.iter() {
        p { "{item}" }
    }
    button { onclick: move |_| items.write().push(4), "添加" }
}
```

---

## 4.4 温度转换器（多状态联动）

这是综合示例——两个输入框互相联动：

```rust
#[component]
fn TemperatureConverter() -> Element {
    let mut celsius = use_signal(|| 0.0);
    let fahrenheit = celsius() * 9.0 / 5.0 + 32.0;   // 派生值

    rsx! {
        input {
            value: "{celsius():.1}",
            oninput: move |e| {
                if let Ok(val) = e.value().parse::<f64>() {
                    celsius.set(val);
                }
            },
        }
        input {
            value: "{fahrenheit:.1}",
            oninput: move |e| {
                if let Ok(val) = e.value().parse::<f64>() {
                    celsius.set((val - 32.0) * 5.0 / 9.0);   // 反向转换
                }
            },
        }
    }
}
```

**关键点**：只存一个状态（celsius），华氏度是派生值。修改任一输入框都更新 celsius，另一个自动同步。

---

## ⭐ use_signal vs React useState

| 概念 | Dioxus | React |
|------|--------|-------|
| 创建 | `use_signal(\|\| 0)` | `useState(0)` |
| 返回 | Signal 对象 | `[value, setter]` 元组 |
| 读取 | `count()` （调用） | `count`（直接） |
| 修改 | `count += 1` / `count.set(x)` | `setCount(c => c+1)` |
| 派生 | 直接计算 | `useMemo(() => ..., [deps])` |
| 必须可变 | `let mut count` | const 解构 |

### Dioxus 的优势

Dioxus 的 Signal 比 React useState 更智能：
- **不需要依赖数组**（React 的 useMemo/useEffect 那个 `[deps]`）
- Signal 自动追踪依赖，状态变化精确触发重新渲染
- 可以直接 `+=`/`-=`，不需要函数式更新

---

## ⭐ Hook 的规则（和 React 一样）

### 规则 1：只能在组件顶层调用

```rust
#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);   // ✅ 顶层

    // ❌ 不能在条件分支里
    // if cond { let x = use_signal(|| 0); }

    // ❌ 不能在循环里
    // for _ in 0..3 { let x = use_signal(|| 0); }

    rsx! { ... }
}
```

### 规则 2：必须每次渲染都按相同顺序调用

Hook 靠调用顺序记住状态，所以不能放在 if/for 里。

---

## 📋 事件读取：e.value()

处理输入时，用 `e.value()` 获取输入值：

```rust
input {
    oninput: move |e| {
        let text = e.value();     // ⭐ 获取当前输入
        name.set(text);
    },
}
```

常见事件类型：
| 事件 | 获取值 |
|------|--------|
| `oninput`（输入框） | `e.value()` |
| `onclick`（点击） | 无（`_`） |
| `onchange`（变化） | `e.value()` |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

（暂无提问）

---

## ✅ 第 4 章 小结

学完本章你应该掌握：
1. ✅ 用 `use_signal` 创建状态
2. ✅ 用 `()` 读取、`+=`/`set()`/`write()` 修改
3. ✅ 从状态计算派生值
4. ✅ 处理 String、bool、Vec 等各种类型
5. ✅ 理解 Signal 和 React useState 的对应关系

---

## 📂 本章练习目录

- `hooks/src/main.rs` —— 4 个状态管理演示
- `hooks/assets/main.css` —— 样式
