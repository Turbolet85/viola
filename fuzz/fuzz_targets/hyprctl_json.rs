//! Every `hyprctl` reply parser over arbitrary bytes: none panics, each kept address is in its
//! closed form, and a version read back prints its own tag.

#![no_main]

use libfuzzer_sys::fuzz_target;
use viola_hyprland::{is_address, reply};

fuzz_target!(|bytes: &[u8]| {
    if let Ok(windows) = reply::parse_clients(bytes) {
        for window in windows {
            assert!(is_address(window.address.as_ref()));
        }
    }
    if let Ok(Some(window)) = reply::parse_active_window(bytes) {
        assert!(is_address(window.address.as_ref()));
    }
    if let Ok(workspaces) = reply::parse_workspaces(bytes) {
        for workspace in workspaces {
            if let Some(last) = workspace.last_window {
                assert!(is_address(last.as_ref()));
            }
        }
    }
    if let Ok(version) = reply::parse_version(bytes) {
        let tag = format!("v{version}");
        assert_eq!(reply::HyprlandVersion::from_tag(&tag), Some(version));
    }
    let _ = reply::parse_dispatch(bytes);
});
