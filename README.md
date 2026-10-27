# SnapRust - 零本地编译环境 GitHub Actions 云端自动打包指南

本项目已内置完整的 **GitHub Actions 自动云端编译流水线** (`.github/workflows/build.yml`)。你**不需要在自己的电脑上下载或安装 Rust、Node.js、Visual Studio C++ 或任何编译软件**，只需通过浏览器上传到 GitHub，云端服务器就会自动为你编译生成：
- **Windows 版**：单文件免安装版 `rust-tauri-snip.exe` + 安装包 `.msi` / `.exe`
- **macOS 版**：苹果安装包 `.dmg`
- **Linux 版**：`.deb` 与 `.AppImage`

---

## 三步完成云端自动打包（全程仅用浏览器）

### 第一步：在 GitHub 创建一个新仓库
1. 打开浏览器访问 [github.com/new](https://github.com/new)（如无账号先免费注册一个）。
2. **Repository name** 填写 `snaprust`（或其他任意英文名）。
3. 选择 **Public**（公开仓库可无限次免费使用 GitHub Actions 高性能编译服务器）。
4. 勾选 **Add a README file**，然后点击底部的绿色按钮 **Create repository**。

### 第二步：在网页端上传项目文件
1. 解压本压缩包 `rust_tauri_snip_project.zip`，你会看到里面的 `rust_tauri_snip` 文件夹。
2. 在你刚创建的 GitHub 仓库页面，点击顶部的 **Add file** -> **Upload files**。
3. 把 `rust_tauri_snip` 文件夹里的**所有内容**（`dist` 文件夹、`src-tauri` 文件夹、`.github` 文件夹、`package.json`）直接拖拽到浏览器上传框中，然后点击底部的绿色按钮 **Commit changes**。
   > **注意**：如果你的电脑系统默认隐藏了以点开头的 `.github` 文件夹，导致拖拽时没把 `.github` 传上去，你可以直接在仓库主页点击 **Add file** -> **Create new file**，在文件名输入框粘贴 `.github/workflows/build.yml`，然后把压缩包里的 `github-actions-build.yml` 文件内容粘贴进去并点击右上角 **Commit changes** 即可！

### 第三步：下载云端编译好的 `.exe` / `.dmg` 软件
1. 提交文件后，点击 GitHub 仓库顶部菜单栏的 **Actions** 选项卡。
2. 你会看到名为 **Build SnapRust Desktop Binaries** 的任务正在自动运行（黄色转圈图标，你也可以随时点击左侧任务名 -> 右侧 **Run workflow** 手动触发）。
3. 等待约 **5 ~ 8 分钟**，当任务变成绿色对勾 `✓` 后，点进这次运行记录，拉到页面最底部的 **Artifacts** 区域：
   - 点击 **`SnapRust-Windows-x64`** 即可下载 Windows `.exe` / `.msi` 程序！
   - 点击 **`SnapRust-macOS-Universal`** 即可下载 macOS `.dmg` 程序！
   - 点击 **`SnapRust-Linux-x64`** 即可下载 Linux `.deb` / `.AppImage` 程序！
