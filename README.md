<div align="center">
  <img alt="软音桥 WinSoftVol" src="assets/icon.svg?v=2" width="110">

  <h1>🔊 软音桥 · WinSoftVol</h1>

  <p><b>让 USB 音频设备的系统音量控制正常工作</b></p>

  <p><i>中文名「软音桥」——把系统音量「桥」接到软件层</i></p>

  <p>
    <a href="https://opensource.org/licenses/MIT">
      <img src="https://img.shields.io/badge/License-MIT-brightgreen.svg" alt="License: MIT">
    </a>
    <a href="https://img.shields.io/badge/platform-Windows-blue">
      <img src="https://img.shields.io/badge/platform-Windows-blue" alt="Platform: Windows">
    </a>
  </p>
</div>

---

> **源码来源**：本项目是 [**jeffreytse/winsoftvol**](https://github.com/jeffreytse/winsoftvol) 的第三方 fork，由 [tianhuoliuhun](https://github.com/tianhuoliuhun) 维护，在**原作者代码**的基础上增加了简体中文 / 繁體中文界面等改进。原项目由 **Jeffrey Tse（[@jeffreytse](https://github.com/jeffreytse)）** 开发，遵循 MIT 许可证，本 fork 保留原作者的全部版权声明。

---

## 🤔 这是什么？

你有没有遇到过这样的情况：插上一个 USB 声卡、DAC 或 Type-C 耳机转接头（比如使用 Realtek ALC5686 芯片的转接头），播放音乐一切正常，但拖动 Windows 音量滑块——数字在变、滑块在动，**声音却一点不变**；按键盘音量键、静音键也毫无反应。

本 fork 的作者正是在 **ASUS 华硕 ROG 游戏手机 3 配机的 Type-C 转 3.5mm 耳机转接头**（Windows 中显示为 `ASUS USB2.0 Audio`）上遇到并解决了这个问题，详见下文「已验证适配设备」。

原因是：Windows 把音量变化写进了音频端点（endpoint），期望**硬件**去执行；而很多 USB 音频设备并没有实现硬件音量控制（Feature Unit），驱动就默默忽略了这些请求。

**WinSoftVol 解决这个问题**：它常驻系统托盘，监听端点音量变化，并立即把它作为**软件音量**（会话音量）应用到所有正在播放的应用上。于是——

- 任务栏音量滑块 ✅
- 键盘音量键 / 静音键 ✅
- 每个应用独立的音量（会话音量）✅

全部恢复正常工作。

## ✨ 功能特性

- 🎚️ 拦截系统音量变化（任务栏滑块 / 键盘音量键 / 静音键），以软件音量应用到所有音频会话
- 🔇 **强制软件音量模式**：对有硬件音量的设备也可强制走软件衰减
- 🔒 音量上限：限制最大输出，可配置多个预设
- 🌙 夜间模式：定时自动降低音量上限
- 🔈 启动音量：每次启动时设定固定音量
- 📌 设备绑定：把程序锁定到指定音频设备
- 🎛️ **设备允许列表**（本 fork 新增）：多选，只在指定设备上生效
- 🛡️ **单实例保护**（本 fork 新增）：防止多开导致音量缩放翻倍
- 🌐 **多语言界面**（本 fork 新增）：简体中文 / 繁體中文 / English
- ⚙️ 配置文件热重载（`%APPDATA%\WinSoftVol\config.toml`，改完即生效）
- 🔔 自动检查新版本
- ⚠️ 独占模式检测（游戏 / DAW 绕过混音器时通知）
- 🖱️ 托盘滚轮调音量、左键静音、动态音量条图标（静音变红）
- 🖥️ 支持 **x86（32 位）**、**x64** 与 **ARM64**（Windows on ARM）三种架构
- 🦀 Rust 编写：单文件绿色程序，无需安装、无需管理员权限

## 🌐 中文界面说明

本 fork 在原作者代码基础上加入了完整的本地化支持：

- 自动检测 Windows 显示语言：简体中文 → `zh-CN`，繁体中文 → `zh-TW`，其他 → English
- 也可以在托盘菜单「**语言**」中随时切换，或在 `config.toml` 中设置 `language = "zh-CN"`
- 翻译范围覆盖：托盘菜单、悬停提示、关于对话框、所有通知气泡

## 🎧 已验证适配设备

### ASUS 华硕 ROG 游戏手机 3 配机 Type-C 转 3.5mm 耳机转接头（DAC 转接器 / 音频转接线）

本 fork 正是在这款设备上实测开发并验证的：

| 项目 | 说明 |
| --- | --- |
| Windows 设备名 | **ASUS USB2.0 Audio** |
| 主控芯片 | Realtek ALC5686（USB `VID_0BDA&PID_4BAF`） |
| 典型问题 | 插入电脑后 Windows 默认走**硬件音量**，任务栏滑块 / 键盘音量键调节无效或表现异常 |
| 解决效果 | 启用 WinSoftVol 后，音量滑块、键盘音量键、静音键全部恢复正常（软件音量） |

推荐配置（只对该转接头生效，不影响其它音频设备）：

```toml
[general]
devices = ["耳机 (ASUS USB2.0 Audio)"]
```

> 💡 若设备名称不完全一致，请在 Windows「声音设置」中查看实际显示的名称后填入 `devices`，或直接在托盘菜单「设备」中勾选。

如果你也使用同款 **ROG Phone 3 配机转接线**，或其它 **Realtek ALC5686 / ALC4050** 方案的 USB-C 转 3.5mm DAC 转接器、音频转接线，欢迎反馈使用体验。

## 🖥️ 系统要求

- Windows 10 或更高版本
- 架构：**x86（32 位）** / **x64** / **ARM64**（Windows on ARM 原生运行）

## 📦 获取与使用

1. 从 [Releases](https://github.com/tianhuoliuhun/winsoftvol/releases) 下载对应架构的可执行文件：
   - **x64**：`winsoftvol-vX.Y.Z-<hash>.exe`
   - **x86（32 位）**：`winsoftvol-vX.Y.Z-<hash>-x86.exe`
   - **ARM64**：`winsoftvol-vX.Y.Z-<hash>-arm64.exe`
2. 运行后系统托盘出现喇叭图标
3. 右键托盘图标打开菜单，所有选项一目了然

> 程序为绿色单文件，不需要安装；首次运行若遇 SmartScreen 提示，选择"更多信息 → 仍要运行"即可。

## ⚙️ 配置文件示例

```toml
[general]
autostart = true                          # 开机自启动
language = "zh-CN"                        # 界面语言：en / zh-CN / zh-TW
devices = ["耳机 (ASUS USB2.0 Audio)"]    # 只对这些设备生效；省略 = 全部设备
cap_presets = [100, 80, 60, 40]           # 音量上限预设
scroll_step_percent = 2                   # 滚轮步长（1-20）
night_start = "22:00"                     # 夜间模式开始时间
night_end = "07:00"                       # 夜间模式结束时间
night_cap = 40                            # 夜间音量上限

[default]
force_sw_volume = false                   # 强制软件音量
cap_percent = 100                         # 默认音量上限

[device."设备名"]                          # 按设备覆盖配置
force_sw_volume = true
cap_percent = 60
```

## 🛠️ 构建

需要 Rust（stable）：

```sh
# x64
cargo build --release

# x86（32 位）
rustup target add i686-pc-windows-msvc
cargo build --release --target i686-pc-windows-msvc

# ARM64（在 x64 主机上交叉编译）
rustup target add aarch64-pc-windows-msvc
cargo build --release --target aarch64-pc-windows-msvc
```

产物分别位于 `target/release/winsoftvol.exe`（x64）、`target/i686-pc-windows-msvc/release/winsoftvol.exe`（x86）和 `target/aarch64-pc-windows-msvc/release/winsoftvol.exe`（ARM64）。

## 📖 源码来源与致谢

| | |
|---|---|
| **原项目** | [jeffreytse/winsoftvol](https://github.com/jeffreytse/winsoftvol) |
| **原作者** | Jeffrey Tse（[@jeffreytse](https://github.com/jeffreytse)） |
| **本 fork** | [tianhuoliuhun/winsoftvol](https://github.com/tianhuoliuhun/winsoftvol) |
| **许可证** | MIT License（保留原作者版权声明，详见 [LICENSE](LICENSE)） |

感谢原作者开发了这个优秀的工具。本 fork 的改进（多语言、设备允许列表、单实例保护等）也已整理为 PR 提交给上游项目。

## 🌈 许可证

[MIT License](LICENSE) © Jeffrey Tse（原作者）

本 fork 遵循相同的 MIT 许可证，并完整保留原作者的版权声明。
