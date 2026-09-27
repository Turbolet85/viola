//! The server: one listener per `viola run`, one accept thread, one worker thread per connection
//! (architecture §Established Decisions [Message Broker / IPC]). The exclusive bind arbitrates two
//! starts of one name: the first-pipe-instance flag on Windows, a held lock beside the socket on
//! Unix. Every line goes through `obs_event!`; `params` and result bodies are never logged.

use std::io::BufReader;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;
use std::time::Instant;

use interprocess::local_socket::traits::ListenerExt as _;
use interprocess::local_socket::{
    GenericFilePath, Listener, ListenerOptions, Stream, ToFsName as _,
};
use serde_json::Value;
use tracing::instrument;
use viola_core::obs::ObsEvent;
use viola_core::obs_event;

use crate::frame::{
    Params, Request, error_response, ok_response, parse_request, read_frame, write_frame,
};
use crate::{ChannelError, ENDPOINT_KIND, PROTOCOL_V, ProtocolError};

/// What the wrapper answers. Called only for a request the server admitted (its `v` supported).
pub trait Dispatch: Send + Sync {
    /// The `result` member; `Err` is the protocol fault to answer with instead.
    fn dispatch(&self, method: &str, params: &Value) -> Result<Value, ProtocolError>;
}

/// A bound endpoint, not yet accepting.
pub struct Server {
    listener: Listener,
    guard: Guard,
}

/// A served endpoint. On Unix dropping it takes the name down (the socket file, then its lock); a
/// Windows pipe lives in the accept thread until the process exits.
pub struct Serving {
    _guard: Guard,
}

impl Server {
    /// `BindTaken` when another live process holds the endpoint.
    #[instrument(skip_all, name = "channel.bind", fields(endpoint_kind = ENDPOINT_KIND))]
    pub fn bind(endpoint: &str) -> Result<Self, ChannelError> {
        let (listener, guard) = listen(endpoint)?;
        Ok(Self { listener, guard })
    }

    /// One accept thread; each connection gets its own worker thread.
    pub fn serve(self, dispatch: Arc<dyn Dispatch>) -> Serving {
        let Self { listener, guard } = self;
        let _ = std::thread::Builder::new()
            .name("channel-accept".to_owned())
            .spawn(move || accept_loop(&listener, &dispatch));
        Serving { _guard: guard }
    }
}

#[cfg(windows)]
struct Guard;

/// The lock is held for the process life; the socket file goes before the lock is released, so a
/// next start never finds a live-looking leftover.
#[cfg(unix)]
struct Guard {
    socket: std::path::PathBuf,
    _lock: std::fs::File,
}

#[cfg(unix)]
impl Drop for Guard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.socket);
    }
}

/// interprocess already creates the pipe non-inheritable, local-only (`PIPE_REJECT_REMOTE_CLIENTS`)
/// and first-instance; the DACL is the one control it leaves open.
#[cfg(windows)]
fn listen(endpoint: &str) -> Result<(Listener, Guard), ChannelError> {
    use interprocess::os::windows::local_socket::ListenerOptionsExt as _;
    let descriptor = win::owner_only_descriptor().map_err(ChannelError::Bind)?;
    let name = endpoint
        .to_fs_name::<GenericFilePath>()
        .map_err(ChannelError::Bind)?;
    let listener = ListenerOptions::new()
        .name(name)
        .security_descriptor(descriptor)
        .create_sync()
        .map_err(|e| {
            // A second first-instance create of a live pipe name is refused access.
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                ChannelError::BindTaken
            } else {
                ChannelError::Bind(e)
            }
        })?;
    Ok((listener, Guard))
}

/// The lock decides first; with it held, whatever sits at the socket path is a dead wrapper's
/// leftover. `try_overwrite` stays off (the default); the name is reclaimed by `Guard`, not by
/// interprocess, so it goes before the lock does.
#[cfg(unix)]
fn listen(endpoint: &str) -> Result<(Listener, Guard), ChannelError> {
    use std::fs::{self, OpenOptions, Permissions, TryLockError};
    use std::os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _};

    let socket = std::path::PathBuf::from(endpoint);
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .mode(0o600)
        .open(socket.with_extension("lock"))
        .map_err(ChannelError::Bind)?;
    match lock.try_lock() {
        Ok(()) => {}
        Err(TryLockError::WouldBlock) => return Err(ChannelError::BindTaken),
        Err(TryLockError::Error(e)) => return Err(ChannelError::Bind(e)),
    }
    // A leftover that cannot be removed fails the bind below; that failure is the one reported.
    let _ = fs::remove_file(&socket);
    let name = socket
        .as_path()
        .to_fs_name::<GenericFilePath>()
        .map_err(ChannelError::Bind)?;
    let listener = ListenerOptions::new()
        .name(name)
        .reclaim_name(false)
        .create_sync()
        .map_err(ChannelError::Bind)?;
    let guard = Guard {
        socket,
        _lock: lock,
    };
    // After the bind, not `ListenerOptionsExt::mode`: macOS refuses a mode on the socket fd, and
    // the 0700 directory already closes the umask window (security-plan IPC access control, Unix).
    fs::set_permissions(&guard.socket, Permissions::from_mode(0o600))
        .map_err(ChannelError::Bind)?;
    Ok((listener, guard))
}

fn accept_loop(listener: &Listener, dispatch: &Arc<dyn Dispatch>) {
    let accepted = listener.incoming().map_while(Result::ok);
    for (n, stream) in (1u64..).zip(accepted) {
        let dispatch = Arc::clone(dispatch);
        let _ = std::thread::Builder::new()
            .name("channel-conn".to_owned())
            .spawn(move || serve_conn(&stream, &format!("srv-{n}"), dispatch.as_ref()));
    }
}

/// The frames of one connection, answered in order, until the peer closes, an oversize frame, or a
/// dispatch that panicked.
fn serve_conn(stream: &Stream, srv_conn: &str, dispatch: &dyn Dispatch) {
    let mut reader = BufReader::new(stream);
    let mut writer = stream;
    loop {
        let reply = match read_frame(&mut reader) {
            Ok(line) => answer(&line, srv_conn, dispatch),
            Err(ChannelError::Oversize) => Some(refuse_oversize(srv_conn)),
            Err(_) => return,
        };
        let Some(reply) = reply else {
            continue;
        };
        if write_frame(&mut writer, &reply.frame).is_err() || reply.close {
            return;
        }
    }
}

struct Reply {
    frame: Value,
    close: bool,
}

/// The reply a frame earns; `None` for a notification, which is dispatched but never answered.
fn answer(line: &[u8], srv_conn: &str, dispatch: &dyn Dispatch) -> Option<Reply> {
    let started = Instant::now();
    let (mut request, params) = match parse_request(line) {
        Ok(parsed) => parsed,
        Err((id, fault)) => {
            let mut logged = ResponseLine::new(started, id, Peer::Server(srv_conn), None);
            logged.class = Class::Error(Some(fault));
            return Some(Reply {
                frame: error_response(id, fault),
                close: false,
            });
        }
    };
    let peer = Peer::of(params.conn.as_deref(), srv_conn);
    // `conn` joins log lines and nothing else: the wrapper's own answer never sees it, so it can
    // never become identity (obs-plan D-10).
    if let Some(fields) = request.params.as_object_mut() {
        fields.remove("conn");
    }
    let method = method_label(&request.method);
    // `hook.event` is the one notification (architecture [API Style]); any other frame without an
    // id breaks the contract, is never answered, and has no id to log.
    if request.id.is_none() && method != Some(NOTIFICATION) {
        reject_frame("malformed");
        return None;
    }
    log_request(request.id, peer, method, &params);
    let Some(id) = request.id else {
        let _ = dispatched(dispatch, &request, &params, method, peer);
        return None;
    };
    let mut logged = ResponseLine::new(started, Some(id), peer, method);
    let frame = match dispatched(dispatch, &request, &params, method, peer) {
        Ok(result) => {
            logged.class = Class::of(&result);
            ok_response(id, result)
        }
        Err(fault) => {
            logged.class = Class::Error(Some(fault));
            error_response(Some(id), fault)
        }
    };
    Some(Reply {
        close: logged.class == Class::Error(Some(ProtocolError::Internal)),
        frame,
    })
}

/// The version gate, then the wrapper's own answer; a panic in it is `-32603` (obs-plan §7).
#[instrument(skip_all, name = "channel.dispatch", fields(method = method, conn = peer.label()))]
fn dispatched(
    dispatch: &dyn Dispatch,
    request: &Request,
    params: &Params,
    method: Option<&'static str>,
    peer: Peer<'_>,
) -> Result<Value, ProtocolError> {
    if params.v > PROTOCOL_V {
        return Err(ProtocolError::UnsupportedVersion);
    }
    catch_unwind(AssertUnwindSafe(|| {
        dispatch.dispatch(&request.method, &request.params)
    }))
    .unwrap_or(Err(ProtocolError::Internal))
}

/// `detail` is one of the diag-line schema's closed `parse-rejected` codes.
fn reject_frame(detail: &'static str) {
    obs_event!(
        WARN,
        ObsEvent::ParseRejected,
        parser = "channel-frame",
        detail = detail,
        count = 1u64,
    );
}

fn refuse_oversize(srv_conn: &str) -> Reply {
    reject_frame("oversize");
    let mut logged = ResponseLine::new(Instant::now(), None, Peer::Server(srv_conn), None);
    logged.class = Class::Error(Some(ProtocolError::InvalidRequest));
    Reply {
        frame: error_response(None, ProtocolError::InvalidRequest),
        close: true,
    }
}

/// The connection discriminator a line carries: the client's `conn` when it has the documented
/// shape, else this server's own `srv-<n>` (obs-plan D-10). It correlates logs; it is never identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Peer<'a> {
    Client(&'a str),
    Server(&'a str),
}

impl<'a> Peer<'a> {
    fn of(conn: Option<&'a str>, srv_conn: &'a str) -> Self {
        match conn.filter(|c| is_conn(c)) {
            Some(conn) => Self::Client(conn),
            None => Self::Server(srv_conn),
        }
    }

    fn conn(self) -> Option<&'a str> {
        match self {
            Self::Client(conn) => Some(conn),
            Self::Server(_) => None,
        }
    }

    fn srv_conn(self) -> Option<&'a str> {
        match self {
            Self::Server(srv) => Some(srv),
            Self::Client(_) => None,
        }
    }

    fn label(self) -> &'a str {
        match self {
            Self::Client(s) | Self::Server(s) => s,
        }
    }
}

/// `<process>-<pid>-<t0>-<n>` (obs-plan D-32), at most 64 bytes: anything else from a peer is not
/// written into a codes-only log.
fn is_conn(conn: &str) -> bool {
    let mut parts = conn.split('-');
    let process = parts.next().unwrap_or_default();
    let numbers: Vec<&str> = parts.collect();
    conn.len() <= 64
        && !process.is_empty()
        && process.bytes().all(|b| b.is_ascii_lowercase())
        && numbers.len() == 3
        && numbers
            .iter()
            .all(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}

/// A peer's `sender` is logged only as a plain `MAJOR.MINOR.PATCH` of digits: any other text, a
/// pre-release suffix included, could carry content into a codes-only log.
fn version_label(sender: &str) -> Option<&str> {
    let parts: Vec<&str> = sender.split('.').collect();
    let ok = parts.len() == 3
        && parts
            .iter()
            .all(|p| (1..=9).contains(&p.len()) && p.bytes().all(|b| b.is_ascii_digit()));
    ok.then_some(sender)
}

/// The closed method list (architecture §Standard Contracts); a peer's other text never reaches a log.
const METHODS: [&str; 10] = [
    "send",
    "wait",
    "last",
    "answer",
    "pause",
    "release",
    "link",
    "unlink",
    "hook.dialog",
    "hook.event",
];

/// The one method sent as a notification, without an id.
const NOTIFICATION: &str = "hook.event";

pub(crate) fn method_label(method: &str) -> Option<&'static str> {
    METHODS.iter().find(|m| **m == method).copied()
}

fn log_request(corr: Option<u64>, peer: Peer<'_>, method: Option<&str>, params: &Params) {
    obs_event!(
        INFO,
        ObsEvent::ChannelRequest,
        corr = corr,
        conn = peer.conn(),
        srv_conn = peer.srv_conn(),
        method = method,
        sender = params.sender.as_deref().and_then(version_label),
        v = params.v,
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Class {
    Ok,
    Refusal,
    /// The fault, when its code is one of the five.
    Error(Option<ProtocolError>),
}

impl Class {
    /// A `result` object carrying `refusal` is a refusal; any other result is `ok`.
    pub(crate) fn of(result: &Value) -> Self {
        if result.get("refusal").is_some() {
            Self::Refusal
        } else {
            Self::Ok
        }
    }
}

/// Logs `channel-response` when dropped, so every reply has its line whichever path sent it.
pub(crate) struct ResponseLine<'a> {
    started: Instant,
    corr: Option<u64>,
    peer: Peer<'a>,
    method: Option<&'static str>,
    pub(crate) class: Class,
}

impl<'a> ResponseLine<'a> {
    /// Until the caller records the outcome, the line reads as an internal error.
    pub(crate) fn new(
        started: Instant,
        corr: Option<u64>,
        peer: Peer<'a>,
        method: Option<&'static str>,
    ) -> Self {
        Self {
            started,
            corr,
            peer,
            method,
            class: Class::Error(Some(ProtocolError::Internal)),
        }
    }

    pub(crate) fn client(
        started: Instant,
        corr: u64,
        conn: &'a str,
        method: Option<&'static str>,
    ) -> Self {
        Self::new(started, Some(corr), Peer::Client(conn), method)
    }
}

impl Drop for ResponseLine<'_> {
    fn drop(&mut self) {
        let duration_ms = u64::try_from(self.started.elapsed().as_millis()).unwrap_or(u64::MAX);
        let (result_class, fault) = match self.class {
            Class::Ok => ("ok", None),
            Class::Refusal => ("refusal", None),
            Class::Error(fault) => ("error", fault),
        };
        macro_rules! response_line {
            ($level:ident) => {
                obs_event!(
                    $level,
                    ObsEvent::ChannelResponse,
                    corr = self.corr,
                    conn = self.peer.conn(),
                    srv_conn = self.peer.srv_conn(),
                    method = self.method,
                    result_class = result_class,
                    error_code = fault.map(ProtocolError::code),
                    duration_ms = duration_ms,
                )
            };
        }
        match fault {
            Some(ProtocolError::Internal) => response_line!(ERROR),
            Some(ProtocolError::InvalidRequest | ProtocolError::UnsupportedVersion) => {
                response_line!(WARN)
            }
            _ => response_line!(INFO),
        }
    }
}

/// The owner-only pipe DACL (security-plan §Authentication & Authorization, IPC access control
/// (Windows)): protected, the current user's SID and SYSTEM, nothing inherited.
#[cfg(windows)]
mod win {
    use std::io;
    use std::ptr;

    use interprocess::os::windows::security_descriptor::{
        AsSecurityDescriptorExt as _, BorrowedSecurityDescriptor, SecurityDescriptor,
    };
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, LocalFree};
    use windows_sys::Win32::Security::Authorization::{
        ConvertSidToStringSidW, ConvertStringSecurityDescriptorToSecurityDescriptorW,
        SDDL_REVISION_1,
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
        let filled = unsafe {
            GetTokenInformation(token, TokenUser, buf.as_mut_ptr().cast(), len, &mut len)
        };
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_capture::{Captured, capture};
    use rstest::rstest;
    use serde_json::json;

    struct Echo;

    impl Dispatch for Echo {
        fn dispatch(&self, method: &str, params: &Value) -> Result<Value, ProtocolError> {
            match method {
                "send" => Ok(json!({"ok": {"echo": params["n"]}})),
                "pause" => Ok(json!({"refusal": "human-typing", "detail": null})),
                "wait" => panic!("dispatch panicked"),
                _ => Err(ProtocolError::MethodNotFound),
            }
        }
    }

    fn reply_to(line: &str) -> (Option<Reply>, Captured) {
        capture(|| answer(line.as_bytes(), "srv-3", &Echo))
    }

    fn frame(line: &str) -> Value {
        reply_to(line).0.expect("a reply").frame
    }

    #[test]
    fn answer_ok_echoes_the_id_and_logs_both_lines() {
        let (reply, got) = reply_to(
            r#"{"jsonrpc":"2.0","id":7,"method":"send","params":{"v":1,"sender":"0.1.0","conn":"cli-12-1790219525118-1","n":5,"text":"canary-chain-value-5c1e"}}"#,
        );
        let reply = reply.expect("reply");
        assert!(!reply.close);
        assert_eq!(
            reply.frame.to_string(),
            r#"{"jsonrpc":"2.0","id":7,"result":{"ok":{"echo":5}}}"#
        );
        let request = got.event("channel-request");
        assert_eq!(request["corr"], 7);
        assert_eq!(request["conn"], "cli-12-1790219525118-1");
        assert!(request.get("srv_conn").is_none());
        assert_eq!(request["method"], "send");
        assert_eq!(request["sender"], "0.1.0");
        assert_eq!(request["v"], 1);
        assert_eq!(request["level"], "INFO");
        let response = got.event("channel-response");
        assert_eq!(response["corr"], 7);
        assert_eq!(response["conn"], "cli-12-1790219525118-1");
        assert_eq!(response["result_class"], "ok");
        assert!(response.get("error_code").is_none());
        assert!(response["duration_ms"].is_u64());
        assert!(!got.text().contains("canary"), "params reached a line");
    }

    #[test]
    fn answer_without_conn_logs_the_server_discriminator() {
        let (_, got) = reply_to(r#"{"jsonrpc":"2.0","id":1,"method":"nope","params":{"v":1}}"#);
        let request = got.event("channel-request");
        assert_eq!(request["srv_conn"], "srv-3");
        assert!(request.get("conn").is_none());
        assert!(
            request.get("method").is_none(),
            "an unknown method name is not logged"
        );
        let response = got.event("channel-response");
        assert_eq!(response["srv_conn"], "srv-3");
        assert_eq!(response["error_code"], -32601);
        assert_eq!(response["result_class"], "error");
        assert_eq!(response["level"], "INFO");
    }

    #[test]
    fn answer_refuses_a_newer_version_with_supported_and_wrapper() {
        let (reply, got) = reply_to(
            r#"{"jsonrpc":"2.0","id":2,"method":"send","params":{"v":2,"sender":"9.0.0"}}"#,
        );
        assert_eq!(
            reply.expect("reply").frame.to_string(),
            r#"{"jsonrpc":"2.0","id":2,"error":{"code":-32602,"message":"unsupported protocol version","data":{"supported":1,"wrapper":"0.1.0"}}}"#
        );
        let response = got.event("channel-response");
        assert_eq!(response["error_code"], -32602);
        assert_eq!(response["level"], "WARN");
        assert_eq!(got.event("channel-request")["v"], 2);
    }

    #[test]
    fn answer_takes_the_current_version_and_an_older_one() {
        for v in [0, 1] {
            let line =
                format!(r#"{{"jsonrpc":"2.0","id":4,"method":"send","params":{{"v":{v},"n":1}}}}"#);
            assert_eq!(frame(&line)["result"]["ok"]["echo"], 1);
        }
    }

    #[test]
    fn answer_classes_a_refusal_result() {
        let (reply, got) =
            reply_to(r#"{"jsonrpc":"2.0","id":3,"method":"pause","params":{"v":1}}"#);
        assert_eq!(
            reply.expect("reply").frame["result"]["refusal"],
            "human-typing"
        );
        assert_eq!(got.event("channel-response")["result_class"], "refusal");
    }

    #[test]
    fn answer_to_a_notification_is_nothing() {
        let (reply, got) = reply_to(r#"{"jsonrpc":"2.0","method":"hook.event","params":{"v":1}}"#);
        assert!(reply.is_none());
        let request = got.event("channel-request");
        assert!(request.get("corr").is_none());
        assert_eq!(request["method"], "hook.event");
        assert!(got.events("channel-response").is_empty());
        assert_eq!(got.span("channel.dispatch")["method"], "hook.event");
    }

    /// Only `hook.event` travels without an id: another id-less frame is rejected, not dispatched.
    #[rstest]
    #[case::request_method(r#"{"jsonrpc":"2.0","method":"send","params":{"v":1}}"#)]
    #[case::unknown_method(r#"{"jsonrpc":"2.0","method":"nope","params":{"v":1}}"#)]
    fn answer_rejects_a_notification_of_another_method(#[case] line: &str) {
        let (reply, got) = reply_to(line);
        assert!(reply.is_none());
        let rejected = got.event("parse-rejected");
        assert_eq!(rejected["parser"], "channel-frame");
        assert_eq!(rejected["detail"], "malformed");
        assert!(got.events("channel-request").is_empty());
        assert!(got.events("channel-response").is_empty());
        assert!(!got.has_span("channel.dispatch"));
    }

    #[test]
    fn answer_a_panicking_dispatch_is_internal_error_and_closes() {
        let (reply, got) = reply_to(r#"{"jsonrpc":"2.0","id":5,"method":"wait","params":{"v":1}}"#);
        let reply = reply.expect("reply");
        assert!(reply.close);
        assert_eq!(
            reply.frame["error"].to_string(),
            r#"{"code":-32603,"message":"internal error","data":null}"#
        );
        let response = got.event("channel-response");
        assert_eq!(response["error_code"], -32603);
        assert_eq!(response["level"], "ERROR");
    }

    #[rstest]
    #[case::not_json("{not json", -32700, "INFO")]
    #[case::not_a_request(r#"{"jsonrpc":"2.0","id":9}"#, -32600, "WARN")]
    fn answer_faults_log_a_response_without_a_request(
        #[case] line: &str,
        #[case] code: i64,
        #[case] level: &str,
    ) {
        let (reply, got) = reply_to(line);
        let reply = reply.expect("reply");
        assert!(!reply.close);
        assert_eq!(reply.frame["error"]["code"], code);
        assert!(got.events("channel-request").is_empty());
        let response = got.event("channel-response");
        assert_eq!(response["error_code"], code);
        assert_eq!(response["srv_conn"], "srv-3");
        assert_eq!(response["level"], level);
    }

    #[test]
    fn refuse_oversize_logs_parse_rejected_and_closes() {
        let (reply, got) = capture(|| refuse_oversize("srv-9"));
        assert!(reply.close);
        assert_eq!(
            reply.frame.to_string(),
            r#"{"jsonrpc":"2.0","id":null,"error":{"code":-32600,"message":"invalid request","data":null}}"#
        );
        let rejected = got.event("parse-rejected");
        assert_eq!(rejected["parser"], "channel-frame");
        assert_eq!(rejected["detail"], "oversize");
        assert_eq!(rejected["count"], 1);
        assert_eq!(rejected["level"], "WARN");
        let response = got.event("channel-response");
        assert_eq!(response["error_code"], -32600);
        assert_eq!(response["srv_conn"], "srv-9");
    }

    /// What the wrapper's own answer is handed: the request's params, with `conn` taken out.
    #[derive(Default)]
    struct Recording(std::sync::Mutex<Vec<Value>>);

    impl Dispatch for Recording {
        fn dispatch(&self, _: &str, params: &Value) -> Result<Value, ProtocolError> {
            self.0.lock().expect("recorded").push(params.clone());
            Ok(json!({"ok": {}}))
        }
    }

    /// `conn` is correlation only (obs-plan D-10): the dispatch never receives it, so no method
    /// can treat it as identity; it still reaches the log lines.
    #[test]
    fn dispatch_never_receives_the_peer_conn() {
        let recording = Recording::default();
        let line = r#"{"jsonrpc":"2.0","id":1,"method":"send","params":{"v":1,"conn":"cli-1-2-3","text":"t"}}"#;
        let (_, got) = capture(|| answer(line.as_bytes(), "srv-1", &recording));
        let seen = recording.0.lock().expect("recorded");
        assert_eq!(seen.as_slice(), [json!({"v": 1, "text": "t"})]);
        assert_eq!(got.event("channel-request")["conn"], "cli-1-2-3");
        assert_eq!(got.event("channel-response")["conn"], "cli-1-2-3");
    }

    #[test]
    fn dispatch_span_carries_method_and_conn_never_params() {
        let (_, got) = reply_to(
            r#"{"jsonrpc":"2.0","id":6,"method":"send","params":{"v":1,"conn":"mcp-1-2-3","n":0}}"#,
        );
        let span = got.span("channel.dispatch");
        assert_eq!(span["method"], "send");
        assert_eq!(span["conn"], "mcp-1-2-3");
        assert!(span.get("params").is_none());
    }

    #[rstest]
    #[case::valid("cli-4812-1790219525118-1", true)]
    #[case::at_64_bytes(&format!("cli-1-1-{}", "9".repeat(56)), true)]
    #[case::over_64_bytes(&format!("cli-1-1-{}", "9".repeat(57)), false)]
    #[case::empty_process("-1-2-3", false)]
    #[case::upper_process("CLI-1-2-3", false)]
    #[case::two_numbers("cli-1-2", false)]
    #[case::four_numbers("cli-1-2-3-4", false)]
    #[case::empty_number("cli-1--3", false)]
    #[case::letter_number("cli-1-x-3", false)]
    #[case::canary("canary-chain-value-5c1e", false)]
    fn is_conn_takes_only_the_documented_shape(#[case] conn: &str, #[case] ok: bool) {
        assert_eq!(is_conn(conn), ok);
    }

    #[rstest]
    #[case::semver("0.1.0", Some("0.1.0"))]
    #[case::nine_digits("123456789.20.300", Some("123456789.20.300"))]
    #[case::ten_digits("1234567890.1.1", None)]
    #[case::pre_release("1.2.3-rc.1", None)]
    #[case::canary("canary-chain-value-5c1e", None)]
    #[case::empty("", None)]
    #[case::two_parts("1.2", None)]
    #[case::four_parts("1.2.3.4", None)]
    #[case::empty_part("1..3", None)]
    #[case::letters("a.b.c", None)]
    #[case::space("0.1 x", None)]
    fn version_label_takes_version_strings_only(#[case] sender: &str, #[case] want: Option<&str>) {
        assert_eq!(version_label(sender), want);
    }

    #[test]
    fn method_label_is_the_closed_list() {
        assert_eq!(method_label("hook.dialog"), Some("hook.dialog"));
        assert_eq!(method_label("unlink"), Some("unlink"));
        assert_eq!(method_label("Send"), None);
        assert_eq!(method_label(""), None);
    }

    #[test]
    fn peer_labels_are_one_or_the_other() {
        let client = Peer::of(Some("cli-1-2-3"), "srv-1");
        assert_eq!(
            (client.conn(), client.srv_conn()),
            (Some("cli-1-2-3"), None)
        );
        assert_eq!(client.label(), "cli-1-2-3");
        let server = Peer::of(Some("not a conn"), "srv-1");
        assert_eq!((server.conn(), server.srv_conn()), (None, Some("srv-1")));
        assert_eq!(server.label(), "srv-1");
        assert_eq!(Peer::of(None, "srv-2"), Peer::Server("srv-2"));
    }

    #[test]
    fn bind_span_carries_the_endpoint_kind() {
        let dir = tempfile::tempdir().expect("tempdir");
        let endpoint = crate::test_capture::test_endpoint(dir.path(), "bind-span");
        let (server, got) = capture(|| Server::bind(&endpoint));
        server.expect("bound");
        let span = got.span("channel.bind");
        assert_eq!(span["endpoint_kind"], ENDPOINT_KIND);
    }

    /// Two binds of one endpoint: exactly one wins while it lives; the name is free once it drops.
    #[test]
    fn bind_of_a_live_endpoint_is_taken_until_it_drops() {
        let dir = tempfile::tempdir().expect("tempdir");
        let endpoint = crate::test_capture::test_endpoint(dir.path(), "arbiter");
        let first = Server::bind(&endpoint).expect("first bind");
        assert!(matches!(
            Server::bind(&endpoint),
            Err(ChannelError::BindTaken)
        ));
        drop(first);
        #[cfg(unix)]
        assert!(
            !std::path::Path::new(&endpoint).exists(),
            "the socket outlived its server"
        );
        {
            let again = Server::bind(&endpoint).expect("free once the first dropped");
            let _serving = again.serve(Arc::new(Echo));
            assert!(matches!(
                Server::bind(&endpoint),
                Err(ChannelError::BindTaken)
            ));
        }
        #[cfg(unix)]
        Server::bind(&endpoint).expect("free once the served one dropped");
    }

    /// A dead wrapper's socket file is taken over, and the new one is owner-only.
    #[cfg(unix)]
    #[test]
    fn bind_takes_over_a_leftover_socket_file_and_makes_it_0600() {
        use std::os::unix::fs::PermissionsExt as _;
        let dir = tempfile::tempdir().expect("tempdir");
        let endpoint = crate::test_capture::test_endpoint(dir.path(), "leftover");
        std::fs::write(&endpoint, b"left by a crashed wrapper").expect("leftover");
        let _server = Server::bind(&endpoint).expect("taken over");
        let mode = |p: &str| std::fs::metadata(p).expect("meta").permissions().mode() & 0o777;
        assert_eq!(mode(&endpoint), 0o600);
        let lock = std::path::Path::new(&endpoint).with_extension("lock");
        assert_eq!(mode(lock.to_str().expect("utf-8")), 0o600);
    }

    #[cfg(windows)]
    #[test]
    fn owner_only_sddl_is_protected_user_and_system() {
        assert_eq!(
            win::owner_only_sddl("S-1-5-21-1-2-3-1001"),
            "D:P(A;;GA;;;S-1-5-21-1-2-3-1001)(A;;GA;;;SY)"
        );
        let sid = win::user_sid().expect("sid");
        assert!(sid.starts_with("S-1-5-"), "{sid}");
    }

    /// The bound pipe's DACL, read back through a client handle: protected, the user and SYSTEM
    /// only. Windows reports the `GA` it was given as `FA` (measured).
    #[cfg(windows)]
    #[test]
    fn bound_pipe_dacl_reads_back_protected_user_and_system() {
        use std::os::windows::io::AsRawHandle as _;
        use std::ptr;
        use windows_sys::Win32::Foundation::LocalFree;
        use windows_sys::Win32::Security::Authorization::{
            ConvertSecurityDescriptorToStringSecurityDescriptorW, GetSecurityInfo, SDDL_REVISION_1,
            SE_KERNEL_OBJECT,
        };
        use windows_sys::Win32::Security::{DACL_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR};
        use windows_sys::core::PWSTR;

        let dir = tempfile::tempdir().expect("tempdir");
        let endpoint = crate::test_capture::test_endpoint(dir.path(), "dacl");
        let _server = Server::bind(&endpoint).expect("bound");
        let pipe = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&endpoint)
            .expect("pipe");
        let mut sd: PSECURITY_DESCRIPTOR = ptr::null_mut();
        let mut dacl = ptr::null_mut();
        let mut text: PWSTR = ptr::null_mut();
        // SAFETY: a live handle and local out-pointers; `sd` and `text` are freed once each below.
        let shown = unsafe {
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
            assert_ne!(converted, 0, "to SDDL");
            let len = (0..).take_while(|&i| *text.add(i) != 0).count();
            let shown = String::from_utf16_lossy(std::slice::from_raw_parts(text, len));
            LocalFree(text.cast());
            shown
        };
        let sid = win::user_sid().expect("sid");
        let expected = canonical_sddl(&format!("D:P(A;;FA;;;{sid})(A;;FA;;;SY)"));
        assert_eq!(shown, expected);
    }

    /// `sddl` as Windows itself renders it after a string → descriptor → string round trip: a
    /// well-known SID comes back as its alias (the built-in Administrator, CI's runner user, as `LA`).
    #[cfg(windows)]
    fn canonical_sddl(sddl: &str) -> String {
        use std::ptr;
        use windows_sys::Win32::Foundation::LocalFree;
        use windows_sys::Win32::Security::Authorization::{
            ConvertSecurityDescriptorToStringSecurityDescriptorW,
            ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1,
        };
        use windows_sys::Win32::Security::{DACL_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR};
        use windows_sys::core::PWSTR;

        let wide: Vec<u16> = sddl.encode_utf16().chain(Some(0)).collect();
        let mut sd: PSECURITY_DESCRIPTOR = ptr::null_mut();
        let mut text: PWSTR = ptr::null_mut();
        // SAFETY: `wide` is NUL-terminated; `sd` and `text` are LocalAlloc'd and freed once each.
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
            let len = (0..).take_while(|&i| *text.add(i) != 0).count();
            let canonical = String::from_utf16_lossy(std::slice::from_raw_parts(text, len));
            LocalFree(text.cast());
            canonical
        }
    }

    #[cfg(windows)]
    #[test]
    fn canonical_sddl_renders_well_known_sids_as_their_aliases() {
        assert_eq!(
            canonical_sddl("D:P(A;;FA;;;S-1-5-32-544)(A;;FA;;;S-1-5-18)"),
            "D:P(A;;FA;;;BA)(A;;FA;;;SY)"
        );
    }

    /// Neither end of the pipe is inheritable: no channel handle can reach the wrapped child.
    #[cfg(windows)]
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
        let endpoint = crate::test_capture::test_endpoint(dir.path(), "inherit");
        let server = Server::bind(&endpoint).expect("bound");
        let client = crate::client::open(&endpoint).expect("client open");
        let accepted = server.listener.accept().expect("accept");
        for stream in [&client, &accepted] {
            let Stream::NamedPipe(pipe) = stream;
            assert_eq!(flags(pipe.as_handle()) & HANDLE_FLAG_INHERIT, 0);
        }
    }
}
