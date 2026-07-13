// ===== 第 3 章：组件与 Props =====
// 运行：dx serve --platform web
// 浏览器访问 http://127.0.0.1:8080

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

// ========================================================
// 1. 最简单的组件：无 props
// ========================================================
// Dioxus 组件就是一个返回 Element 的函数
// 类比 React：function Header() { return <h1>...</h1>; }
#[component]
fn Header() -> Element {
    rsx! {
        header {
            h1 { "🦀 Dioxus 组件演示" }
            p { "学习如何拆分和复用组件" }
        }
    }
}

// ========================================================
// 2. ⭐ 带 Props 的组件
// ========================================================
// Props 用 #[component] 宏自动生成
// 类比 React：function Greeting({ name, age }) { ... }

#[component]
fn Greeting(name: String, age: i32) -> Element {
    rsx! {
        div { class: "card",
            h3 { "你好，{name}！" }
            p { "年龄：{age} 岁" }
        }
    }
}

// ========================================================
// 3. Props 带默认值（用 Option + 内部处理）
// ========================================================
// Dioxus 0.7 的 #[default] 只支持字面量
// 对于字符串默认值，推荐用 Option<String> + 内部处理
#[component]
fn Badge(text: String, color: Option<String>) -> Element {
    let c = color.unwrap_or_else(|| "blue".to_string());
    let style_str = format!("background: {}; color: white; padding: 4px 8px; border-radius: 4px;", c);
    rsx! {
        span { style: "{style_str}", "{text}" }
    }
}

// ========================================================
// 4. ⭐ 可选 Props（Option<T>）
// ========================================================
#[component]
fn UserCard(name: String, email: Option<String>) -> Element {
    rsx! {
        div { class: "card",
            h3 { "👤 {name}" }
            // 处理可选的 email
            {match email {
                Some(addr) => rsx! { p { "📧 {addr}" } },
                None => rsx! { p { class: "muted", "📧 无邮箱" } },
            }}
        }
    }
}

// ========================================================
// 5. children：组件嵌套子内容（类比 React 的 props.children）
// ========================================================
#[component]
fn Card(title: String, children: Element) -> Element {
    rsx! {
        div { class: "card",
            h3 { "{title}" }
            div { class: "card-content",
                {children}    // ⭐ 渲染子内容
            }
        }
    }
}

// ========================================================
// 6. 组件复用：用循环生成多个组件
// ========================================================
#[component]
fn ProductList() -> Element {
    // 产品数据
    struct Product { name: String, price: f64 }
    let products = vec![
        Product { name: "Rust 书".to_string(), price: 99.0 },
        Product { name: "机械键盘".to_string(), price: 599.0 },
        Product { name: "咖啡杯".to_string(), price: 39.0 },
    ];

    rsx! {
        div {
            h3 { "商品列表" }
            div { class: "product-grid",
                for p in products {
                    ProductItem { name: p.name, price: p.price }
                }
            }
        }
    }
}

#[component]
fn ProductItem(name: String, price: f64) -> Element {
    rsx! {
        div { class: "product-item",
            h4 { "{name}" }
            p { class: "price", "¥{price}" }
            button { "加入购物车" }
        }
    }
}

// ========================================================
// 根组件：组合使用所有组件
// =================================================-------
fn App() -> Element {
    rsx! {
        div { class: "container",
            // 1. 无 props 组件
            Header {}

            hr {}

            // 2. 带 props 的组件
            h2 { "带 Props 的组件" }
            Greeting { name: "Alice".to_string(), age: 25 }
            Greeting { name: "Bob".to_string(), age: 30 }

            hr {}

            // 3. 带默认值的组件
            h2 { "默认值 Props" }
            Badge { text: "Rust".to_string() }
            {" "}
            Badge { text: "Dioxus".to_string(), color: "red".to_string() }

            hr {}

            // 4. 可选 props
            h2 { "可选 Props" }
            UserCard { name: "张三".to_string(), email: Some("zhangsan@example.com".to_string()) }
            UserCard { name: "李四".to_string() }   // 不传 email

            hr {}

            // 5. children 组件
            h2 { "children 组件" }
            Card { title: "关于我".to_string(),
                p { "我是一个 Rust 开发者" }
                p { "正在学习 Dioxus" }
                button { "关注" }
            }

            Card { title: "项目介绍".to_string(),
                p { "这是一个跨平台项目" }
                ul {
                    li { "Web" }
                    li { "桌面" }
                    li { "移动" }
                }
            }

            hr {}

            // 6. 组件复用
            h2 { "组件复用" }
            ProductList {}
        }
    }
}
