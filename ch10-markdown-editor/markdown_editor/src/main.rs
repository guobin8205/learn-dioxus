// ===== 第 10 章：Markdown 编辑器（上）=====
// 运行：dx serve --platform web
// 浏览器访问 http://127.0.0.1:8080

use dioxus::prelude::*;
use pulldown_cmark::{Parser, Options, html};

fn main() {
    dioxus::launch(App);
}

// ========================================================
// ⭐ Markdown 解析函数
// ========================================================
// 用 pulldown-cmark 把 Markdown 文本转成 HTML

fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[component]
fn App() -> Element {
    // ⭐ 核心状态：编辑器内容
    let mut content = use_signal(|| {
        "# 欢迎使用 Markdown 编辑器\n\n\
        ## 功能演示\n\n\
        这是一个用 **Dioxus** 写的 Markdown 编辑器。\n\n\
        ### 支持的语法\n\n\
        - **粗体** 文本\n\
        - *斜体* 文本\n\
        - `行内代码`\n\
        - [链接](https://dioxuslabs.com)\n\n\
        ### 代码块\n\n\
        ```rust\n\
        fn main() {\n\
            println!(\"Hello, Markdown!\");\n\
        }\n\
        ```\n\n\
        ### 表格\n\n\
        | 功能 | 状态 |\n\
        |------|------|\n\
        | 实时预览 | ✅ |\n\
        | 暗色模式 | ✅ |\n\n\
        > 💡 左侧编辑，右侧实时预览\n\n\
        ---\n\n\
        试试修改左侧的文字！"
            .to_string()
    });

    let mut dark_mode = use_signal(|| false);

    // 派生值：把 Markdown 转 HTML
    let markdown_html = render_markdown(&content());
    let theme_class = if dark_mode() { "dark" } else { "light" };

    rsx! {
        div {
            class: "app {theme_class}",
            style: "min-height: 100vh;",

            // ========================================================
            // 1. 顶部工具栏
            // ========================================================
            header { class: "toolbar",
                h1 { "📝 Markdown 编辑器" }

                div { class: "toolbar-actions",
                    span { class: "char-count", "字数：{content().chars().count()}" }
                    button {
                        class: "btn-icon",
                        onclick: move |_| dark_mode.toggle(),
                        if dark_mode() { "☀️" } else { "🌙" }
                    }
                }
            }

            // ========================================================
            // 2. ⭐ 双栏布局：左编辑 + 右预览（用 inline style 保证布局生效）
            // ========================================================
            div {
                style: "display: flex; height: calc(100vh - 53px);",

                // 左栏：编辑器
                div {
                    style: "flex: 1; border-right: 1px solid #e0e0e0; display: flex; flex-direction: column;",
                    div { class: "pane-header", "✏️ 编辑" }
                    textarea {
                        class: "editor-textarea",
                        style: "flex: 1;",
                        value: "{content}",
                        oninput: move |e| {
                            content.set(e.value());    // ⭐ 实时更新
                        },
                        placeholder: "在此输入 Markdown...",
                        spellcheck: "false",
                    }
                }

                // 右栏：预览
                div {
                    style: "flex: 1; display: flex; flex-direction: column;",
                    div { class: "pane-header", "👁️ 预览" }
                    div {
                        class: "preview-content",
                        style: "flex: 1; overflow-y: auto;",
                        // ⭐ 用 dangerous_inner_html 渲染 HTML
                        dangerous_inner_html: "{markdown_html}",
                    }
                }
            }
        }
    }
}
