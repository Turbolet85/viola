//! Security control negative, Windows client SQOS (test-plan §6 Security control negatives): a
//! same-user server that impersonates the viola client gets `SecurityIdentification`, so a squatted
//! or rogue endpoint can identify the caller but never act as it.
#![cfg(windows)]

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt as _;
use std::os::windows::io::{AsRawHandle as _, FromRawHandle as _, OwnedHandle};
use std::ptr;
use std::thread;

use serde_json::Map;
use viola_channel::Client;
use windows_sys::Win32::Foundation::{
    CloseHandle, ERROR_PIPE_CONNECTED, GetLastError, HANDLE, INVALID_HANDLE_VALUE,
};
use windows_sys::Win32::Security::{
    GetTokenInformation, RevertToSelf, SecurityIdentification, TOKEN_QUERY, TokenImpersonationLevel,
};
use windows_sys::Win32::Storage::FileSystem::{
    FlushFileBuffers, PIPE_ACCESS_DUPLEX, ReadFile, WriteFile,
};
use windows_sys::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, ImpersonateNamedPipeClient, PIPE_READMODE_BYTE,
    PIPE_TYPE_BYTE, PIPE_WAIT,
};
use windows_sys::Win32::System::Threading::{GetCurrentThread, OpenThreadToken};

const REPLY: &[u8] = b"{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"ok\":{}}}\n";

fn listen(path: &str) -> OwnedHandle {
    let name: Vec<u16> = OsStr::new(path).encode_wide().chain(Some(0)).collect();
    // SAFETY: `name` is NUL-terminated and outlives the call; null security attributes are allowed.
    let handle = unsafe {
        CreateNamedPipeW(
            name.as_ptr(),
            PIPE_ACCESS_DUPLEX,
            PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
            1,
            4096,
            4096,
            0,
            ptr::null(),
        )
    };
    assert_ne!(
        handle,
        INVALID_HANDLE_VALUE,
        "{}",
        std::io::Error::last_os_error()
    );
    // SAFETY: a valid handle just created; nothing else owns it.
    unsafe { OwnedHandle::from_raw_handle(handle) }
}

/// Accepts the client, reads its request, impersonates it and reads the level it granted, answers.
fn impersonation_level(pipe: &OwnedHandle) -> i32 {
    let h = pipe.as_raw_handle();
    let mut request = [0u8; 4096];
    let mut n = 0u32;
    let mut token: HANDLE = ptr::null_mut();
    let mut level: i32 = -1;
    let mut len = 0u32;
    // SAFETY: `h` is the live server end; every out-pointer is a local of the right size.
    unsafe {
        if ConnectNamedPipe(h, ptr::null_mut()) == 0 {
            assert_eq!(GetLastError(), ERROR_PIPE_CONNECTED, "ConnectNamedPipe");
        }
        assert_ne!(
            ReadFile(h, request.as_mut_ptr(), 4096, &mut n, ptr::null_mut()),
            0
        );
        assert_ne!(
            ImpersonateNamedPipeClient(h),
            0,
            "ImpersonateNamedPipeClient"
        );
        let opened = OpenThreadToken(GetCurrentThread(), TOKEN_QUERY, 1, &mut token);
        if opened != 0 {
            GetTokenInformation(
                token,
                TokenImpersonationLevel,
                (&raw mut level).cast(),
                4,
                &mut len,
            );
            CloseHandle(token);
        }
        assert_ne!(RevertToSelf(), 0, "RevertToSelf");
        assert_ne!(opened, 0, "OpenThreadToken");
        let reply_len = u32::try_from(REPLY.len()).expect("short reply");
        assert_ne!(
            WriteFile(h, REPLY.as_ptr(), reply_len, &mut n, ptr::null_mut()),
            0
        );
        FlushFileBuffers(h);
    }
    let read = usize::try_from(n).expect("bytes read");
    assert!(request[..read].starts_with(b"{"), "a frame arrived");
    level
}

#[test]
fn security_negative_viola_client_grants_only_identification() {
    let path = format!(r"\\.\pipe\viola-test-chan-{}-sqos", std::process::id());
    let server = listen(&path);
    let serving = thread::spawn(move || impersonation_level(&server));
    let mut client = Client::connect(&path, "cli").expect("connect");
    let reply = client.request("last", Map::new()).expect("reply");
    assert_eq!(reply["result"]["ok"], serde_json::json!({}));
    assert_eq!(
        serving.join().expect("server thread"),
        SecurityIdentification,
        "the server could impersonate the viola client"
    );
}
