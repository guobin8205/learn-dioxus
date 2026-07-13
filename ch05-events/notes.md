# 第 5 章：事件处理

> Dioxus 客户端课程第 5 章
> 学习目标：掌握各种事件（点击、输入、键盘、鼠标、表单）

## 📂 示例代码
```bash
cd ch05-events/events
dx serve --platform web       # 浏览器访问 http://127.0.0.1:8080
```

---

## 5.1 事件的基本语法

事件用 `on事件名: 闭包` 绑定：

```rust
rsx! {
    button {
        onclick: move |_| count += 1,    // ⭐ 事件处理
        "点击"
    }
}
```

### 为什么用 `move |_|`？

- `move`：把外部变量（如 `count`）的所有权移入闭包（必要，因为闭包要修改状态）
- `|_|`：事件参数用不到时用 `_` 忽略

类比 React：
```jsx
<button onClick={() => setCount(c => c + 1)}>  // React
```
```rust
button { onclick: move |_| count += 1, ... }   // Dioxus
```

---

## 5.2 常用事件类型 ⭐

### onclick（点击）

```rust
button {
    onclick: move |_| {
        count += 1;
        println!("点击了");
    },
    "点我"
}
```

### oninput（输入，实时触发）

```rust
input {
    value: "{text}",
    oninput: move |e| text.set(e.value()),   // ⭐ e.value() 获取输入
}
```

### onkeydown（键盘按键）

```rust
input {
    onkeydown: move |e| {
        // e.key() 返回 Key 枚举
        if e.key() == Key::Enter {
            submit();
        }
        // e.code() 返回物理按键代码（如 "KeyA"）
    },
}
```

### onchange（值变化，失焦时触发）

```rust
select {
    onchange: move |e| color.set(e.value()),
    option { value: "红", "红色" }
}
```

### onmousemove（鼠标移动）

```rust
div {
    onmousemove: move |e| {
        let coords = e.data().client_coordinates();
        x.set(coords.x as i32);
        y.set(coords.y as i32);
    },
}
```

---

## 5.3 事件对象的常用方法 ⭐

### FormEvent（oninput/onchange）

```rust
oninput: move |e| {
    let value: String = e.value();       // 获取输入值
}
```

### KeyboardEvent（onkeydown/onkeyup）

```rust
onkeydown: move |e| {
    let key = e.key();          // Key 枚举（Key::Enter、Key::Escape...）
    let code = e.code();        // 物理按键代码（字符串）
}
```

### MouseEvent（onclick/onmousemove）

```rust
onmousemove: move |e| {
    let coords = e.data().client_coordinates();
    let x = coords.x;
    let y = coords.y;
}
```

---

## 5.4 实战：TodoApp

综合运用 oninput + onkeydown + onclick：

```rust
#[component]
fn TodoApp() -> Element {
    let mut todos = use_signal(|| vec!["学习 Rust".to_string()]);
    let mut new_todo = use_signal(|| String::new());

    rsx! {
        // 输入框：oninput 实时更新 + onkeydown 检测 Enter
        input {
            value: "{new_todo}",
            oninput: move |e| new_todo.set(e.value()),
            onkeydown: move |e| {
                if e.key() == Key::Enter {
                    if !new_todo.read().is_empty() {
                        todos.write().push(new_todo.read().clone());
                        new_todo.set(String::new());
                    }
                }
            },
        }

        // 添加按钮
        button {
            onclick: move |_| {
                if !new_todo.read().is_empty() {
                    todos.write().push(new_todo.read().clone());
                    new_todo.set(String::new());
                }
            },
            "添加"
        }

        // 列表（带删除按钮）
        ul {
            for (i, todo) in todos.read().iter().enumerate() {
                li {
                    span { "{todo}" }
                    button {
                        onclick: move |_| {
                            todos.write().remove(i);
                        },
                        "删除"
                    }
                }
            }
        }
    }
}
```

---

## ⭐ Dioxus vs React 事件对比

| 概念 | Dioxus | React |
|------|--------|-------|
| 绑定 | `onclick: fn` | `onClick={fn}` |
| 事件名 | 全小写 `onclick` | 驼峰 `onClick` |
| 输入值 | `e.value()` | `e.target.value` |
| 键盘 | `e.key()` (Key 枚举) | `e.key` (字符串) |
| 阻止默认 | `e.prevent_default()` | `e.preventDefault()` |

---

## ⭐ 常见踩坑

### 1. 闭包需要 `move`

```rust
// ❌ 编译错误：count 没有移入闭包
button { onclick: |_| count += 1 }

// ✅ 正确：用 move 把 count 移入
button { onclick: move |_| count += 1 }
```

### 2. 修改状态的闭包要 `mut`

```rust
// 如果闭包内部修改了 Signal，闭包变量需要 mut
let mut add_log = move |msg: String| {
    logs.write().push(msg);    // 修改了 logs
};
```

### 3. for 循环里的列表删除

```rust
for (i, todo) in todos.read().iter().enumerate() {
    button {
        onclick: move |_| {
            todos.write().remove(i);    // ⭐ i 被 move 进闭包
        },
    }
}
```

> 注意：`i` 是索引，被 `move` 进每个按钮的闭包，点击时删除对应项。

---

## 📋 事件速查表

| 事件 | 触发时机 | 获取数据 |
|------|---------|---------|
| `onclick` | 点击 | 无 |
| `oninput` | 输入（实时） | `e.value()` |
| `onchange` | 值变化（失焦） | `e.value()` |
| `onkeydown` | 按键按下 | `e.key()` / `e.code()` |
| `onkeyup` | 按键释放 | `e.key()` |
| `onmousemove` | 鼠标移动 | `e.data().client_coordinates()` |
| `onmouseover` | 鼠标进入 | - |
| `onsubmit` | 表单提交 | `e.value()` |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：oninput 和 onchange 有什么区别？

**A：**
- `oninput`：**每次输入**都触发（实时）
- `onchange`：**失焦时**触发（输入框失去焦点）

| 事件 | 触发时机 | 适用 |
|------|---------|------|
| `oninput` | 每按一个键 | 实时搜索、实时预览 |
| `onchange` | 失焦时 | 表单验证、提交 |

---

## ✅ 第 5 章 小结

学完本章你应该掌握：
1. ✅ 绑定 onclick/oninput/onkeydown/onchange 等事件
2. ✅ 用 `e.value()` 获取输入值
3. ✅ 用 `e.key()` 检测按键（Key::Enter 等）
4. ✅ 理解闭包的 `move` 和 `mut`
5. ✅ 实现完整的 TodoApp（输入 + 列表 + 删除）

---

## 📂 本章练习目录

- `events/src/main.rs` —— 7 种事件演示 + TodoApp
- `events/assets/main.css` —— 样式
