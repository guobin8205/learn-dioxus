# Dioxus 跨平台客户端开发课程

> 基于 Dioxus 0.7，一套 Rust 代码同时跑 Web、桌面、移动、TUI
> 前置课程：[learn-rust](https://github.com/guobin8205/learn-rust)（Rust 基础 20 章）

## 📚 课程目标

用 Dioxus 框架开发跨平台客户端应用：
- 🖥️ **桌面应用**（Windows/macOS/Linux）
- 🌐 **Web 前端**（WebAssembly）
- 📱 **移动应用**（iOS/Android，实验性）
- 📟 **终端 TUI**

一套 Rust 代码，多端运行。

---

## 📖 课程大纲（12 章 / 4 阶段）

### 阶段一：入门（Ch1-3）
| 章节 | 内容 |
|------|------|
| Ch1 | 环境搭建与第一个应用（dx CLI、Web/桌面启动） |
| Ch2 | RSX 语法（类 JSX 的 Rust UI 语法） |
| Ch3 | 组件与 Props（函数组件、属性、children） |

### 阶段二：状态管理（Ch4-6）
| 章节 | 内容 |
|------|------|
| Ch4 | Hooks 状态管理（use_signal、use_resource） |
| Ch5 | 事件处理（onclick、oninput、表单） |
| Ch6 | 状态共享（use_context、全局状态） |

### 阶段三：工程化（Ch7-9）
| 章节 | 内容 |
|------|------|
| Ch7 | 路由（多页面导航，dioxus-router） |
| Ch8 | 样式与资源（CSS、图片、Tailwind） |
| Ch9 | 平台适配（Web vs 桌面差异、条件编译） |

### 阶段四：实战项目（Ch10-12）
| 章节 | 内容 |
|------|------|
| Ch10 | 工具应用（上）：UI 搭建、核心功能 |
| Ch11 | 工具应用（中）：数据持久化、文件操作 |
| Ch12 | 工具应用（下）：打包发布、多端部署 |

---

## 🛠️ 环境要求

- Rust 工具链（rustc 1.96+）
- Dioxus CLI：`cargo binstall dioxus-cli`
- Node.js（Web 构建）
- 平台依赖（桌面/移动各自的 SDK）

## 🚀 快速开始

```bash
# 进入对应章节
cd ch01-getting-started/my_first_app

# Web 模式运行（热重载）
dx serve --platform web

# 桌面模式运行
dx serve --platform desktop
```

## 📁 目录结构

```
dioxus-client/
├── README.md
├── ch01-getting-started/
├── ch02-rsx/
├── ch03-components/
├── ...
└── ch12-project-deploy/
```

## 📝 学习方式

- 每章有 `notes.md`（笔记 + Q&A）
- 每章有可运行的项目代码
- 类比 React 概念（你有前端基础，上手快）
