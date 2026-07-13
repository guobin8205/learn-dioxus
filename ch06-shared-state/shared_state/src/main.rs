// ===== 第 6 章：状态共享 =====
// 运行：dx serve --platform web
// 浏览器访问 http://127.0.0.1:8080

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

// ========================================================
// 问题：为什么需要状态共享？
// ========================================================
// 想象一个购物车应用：
//   ProductList 组件 → 需要「添加商品」
//   Cart 组件        → 需要「显示商品」
//   Navbar 组件      → 需要「显示数量」
//
// 如果把 cart 状态从 App 一层层传下去，叫「prop drilling」，很痛苦
// Context API 让任何子组件直接访问共享状态，不需要层层传递

// ========================================================
// ⭐ 方式 1：use_context_provider + use_context（推荐）
// ========================================================
// 在根组件提供状态，任何子组件都能用 use_context 消费

/// 应用全局状态
#[derive(Clone, Default)]
struct AppState {
    cart_count: Signal<i32>,
    user_name: Signal<String>,
    theme: Signal<String>,
}

#[component]
fn App() -> Element {
    // ⭐ 在根组件提供状态
    // 注意：不能在 use_context_provider 的闭包里调用 use_signal（违反 Hooks 规则）
    // 用 Signal::new 创建（等价于 use_signal）
    use_context_provider(|| AppState {
        cart_count: Signal::new(0),
        user_name: Signal::new("访客".to_string()),
        theme: Signal::new("light".to_string()),
    });

    rsx! {
        div { class: "container",
            h1 { "🔗 状态共享演示" }

            // 1. Navbar（用 use_context 读取）
            Navbar {}

            hr {}

            // 2. 商品列表（用 use_context 修改购物车）
            ProductGrid {}

            hr {}

            // 3. 主题切换器
            ThemeSwitcher {}

            hr {}

            // 4. 对比：不用 Context 的 prop drilling
            h2 { "对比：Prop Drilling（繁琐）" }
            PropDrillingExample {}
        }
    }
}

// ========================================================
// Navbar：消费 Context（读取购物车数量 + 用户名）
// ========================================================
#[component]
fn Navbar() -> Element {
    // ⭐ use_context 从父组件获取共享状态
    let state = use_context::<AppState>();

    // 读取状态的当前值
    let count = (state.cart_count)();
    let name = (state.user_name)();
    let bg = if (state.theme)() == "dark" { "#2c3e50" } else { "#ecf0f1" };
    let fg = if (state.theme)() == "dark" { "#ecf0f1" } else { "#2c3e50" };

    rsx! {
        nav { class: "navbar",
            style: "background: {bg}; color: {fg};",
            div { class: "navbar-content",
                span { "👤 {name}" }
                span { "🛒 购物车：{count}" }
            }
        }
    }
}

// ========================================================
// ProductGrid：消费 Context（修改购物车）
// ========================================================
#[component]
fn ProductGrid() -> Element {
    let mut state = use_context::<AppState>();

    let products = vec![
        ("Rust 书", 99.0),
        ("机械键盘", 599.0),
        ("咖啡杯", 39.0),
        ("显示器", 1999.0),
    ];

    rsx! {
        div { class: "card",
            h2 { "商品列表（点击添加到购物车）" }
            div { class: "grid",
                for (name, price) in products {
                    ProductCard {
                        name: name.to_string(),
                        price,
                        on_add: move |_| {
                            // ⭐ 修改共享状态（Navbar 自动更新）
                            state.cart_count += 1;
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn ProductCard(name: String, price: f64, on_add: EventHandler<()>) -> Element {
    rsx! {
        div { class: "product",
            h4 { "{name}" }
            p { "¥{price}" }
            button {
                onclick: move |_| on_add.call(()),
                "加入购物车"
            }
        }
    }
}

// ========================================================
// ThemeSwitcher：消费 Context（修改主题）
// ========================================================
#[component]
fn ThemeSwitcher() -> Element {
    let mut state = use_context::<AppState>();
    let current = (state.theme)();
    let is_dark = current == "dark";

    rsx! {
        div { class: "card",
            h2 { "主题切换（共享状态联动）" }
            p { "当前主题：{current}" }
            button {
                onclick: move |_| {
                    state.theme.set(if is_dark { "light".to_string() } else { "dark".to_string() });
                },
                if is_dark { "☀️ 切换到亮色" } else { "🌙 切换到暗色" }
            }
            p { class: "hint", "切换后 Navbar 的颜色也会变！" }
        }
    }
}

// ========================================================
// ⭐ 对比：Prop Drilling（不用 Context 的痛苦）
// ========================================================
// 状态从 App → Middle → Inner 层层传递

#[component]
fn PropDrillingExample() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div { class: "card",
            p { "App 层：{count}" }
            MiddleLayer { count, on_increment: move |_| count += 1 }
        }
    }
}

// 中间层：只是传递 count 和 on_increment，自己不用
#[component]
fn MiddleLayer(count: Signal<i32>, on_increment: EventHandler<()>) -> Element {
    rsx! {
        div { class: "nested",
            p { "↳ Middle 层（只是传递）：{count}" }
            InnerLayer { count, on_increment }
        }
    }
}

// 最内层：实际使用 count 和 on_increment
#[component]
fn InnerLayer(count: Signal<i32>, on_increment: EventHandler<()>) -> Element {
    rsx! {
        div { class: "nested",
            p { "↳↳ Inner 层（真正使用）：{count}" }
            button { onclick: move |_| on_increment.call(()), "+1" }
        }
    }
}

// ========================================================
// ⭐ Context 的核心价值
// ========================================================
// 没有 Context（prop drilling）：
//   App → Middle(传 count) → Inner(用 count)
//   每加一层中间组件，都要传递 count 和回调
//
// 有 Context：
//   App (provide) → ... → Inner (直接 use_context)
//   中间任何层都不需要知道 count 的存在
//
// 类比 React 的 Context API：
//   <App><Context.Provider value={state}>...</Context.Provider></App>
//   const state = useContext(AppContext)
