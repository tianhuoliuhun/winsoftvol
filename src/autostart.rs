use winreg::{enums::*, RegKey};

const APP_NAME: &str = "WinSoftVol";
const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";

pub fn set(enable: bool) -> anyhow::Result<()> {
    let exe = std::env::current_exe()?;
    set_at(RUN_KEY, APP_NAME, enable, &exe.to_string_lossy())
}

/// Write (or remove) an autostart entry under an arbitrary Run key. Split out
/// so tests can exercise the logic without touching the real Run key, which
/// would otherwise be clobbered with the test binary's path.
fn set_at(run_key: &str, app_name: &str, enable: bool, exe: &str) -> anyhow::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey(run_key)?;
    if enable {
        key.set_value(app_name, &exe.to_string())?;
    } else {
        let _ = key.delete_value(app_name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_name_matches_registry_key() {
        assert_eq!(APP_NAME, "WinSoftVol");
    }

    #[test]
    fn run_key_path_is_correct() {
        assert!(RUN_KEY.contains(r"CurrentVersion\Run"));
    }

    #[cfg(windows)]
    #[test]
    fn round_trip_uses_isolated_key() {
        const TEST_KEY: &str = r"Software\WinSoftVol\TestRun";
        const TEST_APP: &str = "WinSoftVolTest";
        const TEST_EXE: &str = r"C:\test\winsoftvol.exe";

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let read = || {
            hkcu.open_subkey(TEST_KEY)
                .and_then(|k| k.get_value::<String, _>(TEST_APP))
                .ok()
        };

        set_at(TEST_KEY, TEST_APP, true, TEST_EXE).expect("enable");
        assert_eq!(read().as_deref(), Some(TEST_EXE));

        set_at(TEST_KEY, TEST_APP, false, TEST_EXE).expect("disable");
        assert!(read().is_none());

        // Clean up the temporary key.
        let _ = hkcu.delete_subkey_all(TEST_KEY);
    }
}
