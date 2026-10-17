//! The stdio MCP server for drivers (architecture §Established Decisions [MCP]): the tools `send ·
//! wait · last · answer · list`, each answering what its CLI verb's `--json` form answers less `v`,
//! a refusal as a code-only `isError` result. No wheel or link tool exists here. The workspace's one
//! Tokio runtime lives in this crate; the root bin hands in the home, the server's own instance name
//! and the board, and this crate reads no environment variable.

mod result;
mod tools;

use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use rmcp::ServiceExt as _;
use rmcp::model::{CallToolResult, ErrorData, JsonObject};
use serde_json::Value;
use viola_core::ViolaName;

pub use tools::{TOOL_NAMES, WAIT_CAP, wait_timeout};

/// The board as CLI `list --json` prints it: `{"v":1,"ok":{…}}`, or its refusal document. Called on
/// a blocking thread, once per `list` call.
pub type ListProvider = Arc<dyn Fn() -> Value + Send + Sync>;

/// A fault of an MCP call itself, never of the wrapper it reached: rmcp answers it as the JSON-RPC
/// error `-32603` `internal error` with `data` null. Its message is fixed (security-plan §Error
/// Handling).
#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error("internal error")]
    Internal,
}

/// Why `serve` stopped before stdin closed. Fixed messages only; the cause stays the source.
#[derive(Debug, thiserror::Error)]
pub enum ServeError {
    #[error("the MCP runtime could not be built")]
    Runtime(#[source] io::Error),
    #[error("the MCP session could not be initialized")]
    Initialize,
    #[error("the MCP session ended abnormally")]
    Session,
}

/// The five tools without a transport: each call answered as `serve` answers a `tools/call`.
pub struct Tools(tools::Server);

impl Tools {
    pub fn new(home: PathBuf, own_name: Option<ViolaName>, list: ListProvider) -> Self {
        Self(tools::Server::new(home, own_name, list))
    }

    /// Runs inside a Tokio runtime. An unknown tool is the JSON-RPC `-32602`, an internal fault
    /// `-32603` `internal error` with `data` null; every other outcome is a result.
    pub async fn call(&self, name: &str, args: JsonObject) -> Result<CallToolResult, ErrorData> {
        self.0.call(name, args).await
    }
}

/// Serves the five tools over stdin / stdout until stdin closes. stdout carries MCP frames only.
pub fn serve(
    home: PathBuf,
    own_name: Option<ViolaName>,
    list: ListProvider,
) -> Result<(), ServeError> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(ServeError::Runtime)?;
    runtime.block_on(async move {
        let server = tools::Server::new(home, own_name, list);
        let running = server
            .serve(rmcp::transport::stdio())
            .await
            .map_err(|_| ServeError::Initialize)?;
        match running.waiting().await {
            Ok(rmcp::service::QuitReason::JoinError(_)) | Err(_) => Err(ServeError::Session),
            Ok(_) => Ok(()),
        }
    })
}
