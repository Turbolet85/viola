//! The five tools and their inputs. A tool's schema is advertised (schemars) but never trusted: each
//! handler takes its arguments as a JSON object and re-validates every value itself, so no serde
//! text reaches a result (security-plan §Input Validation, §Error Handling). A frame tool reaches
//! its instance's wrapper after the liveness-only pre-check, a dated gap closed by "Server
//! verification before any frame" (security-plan §Authentication & Authorization).

use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;

use rmcp::model::{
    CacheScope, CallToolRequestParams, CallToolResponse, CallToolResult, ErrorCode, ErrorData,
    Implementation, JsonObject, ListToolsResult, PaginatedRequestParams, ServerCapabilities,
    ServerConfig, Tool,
};
use rmcp::service::RequestContext;
use rmcp::{RoleServer, ServerHandler};
use schemars::JsonSchema;
use serde_json::{Map, Value};
use tracing::instrument;
use viola_agent_claude::dialog::Response;
use viola_agent_claude::hook::typed_text;
use viola_channel::tokio_client::TokioClient;
use viola_core::obs::ObsEvent;
use viola_core::{NotDelivered, RefusalReason, VERSION, ViolaName, obs_event};
use viola_state::liveness::live_endpoint;

use crate::{ListProvider, McpError, result};

/// The tool list, in order: no wheel and no link tool (architecture §Established Decisions [MCP]).
pub const TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"];

/// A `wait`'s bound in ms, its default and its cap: below the client's smallest tool-call default
/// read in the 2.1.287 bundle (60 s) and above `send`'s own bound (architecture [Delivery
/// Confirmation]). The wrapper's own deadline ends the call, so no timer of this crate's exists.
pub const WAIT_CAP: u64 = 50_000;

/// The `timeout_ms` a `wait` sends: the given one under the cap, else the cap.
pub fn wait_timeout(given: Option<u64>) -> u64 {
    given.map_or(WAIT_CAP, |given| given.min(WAIT_CAP))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ToolName {
    Send,
    Wait,
    Last,
    Answer,
    List,
}

impl ToolName {
    fn of(name: &str) -> Option<Self> {
        match name {
            "send" => Some(Self::Send),
            "wait" => Some(Self::Wait),
            "last" => Some(Self::Last),
            "answer" => Some(Self::Answer),
            "list" => Some(Self::List),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Send => "send",
            Self::Wait => "wait",
            Self::Last => "last",
            Self::Answer => "answer",
            Self::List => "list",
        }
    }
}

// The advertised input schemas. They are never deserialised into: the handlers read the raw object.

#[derive(JsonSchema)]
#[allow(dead_code)]
struct SendInput {
    /// The instance to type into: [a-z0-9-], 1-32 characters, starting with a letter
    target: String,
    /// The prompt text, typed as one bracketed paste and read back after the fact
    text: String,
}

#[derive(JsonSchema)]
#[allow(dead_code)]
struct WaitInput {
    /// The instance to wait on: [a-z0-9-], 1-32 characters, starting with a letter
    target: String,
    /// Return the first event whose line starts at or after this cursor (a byte offset)
    after: Option<u64>,
    /// Give up after this many milliseconds; at most and by default 50000
    timeout_ms: Option<u64>,
}

#[derive(JsonSchema)]
#[allow(dead_code)]
struct LastInput {
    /// The instance to read: [a-z0-9-], 1-32 characters, starting with a letter
    target: String,
}

#[derive(JsonSchema)]
#[allow(dead_code)]
struct AnswerInput {
    /// The instance whose dialog to answer: [a-z0-9-], 1-32 characters, starting with a letter
    target: String,
    /// The pending dialog's id, as `wait` returned it
    dialog_id: u64,
    /// The answer: {"behavior": "allow"|"deny", "message"?} for a permission, {"behavior":
    /// "approve"|"reject", "feedback"?} for a plan, {"answers": {...}, "annotations"?} for a question
    response: Map<String, Value>,
}

#[derive(JsonSchema)]
#[allow(dead_code)]
struct ListInput {}

/// The five tools as `tools/list` returns them.
pub(crate) fn tool_list() -> Vec<Tool> {
    vec![
        Tool::new(
            "send",
            "Type a prompt into the named instance and read it back once it is submitted",
            Arc::new(JsonObject::new()),
        )
        .with_input_schema::<SendInput>(),
        Tool::new(
            "wait",
            "Wait for the named instance's next turn end, dialog or session end at or after a cursor",
            Arc::new(JsonObject::new()),
        )
        .with_input_schema::<WaitInput>(),
        Tool::new(
            "last",
            "Read the named instance's newest turn message",
            Arc::new(JsonObject::new()),
        )
        .with_input_schema::<LastInput>(),
        Tool::new(
            "answer",
            "Answer the named instance's pending dialog by its id",
            Arc::new(JsonObject::new()),
        )
        .with_input_schema::<AnswerInput>(),
        Tool::new(
            "list",
            "Show every session: its liveness, status, dialog and CLI, and the usage budget",
            Arc::new(JsonObject::new()),
        )
        .with_input_schema::<ListInput>(),
    ]
}

/// `tools/list`'s result. A client on protocol `2026-07-28` refuses a list result without `ttlMs`
/// and `cacheScope` (SEP-2549; measured on claude 2.1.287, which then drops every tool): the list is
/// marked stale at once and private to this session's client.
pub(crate) fn tools_result() -> ListToolsResult {
    ListToolsResult::with_all_items(tool_list())
        .with_ttl_ms(0)
        .with_cache_scope(CacheScope::Private)
}

/// What a call reads: the home, the server's own instance name (sent as `from`) and the board.
struct Ctx {
    home: PathBuf,
    from: Option<ViolaName>,
    list: ListProvider,
}

#[derive(Clone)]
pub(crate) struct Server {
    ctx: Arc<Ctx>,
}

impl Server {
    pub(crate) fn new(home: PathBuf, from: Option<ViolaName>, list: ListProvider) -> Self {
        Self {
            ctx: Arc::new(Ctx { home, from, list }),
        }
    }
}

/// `-32603` `internal error`, `data` null: an MCP call's own fault.
fn internal_fault() -> ErrorData {
    ErrorData::new(
        ErrorCode::INTERNAL_ERROR,
        McpError::Internal.to_string(),
        Some(Value::Null),
    )
}

impl ServerHandler for Server {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("viola", VERSION))
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(tools_result())
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        self.call(&request.name, request.arguments.unwrap_or_default())
            .await
            .map(Into::into)
    }
}

impl Server {
    /// One `tools/call`, as `serve` answers it. Each call runs as its own task, so a panic inside it
    /// is contained at that task and answered as the internal fault (obs-plan §7).
    pub(crate) async fn call(
        &self,
        name: &str,
        args: JsonObject,
    ) -> Result<CallToolResult, ErrorData> {
        let Some(tool) = ToolName::of(name) else {
            return Err(ErrorData::invalid_params(
                Cow::Borrowed("unknown tool"),
                Some(Value::Null),
            ));
        };
        let ctx = Arc::clone(&self.ctx);
        let task = tokio::spawn(async move { call(&ctx, tool, args).await });
        match task.await {
            Ok(Ok(result)) => Ok(result),
            Ok(Err(McpError::Internal)) | Err(_) => Err(internal_fault()),
        }
    }
}

#[instrument(skip_all, name = "mcp.tool_call", fields(tool = tool.as_str()))]
async fn call(ctx: &Ctx, tool: ToolName, args: JsonObject) -> Result<CallToolResult, McpError> {
    match tool {
        ToolName::Send => send(ctx, &args).await,
        ToolName::Wait => wait(ctx, &args).await,
        ToolName::Last => last(ctx, &args).await,
        ToolName::Answer => answer(ctx, &args).await,
        ToolName::List => list(ctx).await,
    }
}

/// A refused input names the field, a closed word.
type Checked<T> = Result<T, &'static str>;

fn target_of(args: &JsonObject) -> Checked<ViolaName> {
    args.get("target")
        .and_then(Value::as_str)
        .and_then(|name| ViolaName::try_new(name.to_owned()).ok())
        .ok_or("target")
}

fn text_of(args: &JsonObject) -> Checked<&str> {
    args.get("text").and_then(Value::as_str).ok_or("text")
}

/// Absent (or `null`) is `None`; anything but a `u64` is refused.
fn optional_u64(args: &JsonObject, field: &'static str) -> Checked<Option<u64>> {
    match args.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => value.as_u64().map(Some).ok_or(field),
    }
}

fn dialog_id_of(args: &JsonObject) -> Checked<u64> {
    args.get("dialog_id")
        .and_then(Value::as_u64)
        .ok_or("dialog_id")
}

fn response_of(args: &JsonObject) -> Checked<Response> {
    args.get("response")
        .filter(|value| value.is_object())
        .and_then(|value| Response::parse(value).ok())
        .ok_or("response")
}

/// The text's own refusals, in the wrapper's order: a refused character, then a typed text with
/// nothing in it.
fn refusal_of(text: &str) -> Option<NotDelivered> {
    if let Err(detail) = viola_core::validate_paste_text(text) {
        return Some(detail);
    }
    typed_text(text)
        .is_empty()
        .then_some(NotDelivered::EmptyText)
}

fn refused_client_side(detail: NotDelivered) -> CallToolResult {
    obs_event!(
        INFO,
        ObsEvent::SendRefused,
        side = "client",
        refusal = RefusalReason::NotDelivered.as_str(),
        detail = detail.as_str(),
    );
    result::refused(RefusalReason::NotDelivered.as_str(), Some(detail.as_str()))
}

/// `params` with `from` set to this server's own name, when it has one.
fn params_from(ctx: &Ctx, mut params: Map<String, Value>) -> Map<String, Value> {
    if let Some(from) = &ctx.from {
        params.insert("from".to_owned(), Value::from(AsRef::<str>::as_ref(from)));
    }
    params
}

/// One request to `target`'s wrapper after the liveness-only pre-check.
async fn frame(
    ctx: &Ctx,
    target: &ViolaName,
    method: &'static str,
    params: Map<String, Value>,
    refusable: bool,
) -> Result<CallToolResult, McpError> {
    let instance_dir = ctx
        .home
        .join("instances")
        .join(AsRef::<str>::as_ref(target));
    let Some(endpoint) = live_endpoint(&instance_dir) else {
        return Ok(result::unreachable());
    };
    let sent = match TokioClient::connect(&endpoint, "mcp").await {
        Ok(mut client) => client.request(method, params_from(ctx, params)).await,
        Err(error) => Err(error),
    };
    result::of_reply(sent, refusable)
}

async fn send(ctx: &Ctx, args: &JsonObject) -> Result<CallToolResult, McpError> {
    let (target, text) = match (target_of(args), text_of(args)) {
        (Ok(target), Ok(text)) => (target, text),
        (Err(field), _) | (_, Err(field)) => return Ok(result::invalid(field)),
    };
    if let Some(detail) = refusal_of(text) {
        return Ok(refused_client_side(detail));
    }
    let mut params = Map::new();
    params.insert("text".to_owned(), Value::from(text));
    frame(ctx, &target, "send", params, true).await
}

async fn wait(ctx: &Ctx, args: &JsonObject) -> Result<CallToolResult, McpError> {
    let checked = (|| {
        Ok::<_, &'static str>((
            target_of(args)?,
            optional_u64(args, "after")?,
            optional_u64(args, "timeout_ms")?,
        ))
    })();
    let (target, after, timeout_ms) = match checked {
        Ok(checked) => checked,
        Err(field) => return Ok(result::invalid(field)),
    };
    let mut params = Map::new();
    if let Some(after) = after {
        params.insert("after".to_owned(), Value::from(after));
    }
    params.insert(
        "timeout_ms".to_owned(),
        Value::from(wait_timeout(timeout_ms)),
    );
    frame(ctx, &target, "wait", params, false).await
}

async fn last(ctx: &Ctx, args: &JsonObject) -> Result<CallToolResult, McpError> {
    let target = match target_of(args) {
        Ok(target) => target,
        Err(field) => return Ok(result::invalid(field)),
    };
    frame(ctx, &target, "last", Map::new(), false).await
}

async fn answer(ctx: &Ctx, args: &JsonObject) -> Result<CallToolResult, McpError> {
    let checked =
        (|| Ok::<_, &'static str>((target_of(args)?, dialog_id_of(args)?, response_of(args)?)))();
    let (target, dialog_id, response) = match checked {
        Ok(checked) => checked,
        Err(field) => return Ok(result::invalid(field)),
    };
    let refused = response
        .free_text()
        .into_iter()
        .find_map(|text| viola_core::validate_paste_text(text).err());
    if let Some(detail) = refused {
        return Ok(result::refused(
            RefusalReason::NotDelivered.as_str(),
            Some(detail.as_str()),
        ));
    }
    let mut params = Map::new();
    params.insert("dialog_id".to_owned(), Value::from(dialog_id));
    params.insert("response".to_owned(), response.to_value());
    frame(ctx, &target, "answer", params, true).await
}

/// The board, read on a blocking thread: the provider reads the disk and one `claude agents --json`.
async fn list(ctx: &Ctx) -> Result<CallToolResult, McpError> {
    let provider = Arc::clone(&ctx.list);
    let document = tokio::task::spawn_blocking(move || provider())
        .await
        .map_err(|_| McpError::Internal)?;
    result::of_board(&document)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_json::json;

    use super::*;

    fn args(value: Value) -> JsonObject {
        value.as_object().expect("an object").clone()
    }

    fn runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime")
    }

    /// A server over an empty home: any frame tool that passed its checks reads `instance-unreachable`.
    fn empty_home() -> (tempfile::TempDir, Ctx) {
        let home = tempfile::tempdir().expect("tempdir");
        let ctx = Ctx {
            home: home.path().to_path_buf(),
            from: None,
            list: Arc::new(|| json!({"v": 1, "ok": {"items": []}})),
        };
        (home, ctx)
    }

    #[test]
    fn tool_list_is_exactly_the_five_tools_in_order() {
        let tools = tool_list();
        let names: Vec<&str> = tools.iter().map(|tool| tool.name.as_ref()).collect();
        assert_eq!(names, TOOL_NAMES);
        for tool in &tools {
            let shown = serde_json::to_string(tool).expect("json").to_lowercase();
            for word in ["release", "pause", "link", "unlink"] {
                assert!(!shown.contains(word), "{} names {word}", tool.name);
            }
            assert_eq!(tool.input_schema["type"], "object", "{}", tool.name);
        }
        let send = &tools[0].input_schema;
        assert_eq!(send["required"], json!(["target", "text"]));
    }

    /// The two SEP-2549 fields a `2026-07-28` client requires on a list result are on the wire.
    #[test]
    fn tools_result_carries_its_ttl_and_cache_scope() {
        let wire = serde_json::to_value(tools_result()).expect("json");
        assert_eq!(wire["ttlMs"], 0);
        assert_eq!(wire["cacheScope"], "private");
        assert_eq!(wire["tools"].as_array().map(Vec::len), Some(5));
    }

    /// The handler itself on the wire: a served `Server` answers `initialize` with the tools
    /// capability under its own name, then `tools/list` with the five tools and the two SEP-2549
    /// fields.
    #[test]
    fn served_initialize_advertises_tools_and_tools_list_carries_the_five() {
        use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _};
        let (_home, ctx) = empty_home();
        let server = Server { ctx: Arc::new(ctx) };
        let (initialized, listed) = runtime().block_on(async move {
            let (client, served) = tokio::io::duplex(1 << 16);
            let serving = tokio::spawn(async move {
                if let Ok(running) = rmcp::ServiceExt::serve(server, tokio::io::split(served)).await
                {
                    let _ = running.waiting().await;
                }
            });
            let (read, mut write) = tokio::io::split(client);
            let mut lines = tokio::io::BufReader::new(read).lines();
            let frames = [
                concat!(
                    r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"#,
                    r#""2026-07-28","capabilities":{},"clientInfo":{"name":"t","version":"0"}}}"#,
                    "\n"
                ),
                "{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n",
                "{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/list\"}\n",
            ];
            let mut answers = Vec::new();
            for (n, frame) in frames.into_iter().enumerate() {
                write.write_all(frame.as_bytes()).await.expect("a frame");
                if n != 1 {
                    let line = lines.next_line().await.expect("a read").expect("a line");
                    answers.push(serde_json::from_str::<Value>(&line).expect("a response"));
                }
            }
            write.shutdown().await.expect("the client's end");
            let [initialized, listed] = <[Value; 2]>::try_from(answers).expect("two responses");
            serving.await.expect("the served session");
            (initialized, listed)
        });
        assert!(
            initialized["result"]["capabilities"]["tools"].is_object(),
            "{initialized}"
        );
        assert_eq!(initialized["result"]["serverInfo"]["name"], "viola");
        let names: Vec<&str> = listed["result"]["tools"]
            .as_array()
            .expect("tools")
            .iter()
            .filter_map(|tool| tool["name"].as_str())
            .collect();
        assert_eq!(
            names,
            ["send", "wait", "last", "answer", "list"],
            "{listed}"
        );
        assert_eq!(listed["result"]["ttlMs"], 0);
        assert_eq!(listed["result"]["cacheScope"], "private");
    }

    #[rstest]
    #[case::absent(None, 50_000)]
    #[case::under(Some(1_000), 1_000)]
    #[case::at_the_cap(Some(50_000), 50_000)]
    #[case::one_over(Some(50_001), 50_000)]
    #[case::the_largest(Some(u64::MAX), 50_000)]
    fn wait_timeout_is_clamped_to_the_cap(#[case] given: Option<u64>, #[case] sent: u64) {
        assert_eq!(wait_timeout(given), sent);
    }

    /// Every refused input, before any frame: an `invalid-params` naming its field, or the
    /// `not-delivered` pair of a text.
    #[rstest]
    #[case::send_bad_target(ToolName::Send, json!({"target": "Not A Name", "text": "x"}), json!({"error": "invalid-params", "detail": "target"}))]
    #[case::send_no_target(ToolName::Send, json!({"text": "x"}), json!({"error": "invalid-params", "detail": "target"}))]
    #[case::send_target_not_string(ToolName::Send, json!({"target": 7, "text": "x"}), json!({"error": "invalid-params", "detail": "target"}))]
    #[case::send_text_not_string(ToolName::Send, json!({"target": "builder", "text": 7}), json!({"error": "invalid-params", "detail": "text"}))]
    #[case::send_no_text(ToolName::Send, json!({"target": "builder"}), json!({"error": "invalid-params", "detail": "text"}))]
    #[case::send_esc(ToolName::Send, json!({"target": "builder", "text": "a\u{1b}[2J"}), json!({"refusal": "not-delivered", "detail": "control-character"}))]
    #[case::send_empty(ToolName::Send, json!({"target": "builder", "text": ""}), json!({"refusal": "not-delivered", "detail": "empty-text"}))]
    #[case::send_lf_only(ToolName::Send, json!({"target": "builder", "text": "\n"}), json!({"refusal": "not-delivered", "detail": "empty-text"}))]
    #[case::wait_bad_after(ToolName::Wait, json!({"target": "builder", "after": "7"}), json!({"error": "invalid-params", "detail": "after"}))]
    #[case::wait_negative_after(ToolName::Wait, json!({"target": "builder", "after": -1}), json!({"error": "invalid-params", "detail": "after"}))]
    #[case::wait_bad_timeout(ToolName::Wait, json!({"target": "builder", "timeout_ms": 1.5}), json!({"error": "invalid-params", "detail": "timeout_ms"}))]
    #[case::last_bad_target(ToolName::Last, json!({"target": "../x"}), json!({"error": "invalid-params", "detail": "target"}))]
    #[case::answer_bad_dialog_id(ToolName::Answer, json!({"target": "builder", "dialog_id": "3", "response": {"behavior": "allow"}}), json!({"error": "invalid-params", "detail": "dialog_id"}))]
    #[case::answer_negative_dialog_id(ToolName::Answer, json!({"target": "builder", "dialog_id": -3, "response": {"behavior": "allow"}}), json!({"error": "invalid-params", "detail": "dialog_id"}))]
    #[case::answer_open_behavior(ToolName::Answer, json!({"target": "builder", "dialog_id": 3, "response": {"behavior": "maybe"}}), json!({"error": "invalid-params", "detail": "response"}))]
    #[case::answer_response_not_object(ToolName::Answer, json!({"target": "builder", "dialog_id": 3, "response": "allow"}), json!({"error": "invalid-params", "detail": "response"}))]
    #[case::answer_esc_message(ToolName::Answer, json!({"target": "builder", "dialog_id": 3, "response": {"behavior": "deny", "message": "\u{1b}"}}), json!({"refusal": "not-delivered", "detail": "control-character"}))]
    fn tool_inputs_are_revalidated_before_any_frame(
        #[case] tool: ToolName,
        #[case] given: Value,
        #[case] body: Value,
    ) {
        let (_home, ctx) = empty_home();
        let result = runtime()
            .block_on(call(&ctx, tool, args(given)))
            .expect("a result");
        assert_eq!(result.is_error, Some(true));
        assert_eq!(result.structured_content, Some(body));
    }

    /// Inputs that pass every check reach the liveness pre-check, which an empty home fails.
    #[rstest]
    #[case::send(ToolName::Send, json!({"target": "builder", "text": "hello\r\n"}))]
    #[case::wait_nulls(ToolName::Wait, json!({"target": "builder", "after": null, "timeout_ms": null}))]
    #[case::wait(ToolName::Wait, json!({"target": "builder", "after": 7, "timeout_ms": 900}))]
    #[case::last(ToolName::Last, json!({"target": "builder", "extra": true}))]
    #[case::answer(ToolName::Answer, json!({"target": "builder", "dialog_id": 3, "response": {"behavior": "approve"}}))]
    fn tool_inputs_that_pass_reach_the_liveness_check(
        #[case] tool: ToolName,
        #[case] given: Value,
    ) {
        let (_home, ctx) = empty_home();
        let result = runtime()
            .block_on(call(&ctx, tool, args(given)))
            .expect("a result");
        assert_eq!(
            result.structured_content,
            Some(json!({"error": "instance-unreachable", "detail": null}))
        );
    }

    #[test]
    fn list_returns_the_providers_ok_member() {
        let (_home, ctx) = empty_home();
        let result = runtime()
            .block_on(call(&ctx, ToolName::List, JsonObject::new()))
            .expect("a result");
        assert_eq!(result.is_error, Some(false));
        assert_eq!(result.structured_content, Some(json!({"items": []})));
    }

    #[test]
    fn list_provider_panic_is_an_internal_fault() {
        let home = tempfile::tempdir().expect("tempdir");
        let ctx = Ctx {
            home: home.path().to_path_buf(),
            from: None,
            list: Arc::new(|| panic!("provider")),
        };
        let got = runtime().block_on(call(&ctx, ToolName::List, JsonObject::new()));
        assert!(matches!(got, Err(McpError::Internal)));
    }

    #[test]
    fn tool_name_reads_only_the_five_names() {
        for name in TOOL_NAMES {
            assert_eq!(ToolName::of(name).map(ToolName::as_str), Some(name));
        }
        for name in ["release", "pause", "link", "unlink", "Send", ""] {
            assert_eq!(ToolName::of(name), None);
        }
    }

    #[test]
    fn params_from_carries_the_servers_own_name_when_it_has_one() {
        let (_home, mut ctx) = empty_home();
        assert_eq!(params_from(&ctx, Map::new()), Map::new());
        ctx.from = Some(ViolaName::try_new("driver".to_owned()).expect("a name"));
        assert_eq!(
            Value::Object(params_from(&ctx, Map::new())),
            json!({"from": "driver"})
        );
    }

    #[test]
    fn internal_fault_is_the_fixed_jsonrpc_error() {
        let fault = serde_json::to_value(internal_fault()).expect("json");
        assert_eq!(
            fault,
            json!({"code": -32603, "message": "internal error", "data": null})
        );
    }
}
