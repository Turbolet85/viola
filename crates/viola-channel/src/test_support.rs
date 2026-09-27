//! Test helpers shared by this crate's unit tests and the root package's tests, behind the
//! `test-support` feature, which only the root `[dev-dependencies]` enables: the release build never
//! carries it (`scripts/release-check.sh` refuses an artifact built with it).

use serde_json::{Map, Value};
use tracing::field::{Field, Visit};

/// A span's or a line's recorded fields as JSON: numbers and bools as themselves, a `Debug`-only
/// value as its `Debug` text.
#[derive(Default)]
pub struct JsonFields(pub Map<String, Value>);

impl Visit for JsonFields {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.0
            .insert(field.name().to_owned(), format!("{value:?}").into());
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        self.0.insert(field.name().to_owned(), value.into());
    }
    fn record_u64(&mut self, field: &Field, value: u64) {
        self.0.insert(field.name().to_owned(), value.into());
    }
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.0.insert(field.name().to_owned(), value.into());
    }
    fn record_bool(&mut self, field: &Field, value: bool) {
        self.0.insert(field.name().to_owned(), value.into());
    }
}

/// The pipe DACL read-back, with its own SID lookup apart from the product's.
#[cfg(windows)]
mod win {
    use std::os::windows::io::AsRawHandle as _;
    use std::ptr;

    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, LocalFree};
    use windows_sys::Win32::Security::Authorization::{
        ConvertSecurityDescriptorToStringSecurityDescriptorW, ConvertSidToStringSidW,
        ConvertStringSecurityDescriptorToSecurityDescriptorW, GetSecurityInfo, SDDL_REVISION_1,
        SE_KERNEL_OBJECT,
    };
    use windows_sys::Win32::Security::{
        DACL_SECURITY_INFORMATION, GetTokenInformation, PSECURITY_DESCRIPTOR, TOKEN_QUERY,
        TOKEN_USER, TokenUser,
    };
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
    use windows_sys::core::PWSTR;

    /// # Safety
    /// `text` is a NUL-terminated UTF-16 string allocated with `LocalAlloc`; it is freed here.
    unsafe fn take_wide(text: PWSTR) -> String {
        // SAFETY: the caller guarantees the terminator.
        let len = (0..).take_while(|&i| unsafe { *text.add(i) } != 0).count();
        // SAFETY: `len` units precede the terminator.
        let s = String::from_utf16_lossy(unsafe { std::slice::from_raw_parts(text, len) });
        // SAFETY: allocated by the caller's conversion; freed once.
        unsafe { LocalFree(text.cast()) };
        s
    }

    /// `sddl` as Windows renders it after a string → descriptor → string round trip: a well-known
    /// SID comes back as its alias (CI's runner user, the built-in Administrator, as `LA`).
    pub fn canonical_sddl(sddl: &str) -> String {
        let wide: Vec<u16> = sddl.encode_utf16().chain(Some(0)).collect();
        let mut sd: PSECURITY_DESCRIPTOR = ptr::null_mut();
        let mut text: PWSTR = ptr::null_mut();
        // SAFETY: `wide` is NUL-terminated; `sd` is freed once, `text` by take_wide.
        unsafe {
            let parsed = ConvertStringSecurityDescriptorToSecurityDescriptorW(
                wide.as_ptr(),
                SDDL_REVISION_1,
                &mut sd,
                ptr::null_mut(),
            );
            assert_ne!(parsed, 0, "from SDDL");
            let shown = ConvertSecurityDescriptorToStringSecurityDescriptorW(
                sd,
                SDDL_REVISION_1,
                DACL_SECURITY_INFORMATION,
                &mut text,
                ptr::null_mut(),
            );
            LocalFree(sd);
            assert_ne!(shown, 0, "to SDDL");
            take_wide(text)
        }
    }

    /// This process's user SID, read apart from the product's own lookup (`server::win::user_sid`).
    pub fn user_sid() -> String {
        let mut token: HANDLE = ptr::null_mut();
        let mut len = 0u32;
        let mut text: PWSTR = ptr::null_mut();
        // SAFETY: local out-pointers; the token is closed below and the string freed by take_wide.
        unsafe {
            assert_ne!(
                OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token),
                0
            );
            GetTokenInformation(token, TokenUser, ptr::null_mut(), 0, &mut len);
            let mut buf = vec![0u64; usize::try_from(len).expect("len").div_ceil(8)];
            let filled =
                GetTokenInformation(token, TokenUser, buf.as_mut_ptr().cast(), len, &mut len);
            assert_ne!(filled, 0);
            let user = &*buf.as_ptr().cast::<TOKEN_USER>();
            assert_ne!(ConvertSidToStringSidW(user.User.Sid, &mut text), 0);
            CloseHandle(token);
            take_wide(text)
        }
    }

    /// The DACL of the pipe `endpoint`, read back through a client handle as SDDL.
    pub fn dacl_of(endpoint: &str) -> String {
        let pipe = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(endpoint)
            .expect("pipe");
        let mut sd: PSECURITY_DESCRIPTOR = ptr::null_mut();
        let mut dacl = ptr::null_mut();
        let mut text: PWSTR = ptr::null_mut();
        // SAFETY: a live handle; local out-pointers; `sd` freed once, `text` by take_wide.
        unsafe {
            let read = GetSecurityInfo(
                pipe.as_raw_handle(),
                SE_KERNEL_OBJECT,
                DACL_SECURITY_INFORMATION,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut dacl,
                ptr::null_mut(),
                &mut sd,
            );
            assert_eq!(read, 0, "GetSecurityInfo");
            let converted = ConvertSecurityDescriptorToStringSecurityDescriptorW(
                sd,
                SDDL_REVISION_1,
                DACL_SECURITY_INFORMATION,
                &mut text,
                ptr::null_mut(),
            );
            LocalFree(sd);
            assert_ne!(
                converted, 0,
                "ConvertSecurityDescriptorToStringSecurityDescriptorW"
            );
            take_wide(text)
        }
    }
}

#[cfg(windows)]
pub use win::{canonical_sddl, dacl_of, user_sid};

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use tracing::span::{Attributes, Id};
    use tracing_subscriber::layer::{Context, Layer, SubscriberExt as _};

    use super::*;

    /// Every new span's fields, recorded through `JsonFields`.
    #[derive(Clone, Default)]
    struct Spans(Arc<Mutex<Vec<Value>>>);

    impl<S: tracing::Subscriber> Layer<S> for Spans {
        fn on_new_span(&self, attrs: &Attributes<'_>, _: &Id, _: Context<'_, S>) {
            let mut fields = JsonFields::default();
            attrs.record(&mut fields);
            self.0.lock().expect("spans").push(Value::Object(fields.0));
        }
    }

    /// One field of each recorded type: each lands as its own JSON value, and only a `Debug`-only
    /// value as its `Debug` text.
    #[test]
    fn json_fields_records_each_type_as_its_json_value() {
        let spans = Spans::default();
        let subscriber = tracing_subscriber::registry().with(spans.clone());
        tracing::subscriber::with_default(subscriber, || {
            let _span = tracing::span!(
                tracing::Level::INFO,
                "fields",
                s = "text",
                u = 7u64,
                i = -3i64,
                b = true,
                d = ?[1, 2],
            );
        });
        let got = spans.0.lock().expect("spans");
        assert_eq!(
            got.as_slice(),
            [serde_json::json!({"s": "text", "u": 7, "i": -3, "b": true, "d": "[1, 2]"})]
        );
        assert!(got[0]["u"].is_u64() && got[0]["i"].is_i64() && got[0]["b"].is_boolean());
    }

    #[cfg(windows)]
    #[test]
    fn canonical_sddl_renders_well_known_sids_as_their_aliases() {
        assert_eq!(
            canonical_sddl("D:P(A;;FA;;;S-1-5-32-544)(A;;FA;;;S-1-5-18)"),
            "D:P(A;;FA;;;BA)(A;;FA;;;SY)"
        );
    }

    #[cfg(windows)]
    #[test]
    fn user_sid_is_a_user_sid() {
        let sid = user_sid();
        assert!(sid.starts_with("S-1-5-"), "{sid}");
    }
}
