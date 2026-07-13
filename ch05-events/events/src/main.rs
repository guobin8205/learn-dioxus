// ===== 第 5 章：事件处理 =====
// 运行：dx serve --platform web
// 浏览器访问 http://127.0.0.1:8080

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // 各种状态
    let mut click_count = use_signal(|| 0);
    let mut input_text = use_signal(|| String::new());
    let mut last_key = use_signal(|| String::from("无"));
    let mut mouse_x = use_signal(|| 0);
    let mut mouse_y = use_signal(|| 0);
    let mut selected_color = use_signal(|| String::from("红色"));
    let mut logs = use_signal(|| Vec::<String>::new());

    // 辅助函数：添加日志
    let mut add_log = move |msg: String| {
        logs.write().push(msg);
        // 只保留最近 5 条
        if logs.read().len() > 5 {
            logs.write().remove(0);
        }
    };

    // 派生值：根据选择的颜色计算背景色
    let bg_color = match selected_color() {
        s if s.contains("红") => "#e74c3c",
        s if s.contains("绿") => "#2ecc71",
        s if s.contains("蓝") => "#3498db",
        s if s.contains("黄") => "#f1c40f",
        _ => "#999999",
    };

    rsx! {
        div { class: "container",
            h1 { "🎯 事件处理演示" }

            // ========================================================
            // 1. ⭐ onclick：点击事件（最常用）
            // ========================================================
            div { class: "card",
                h2 { "1. 点击事件（onclick）" }
                p { "点击次数：{click_count}" }

                button {
                    onclick: move |_| {
                        click_count += 1;
                        add_log(format!("点击了按钮（第 {} 次）", click_count()));
                    },
                    "点我 +1"
                }
                button {
                    onclick: move |_| {
                        click_count.set(0);
                        add_log("重置计数".to_string());
                    },
                    "重置"
                }
            }

            // ========================================================
            // 2. oninput：输入事件（实时响应）
            // ========================================================
            div { class: "card",
                h2 { "2. 输入事件（oninput）" }
                p { "你输入了：{input_text}（{input_text.len()} 个字符）" }

                input {
                    class: "text-input",
                    value: "{input_text}",
                    placeholder: "实时输入...",
                    oninput: move |e| {
                        input_text.set(e.value());
                    },
                }

                button {
                    onclick: move |_| {
                        input_text.set(String::new());
                    },
                    "清空"
                }
            }

            // ========================================================
            // 3. onkeydown：键盘事件
            // ========================================================
            div { class: "card",
                h2 { "3. 键盘事件（onkeydown）" }
                p { "最后按下的键：{last_key}" }

                input {
                    class: "text-input",
                    placeholder: "在这里按键试试...",
                    onkeydown: move |e| {
                        last_key.set(format!("{:?}（code: {}）", e.key(), e.code()));
                    },
                }

                p { class: "hint",
                    "试试按 Enter、Escape、方向键等"
                }
            }

            // ========================================================
            // 4. onmouseover / onmouseout：鼠标事件
            // ========================================================
            div { class: "card",
                h2 { "4. 鼠标事件（onmousemove）" }
                p { "鼠标位置：({mouse_x}, {mouse_y})" }

                div {
                    class: "hover-area",
                    onmousemove: move |e| {
                        let coords = e.data().client_coordinates();
                        mouse_x.set(coords.x as i32);
                        mouse_y.set(coords.y as i32);
                    },
                    "在这个区域移动鼠标 🖱️"
                }
            }

            // ========================================================
            // 5. ⭐ 表单：onchange（select 下拉框）
            // ========================================================
            // 注意：bg_color 在 rsx! 外部声明（见 App 函数顶部）

            div { class: "card",
                h2 { "5. 表单事件（onchange）" }
                p { "你选择了：{selected_color}" }

                select {
                    class: "text-input",
                    onchange: move |e| {
                        selected_color.set(e.value());
                    },
                    option { value: "红色", "🔴 红色" }
                    option { value: "绿色", "🟢 绿色" }
                    option { value: "蓝色", "🔵 蓝色" }
                    option { value: "黄色", "🟡 黄色" }
                }

                // 根据选择显示不同颜色
                div {
                    class: "color-box",
                    style: "background: {bg_color};",
                    "{selected_color}"
                }
            }

            // ========================================================
            // 6. 事件日志（综合演示）
            // ========================================================
            div { class: "card",
                h2 { "6. 事件日志" }
                p { class: "hint", "最近 5 条事件：" }
                div { class: "log-area",
                    if logs.read().is_empty() {
                        p { class: "muted", "（还没有事件）" }
                    } else {
                        for (i, log) in logs.read().iter().enumerate() {
                            p { "[{i}] {log}" }
                        }
                    }
                }
                button {
                    class: "btn-secondary",
                    onclick: move |_| logs.write().clear(),
                    "清空日志"
                }
            }

            // ========================================================
            // 7. TodoApp：综合实战（输入 + 列表 + 操作）
            // ========================================================
            TodoApp {}
        }
    }
}

// ========================================================
// TodoApp：综合运用输入、点击、列表
// ========================================================
#[component]
fn TodoApp() -> Element {
    let mut todos = use_signal(|| vec![
        "学习 Rust".to_string(),
        "学习 Dioxus".to_string(),
    ]);
    let mut new_todo = use_signal(|| String::new());

    rsx! {
        div { class: "card",
            h2 { "7. TodoApp（综合实战）" }

            // 输入区
            div { class: "todo-input",
                input {
                    class: "text-input",
                    value: "{new_todo}",
                    placeholder: "输入待办事项...",
                    oninput: move |e| new_todo.set(e.value()),
                    // ⭐ onkeydown：按 Enter 添加
                    onkeydown: move |e| {
                        if e.key() == Key::Enter {
                            if !new_todo.read().is_empty() {
                                todos.write().push(new_todo.read().clone());
                                new_todo.set(String::new());
                            }
                        }
                    },
                }
                button {
                    onclick: move |_| {
                        if !new_todo.read().is_empty() {
                            todos.write().push(new_todo.read().clone());
                            new_todo.set(String::new());
                        }
                    },
                    "添加"
                }
            }

            // 列表
            ul { class: "todo-list",
                for (i, todo) in todos.read().iter().enumerate() {
                    li { key: "{i}",
                        span { "{todo}" }
                        button {
                            class: "btn-delete",
                            onclick: move |_| {
                                todos.write().remove(i);
                            },
                            "删除"
                        }
                    }
                }
            }

            p { class: "hint", "共 {todos.read().len()} 项" }
        }
    }
}
