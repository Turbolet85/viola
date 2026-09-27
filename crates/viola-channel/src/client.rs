//! The sync client. On Windows it opens the pipe with SQOS Identification (security-plan
//! §Authentication & Authorization, IPC client), never interprocess's default connect, so a server
//! can identify the caller but never impersonate it. Every `params` carries `v`, `sender` and the
//! additive `conn` (obs-plan D-10, D-32).

use std::io::{self, BufReader};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use interprocess::local_socket::Stream;
use serde_json::{Map, Value, json};
use tracing::instrument;
use viola_core::obs::ObsEvent;
use viola_core::{VERSION, obs_event};

use crate::frame::{read_frame, write_frame};
use crate::server::{Class, ResponseLine, method_label};
use crate::{ChannelError, PROTOCOL_V, ProtocolError};

/// This process's start instant in Unix-epoch milliseconds, taken once.
static T0_MS: OnceLock<u128> = OnceLock::new();
static CONNECTIONS: AtomicU64 = AtomicU64::new(0);

pub struct Client {
    reader: BufReader<Stream>,
    conn: String,
    next_id: u64,
}

impl Client {
    /// `process` is the connecting role (`cli`, `hook`, `mcp`), the first part of `conn`.
    pub fn connect(endpoint: &str, process: &'static str) -> Result<Self, ChannelError> {
        let stream = open(endpoint).map_err(ChannelError::Connect)?;
        let n = CONNECTIONS.fetch_add(1, Ordering::Relaxed) + 1;
        let conn = format!("{process}-{}-{}-{n}", std::process::id(), t0_ms());
        Ok(Self {
            reader: BufReader::new(stream),
            conn,
            next_id: 1,
        })
    }

    /// `<process>-<pid>-<t0>-<n>`: correlation for the logs of both sides, never identity.
    pub fn conn(&self) -> &str {
        &self.conn
    }

    /// One request and its reply, whole. Ids are integers, monotonic per connection.
    #[instrument(skip_all, name = "channel.request", fields(method = method, conn = self.conn.as_str()))]
    pub fn request(
        &mut self,
        method: &str,
        mut params: Map<String, Value>,
    ) -> Result<Value, ChannelError> {
        let started = Instant::now();
        let id = self.next_id;
        self.next_id += 1;
        params.insert("v".to_owned(), PROTOCOL_V.into());
        params.insert("sender".to_owned(), VERSION.into());
        params.insert("conn".to_owned(), self.conn.clone().into());
        let label = method_label(method);
        obs_event!(
            INFO,
            ObsEvent::ChannelRequest,
            corr = id,
            conn = self.conn.as_str(),
            method = label,
            sender = VERSION,
            v = PROTOCOL_V,
        );
        let mut logged = ResponseLine::client(started, id, &self.conn, label);
        let frame = json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params});
        let mut writer = self.reader.get_ref();
        write_frame(&mut writer, &frame)?;
        let line = read_frame(&mut self.reader)?;
        let reply: Value = serde_json::from_slice(&line).map_err(ChannelError::Parse)?;
        logged.class = reply_class(&reply);
        Ok(reply)
    }
}

/// How a reply reads: its fault when it is one of the five codes, else its result's class.
fn reply_class(reply: &Value) -> Class {
    match reply.get("error") {
        Some(error) => Class::Error(error["code"].as_i64().and_then(fault_of)),
        None => Class::of(&reply["result"]),
    }
}

fn fault_of(code: i64) -> Option<ProtocolError> {
    [
        ProtocolError::Parse,
        ProtocolError::InvalidRequest,
        ProtocolError::MethodNotFound,
        ProtocolError::UnsupportedVersion,
        ProtocolError::Internal,
    ]
    .into_iter()
    .find(|fault| fault.code() == code)
}

fn t0_ms() -> u128 {
    *T0_MS.get_or_init(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_millis())
    })
}

/// `GENERIC_READ | GENERIC_WRITE`, written as its value: an operator over disjoint flag bits
/// breeds `|`→`^` mutants no test can tell apart. The test pins it to the named flags.
#[cfg(windows)]
const PIPE_ACCESS: u32 = 0xC000_0000;

/// `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED`, as its value (see
/// `PIPE_ACCESS`). SQOS Identification: the server may identify the caller, never impersonate
/// it. `FILE_FLAG_OVERLAPPED` is required: interprocess adopts the handle as overlapped, and a
/// non-overlapped one hangs its first exchange (measured, security-plan Decisions Log 2026-09-25).
#[cfg(windows)]
const SQOS_OPEN: u32 = 0x4011_0000;

/// `ERROR_PIPE_BUSY` (231) as `raw_os_error` reports it; the value fits an `i32`.
#[cfg(windows)]
const PIPE_BUSY: i32 = windows_sys::Win32::Foundation::ERROR_PIPE_BUSY as i32;

/// A failed open waits for the listener's next pipe instance only when the pipe is busy (the
/// listener makes it once `accept` returns), and only until `until`.
#[cfg(windows)]
fn retry_busy(error: &io::Error, now: Instant, until: Instant) -> bool {
    error.raw_os_error() == Some(PIPE_BUSY) && now < until
}

/// A busy pipe is waited for this long past the first attempt.
#[cfg(windows)]
const BUSY_WITHIN: std::time::Duration = std::time::Duration::from_secs(2);

/// When an open that started at `start` stops waiting for a busy pipe.
#[cfg(windows)]
fn busy_deadline(start: Instant) -> Instant {
    start + BUSY_WITHIN
}

#[cfg(windows)]
pub(crate) fn open(endpoint: &str) -> io::Result<Stream> {
    use std::os::windows::ffi::OsStrExt as _;
    use std::os::windows::io::{FromRawHandle as _, OwnedHandle};
    use std::ptr;

    use interprocess::os::windows::named_pipe::local_socket::Stream as PipeStream;
    use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
    use windows_sys::Win32::Storage::FileSystem::{CreateFileW, OPEN_EXISTING};
    use windows_sys::Win32::System::Pipes::WaitNamedPipeW;

    const BUSY_WAIT_MS: u32 = 100;

    let wide: Vec<u16> = std::ffi::OsStr::new(endpoint)
        .encode_wide()
        .chain(Some(0))
        .collect();
    let busy_until = busy_deadline(Instant::now());
    let handle = loop {
        // SAFETY: `wide` is NUL-terminated and outlives the call; null security attributes and
        // template are allowed.
        let handle = unsafe {
            CreateFileW(
                wide.as_ptr(),
                PIPE_ACCESS,
                0,
                ptr::null(),
                OPEN_EXISTING,
                SQOS_OPEN,
                ptr::null_mut(),
            )
        };
        if handle != INVALID_HANDLE_VALUE {
            break handle;
        }
        let error = io::Error::last_os_error();
        if !retry_busy(&error, Instant::now(), busy_until) {
            return Err(error);
        }
        // SAFETY: `wide` is NUL-terminated and outlives the call.
        unsafe { WaitNamedPipeW(wide.as_ptr(), BUSY_WAIT_MS) };
    };
    // SAFETY: a valid handle this call just opened; nothing else owns it.
    let owned = unsafe { OwnedHandle::from_raw_handle(handle) };
    PipeStream::try_from(owned)
        .map(Stream::from)
        .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))
}

#[cfg(unix)]
pub(crate) fn open(endpoint: &str) -> io::Result<Stream> {
    use interprocess::local_socket::traits::Stream as _;
    use interprocess::local_socket::{GenericFilePath, ToFsName as _};
    Stream::connect(endpoint.to_fs_name::<GenericFilePath>()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_capture::{capture_global, test_endpoint};
    use crate::{Dispatch, Server};
    use rstest::rstest;
    use std::sync::Arc;

    struct Ids;

    impl Dispatch for Ids {
        fn dispatch(&self, method: &str, params: &Value) -> Result<Value, ProtocolError> {
            match method {
                "last" => Ok(
                    json!({"ok": {"conn": params["conn"], "v": params["v"], "sender": params["sender"]}}),
                ),
                "pause" => Ok(json!({"refusal": "budget-paused", "detail": null})),
                _ => Err(ProtocolError::MethodNotFound),
            }
        }
    }

    fn served(label: &str) -> (tempfile::TempDir, String, crate::Serving) {
        let dir = tempfile::tempdir().expect("tempdir");
        let endpoint = test_endpoint(dir.path(), label);
        let serving = Server::bind(&endpoint).expect("bound").serve(Arc::new(Ids));
        (dir, endpoint, serving)
    }

    #[test]
    fn client_stamps_the_envelope_and_counts_ids_per_connection() {
        let (_dir, endpoint, _serving) = served("envelope");
        let mut client = Client::connect(&endpoint, "cli").expect("connect");
        let first = client.request("last", Map::new()).expect("first");
        assert_eq!(first["id"], 1);
        // The server takes `conn` out before its answer runs (the join is proven on the log lines).
        assert!(first["result"]["ok"]["conn"].is_null());
        assert_eq!(first["result"]["ok"]["v"], 1);
        assert_eq!(first["result"]["ok"]["sender"], "0.1.0");
        let second = client.request("frobnicate", Map::new()).expect("second");
        assert_eq!(second["id"], 2);
        assert_eq!(second["error"]["code"], -32601);
    }

    #[test]
    fn client_conn_has_the_documented_shape_and_counts_connections() {
        let (_dir, endpoint, _serving) = served("conn");
        let a = Client::connect(&endpoint, "hook").expect("a");
        let b = Client::connect(&endpoint, "hook").expect("b");
        let parts = |c: &Client| c.conn().split('-').map(str::to_owned).collect::<Vec<_>>();
        let (pa, pb) = (parts(&a), parts(&b));
        assert_eq!(pa.len(), 4, "{}", a.conn());
        assert_eq!(pa[0], "hook");
        assert_eq!(pa[1], std::process::id().to_string());
        assert!(pa[2].parse::<u128>().is_ok_and(|t0| t0 > 1_700_000_000_000));
        assert_eq!(pa[2], pb[2], "t0 is taken once per process");
        let (na, nb) = (
            pa[3].parse::<u64>().expect("n"),
            pb[3].parse::<u64>().expect("n"),
        );
        assert!(nb > na && na >= 1);
    }

    /// Both sides log the call, and `(conn, corr)` joins their lines (obs-plan D-25).
    #[test]
    fn client_and_server_log_the_call_under_one_conn_and_corr() {
        let got = capture_global();
        let (_dir, endpoint, _serving) = served("logs");
        let mut client = Client::connect(&endpoint, "cli").expect("connect");
        let conn = client.conn().to_owned();
        let reply = client.request("pause", Map::new()).expect("reply");
        assert_eq!(reply["result"]["refusal"], "budget-paused");
        let span = got.span("channel.request");
        assert_eq!(span["method"], "pause");
        assert_eq!(span["conn"], conn.as_str());
        assert!(span.get("params").is_none());
        let requests = got.events("channel-request");
        let responses = got.events("channel-response");
        assert_eq!((requests.len(), responses.len()), (2, 2), "{}", got.text());
        for line in requests.iter().chain(&responses) {
            assert_eq!(line["corr"], 1);
            assert_eq!(line["conn"], conn.as_str());
            assert_eq!(line["method"], "pause");
        }
        assert!(responses.iter().all(|r| r["result_class"] == "refusal"));
        assert!(
            requests
                .iter()
                .all(|r| r["sender"] == "0.1.0" && r["v"] == 1)
        );
    }

    #[test]
    fn client_connect_to_a_missing_endpoint_is_not_found() {
        let dir = tempfile::tempdir().expect("tempdir");
        let endpoint = test_endpoint(dir.path(), "absent");
        let Err(ChannelError::Connect(e)) = Client::connect(&endpoint, "cli") else {
            panic!("connected to nothing");
        };
        assert_eq!(e.kind(), io::ErrorKind::NotFound);
    }

    #[rstest]
    #[case::ok(json!({"id": 1, "result": {"ok": {}}}), Class::Ok)]
    #[case::refusal(json!({"id": 1, "result": {"refusal": "unknown"}}), Class::Refusal)]
    #[case::fault(json!({"id": 1, "error": {"code": -32602}}), Class::Error(Some(ProtocolError::UnsupportedVersion)))]
    #[case::foreign_code(json!({"id": 1, "error": {"code": -1}}), Class::Error(None))]
    fn reply_class_reads_the_outcome(#[case] reply: Value, #[case] want: Class) {
        assert_eq!(reply_class(&reply), want);
    }

    #[cfg(windows)]
    #[test]
    fn open_flags_are_the_named_win32_flags() {
        use windows_sys::Win32::Foundation::{GENERIC_READ, GENERIC_WRITE};
        use windows_sys::Win32::Storage::FileSystem::{
            FILE_FLAG_OVERLAPPED, SECURITY_IDENTIFICATION, SECURITY_SQOS_PRESENT,
        };
        assert_eq!(PIPE_ACCESS, GENERIC_READ | GENERIC_WRITE);
        assert_eq!(
            SQOS_OPEN,
            SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED
        );
    }

    /// The deadline is two seconds AFTER the start, never before it.
    #[cfg(windows)]
    #[test]
    fn busy_deadline_is_two_seconds_after_the_start() {
        let start = Instant::now();
        assert_eq!(
            busy_deadline(start).saturating_duration_since(start),
            std::time::Duration::from_secs(2)
        );
    }

    #[cfg(windows)]
    #[test]
    fn retry_busy_waits_only_for_a_busy_pipe_before_the_deadline() {
        let now = Instant::now();
        let until = now + std::time::Duration::from_secs(1);
        let busy = io::Error::from_raw_os_error(231);
        let absent = io::Error::from_raw_os_error(2);
        assert!(retry_busy(&busy, now, until));
        assert!(
            !retry_busy(&busy, until, until),
            "the deadline itself ends the wait"
        );
        assert!(!retry_busy(&busy, until, now));
        assert!(
            !retry_busy(&absent, now, until),
            "only a busy pipe is waited for"
        );
    }

    #[test]
    fn fault_of_maps_all_five_codes() {
        for code in [-32700, -32600, -32601, -32602, -32603] {
            assert_eq!(fault_of(code).map(ProtocolError::code), Some(code));
        }
        assert_eq!(fault_of(-32000), None);
    }
}
