# 第 12 章：Markdown 编辑器（下）—— 打包发布 🎓 终章

> Dioxus 客户端课程第 12 章（实战项目第 3 部分 / 课程终章）
> 学习目标：掌握 Web 部署 + 桌面打包，把应用发布出去

---

## 12.1 开发 vs 发布

| | 开发模式 | 发布模式 |
|---|---------|---------|
| 命令 | `dx serve` | `dx bundle` |
| 优化 | 不优化（编译快） | 最大优化（体积小、运行快） |
| 热重载 | ✅ | ❌ |
| 产物 | 在内存中 | 生成文件（dist/） |

---

## 12.2 ⭐ Web 构建（部署到网站）

### 构建命令

```bash
cd ch11-markdown-editor-2/markdown_editor

# Web release 构建（生成优化的 WASM）
dx bundle --release --platform web
# 或
dx build --release --platform web
```

### 构建产物

```
dist/
├── index.html          ← HTML 入口
├── assets/             ← 静态资源
├── markdown_editor.js  ← JS 胶水代码
└── markdown_editor.wasm ← WebAssembly 二进制
```

### 部署方式

**方式 1：GitHub Pages（免费）**

```bash
# 1. 构建
dx bundle --release --platform web

# 2. 把 dist/ 内容推送到 GitHub Pages
cd dist
git init
git add -A
git commit -m "deploy"
git push origin gh-pages
```

**方式 2：Netlify / Vercel（拖拽部署）**

1. `dx bundle --release --platform web`
2. 把 `dist/` 文件夹拖到 [Netlify Drop](https://app.netlify.com/drop)

**方式 3：任何静态服务器**

```bash
cd dist
python -m http.server 8080
# 或
npx serve .
```

> 💡 Web 模式的产物就是**纯静态文件**（HTML + JS + WASM），任何能托管静态文件的地方都能部署。

---

## 12.3 ⭐ 桌面打包（生成 .exe / .dmg）

### 构建命令

```bash
# 桌面 release 打包
dx bundle --release --platform desktop
```

### 构建产物（Windows）

```
dist/markdown_editor/
├── markdown_editor.exe     ← 可执行文件
├── markdown_editor.msi     ← Windows 安装包
└── ...
```

### ⚠️ 桌面打包的限制

| 平台 | 能在哪打包 |
|------|----------|
| Windows (.msi/.exe) | 只能在 Windows 上打包 |
| macOS (.dmg/.app) | 只能在 macOS 上打包 |
| Linux (.deb/.AppImage) | 只能在 Linux 上打包 |

> **不能交叉编译**——在 Windows 上打不了 macOS 包。用 GitHub Actions 可以在 CI 里跨平台打包。

---

## 12.4 ⭐ release profile 优化

在 `Cargo.toml` 配置发布优化：

```toml
[profile.release]
opt-level = "z"      # 优化体积（WASM 更小）
lto = true            # 链接时优化
codegen-units = 1     # 单线程编译（更好的优化）
strip = true          # 移除调试符号
panic = "abort"       # panic 时直接终止（减小体积）
```

### Web 体积优化

WASM 体积很重要（影响加载速度）。优化措施：
1. `opt-level = "z"` 最小体积
2. `strip = true` 移除调试信息
3. `wasm-opt -Oz` 进一步优化（dx bundle 自动执行）
4. 启用 Brotli/Gzip 压缩（服务器端）

典型 WASM 体积：**100KB - 1MB**（取决于依赖）

---

## 12.5 GitHub Actions 自动发布（进阶）

在 `.github/workflows/release.yml` 配置自动打包：

```yaml
name: Release
on:
  push:
    tags: ['v*']

jobs:
  build-web:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo binstall dioxus-cli -y
      - run: dx bundle --release --platform web
      - uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./dist

  build-desktop:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo binstall dioxus-cli -y
      - run: dx bundle --release --platform desktop
      - uses: actions/upload-artifact@v3
        with:
          name: app-${{ matrix.os }}
          path: dist/
```

这样打 tag 时自动构建 3 个平台的桌面包 + Web 版。

---

## 12.6 ⭐ 完整发布流程

### Web 版发布

```
1. dx bundle --release --platform web
   → 生成 dist/ 目录

2. 部署 dist/ 到：
   - GitHub Pages（免费，适合个人项目）
   - Netlify/Vercel（免费，CDN 加速）
   - 自己的服务器

3. 用户通过浏览器访问
   → 加载 HTML + WASM → 运行
```

### 桌面版发布

```
1. dx bundle --release --platform desktop
   → 生成 .msi（Windows）/ .dmg（macOS）

2. 分发安装包：
   - GitHub Releases
   - S3 / 云存储
   - 应用商店

3. 用户下载安装
   → 像普通桌面应用一样运行
```

---

## 12.7 Markdown 编辑器最终功能清单 ✅

| 功能 | 状态 |
|------|------|
| 双栏布局（编辑+预览） | ✅ |
| 实时预览 | ✅ |
| Markdown 解析（pulldown-cmark） | ✅ |
| 暗色/亮度主题 | ✅ |
| 字数/行数统计 | ✅ |
| 自动保存（localStorage） | ✅ |
| 启动恢复 | ✅ |
| 导出 .md 文件 | ✅ |
| Web 构建 | ✅ |
| 桌面打包 | ✅ |

**你做了一个完整的、可发布的跨平台 Markdown 编辑器！** 🎉

---

## 📋 dx 命令速查

| 命令 | 作用 |
|------|------|
| `dx serve --platform web` | Web 开发（热重载） |
| `dx serve --platform desktop` | 桌面开发 |
| `dx build --release --platform web` | Web release 构建 |
| `dx bundle --release --platform web` | Web 打包（含优化） |
| `dx bundle --release --platform desktop` | 桌面打包 |
| `dx clean` | 清理构建产物 |

---

## 📝 提问与解答（Q&A）

### Q1：dx bundle 和 dx build 有什么区别？

**A：**
- `dx build`：编译项目（生成 WASM/二进制）
- `dx bundle`：编译 + 打包（生成可部署/安装的文件，如 .msi/.dmg）

日常用 `dx bundle`，它包含 build + 额外的打包步骤。

### Q2：为什么 wasm-opt 下载失败？

**A：** `wasm-opt` 是 WASM 优化工具，dx bundle 时自动下载。网络不稳定会失败。解决：
1. 重试（网络恢复后）
2. 手动安装：`npm install -g wasm-opt`
3. 跳过优化：构建时 wasm 已经可用，只是体积稍大

### Q3：Web 版的体积一般多大？

**A：** Dioxus Web 版典型体积 100KB-1MB（取决于依赖）。本章的 Markdown 编辑器约 300KB（含 pulldown-cmark）。用 gzip/brotli 压缩后传输更小。

---

## ✅ 第 12 章（终章）小结

学完本章你应该掌握：
1. ✅ 用 `dx bundle --release` 做发布构建
2. ✅ Web 版部署到 GitHub Pages / Netlify
3. ✅ 桌面版打包成 .msi / .dmg
4. ✅ 配置 release profile 优化体积
5. ✅ 了解 GitHub Actions 自动发布

---

## 📂 本章内容

本章是终章，不包含代码项目。Markdown 编辑器代码在 Ch10-Ch11。
