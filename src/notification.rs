use windows::{
    core::{Result, HSTRING},
    Data::Xml::Dom::XmlDocument,
    UI::Notifications::{ToastNotification, ToastNotificationManager},
};

use crate::i18n;

const AUMID: &str = "WinSoftVol";

const TITLE_APP: &str = "WinSoftVol";

/// Register AppUserModelId in HKCU so Windows associates toasts with this app.
/// Must be called once at startup before showing any toast.
pub fn register_aumid() {
    use winreg::{enums::*, RegKey};
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok((key, _)) = hkcu.create_subkey(r"SOFTWARE\Classes\AppUserModelId\WinSoftVol") {
        let _ = key.set_value("DisplayName", &"WinSoftVol");
    }
}

pub fn show_device_reconnected() {
    let _ = toast(TITLE_APP, i18n::strings().notif_reconnected);
}

pub fn show_config_error(msg: &str) {
    let s = i18n::strings();
    let truncated: String = msg.chars().take(200).collect();
    let body = s.config_error(&truncated);
    let _ = toast(s.notif_title_config_error, &body);
}

pub fn show_exclusive_mode_active() {
    let s = i18n::strings();
    let _ = toast(s.notif_title_exclusive, s.notif_exclusive_start);
}

pub fn show_exclusive_mode_ended() {
    let _ = toast(TITLE_APP, i18n::strings().notif_exclusive_end);
}

fn build_toast_xml(title: &str, body: &str) -> String {
    format!(
        "<toast duration=\"short\"><visual><binding template=\"ToastGeneric\"><text>{title}</text><text>{body}</text></binding></visual></toast>"
    )
}

fn toast(title: &str, body: &str) -> Result<()> {
    toast_xml(&build_toast_xml(title, body))
}

fn toast_xml(xml_str: &str) -> Result<()> {
    let xml = XmlDocument::new()?;
    xml.LoadXml(&HSTRING::from(xml_str))?;
    let notif = ToastNotification::CreateToastNotification(&xml)?;
    ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(AUMID))?.Show(&notif)?;
    Ok(())
}

pub fn show_device_not_found(name: &str) {
    let s = i18n::strings();
    let body = s.device_not_found(name);
    let _ = toast(s.notif_title_device_not_found, &body);
}

pub fn show_update_available(tag: &str, url: &str) {
    let s = i18n::strings();
    let xml = format!(
        "<toast launch=\"{url}\" activationType=\"protocol\" duration=\"short\"><visual><binding template=\"ToastGeneric\"><text>{}</text><text>{}</text></binding></visual></toast>",
        s.notif_title_update,
        s.update_available(tag),
    );
    let _ = toast_xml(&xml);
}

#[cfg(test)]
mod tests {
    use super::{build_toast_xml, AUMID};

    #[test]
    fn xml_contains_duration_short() {
        let xml = build_toast_xml("T", "B");
        assert!(xml.contains("duration=\"short\""));
    }

    #[test]
    fn xml_contains_toast_generic_template() {
        let xml = build_toast_xml("T", "B");
        assert!(xml.contains("template=\"ToastGeneric\""));
    }

    #[test]
    fn xml_title_appears_before_body() {
        let xml = build_toast_xml("TITLE", "BODY");
        let title_pos = xml.find("TITLE").unwrap();
        let body_pos = xml.find("BODY").unwrap();
        assert!(title_pos < body_pos);
    }

    #[test]
    fn xml_wraps_title_in_text_tag() {
        let xml = build_toast_xml("Hello", "World");
        assert!(xml.contains("<text>Hello</text>"));
    }

    #[test]
    fn xml_wraps_body_in_text_tag() {
        let xml = build_toast_xml("Hello", "World");
        assert!(xml.contains("<text>World</text>"));
    }

    #[test]
    fn xml_has_correct_nesting() {
        let xml = build_toast_xml("T", "B");
        // toast > visual > binding > text
        assert!(xml.starts_with("<toast"));
        assert!(xml.contains("<visual>"));
        assert!(xml.contains("<binding "));
        assert!(xml.ends_with("</toast>"));
    }

    #[test]
    fn aumid_is_winsoftvol() {
        assert_eq!(AUMID, "WinSoftVol");
    }
}
