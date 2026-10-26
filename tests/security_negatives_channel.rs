//! Security control negatives, Windows (test-plan §6 Security control negatives): a same-user
//! server that impersonates the viola client gets `SecurityIdentification`, so a squatted or rogue
//! endpoint can identify the caller but never act as it; and the viola listener rejects remote
//! clients and denies a principal outside its DACL, each beside a control that shows the check can
//! fire.
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

fn wide(path: &str) -> Vec<u16> {
    OsStr::new(path).encode_wide().chain(Some(0)).collect()
}

fn test_pipe(label: &str) -> String {
    format!(r"\\.\pipe\viola-test-chan-{}-{label}", std::process::id())
}

/// A test-owned listening instance in `mode`, carrying the default security descriptor.
fn listen_in(path: &str, mode: u32) -> OwnedHandle {
    let name = wide(path);
    // SAFETY: `name` is NUL-terminated and outlives the call; null security attributes are allowed.
    let handle = unsafe {
        CreateNamedPipeW(
            name.as_ptr(),
            PIPE_ACCESS_DUPLEX,
            PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT | mode,
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

/// Opens `path` with `access`; the open's error code when it fails.
fn open(path: &str, access: u32) -> Result<OwnedHandle, u32> {
    use windows_sys::Win32::Storage::FileSystem::{CreateFileW, OPEN_EXISTING};
    let name = wide(path);
    // SAFETY: `name` is NUL-terminated and outlives the call; null security attributes and template
    // are allowed; the error is read before any other call on this thread.
    let handle = unsafe {
        CreateFileW(
            name.as_ptr(),
            access,
            0,
            ptr::null(),
            OPEN_EXISTING,
            0,
            ptr::null_mut(),
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        // SAFETY: no call between the failed open and this read.
        return Err(unsafe { GetLastError() });
    }
    // SAFETY: a valid handle this process just opened; nothing else owns it.
    Ok(unsafe { OwnedHandle::from_raw_handle(handle) })
}

/// The pipe type bits NPFS reports for the pipe at `path`, read through a client open.
fn pipe_type(path: &str) -> u32 {
    use windows_sys::Wdk::Storage::FileSystem::{
        FILE_PIPE_LOCAL_INFORMATION, FilePipeLocalInformation, NtQueryInformationFile,
    };
    use windows_sys::Win32::Storage::FileSystem::FILE_READ_ATTRIBUTES;
    use windows_sys::Win32::System::IO::IO_STATUS_BLOCK;
    let client = open(path, FILE_READ_ATTRIBUTES).expect("client open");
    let mut status = IO_STATUS_BLOCK::default();
    let mut info = FILE_PIPE_LOCAL_INFORMATION::default();
    let len = u32::try_from(std::mem::size_of::<FILE_PIPE_LOCAL_INFORMATION>()).expect("small");
    // SAFETY: `client` is a live pipe handle; both out-pointers are locals of the stated size.
    let nt = unsafe {
        NtQueryInformationFile(
            client.as_raw_handle(),
            &mut status,
            (&raw mut info).cast(),
            len,
            FilePipeLocalInformation,
        )
    };
    assert_eq!(nt, 0, "NtQueryInformationFile");
    info.NamedPipeType
}

/// Opens `path` for reading under the anonymous token, reverting before it returns: the
/// impersonation never outlives the calling thread's own call.
fn anonymous_read(path: &str) -> Result<(), u32> {
    use windows_sys::Win32::Foundation::GENERIC_READ;
    use windows_sys::Win32::Security::ImpersonateAnonymousToken;
    // SAFETY: the pseudo-handle of the calling thread; the impersonation stays on this thread.
    let impersonated = unsafe { ImpersonateAnonymousToken(GetCurrentThread()) };
    assert_ne!(impersonated, 0, "ImpersonateAnonymousToken");
    let opened = open(path, GENERIC_READ).map(drop);
    // SAFETY: ends this thread's impersonation.
    assert_ne!(unsafe { RevertToSelf() }, 0, "RevertToSelf");
    opened
}

#[test]
fn security_negative_viola_listener_rejects_remote_clients() {
    let path = test_pipe("remote");
    let _server = viola_channel::Server::bind(&path).expect("bound");
    assert_ne!(
        pipe_type(&path) & 0x2,
        0,
        "the viola listener accepts remote clients"
    );
}

#[test]
fn security_negative_control_accept_remote_pipe_reads_the_bit_clear() {
    use windows_sys::Win32::System::Pipes::PIPE_ACCEPT_REMOTE_CLIENTS;
    let path = test_pipe("accept-remote");
    let _server = listen_in(&path, PIPE_ACCEPT_REMOTE_CLIENTS);
    assert_eq!(pipe_type(&path) & 0x2, 0);
}

#[test]
fn security_negative_control_reject_remote_pipe_reads_the_bit_set() {
    use windows_sys::Win32::System::Pipes::PIPE_REJECT_REMOTE_CLIENTS;
    let path = test_pipe("reject-remote");
    let _server = listen_in(&path, PIPE_REJECT_REMOTE_CLIENTS);
    assert_ne!(pipe_type(&path) & 0x2, 0);
}

#[test]
fn security_negative_viola_listener_denies_an_anonymous_open() {
    let opened = thread::spawn(|| {
        let path = test_pipe("anon");
        let _server = viola_channel::Server::bind(&path).expect("bound");
        anonymous_read(&path)
    })
    .join()
    .expect("anonymous thread");
    assert_eq!(
        opened,
        Err(5),
        "the anonymous token opened the viola listener"
    );
}

#[test]
fn security_negative_control_default_pipe_admits_an_anonymous_read() {
    use windows_sys::Win32::System::Pipes::PIPE_REJECT_REMOTE_CLIENTS;
    let opened = thread::spawn(|| {
        let path = test_pipe("anon-default");
        let _server = listen_in(&path, PIPE_REJECT_REMOTE_CLIENTS);
        anonymous_read(&path)
    })
    .join()
    .expect("anonymous thread");
    assert_eq!(opened, Ok(()));
}
