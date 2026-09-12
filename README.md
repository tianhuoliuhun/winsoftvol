<div align="center">
  <img alt="WinSoftVol" src="assets/icon.svg" width="110">

  <h1>🔊 WinSoftVol（中文版）</h1>

  <p><b>让 USB 音频设备的系统音量控制正常工作</b></p>

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
- 🦀 Rust 编写：单文件绿色程序，无需安装、无需管理员权限

## 🌐 中文界面说明

本 fork 在原作者代码基础上加入了完整的本地化支持：

- 自动检测 Windows 显示语言：简体中文 → `zh-CN`，繁体中文 → `zh-TW`，其他 → English
- 也可以在托盘菜单「**语言**」中随时切换，或在 `config.toml` 中设置 `language = "zh-CN"`
- 翻译范围覆盖：托盘菜单、悬停提示、关于对话框、所有通知气泡

## 📦 获取与使用

1. 从 [Releases](https://github.com/tianhuoliuhun/winsoftvol/releases) 下载可执行文件（或自行构建，见下文）
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
cargo build --release
```

产物位于 `target/release/winsoftvol.exe`。

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
