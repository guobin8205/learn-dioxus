// ===== 第 4 章：Hooks 状态管理 =====
// 运行：dx serve --platform web
// 浏览器访问 http://127.0.0.1:8080

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

// ========================================================
// 1. ⭐ use_signal：最基本的 Hook（类比 React useState）
// ========================================================

#[component]
fn App() -> Element {
    // use_signal 创建可变状态
    // 类比 React：const [count, setCount] = useState(0)
    let mut count = use_signal(|| 0);

    // 多个状态
    let mut text = use_signal(|| "你好".to_string());
    let mut show_secret = use_signal(|| false);

    // 派生值（从状态计算）—— 等价于 React 的 useMemo
    let double = count() * 2;
    let is_even = count() % 2 == 0;

    rsx! {
        div { class: "container",
            // ========================================================
            // 1. 计数器：use_signal 基础
            // ========================================================
            div { class: "card",
                h2 { "1. 计数器（use_signal）" }

                p { class: "big-number", "计数：{count}" }
                p { "双倍：{double}" }      // 派生值
                p { "{is_even}（{is_even})", }

                // ⭐ 修改状态：用 write() 或 set()
                div { class: "button-group",
                    button {
                        onclick: move |_| count += 1,
                        "+1"
                    }
                    button {
                        onclick: move |_| count -= 1,
                        "-1"
                    }
                    button {
                        onclick: move |_| count.set(0),
                        "重置"
                    }
                    button {
                        onclick: move |_| count += 10,
                        "+10"
                    }
                }
            }

            // ========================================================
            // 2. 文本输入：use_signal 存 String
            // ========================================================
            div { class: "card",
                h2 { "2. 文本输入" }
                p { "当前文本：{text}" }

                input {
                    value: "{text}",
                    oninput: move |e| text.set(e.value()),   // ⭐ e.value() 获取输入值
                    placeholder: "输入文字...",
                }

                button {
                    onclick: move |_| text.set("".to_string()),
                    "清空"
                }
                button {
                    onclick: move |_| text.set("你好 Dioxus".to_string()),
                    "默认值"
                }
            }

            // ========================================================
            // 3. 切换显示：use_signal 存 bool
            // ========================================================
            div { class: "card",
                h2 { "3. 条件切换" }

                button {
                    onclick: move |_| show_secret.toggle(),
                    if show_secret() { "🙈 隐藏秘密" } else { "👀 显示秘密" }
                }

                if show_secret() {
                    p { class: "secret", "🎉 秘密内容：Rust 太棒了！" }
                }
            }

            // ========================================================
            // 4. 多状态联动：温度转换器
            // ========================================================
            TemperatureConverter {}
        }
    }
}

// ========================================================
// 温度转换器：组件内的多状态联动
// ========================================================
#[component]
fn TemperatureConverter() -> Element {
    let mut celsius = use_signal(|| 0.0);

    // 从 celsius 派生华氏度（不需要额外 state）
    let fahrenheit = celsius() * 9.0 / 5.0 + 32.0;

    rsx! {
        div { class: "card",
            h2 { "4. 温度转换器（多状态联动）" }

            div { class: "temp-row",
                label { "摄氏度：" }
                input {
                    r#type: "number",
                    value: "{celsius():.1}",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<f64>() {
                            celsius.set(val);
                        }
                    },
                }
            }

            div { class: "temp-row",
                label { "华氏度：" }
                input {
                    r#type: "number",
                    value: "{fahrenheit:.1}",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<f64>() {
                            celsius.set((val - 32.0) * 5.0 / 9.0);
                        }
                    },
                }
            }

            p { class: "hint",
                "{celsius():.1}°C = {fahrenheit:.1}°F"
            }
        }
    }
}
