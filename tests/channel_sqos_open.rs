//! The Windows client open the channel client reuses (security-plan §Authentication & Authorization,
//! IPC client-side server verification): a `CreateFileW` handle opened with SQOS Identification and adopted
//! by interprocess must leave a same-user server able to identify, never impersonate, the client.
#![cfg(windows)]

use std::ffi::OsStr;
use std::io::{Read, Write};
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::ptr;
use std::thread;

use interprocess::os::windows::named_pipe::local_socket::Stream;
use windows_sys::Win32::Foundation::{
    CloseHandle, ERROR_PIPE_CONNECTED, GENERIC_READ, GENERIC_WRITE, GetLastError, HANDLE,
    INVALID_HANDLE_VALUE,
};
use windows_sys::Win32::Security::{
    GetTokenInformation, RevertToSelf, TOKEN_QUERY, TokenImpersonationLevel,
};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FILE_FLAG_OVERLAPPED, FILE_FLAGS_AND_ATTRIBUTES, FlushFileBuffers, OPEN_EXISTING,
    PIPE_ACCESS_DUPLEX, ReadFile, SECURITY_IDENTIFICATION, SECURITY_SQOS_PRESENT, WriteFile,
};
use windows_sys::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, ImpersonateNamedPipeClient, PIPE_READMODE_BYTE,
    PIPE_TYPE_BYTE, PIPE_WAIT,
};
use windows_sys::Win32::System::Threading::{GetCurrentThread, OpenThreadToken};

/// Without the overlapped flag an adopted handle hangs its first exchange: interprocess re-opens it overlapped.
const SQOS_OPEN: FILE_FLAGS_AND_ATTRIBUTES =
    SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED;

struct Observed {
    level: i32,
    received: [u8; 4],
}

fn wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

fn owned(handle: HANDLE) -> OwnedHandle {
    assert_ne!(
        handle,
        INVALID_HANDLE_VALUE,
        "{}",
        std::io::Error::last_os_error()
    );
    // SAFETY: a valid handle this process just opened and nothing else owns.
    unsafe { OwnedHandle::from_raw_handle(handle) }
}

/// One byte-mode pipe instance, created before the client opens so no wait is needed.
fn listen(path: &str) -> OwnedHandle {
    let name = wide(path);
    // SAFETY: `name` is NUL-terminated and outlives the call; a null security-attributes pointer is allowed.
    owned(unsafe {
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
    })
}

/// Accepts one client, reads its 4 bytes, reads the impersonation level the client granted, answers 4 bytes.
fn serve(pipe: OwnedHandle) -> Observed {
    let h = pipe.as_raw_handle();
    let mut received = [0u8; 4];
    let mut n = 0u32;
    let mut token: HANDLE = ptr::null_mut();
    let mut level: i32 = -1;
    let mut len = 0u32;
    // SAFETY: `h` is the live server end owned by `pipe`; every out-pointer is a local of the right size.
    unsafe {
        if ConnectNamedPipe(h, ptr::null_mut()) == 0 {
            assert_eq!(GetLastError(), ERROR_PIPE_CONNECTED, "ConnectNamedPipe");
        }
        assert_ne!(
            ReadFile(h, received.as_mut_ptr(), 4, &mut n, ptr::null_mut()),
            0,
            "ReadFile"
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
        assert_ne!(
            WriteFile(h, b"pong".as_ptr(), 4, &mut n, ptr::null_mut()),
            0,
            "WriteFile"
        );
        FlushFileBuffers(h);
    }
    Observed { level, received }
}

fn open_adopted(path: &str, flags: FILE_FLAGS_AND_ATTRIBUTES) -> Stream {
    let name = wide(path);
    // SAFETY: `name` is NUL-terminated and outlives the call; null security attributes and template are allowed.
    let handle = owned(unsafe {
        CreateFileW(
            name.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            0,
            ptr::null(),
            OPEN_EXISTING,
            flags,
            ptr::null_mut(),
        )
    });
    Stream::try_from(handle).unwrap_or_else(|e| panic!("Stream::try_from: {e}"))
}

/// A full round trip through a client opened with `flags`; returns what the server saw and what came back.
fn exchange(label: &str, flags: FILE_FLAGS_AND_ATTRIBUTES) -> (Observed, [u8; 4]) {
    let path = format!(r"\\.\pipe\viola-test-sqos-{}-{label}", std::process::id());
    let server = listen(&path);
    let serving = thread::spawn(move || serve(server));
    let mut stream = open_adopted(&path, flags);
    stream.write_all(b"ping").expect("client write");
    let mut back = [0u8; 4];
    stream.read_exact(&mut back).expect("client read");
    drop(stream);
    (serving.join().expect("server thread"), back)
}

#[test]
fn sqos_identification_open_adopted_reads_identification() {
    let (observed, back) = exchange("identification", SQOS_OPEN);
    assert_eq!(observed.level, 1, "server must read SecurityIdentification");
    assert_eq!(&observed.received, b"ping");
    assert_eq!(&back, b"pong");
}

#[test]
fn sqos_flags_absent_open_reads_impersonation() {
    let (observed, back) = exchange("no-sqos", FILE_FLAG_OVERLAPPED);
    assert_eq!(
        observed.level, 2,
        "without SQOS the server reads SecurityImpersonation"
    );
    assert_eq!(&observed.received, b"ping");
    assert_eq!(&back, b"pong");
}
