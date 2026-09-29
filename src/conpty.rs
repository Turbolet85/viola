//! The sideloaded ConPTY: `conpty.dll` and `OpenConsole.exe` (x64) from Microsoft's
//! `Microsoft.Windows.Console.ConPTY` NuGet package, committed under `vendor/conpty/` byte-identical
//! to the package and embedded here. This file is the pins' one textual home:
//! `scripts/conpty-vendor.sh` reads the version and every SHA-256 below from its text.

use viola_state::pin::Companion;

macro_rules! package_version {
    () => {
        "1.24.260710001"
    };
}

macro_rules! vendored {
    ($name:literal) => {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/vendor/conpty/",
            package_version!(),
            "/x64/",
            $name
        ))
    };
}

/// The `.nupkg` the two files come from; only the vendor script checks it, never a build.
#[cfg(test)]
const NUPKG_SHA256: &str = "175640566a3b59c4b132070ee96c2c77e5ab7edd2e92732a5eb3610bbf63d90e";

/// The pinned subdirectory of `bin/<key>/`, kept off the child's `PATH` (the pinned dir is first on it).
pub(crate) const SUBDIR: &str = "conpty";

/// `OpenConsole.exe` first: `conpty.dll` is loaded only when both verify, since a sideloaded dll with
/// no host beside it silently runs the inbox `conhost.exe`.
pub(crate) const FILES: &[Companion] = &[
    Companion {
        name: "OpenConsole.exe",
        bytes: vendored!("OpenConsole.exe"),
        sha256_hex: "b7fd936c2668b87b9ecf7b3366dc6568afc1c6f981874cba3e955a1c35cf8160",
    },
    Companion {
        name: "conpty.dll",
        bytes: vendored!("conpty.dll"),
        sha256_hex: "39fba2713e2495117b1591ae8c32a3b904bea7aa66069cf7815e2844c76d75d8",
    },
];

pub(crate) const DLL: &str = "conpty.dll";

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest as _, Sha256};

    fn hex(bytes: &[u8]) -> String {
        Sha256::digest(bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }

    /// The embedded bytes against literal oracles, never the product constants.
    #[test]
    fn embedded_bytes_hash_to_the_pinned_package_files() {
        assert_eq!(package_version!(), "1.24.260710001");
        assert_eq!(
            NUPKG_SHA256,
            "175640566a3b59c4b132070ee96c2c77e5ab7edd2e92732a5eb3610bbf63d90e"
        );
        let found: Vec<(&str, String, usize)> = FILES
            .iter()
            .map(|f| (f.name, hex(f.bytes), f.bytes.len()))
            .collect();
        assert_eq!(
            found,
            [
                (
                    "OpenConsole.exe",
                    "b7fd936c2668b87b9ecf7b3366dc6568afc1c6f981874cba3e955a1c35cf8160".to_owned(),
                    1_066_296
                ),
                (
                    "conpty.dll",
                    "39fba2713e2495117b1591ae8c32a3b904bea7aa66069cf7815e2844c76d75d8".to_owned(),
                    109_920
                ),
            ]
        );
        assert!(FILES.iter().all(|f| hex(f.bytes) == f.sha256_hex));
        assert!(FILES.iter().any(|f| f.name == DLL));
    }
}
