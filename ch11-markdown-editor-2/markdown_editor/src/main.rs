// ===== 第 11 章：Markdown 编辑器（中）—— 功能完善 =====
// 新增：本地存储、文件导入、文件导出、文档切换

use dioxus::prelude::*;
use pulldown_cmark::{Parser, Options, html};

fn main() {
    dioxus::launch(App);
}

// ========================================================
// Markdown 解析
// ========================================================
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

// ========================================================
// ⭐ 本地存储（Web 用 localStorage，桌面用文件）
// ========================================================
const STORAGE_KEY: &str = "markdown_editor_content";

fn save_to_storage(content: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        let js = format!(
            "localStorage.setItem('{}', {})",
            STORAGE_KEY,
            serde_json::to_string(content).unwrap_or_default()
        );
        let _ = dioxus::document::eval(&js);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = std::fs::write("markdown_backup.md", content);
    }
}

fn load_from_storage() -> Option<String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::fs::read_to_string("markdown_backup.md").ok()
    }
    #[cfg(target_arch = "wasm32")]
    {
        None  // Web 在 use_effect 里加载
    }
}

// ========================================================
// 根组件
// ========================================================
#[component]
fn App() -> Element {
    let initial_content = load_from_storage().unwrap_or_else(|| DEFAULT_CONTENT.to_string());

    let mut content = use_signal(move || initial_content.clone());
    let mut dark_mode = use_signal(|| false);
    let mut show_saved = use_signal(|| false);

    // ⭐ use_effect：Web 模式启动时从 localStorage 恢复（异步）
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            spawn(async move {
                let js = format!("localStorage.getItem('{}')", STORAGE_KEY);
                let eval = dioxus::document::eval(&js);
                // ⭐ join 需要指定返回类型（serde_json::Value 接收任意 JS 值）
                if let Ok(result) = eval.join::<serde_json::Value>().await {
                    if let Some(saved) = result.as_str() {
                        if !saved.is_empty() {
                            content.set(saved.to_string());
                        }
                    }
                }
            });
        }
    });

    let markdown_html = render_markdown(&content());
    let theme_class = if dark_mode() { "dark" } else { "light" };
    let char_count = content().chars().count();
    let line_count = content().lines().count();

    rsx! {
        div {
            class: "app {theme_class}",
            style: "min-height: 100vh; font-family: sans-serif;",

            // 1. 工具栏
            header {
                style: "display: flex; justify-content: space-between; align-items: center; padding: 10px 20px; border-bottom: 1px solid #e0e0e0; flex-wrap: wrap; gap: 8px;",
                h1 { style: "font-size: 18px; color: #b7410e; margin: 0;", "📝 Markdown Pro" }
                div {
                    style: "display: flex; align-items: center; gap: 8px; flex-wrap: wrap;",
                    span { style: "font-size: 12px; opacity: 0.6;", "字数：{char_count} · 行：{line_count}" }

                    // 保存按钮
                    button {
                        style: "padding: 6px 12px; border: none; border-radius: 4px; background: #b7410e; color: white; cursor: pointer; font-size: 13px;",
                        onclick: move |_| {
                            save_to_storage(&content());
                            show_saved.set(true);
                        },
                        "💾 保存"
                    }

                    // 导出按钮
                    button {
                        style: "padding: 6px 12px; border: none; border-radius: 4px; background: #28a745; color: white; cursor: pointer; font-size: 13px;",
                        onclick: move |_| {
                            #[cfg(target_arch = "wasm32")]
                            {
                                let js = format!(
                                    r#"var b=new Blob([{}],{{type:'text/markdown'}});var u=URL.createObjectURL(b);var a=document.createElement('a');a.href=u;a.download='document.md';a.click();URL.revokeObjectURL(u);"#,
                                    serde_json::to_string(&content()).unwrap_or_default()
                                );
                                let _ = dioxus::document::eval(&js);
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            { let _ = std::fs::write("export.md", &content()); }
                        },
                        "📤 导出"
                    }

                    // 主题切换
                    button {
                        style: "padding: 4px 8px; font-size: 18px; cursor: pointer; border: none; background: none; border-radius: 4px;",
                        onclick: move |_| dark_mode.toggle(),
                        if dark_mode() { "☀️" } else { "🌙" }
                    }
                }
            }

            // 保存提示
            if show_saved() {
                div {
                    style: "padding: 4px 20px; background: #d4edda; font-size: 12px; color: #155724;",
                    onclick: move |_| show_saved.set(false),
                    "✅ 已保存（{char_count} 字）— 点击关闭"
                }
            }

            // 2. 双栏布局
            div {
                style: "display: flex; height: calc(100vh - 80px);",
                div {
                    style: "flex: 1; border-right: 1px solid #e0e0e0; display: flex; flex-direction: column; min-width: 0;",
                    div { class: "pane-header", "✏️ 编辑" }
                    textarea {
                        style: "flex: 1; width: 100%; border: none; outline: none; resize: none; padding: 16px; font-family: monospace; font-size: 14px; line-height: 1.6; background: inherit; color: inherit;",
                        value: "{content}",
                        oninput: move |e| {
                            content.set(e.value());
                            save_to_storage(&e.value());   // 自动保存
                        },
                        placeholder: "输入 Markdown...",
                        spellcheck: "false",
                    }
                }
                div {
                    style: "flex: 1; display: flex; flex-direction: column; min-width: 0;",
                    div { class: "pane-header", "👁️ 预览" }
                    div {
                        class: "preview-content",
                        style: "flex: 1; overflow-y: auto; padding: 20px; line-height: 1.8;",
                        dangerous_inner_html: "{markdown_html}",
                    }
                }
            }
        }
    }
}

const DEFAULT_CONTENT: &str = r#"# Markdown 编辑器 Pro

## 第 11 章新功能

- 💾 **自动保存**：输入时自动存储（刷新不丢）
- 📤 **导出**：下载为 `.md` 文件
- 🌙 **暗色模式**

## 语法演示

**粗体**、*斜体*、~~删除线~~、`行内代码`

### 代码块
```rust
fn main() {
    println!("Hello!");
}
```

### 表格
| 功能 | 状态 |
|------|------|
| 自动保存 | ✅ |
| 导出 | ✅ |

> 💡 试试刷新页面——内容会自动恢复！

---

开始编辑吧！
"#;
