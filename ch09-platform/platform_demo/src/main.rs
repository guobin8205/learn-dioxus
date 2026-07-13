// ===== 第 9 章：平台适配 =====
// 运行：
//   dx serve --platform web       → Web 模式
//   dx serve --platform desktop   → 桌面模式
// 同一份代码，自动适配不同平台

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

// ========================================================
// 1. ⭐ 编译期检测平台：#[cfg(target_arch = "wasm32")]
// ========================================================
// wasm32 → Web 模式（编译成 WebAssembly）
// 非 wasm32 → 桌面模式（编译成原生二进制）

/// 返回当前平台名称
fn platform_name() -> String {
    #[cfg(target_arch = "wasm32")]
    { "Web（WebAssembly）".to_string() }

    #[cfg(not(target_arch = "wasm32"))]
    { "桌面（原生）".to_string() }
}

/// ⭐ 平台特定功能：用 cfg 实现不同逻辑
fn read_file_example() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        // Web 模式：不能直接读文件（浏览器沙箱限制）
        "Web 模式不支持直接读文件（用 fetch API 或用户上传）".to_string()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // 桌面模式：可以读文件
        match std::fs::read_to_string("test.txt") {
            Ok(content) => format!("读到文件内容：{}", &content[..content.len().min(50)]),
            Err(_) => "桌面模式：文件不存在，但可以读（这里没创建）".to_string(),
        }
    }
}

// ========================================================
// 2. ⭐ 运行时检测平台（dioxus::document）
// ========================================================
// 有时候需要在运行时判断（比如根据设备特性调整 UI）

fn get_user_agent() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        // Web 模式可以读 navigator.userAgent
        "浏览器（可读 User-Agent）".to_string()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        "桌面应用（无 User-Agent）".to_string()
    }
}

#[component]
fn App() -> Element {
    let platform = platform_name();
    let file_info = read_file_example();
    let ua = get_user_agent();

    rsx! {
        div { class: "container",
            h1 { "🖥️ 平台适配演示" }
            p { class: "hint", "同一份代码，用 dx serve --platform web/desktop 切换" }

            // ========================================================
            // 1. 显示当前平台
            // ========================================================
            div { class: "card",
                h2 { "1. 当前运行平台" }
                p { style: "font-size: 20px; font-weight: bold; color: #b7410e;",
                    "▶ {platform}"
                }
                p { class: "hint", "↑ 根据这个判断代码跑在哪个平台" }
            }

            // ========================================================
            // 2. ⭐ 平台特定功能
            // ========================================================
            div { class: "card",
                h2 { "2. 平台特定功能" }

                div { class: "feature-row",
                    span { "📄 文件读取：" }
                    span { "{file_info}" }
                }

                div { class: "feature-row",
                    span { "🌐 User-Agent：" }
                    span { "{ua}" }
                }

                div { class: "feature-row",
                    span { "💾 本地存储：" }
                    {platform_feature_status("localStorage")}
                }

                div { class: "feature-row",
                    span { "🖥️ 窗口控制：" }
                    {platform_feature_status("window_control")}
                }
            }

            // ========================================================
            // 3. ⭐ 条件渲染（不同平台显示不同 UI）
            // ========================================================
            div { class: "card",
                h2 { "3. 条件渲染（不同平台不同 UI）" }

                // 编译期条件：只在该平台编译时包含
                {platform_specific_ui()}
            }

            // ========================================================
            // 4. 平台能力对比
            // ========================================================
            div { class: "card",
                h2 { "4. 平台能力对比" }

                table { style: "width: 100%; border-collapse: collapse;",
                    tr { style: "background: #f0f0f0;",
                        th { style: "padding: 8px; border: 1px solid #ddd;", "功能" }
                        th { style: "padding: 8px; border: 1px solid #ddd;", "Web" }
                        th { style: "padding: 8px; border: 1px solid #ddd;", "桌面" }
                    }
                    tr {
                        td { style: "padding: 8px; border: 1px solid #ddd;", "文件读写" }
                        td { style: "padding: 8px; border: 1px solid #ddd;", "❌ 受限" }
                        td { style: "padding: 8px; border: 1px solid #ddd;", "✅ 完全支持" }
                    }
                    tr {
                        td { style: "padding: 8px; border: 1px solid #ddd;", "网络请求" }
                        td { style: "padding: 8px; border: 1px solid #ddd;", "✅ fetch" }
                        td { style: "padding: 8px; border: 1px solid #ddd;", "✅ reqwest" }
                    }
                    tr {
                        td { style: "padding: 8px; border: 1px solid #ddd;", "系统通知" }
                        td { style: "padding: 8px; border: 1px solid #ddd;", "⚠️ 需权限" }
                        td { style: "padding: 8px; border: 1px solid #ddd;", "✅ 原生" }
                    }
                    tr {
                        td { style: "padding: 8px; border: 1px solid #ddd;", "数据库" }
                        td { style: "padding: 8px; border: 1px solid #ddd;", "⚠️ IndexedDB" }
                        td { style: "padding: 8px; border: 1px solid #ddd;", "✅ SQLite" }
                    }
                }
            }

            // ========================================================
            // 5. ⭐ 平台适配最佳实践
            // ========================================================
            div { class: "card",
                h2 { "5. 适配最佳实践" }

                div { class: "practice",
                    "✅ 1. 大部分业务逻辑写成平台无关的代码"
                }
                div { class: "practice",
                    "✅ 2. 平台特定代码用 #[cfg(...)] 隔离"
                }
                div { class: "practice",
                    "✅ 3. 用抽象层封装平台差异（如存储接口）"
                }
                div { class: "practice",
                    "✅ 4. 桌面端可以访问完整 Rust 生态"
                }
                div { class: "practice",
                    "✅ 5. Web 端受浏览器沙箱限制（文件、网络等）"
                }
            }
        }
    }
}

/// 根据平台返回功能状态
fn platform_feature_status(feature: &str) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        match feature {
            "localStorage" => "✅ 支持（localStorage/IndexedDB）".to_string(),
            "window_control" => "❌ 不支持（浏览器管窗口）".to_string(),
            _ => "未知".to_string(),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        match feature {
            "localStorage" => "✅ 支持（可用本地文件/数据库）".to_string(),
            "window_control" => "✅ 支持（可以控制窗口大小/位置）".to_string(),
            _ => "未知".to_string(),
        }
    }
}

/// ⭐ 不同平台返回不同的 UI 片段
fn platform_specific_ui() -> Element {
    #[cfg(target_arch = "wasm32")]
    {
        // Web 平台：显示分享按钮
        rsx! {
            div {
                p { "🌐 Web 版本特有：" }
                button { "🔗 分享到社交媒体" }
                button { "📋 复制链接" }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // 桌面平台：显示窗口控制
        rsx! {
            div {
                p { "🖥️ 桌面版本特有：" }
                button { "📁 打开本地文件" }
                button { "⚙️ 系统设置" }
            }
        }
    }
}
