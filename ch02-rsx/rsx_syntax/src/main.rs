// ===== 第 2 章：RSX 语法全解 =====
// 运行：dx serve --platform web
// 浏览器访问 http://127.0.0.1:8080

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

/// 根组件：演示各种 RSX 语法
fn App() -> Element {
    // ⭐ 所有变量必须先在 rsx! 之外声明
    let name = "Rust 学习者";
    let version = "0.7";
    let items = vec!["苹果", "香蕉", "橙子"];
    let is_logged_in = true;
    let upper = "HELLO".to_lowercase();
    let truth = if true { "真" } else { "假" };
    let is_active = true;
    let box_class = if is_active { "box active" } else { "box" };
    let color = if is_active { "red" } else { "black" };

    rsx! {
        div { class: "container",
            h1 { "RSX 语法演示" }
            p { "你好，{name}！Dioxus {version}" }

            // 2. 属性（class、id、style 等）
            div {
                class: "highlight",
                id: "my-div",
                style: "color: blue; font-weight: bold;",
                "我有样式和属性"
            }

            // 3. 插入 Rust 表达式
            div {
                p { "1 + 1 = {1 + 1}" }
                p { "名字长度: {name.len()}" }
                p { "转小写: {upper}" }
                p { "布尔: {truth}" }
            }

            // 4. 条件渲染
            div {
                h3 { "条件渲染" }
                {if is_logged_in {
                    rsx! { p { "✅ 已登录" } }
                } else {
                    rsx! { p { "❌ 未登录" } }
                }}
                if is_logged_in {
                    p { "欢迎回来！" }
                }
            }

            // 5. 列表渲染（for 循环）
            div {
                h3 { "列表渲染（for 循环）" }
                ul {
                    for item in &items {
                        li { "{item}" }
                    }
                }
                ol {
                    for (i, item) in items.iter().enumerate() {
                        li { "[{i}] {item}" }
                    }
                }
            }

            // 6. 事件处理（下一章详讲）
            div {
                h3 { "事件（下一章详讲）" }
                button {
                    onclick: move |_| println!("点击了！"),
                    "点击打印到控制台"
                }
            }

            // 7. 动态属性
            div {
                h3 { "动态属性" }
                div {
                    class: "{box_class}",
                    "动态 class"
                }
                div {
                    style: "color: {color};",
                    "动态颜色"
                }
            }

            // 8. fragment：多个根元素
            hr {}
            p { "上面是分隔线" }

            // 9. 特殊字符：用 {} 包裹 Rust 表达式
            div {
                p { "用变量插入特殊文本：" }
                {rsx! { p { {"<div> 字符串里的 HTML 标签"} } }}
                {rsx! { p { {"引号 \" 也能显示"} } }}
            }
        }
    }
}
