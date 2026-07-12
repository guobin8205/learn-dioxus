// ===== 第 1 章：第一个 Dioxus 应用 =====
// 运行方式：
//   dx serve --platform web       → Web 模式（浏览器访问）
//   dx serve --platform desktop   → 桌面模式（弹出窗口）

use dioxus::prelude::*;

fn main() {
    // ⭐ dioxus::launch 是入口函数
    // 它会根据 Cargo.toml 的 features 自动选择对应的渲染器
    // 类比 React：相当于 ReactDOM.createRoot(document.getElementById('root')).render(<App />)
    dioxus::launch(App);
}

/// 根组件 App
/// Dioxus 组件就是一个返回 Element 的普通函数
/// 类比 React：function App() { return <h1>Hello</h1>; }
fn App() -> Element {
    // ⭐ rsx! 宏：声明 UI 结构，类似 JSX
    // 注意：rsx! 里的标签不是字符串，是真正的 Rust 代码（宏展开后）
    rsx! {
        // 类 HTML 的写法，但用的是 Rust 语法
        h1 { "Hello, Dioxus!" }
        p { "这是我的第一个 Dioxus 应用 🦀" }

        // 按钮（下一章会讲事件处理）
        button {
            "点我"
        }
    }
}
