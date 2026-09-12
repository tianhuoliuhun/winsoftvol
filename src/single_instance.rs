//! Single-instance guard.
//!
//! WinSoftVol must run at most once per user session: when two instances are
//! running, both observe endpoint volume changes and scale session volumes,
//! which doubles the applied attenuation.

use windows::core::w;
use windows::Win32::Foundation::{BOOL, ERROR_ALREADY_EXISTS};
use windows::Win32::System::Threading::CreateMutexW;

/// Returns `true` when this process is the only WinSoftVol instance running in
/// the current user session.
///
/// The mutex handle is intentionally left open for the lifetime of the
/// process: Windows releases the mutex automatically when the process exits
/// (including crashes), so no cleanup is needed.
pub fn acquire() -> bool {
    match unsafe { CreateMutexW(None, BOOL(1), w!("Local\\WinSoftVol_SingleInstance")) } {
        Ok(_handle) => !already_exists(),
        // If the mutex cannot be created (e.g. due to permissions), keep
        // running rather than blocking the app entirely.
        Err(_) => true,
    }
}

/// Whether `CreateMutexW` reported that the named mutex already existed.
fn already_exists() -> bool {
    std::io::Error::last_os_error().raw_os_error() == Some(ERROR_ALREADY_EXISTS.0 as i32)
}

#[cfg(test)]
mod tests {
    use super::acquire;

    #[test]
    fn second_acquire_detects_running_instance() {
        // The first call may report an existing instance if WinSoftVol is
        // already running on this machine; the second call must always see the
        // mutex created by the first one.
        let _first = acquire();
        assert!(!acquire());
    }
}
