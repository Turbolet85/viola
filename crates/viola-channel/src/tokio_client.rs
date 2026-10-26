//! The Tokio client, for `viola-mcp` alone (behind the `tokio` feature): the sync
//! [`Client`](crate::Client)'s contract over an async stream. It connects to a recorded `endpoint`,
//! then makes one request per call; every `params` carries `v`, `sender` and `conn`, the reply is
//! read through `MAX_FRAME`, and its lines and `channel.request` span are the sync client's. On
//! Windows the pipe is opened with SQOS Identification and overlapped, never interprocess's default
//! connect (security-plan §Authentication & Authorization, IPC client).

use std::io;
use std::time::Instant;

use interprocess::local_socket::tokio::Stream;
use serde_json::{Map, Value};
use tokio::io::{AsyncBufRead, AsyncBufReadExt as _, AsyncReadExt as _, AsyncWriteExt as _};
use tracing::instrument;
use viola_core::MAX_FRAME;

use crate::ChannelError;
use crate::client::{next_conn, reply_from, request_frame, server_pid_of};
use crate::frame::framed;
use crate::server::ResponseLine;

pub struct TokioClient {
    reader: tokio::io::BufReader<Stream>,
    conn: String,
    next_id: u64,
}

impl TokioClient {
    /// `process` is the connecting role (`mcp`), the first part of `conn`.
    pub async fn connect(endpoint: &str, process: &'static str) -> Result<Self, ChannelError> {
        let stream = open(endpoint).await.map_err(ChannelError::Connect)?;
        Ok(Self {
            reader: tokio::io::BufReader::new(stream),
            conn: next_conn(process),
            next_id: 1,
        })
    }

    /// `<process>-<pid>-<t0>-<n>`: correlation for the logs of both sides, never identity.
    pub fn conn(&self) -> &str {
        &self.conn
    }

    /// The pid of the process serving this connection, as the sync client reads it.
    pub fn server_pid(&self) -> Option<u32> {
        server_pid_of(self.reader.get_ref())
    }

    /// One request and its reply, whole. Ids are integers, monotonic per connection.
    #[instrument(skip_all, name = "channel.request", fields(method = method, conn = self.conn.as_str()))]
    pub async fn request(
        &mut self,
        method: &str,
        params: Map<String, Value>,
    ) -> Result<Value, ChannelError> {
        let started = Instant::now();
        let id = self.next_id;
        self.next_id += 1;
        let (frame, label) = request_frame(id, &self.conn, method, params);
        let mut logged = ResponseLine::client(started, id, &self.conn, label);
        let exchanged = exchange(&mut self.reader, &frame).await;
        reply_from(&mut logged, label, exchanged)
    }
}

/// The frame and its `\n` in one `write_all`, then the reply line.
async fn exchange(
    reader: &mut tokio::io::BufReader<Stream>,
    frame: &Value,
) -> Result<Vec<u8>, ChannelError> {
    let mut line = frame.to_string();
    line.push('\n');
    reader
        .get_mut()
        .write_all(line.as_bytes())
        .await
        .map_err(ChannelError::Io)?;
    read_frame(reader).await
}

/// [`read_frame`](crate::read_frame) over an async reader: the read is bounded before the end of
/// the line is looked for, and the line is judged as the sync reader judges it.
async fn read_frame(reader: &mut (impl AsyncBufRead + Unpin)) -> Result<Vec<u8>, ChannelError> {
    let mut line = Vec::new();
    (&mut *reader)
        .take(MAX_FRAME)
        .read_until(b'\n', &mut line)
        .await
        .map_err(ChannelError::Io)?;
    framed(line, MAX_FRAME)
}

#[cfg(unix)]
async fn open(endpoint: &str) -> io::Result<Stream> {
    use interprocess::local_socket::traits::tokio::Stream as _;
    use interprocess::local_socket::{GenericFilePath, ToFsName as _};
    Stream::connect(endpoint.to_fs_name::<GenericFilePath>()?).await
}

/// The sync client's SQOS open, adopted by the runtime. A busy pipe is waited for on this thread,
/// in 100 ms steps for at most the sync client's two seconds.
#[cfg(windows)]
async fn open(endpoint: &str) -> io::Result<Stream> {
    use interprocess::os::windows::named_pipe::local_socket::tokio::Stream as PipeStream;
    let handle =
        crate::client::open_handle(endpoint, crate::client::busy_deadline(Instant::now()))?;
    PipeStream::try_from(handle)
        .map(Stream::from)
        .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead as _, Write as _};
    use std::sync::Arc;

    use interprocess::local_socket::traits::Listener as _;
    use interprocess::local_socket::{GenericFilePath, ListenerOptions, ToFsName as _};
    use serde_json::json;

    use super::*;
    use crate::test_capture::test_endpoint;
    use crate::{Dispatch, ProtocolError, Server};

    struct Shapes;

    impl Dispatch for Shapes {
        fn dispatch(&self, method: &str, params: &Value) -> Result<Value, ProtocolError> {
            match method {
                "send" => Ok(json!({"ok": {"submitted_at": "t", "cursor": 7}})),
                "wait" => Ok(json!({"ok": {"timed_out": true}})),
                "answer" => Ok(json!({"refusal": "not-delivered", "detail": "unknown-dialog"})),
                "last" => Ok(json!({"ok": {"v": params["v"], "sender": params["sender"]}})),
                _ => Err(ProtocolError::MethodNotFound),
            }
        }
    }

    fn runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime")
    }

    #[test]
    fn tokio_client_reads_each_reply_shape_and_counts_ids() {
        let dir = tempfile::tempdir().expect("tempdir");
        let endpoint = test_endpoint(dir.path(), "tokio-shapes");
        let _serving = Server::bind(&endpoint)
            .expect("bound")
            .serve(Arc::new(Shapes));
        runtime().block_on(async {
            let mut client = TokioClient::connect(&endpoint, "mcp")
                .await
                .expect("connect");
            assert!(client.conn().starts_with("mcp-"), "{}", client.conn());
            let ok = client.request("send", Map::new()).await.expect("send");
            assert_eq!(
                (ok["id"].clone(), ok["result"]["ok"]["cursor"].clone()),
                (json!(1), json!(7))
            );
            let timed_out = client.request("wait", Map::new()).await.expect("wait");
            assert_eq!(timed_out["result"]["ok"]["timed_out"], true);
            let refused = client.request("answer", Map::new()).await.expect("answer");
            assert_eq!(refused["result"]["refusal"], "not-delivered");
            assert_eq!(refused["result"]["detail"], "unknown-dialog");
            let envelope = client.request("last", Map::new()).await.expect("last");
            assert_eq!(envelope["result"]["ok"], json!({"v": 1, "sender": "0.1.0"}));
            let fault = client.request("link", Map::new()).await.expect("fault");
            assert_eq!(
                (fault["id"].clone(), fault["error"]["code"].clone()),
                (json!(5), json!(-32601))
            );
        });
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn tokio_client_server_pid_is_the_serving_process() {
        let dir = tempfile::tempdir().expect("tempdir");
        let endpoint = test_endpoint(dir.path(), "tokio-server-pid");
        let _serving = Server::bind(&endpoint)
            .expect("bound")
            .serve(Arc::new(Shapes));
        let pid = runtime().block_on(async {
            let client = TokioClient::connect(&endpoint, "mcp")
                .await
                .expect("connect");
            client.server_pid()
        });
        assert_eq!(pid, Some(std::process::id()));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn tokio_client_server_pid_is_none_where_peer_credentials_hold_no_pid() {
        let dir = tempfile::tempdir().expect("tempdir");
        let endpoint = test_endpoint(dir.path(), "tokio-server-pid");
        let _serving = Server::bind(&endpoint)
            .expect("bound")
            .serve(Arc::new(Shapes));
        let pid = runtime().block_on(async {
            let client = TokioClient::connect(&endpoint, "mcp")
                .await
                .expect("connect");
            client.server_pid()
        });
        assert_eq!(pid, None);
    }

    #[test]
    fn tokio_client_connect_to_a_missing_endpoint_is_connect() {
        let dir = tempfile::tempdir().expect("tempdir");
        let endpoint = test_endpoint(dir.path(), "tokio-absent");
        let got = runtime().block_on(TokioClient::connect(&endpoint, "mcp"));
        let Err(ChannelError::Connect(e)) = got else {
            panic!("connected to nothing");
        };
        assert_eq!(e.kind(), io::ErrorKind::NotFound);
    }

    /// A raw peer that reads one request line, then answers it with `reply` (none: it hangs up).
    fn raw_peer(endpoint: &str, reply: Option<Vec<u8>>) -> std::thread::JoinHandle<()> {
        let name = endpoint.to_fs_name::<GenericFilePath>().expect("name");
        let listener = ListenerOptions::new()
            .name(name)
            .create_sync()
            .expect("listener");
        std::thread::spawn(move || {
            let stream = listener.accept().expect("accept");
            let mut reader = std::io::BufReader::new(stream);
            let mut request = Vec::new();
            reader.read_until(b'\n', &mut request).expect("request");
            if let Some(reply) = reply {
                let _ = reader.get_mut().write_all(&reply);
            }
        })
    }

    #[test]
    fn tokio_client_reads_a_peer_that_hangs_up_as_closed() {
        let dir = tempfile::tempdir().expect("tempdir");
        let endpoint = test_endpoint(dir.path(), "tokio-closed");
        let peer = raw_peer(&endpoint, None);
        let got = runtime().block_on(async {
            let mut client = TokioClient::connect(&endpoint, "mcp")
                .await
                .expect("connect");
            client.request("last", Map::new()).await
        });
        peer.join().expect("peer");
        assert!(matches!(got, Err(ChannelError::Closed)), "{got:?}");
    }

    #[test]
    fn tokio_client_refuses_a_reply_over_the_frame_bound() {
        let dir = tempfile::tempdir().expect("tempdir");
        let endpoint = test_endpoint(dir.path(), "tokio-oversize");
        let cap = usize::try_from(MAX_FRAME).expect("fits");
        let peer = raw_peer(&endpoint, Some(vec![b'x'; cap + 1]));
        let got = runtime().block_on(async {
            let mut client = TokioClient::connect(&endpoint, "mcp")
                .await
                .expect("connect");
            client.request("last", Map::new()).await
        });
        drop(peer);
        assert!(matches!(got, Err(ChannelError::Oversize)), "{got:?}");
    }

    #[test]
    fn tokio_read_frame_takes_a_line_up_to_the_bound_and_closes_on_a_torn_one() {
        let cap = usize::try_from(MAX_FRAME).expect("fits");
        let mut at_bound = vec![b'x'; cap - 1];
        at_bound.push(b'\n');
        at_bound.extend_from_slice(b"{}\n");
        runtime().block_on(async {
            let mut reader = tokio::io::BufReader::new(at_bound.as_slice());
            assert_eq!(read_frame(&mut reader).await.expect("frame").len(), cap);
            assert_eq!(read_frame(&mut reader).await.expect("next"), b"{}\n");
            let mut torn = tokio::io::BufReader::new(&b"{\"jsonrpc\""[..]);
            assert!(matches!(
                read_frame(&mut torn).await,
                Err(ChannelError::Closed)
            ));
            let mut empty = tokio::io::BufReader::new(&b""[..]);
            assert!(matches!(
                read_frame(&mut empty).await,
                Err(ChannelError::Closed)
            ));
        });
    }
}
