// ===== 第 7 章：路由 =====
// 运行：dx serve --platform web
// 浏览器访问 http://127.0.0.1:8080

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

// ========================================================
// 1. ⭐ 定义路由：用 enum + #[derive(Routable)]
// ========================================================
// 这是 Dioxus 的类型安全路由——编译器保证所有路由都被处理
// 类比 React Router 的 <Routes>，但用 Rust 的 enum 更安全

#[derive(Clone, Routable, PartialEq, Debug)]
enum Route {
    // 首页
    #[route("/")]
    Home {},

    // 关于页面
    #[route("/about")]
    About {},

    // ⭐ 带参数的路由（URL 里的动态部分）
    // 比如 /user/123 → user_id = 123
    #[route("/user/:id")]
    User { id: i32 },

    // 404 页面（找不到路由时）
    #[route("/:route")]
    NotFound { route: String },
}

// ========================================================
// 2. 根组件：用 Router::<Route> 渲染
// ========================================================
#[component]
fn App() -> Element {
    rsx! {
        div { class: "container",
            // ⭐ Router 组件：根据当前 URL 渲染对应页面
            // 它会读取浏览器 URL，匹配 Route enum 的变体
            Router::<Route> {}
        }
    }
}

// ========================================================
// 3. ⭐ 布局组件：用 #[layout] 给所有页面加导航栏
// ========================================================
// 标注 #[layout(组件名)] 的路由，会在页面内容外包一层布局
// 所有路由页面都会自动有 Navbar

#[component]
fn Navbar() -> Element {
    rsx! {
        nav { class: "navbar",
            // ⭐ Link 组件：点击跳转（类比 React Router 的 <Link>）
            Link { to: Route::Home {}, "🏠 首页" }
            Link { to: Route::About {}, "ℹ️ 关于" }
            Link { to: Route::User { id: 1 }, "👤 用户1" }
            Link { to: Route::User { id: 2 }, "👤 用户2" }
        }
    }
}

// ========================================================
// 4. 各个页面组件
// ========================================================

// 首页
#[component]
fn Home() -> Element {
    rsx! {
        div { class: "page",
            h1 { "🏠 首页" }
            p { "欢迎来到 Dioxus 路由演示！" }
            p { "点击上方导航栏切换页面" }

            div { class: "card",
                h3 { "本演示包含：" }
                ul {
                    li { "基本路由（首页/关于）" }
                    li { "带参数的路由（/user/:id）" }
                    li { "404 页面" }
                    li { "编程式导航（use_navigator）" }
                }
            }

            // ⭐ 编程式导航按钮
            GotoAboutButton {}
        }
    }
}

// 关于页面
#[component]
fn About() -> Element {
    rsx! {
        div { class: "page",
            h1 { "ℹ️ 关于" }
            p { "这是一个 Dioxus 路由演示应用" }
            p { "Dioxus Router 提供类型安全的路由" }

            div { class: "card",
                h3 { "路由特性：" }
                ul {
                    li { "✅ 类型安全（enum 保证路由合法）" }
                    li { "✅ 带参数路由（/user/:id）" }
                    li { "✅ 嵌套布局（所有页面共享 Navbar）" }
                    li { "✅ Link 组件（声明式导航）" }
                    li { "✅ use_navigator（编程式导航）" }
                }
            }
        }
    }
}

// ⭐ 用户页面：带参数（从 URL 读取 id）
#[component]
fn User(id: i32) -> Element {
    rsx! {
        div { class: "page",
            h1 { "👤 用户详情" }
            p { "用户 ID：{id}" }

            div { class: "card",
                h3 { "模拟用户信息" }
                p { "姓名：用户{id}" }
                p { "邮箱：user{id}@example.com" }
            }

            // 跳转到其他用户的链接
            p { "查看其他用户：" }
            div { class: "link-group",
                Link { to: Route::User { id: id - 1 }, class: "link-btn", "← 用户{id - 1}" }
                Link { to: Route::User { id: id + 1 }, class: "link-btn", "用户{id + 1} →" }
            }

            br {}
            Link { to: Route::Home {}, "← 返回首页" }
        }
    }
}

// 404 页面
#[component]
fn NotFound(route: String) -> Element {
    rsx! {
        div { class: "page not-found",
            h1 { "404" }
            p { "页面不存在：/{route}" }
            Link { to: Route::Home {}, "← 返回首页" }
        }
    }
}

// ========================================================
// 5. ⭐ 编程式导航：use_navigator
// ========================================================
// 除了用 Link，还可以用代码控制跳转（比如表单提交后跳转）

#[component]
fn GotoAboutButton() -> Element {
    rsx! {
        div { class: "card",
            p { "编程式导航演示：" }
            button {
                class: "btn",
                onclick: move |_| {
                    // ⭐ 用 use_navigator 获取导航器
                    let nav = navigator();
                    nav.push(Route::About {});
                },
                "点我跳转到关于页面 →"
            }
        }
    }
}
