//! The owner-only pipe DACL (security-plan §Authentication & Authorization, IPC access control
//! (Windows)): protected, the current user's SID and SYSTEM, nothing inherited.

use std::io;
use std::ptr;

use interprocess::os::windows::security_descriptor::{
    AsSecurityDescriptorExt as _, BorrowedSecurityDescriptor, SecurityDescriptor,
};
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, LocalFree};
use windows_sys::Win32::Security::Authorization::{
    ConvertSidToStringSidW, ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1,
};
use windows_sys::Win32::Security::{
    GetTokenInformation, PSECURITY_DESCRIPTOR, TOKEN_QUERY, TOKEN_USER, TokenUser,
};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use windows_sys::core::PWSTR;

pub(crate) fn owner_only_sddl(user_sid: &str) -> String {
    format!("D:P(A;;GA;;;{user_sid})(A;;GA;;;SY)")
}

pub(super) fn owner_only_descriptor() -> io::Result<SecurityDescriptor> {
    from_sddl(&owner_only_sddl(&user_sid()?))
}

/// The process token's user SID (never a logon SID), as `S-1-…`.
pub(crate) fn user_sid() -> io::Result<String> {
    let mut token: HANDLE = ptr::null_mut();
    // SAFETY: the current-process pseudo-handle needs no close; `token` is a local out-pointer.
    if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) } == 0 {
        return Err(io::Error::last_os_error());
    }
    let sid = token_user_sid(token);
    // SAFETY: `token` was opened above and is closed exactly once.
    unsafe { CloseHandle(token) };
    sid
}

fn token_user_sid(token: HANDLE) -> io::Result<String> {
    let mut len = 0u32;
    // SAFETY: a size query: a null buffer of length 0, `len` a local out-pointer.
    unsafe { GetTokenInformation(token, TokenUser, ptr::null_mut(), 0, &mut len) };
    // u64 words: the buffer must be aligned for TOKEN_USER's pointer field.
    let mut buf = vec![0u64; usize::try_from(len).unwrap_or(0).div_ceil(8)];
    // SAFETY: `buf` holds at least `len` bytes and outlives the call.
    let filled =
        unsafe { GetTokenInformation(token, TokenUser, buf.as_mut_ptr().cast(), len, &mut len) };
    if filled == 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: the call above wrote a TOKEN_USER at the start of the aligned buffer.
    let user = unsafe { &*buf.as_ptr().cast::<TOKEN_USER>() };
    let mut text: PWSTR = ptr::null_mut();
    // SAFETY: the SID lives in `buf`; `text` receives a LocalAlloc'd string freed below.
    if unsafe { ConvertSidToStringSidW(user.User.Sid, &mut text) } == 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: `text` is the NUL-terminated string the call above allocated.
    let sid = unsafe { wide_string(text) };
    // SAFETY: allocated by ConvertSidToStringSidW with LocalAlloc; freed once.
    unsafe { LocalFree(text.cast()) };
    Ok(sid)
}

/// # Safety
/// `text` points to a NUL-terminated UTF-16 string.
unsafe fn wide_string(text: PWSTR) -> String {
    // SAFETY: the caller guarantees the terminator, so every read is in bounds.
    let len = (0..).take_while(|&i| unsafe { *text.add(i) } != 0).count();
    // SAFETY: `len` units precede the terminator.
    String::from_utf16_lossy(unsafe { std::slice::from_raw_parts(text, len) })
}

fn from_sddl(sddl: &str) -> io::Result<SecurityDescriptor> {
    let wide: Vec<u16> = sddl.encode_utf16().chain(Some(0)).collect();
    let mut raw: PSECURITY_DESCRIPTOR = ptr::null_mut();
    // SAFETY: `wide` is NUL-terminated; `raw` receives a LocalAlloc'd self-relative descriptor.
    let converted = unsafe {
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            wide.as_ptr(),
            SDDL_REVISION_1,
            &mut raw,
            ptr::null_mut(),
        )
    };
    if converted == 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: `raw` is a valid descriptor until the LocalFree below; `to_owned_sd` copies it.
    let owned = unsafe { BorrowedSecurityDescriptor::from_ptr(raw.cast_const()) }.to_owned_sd();
    // SAFETY: allocated by the conversion above; freed once, after the copy.
    unsafe { LocalFree(raw) };
    owned
}

#[cfg(test)]
mod tests {
    use interprocess::local_socket::Stream;

    use super::*;
    use crate::server::Server;
    use crate::test_capture::test_endpoint;
    use crate::test_support::{canonical_sddl, dacl_of, user_sid as independent_user_sid};

    #[test]
    fn owner_only_sddl_is_protected_user_and_system() {
        assert_eq!(
            owner_only_sddl("S-1-5-21-1-2-3-1001"),
            "D:P(A;;GA;;;S-1-5-21-1-2-3-1001)(A;;GA;;;SY)"
        );
        let sid = user_sid().expect("sid");
        assert!(sid.starts_with("S-1-5-"), "{sid}");
    }

    /// The bound pipe's DACL, read back through a client handle: protected, the user and SYSTEM
    /// only. Windows reports the `GA` it was given as `FA` (measured). The expected SID is read apart
    /// from the product's own lookup.
    #[test]
    fn bound_pipe_dacl_reads_back_protected_user_and_system() {
        let dir = tempfile::tempdir().expect("tempdir");
        let endpoint = test_endpoint(dir.path(), "dacl");
        let _server = Server::bind(&endpoint).expect("bound");
        let expected = canonical_sddl(&format!(
            "D:P(A;;FA;;;{})(A;;FA;;;SY)",
            independent_user_sid()
        ));
        assert_eq!(dacl_of(&endpoint), expected);
    }

    /// Neither end of the pipe is inheritable: no channel handle can reach the wrapped child.
    #[test]
    fn pipe_handles_on_both_ends_are_not_inheritable() {
        use interprocess::local_socket::traits::Listener as _;
        use std::os::windows::io::{AsHandle as _, AsRawHandle as _};
        use windows_sys::Win32::Foundation::{GetHandleInformation, HANDLE_FLAG_INHERIT};

        fn flags(handle: std::os::windows::io::BorrowedHandle<'_>) -> u32 {
            let mut flags = 0u32;
            // SAFETY: a live handle borrowed for the call; `flags` is a local out-pointer.
            let ok = unsafe { GetHandleInformation(handle.as_raw_handle(), &mut flags) };
            assert_ne!(ok, 0, "{}", std::io::Error::last_os_error());
            flags
        }
        let dir = tempfile::tempdir().expect("tempdir");
        let endpoint = test_endpoint(dir.path(), "inherit");
        let server = Server::bind(&endpoint).expect("bound");
        let client = crate::client::open(&endpoint).expect("client open");
        let accepted = server.listener.accept().expect("accept");
        for stream in [&client, &accepted] {
            let Stream::NamedPipe(pipe) = stream;
            assert_eq!(flags(pipe.as_handle()) & HANDLE_FLAG_INHERIT, 0);
        }
    }
}
