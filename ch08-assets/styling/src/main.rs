// ===== 第 8 章：样式与资源 =====
// 运行：dx serve --platform web
// 浏览器访问 http://127.0.0.1:8080

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

// ========================================================
// ⭐ manganis：用 Rust 宏引用资源
// ========================================================
// asset!("/path/to/file") 会在编译期处理资源，返回正确的 URL
// 适用于 CSS、图片、字体、SVG 等

// 引用图片资源（编译期检查文件存在）
static RUST_LOGO: manganis::Asset = manganis::asset!("/assets/icons/rust-logo.svg");

#[component]
fn App() -> Element {
    let mut dark_mode = use_signal(|| false);
    let mut font_size = use_signal(|| 16);

    // ⭐ 动态样式：根据状态计算 class 和 style
    let theme_class = if dark_mode() { "dark" } else { "light" };
    let bg_color = if dark_mode() { "#1a1a2e" } else { "#fafafa" };
    let text_color = if dark_mode() { "#e0e0e0" } else { "#333333" };
    let active = font_size() > 18;
    let box_class = if active { "box active" } else { "box" };

    rsx! {
        div {
            class: "app {theme_class}",
            style: "background: {bg_color}; color: {text_color}; min-height: 100vh;",

            h1 { "🎨 样式与资源" }

            // ========================================================
            // 1. 内联样式（style 属性）
            // ========================================================
            div { class: "card",
                h2 { "1. 内联样式" }
                p {
                    style: "color: #b7410e; font-size: {font_size}px; font-weight: bold;",
                    "这段文字是内联样式（字号 {font_size}px）"
                }
                div { class: "button-group",
                    button {
                        onclick: move |_| font_size -= 2,
                        "A- 缩小"
                    }
                    button {
                        onclick: move |_| font_size += 2,
                        "A+ 放大"
                    }
                }
            }

            // ========================================================
            // 2. CSS 类（外部样式表 main.css）
            // ========================================================
            div { class: "card",
                h2 { "2. CSS 类" }
                p { class: "highlight", "这个用 .highlight 类" }
                p { class: "muted", "这个用 .muted 类" }
                p { class: "badge success", "✅ 成功标签" }
                p { class: "badge warning", "⚠️ 警告标签" }
                p { class: "badge error", "❌ 错误标签" }
            }

            // ========================================================
            // 3. ⭐ 动态样式（主题切换）
            // ========================================================
            div { class: "card",
                h2 { "3. 动态样式（主题切换）" }
                p { "当前主题：{theme_class}" }
                button {
                    class: "btn",
                    onclick: move |_| dark_mode.toggle(),
                    if dark_mode() { "☀️ 切换亮色" } else { "🌙 切换暗色" }
                }
            }

            // ========================================================
            // 4. ⭐ 图片资源（manganis asset!）
            // ========================================================
            div { class: "card",
                h2 { "4. 图片资源（asset!）" }
                p { class: "hint", "用 manganis::asset! 引用图片，编译期检查存在" }

                // ⭐ 用 asset! 引用的资源
                img {
                    src: RUST_LOGO,
                    alt: "Rust Logo",
                    style: "width: 80px; height: 80px;",
                }

                // 也可以直接写路径字符串
                img {
                    src: "/assets/icons/rust-logo.svg",
                    alt: "Rust Logo 2",
                    style: "width: 40px; height: 40px;",
                }
            }

            // ========================================================
            // 5. 条件 class（根据状态切换样式）
            // ========================================================
            div { class: "card",
                h2 { "5. 条件 class" }
                div {
                    class: "{box_class}",
                    "字号大于 18 时这个盒子会高亮"
                }
                p { class: "hint", "当前字号 {font_size}，大于 18 → 高亮" }
            }

            // ========================================================
            // 6. 响应式布局（CSS Grid/Flex）
            // ========================================================
            div { class: "card",
                h2 { "6. 响应式布局" }
                div { class: "grid-3",
                    div { class: "grid-item", "项目 1" }
                    div { class: "grid-item", "项目 2" }
                    div { class: "grid-item", "项目 3" }
                    div { class: "grid-item", "项目 4" }
                    div { class: "grid-item", "项目 5" }
                    div { class: "grid-item", "项目 6" }
                }
            }
        }
    }
}
