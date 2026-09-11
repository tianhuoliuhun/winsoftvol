//! Lightweight built-in localization.
//!
//! The language is resolved once at startup: an explicit `language` setting in
//! config.toml takes precedence, otherwise the Windows UI language is detected
//! and English is used as the fallback.

use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    En,
    ZhCn,
    ZhTw,
}

const LANG_EN: u8 = 0;
const LANG_ZH_CN: u8 = 1;
const LANG_ZH_TW: u8 = 2;

static CURRENT: AtomicU8 = AtomicU8::new(LANG_EN);

impl Lang {
    /// Parse a language code such as `en`, `zh-CN`, `zh-Hans`, `zh-TW`, `zh-Hant`.
    pub fn from_code(code: &str) -> Option<Self> {
        let normalized = code.trim().to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "en" | "en-us" | "en-gb" => Some(Lang::En),
            "zh" | "zh-cn" | "zh-hans" | "zh-sg" => Some(Lang::ZhCn),
            "zh-tw" | "zh-hk" | "zh-mo" | "zh-hant" => Some(Lang::ZhTw),
            _ => None,
        }
    }

    /// Detect the language from the Windows UI language, falling back to English.
    pub fn detect() -> Self {
        #[cfg(windows)]
        {
            use windows::Win32::Globalization::GetUserDefaultUILanguage;
            let langid = unsafe { GetUserDefaultUILanguage() };
            match langid {
                0x0804 | 0x1004 => Lang::ZhCn,          // zh-CN / zh-SG
                0x0404 | 0x0C04 | 0x1404 => Lang::ZhTw, // zh-TW / zh-HK / zh-MO
                _ => Lang::En,
            }
        }
        #[cfg(not(windows))]
        {
            Lang::En
        }
    }

    /// Translation table for this language.
    pub fn strings(self) -> &'static Strings {
        match self {
            Lang::En => &EN,
            Lang::ZhCn => &ZH_CN,
            Lang::ZhTw => &ZH_TW,
        }
    }

    fn as_u8(self) -> u8 {
        match self {
            Lang::En => LANG_EN,
            Lang::ZhCn => LANG_ZH_CN,
            Lang::ZhTw => LANG_ZH_TW,
        }
    }

    fn from_u8(value: u8) -> Self {
        match value {
            LANG_ZH_CN => Lang::ZhCn,
            LANG_ZH_TW => Lang::ZhTw,
            _ => Lang::En,
        }
    }
}

/// Store the active language for this process.
pub fn set(lang: Lang) {
    CURRENT.store(lang.as_u8(), Ordering::Relaxed);
}

/// The active language.
pub fn lang() -> Lang {
    Lang::from_u8(CURRENT.load(Ordering::Relaxed))
}

/// Resolve the language to use, given an optional configured code.
pub fn resolve(configured: Option<&str>) -> Lang {
    configured
        .and_then(Lang::from_code)
        .unwrap_or_else(Lang::detect)
}

/// Translation table for the active language.
pub fn strings() -> &'static Strings {
    lang().strings()
}

/// All user-visible strings for one language.
///
/// Templates use `{placeholder}` markers which are filled by the helper
/// methods below (see [`Strings::tooltip`], [`Strings::about_body`], ...).
pub struct Strings {
    // Tray menu
    pub menu_about: &'static str,
    pub menu_autostart: &'static str,
    pub menu_softvol: &'static str,
    pub menu_night: &'static str,
    pub menu_volcap: &'static str,
    pub menu_startup_vol: &'static str,
    pub menu_off: &'static str,
    pub menu_quit: &'static str,

    // Tray tooltip
    pub tooltip_active: &'static str,
    pub tooltip_volume: &'static str,
    pub tooltip_muted: &'static str,

    // About dialog
    pub about_title: &'static str,
    pub about_body: &'static str,
    pub about_update: &'static str,

    // Toast notifications
    pub notif_title_exclusive: &'static str,
    pub notif_reconnected: &'static str,
    pub notif_exclusive_start: &'static str,
    pub notif_exclusive_end: &'static str,
    pub notif_title_config_error: &'static str,
    pub notif_config_error: &'static str,
    pub notif_title_device_not_found: &'static str,
    pub notif_device_not_found: &'static str,
    pub notif_title_update: &'static str,
    pub notif_update: &'static str,
}

impl Strings {
    /// Tooltip text, e.g. `75% | cap: 100%`.
    pub fn tooltip(&self, pct: u32, cap: u32, muted: bool) -> String {
        let template = if muted {
            self.tooltip_muted
        } else {
            self.tooltip_volume
        };
        fill(template, &[("pct", &pct.to_string()), ("cap", &cap.to_string())])
    }

    /// Content of the About dialog.
    #[allow(clippy::too_many_arguments)]
    pub fn about_body(
        &self,
        version: &str,
        hash: &str,
        description: &str,
        authors: &str,
        build_time: &str,
        homepage: &str,
        sponsor: &str,
        update_line: &str,
    ) -> String {
        fill(
            self.about_body,
            &[
                ("version", version),
                ("hash", hash),
                ("desc", description),
                ("authors", authors),
                ("build", build_time),
                ("homepage", homepage),
                ("sponsor", sponsor),
                ("update", update_line),
            ],
        )
    }

    /// "new version available" line shown at the bottom of the About dialog.
    pub fn about_update(&self, url: &str, tag: &str) -> String {
        fill(self.about_update, &[("url", url), ("tag", tag)])
    }

    /// Body of the config error toast.
    pub fn config_error(&self, error: &str) -> String {
        fill(self.notif_config_error, &[("error", error)])
    }

    /// Body of the "pinned device not found" toast.
    pub fn device_not_found(&self, name: &str) -> String {
        fill(self.notif_device_not_found, &[("name", name)])
    }

    /// Body of the update available toast.
    pub fn update_available(&self, tag: &str) -> String {
        fill(self.notif_update, &[("tag", tag)])
    }
}

/// Replace `{key}` placeholders in a template.
fn fill(template: &str, values: &[(&str, &str)]) -> String {
    let mut out = template.to_string();
    for (key, value) in values {
        out = out.replace(&format!("{{{key}}}"), value);
    }
    out
}

static EN: Strings = Strings {
    menu_about: "About WinSoftVol",
    menu_autostart: "Start on Windows startup",
    menu_softvol: "Force software volume",
    menu_night: "Night mode",
    menu_volcap: "Max volume",
    menu_startup_vol: "Startup volume",
    menu_off: "Off",
    menu_quit: "Quit WinSoftVol",

    tooltip_active: "WinSoftVol — active",
    tooltip_volume: "{pct}% | cap: {cap}%",
    tooltip_muted: "Muted | cap: {cap}%",

    about_title: "About WinSoftVol",
    about_body: "v{version} ({hash})\n{desc}\n\nAuthor:  {authors}\nBuilt:   {build}\n\n<a href=\"{homepage}\">Project Homepage</a>\n\nIf you find WinSoftVol useful, please consider supporting its development.\n<a href=\"{sponsor}\">Sponsor on GitHub \u{2665}</a>{update}",
    about_update: "\n\n<a href=\"{url}\">\u{1F195} New version {tag} available \u{2014} click to download</a>",

    notif_title_exclusive: "WinSoftVol — exclusive mode detected",
    notif_reconnected: "USB audio device reconnected — volume control restored.",
    notif_exclusive_start: "An app bypassed the audio mixer. Volume control won't apply to it until it releases the device.",
    notif_exclusive_end: "Exclusive audio mode ended — volume control restored.",
    notif_title_config_error: "WinSoftVol — Config Error",
    notif_config_error: "config.toml: {error}\nPrevious settings kept. Fix the file to apply changes.",
    notif_title_device_not_found: "WinSoftVol — Device Not Found",
    notif_device_not_found: "Pinned device \"{name}\" not found — using default audio device.",
    notif_title_update: "WinSoftVol Update Available",
    notif_update: "{tag} is ready — click to open release page",
};

static ZH_CN: Strings = Strings {
    menu_about: "关于 WinSoftVol",
    menu_autostart: "开机自启动",
    menu_softvol: "强制软件音量",
    menu_night: "夜间模式",
    menu_volcap: "音量上限",
    menu_startup_vol: "启动音量",
    menu_off: "关闭",
    menu_quit: "退出 WinSoftVol",

    tooltip_active: "WinSoftVol — 运行中",
    tooltip_volume: "{pct}% | 上限 {cap}%",
    tooltip_muted: "已静音 | 上限 {cap}%",

    about_title: "关于 WinSoftVol",
    about_body: "v{version} ({hash})\n{desc}\n\n作者:  {authors}\n构建:  {build}\n\n<a href=\"{homepage}\">项目主页</a>\n\n如果 WinSoftVol 对你有帮助，欢迎支持它的开发。\n<a href=\"{sponsor}\">在 GitHub 上赞助 \u{2665}</a>{update}",
    about_update: "\n\n<a href=\"{url}\">\u{1F195} 新版本 {tag} 已发布 \u{2014} 点击下载</a>",

    notif_title_exclusive: "WinSoftVol — 检测到独占模式",
    notif_reconnected: "USB 音频设备已重新连接 — 音量控制已恢复。",
    notif_exclusive_start: "有应用绕过了音频混音器，在它释放设备之前，音量控制不会对它生效。",
    notif_exclusive_end: "独占音频模式已结束 — 音量控制已恢复。",
    notif_title_config_error: "WinSoftVol — 配置错误",
    notif_config_error: "config.toml: {error}\n已保留原设置，修复配置文件后生效。",
    notif_title_device_not_found: "WinSoftVol — 未找到设备",
    notif_device_not_found: "未找到绑定的设备 \"{name}\" — 将使用默认音频设备。",
    notif_title_update: "WinSoftVol 有可用更新",
    notif_update: "{tag} 已发布 — 点击打开发布页面",
};

static ZH_TW: Strings = Strings {
    menu_about: "關於 WinSoftVol",
    menu_autostart: "開機自動啟動",
    menu_softvol: "強制軟體音量",
    menu_night: "夜間模式",
    menu_volcap: "音量上限",
    menu_startup_vol: "啟動音量",
    menu_off: "關閉",
    menu_quit: "結束 WinSoftVol",

    tooltip_active: "WinSoftVol — 執行中",
    tooltip_volume: "{pct}% | 上限 {cap}%",
    tooltip_muted: "已靜音 | 上限 {cap}%",

    about_title: "關於 WinSoftVol",
    about_body: "v{version} ({hash})\n{desc}\n\n作者:  {authors}\n建置:  {build}\n\n<a href=\"{homepage}\">專案首頁</a>\n\n如果 WinSoftVol 對你有幫助，歡迎支持它的開發。\n<a href=\"{sponsor}\">在 GitHub 上贊助 \u{2665}</a>{update}",
    about_update: "\n\n<a href=\"{url}\">\u{1F195} 新版本 {tag} 已發布 \u{2014} 點擊下載</a>",

    notif_title_exclusive: "WinSoftVol — 偵測到獨佔模式",
    notif_reconnected: "USB 音訊裝置已重新連線 — 音量控制已恢復。",
    notif_exclusive_start: "有應用程式繞過了音訊混音器，在它釋放裝置之前，音量控制不會對它生效。",
    notif_exclusive_end: "獨佔音訊模式已結束 — 音量控制已恢復。",
    notif_title_config_error: "WinSoftVol — 設定錯誤",
    notif_config_error: "config.toml: {error}\n已保留原設定，修復設定檔後生效。",
    notif_title_device_not_found: "WinSoftVol — 找不到裝置",
    notif_device_not_found: "找不到綁定的裝置 \"{name}\" — 將使用預設音訊裝置。",
    notif_title_update: "WinSoftVol 有新版本可用",
    notif_update: "{tag} 已發布 — 點擊開啟發布頁面",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_code_parses_supported_codes() {
        assert_eq!(Lang::from_code("en"), Some(Lang::En));
        assert_eq!(Lang::from_code("EN-US"), Some(Lang::En));
        assert_eq!(Lang::from_code("zh-CN"), Some(Lang::ZhCn));
        assert_eq!(Lang::from_code("zh_Hans"), Some(Lang::ZhCn));
        assert_eq!(Lang::from_code("zh-TW"), Some(Lang::ZhTw));
        assert_eq!(Lang::from_code("zh-Hant"), Some(Lang::ZhTw));
        assert_eq!(Lang::from_code("fr"), None);
    }

    #[test]
    fn all_languages_have_non_empty_strings() {
        for lang in [Lang::En, Lang::ZhCn, Lang::ZhTw] {
            let s = lang.strings();
            assert!(!s.menu_about.is_empty());
            assert!(!s.menu_softvol.is_empty());
            assert!(!s.about_body.is_empty());
            assert!(!s.notif_reconnected.is_empty());
            assert!(!s.notif_device_not_found.is_empty());
        }
    }

    #[test]
    fn tooltip_fills_placeholders() {
        let en = Lang::En.strings();
        assert_eq!(en.tooltip(50, 100, false), "50% | cap: 100%");
        assert_eq!(en.tooltip(0, 80, true), "Muted | cap: 80%");

        let zh = Lang::ZhCn.strings();
        assert_eq!(zh.tooltip(50, 100, false), "50% | 上限 100%");
        assert_eq!(zh.tooltip(0, 80, true), "已静音 | 上限 80%");
    }

    #[test]
    fn resolve_prefers_configured_language() {
        assert_eq!(resolve(Some("zh-TW")), Lang::ZhTw);
        assert_eq!(resolve(Some("zh-CN")), Lang::ZhCn);
    }

    #[test]
    fn about_body_fills_all_placeholders() {
        let s = Lang::ZhCn.strings();
        let body = s.about_body(
            "0.3.2", "abc1234", "desc", "author", "2026-01-01", "https://home", "https://sponsor", "",
        );
        assert!(!body.contains('{'));
        assert!(body.contains("0.3.2"));
        assert!(body.contains("https://home"));
    }
}
