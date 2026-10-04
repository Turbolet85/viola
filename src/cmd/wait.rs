//! `viola wait <name> [--after <cursor>] [--timeout-ms <n>] [--json]`: parks on the instance's
//! wrapper until the next driver-relevant event whose line starts at or after `--after` (the log's
//! end at the call without it), returned at once when it is already logged (architecture
//! [Message Broker / IPC]). The result is one line or one `--json` document and a typed exit.

use std::io::{self, IsTerminal as _};
use std::path::Path;
use std::process::ExitCode;

use serde_json::{Map, Value, json};
use tracing::instrument;
use viola_channel::{ChannelError, Client};
use viola_core::ViolaName;

use super::client::{self, Reply, clock_part, live_endpoint, own_name};
use crate::{human, run};

#[derive(clap::Args)]
pub(crate) struct WaitArgs {
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = super::send::parse_name)]
    pub(super) name: ViolaName,
    /// Return the first event whose line starts at or after this cursor (a byte offset)
    #[arg(long, value_name = "CURSOR")]
    after: Option<u64>,
    /// Give up after this many milliseconds with `timed out`
    #[arg(long, value_name = "MS")]
    timeout_ms: Option<u64>,
    /// Print one JSON document on stdout instead of the result line
    #[arg(long)]
    json: bool,
}

/// The kinds a dialog wakes `wait` with: their line names the dialog, not the time.
const DIALOG_KINDS: [&str; 3] = ["question", "permission", "plan"];

pub(crate) fn wait(home: &Path, args: &WaitArgs) -> anyhow::Result<ExitCode> {
    let home = client::start(home, &args.name)?;
    let name: &str = args.name.as_ref();
    let instance_dir = home.join("instances").join(name);
    let unreachable =
        |during| client::unreachable(args.json, during, || client::unable_unreachable(name));
    let Some(endpoint) = live_endpoint(&instance_dir) else {
        return Ok(ExitCode::from(unreachable("connect")));
    };
    let mut params = Map::new();
    if let Some(after) = args.after {
        params.insert("after".to_owned(), after.into());
    }
    if let Some(timeout_ms) = args.timeout_ms {
        params.insert("timeout_ms".to_owned(), timeout_ms.into());
    }
    if let Some(from) = own_name() {
        params.insert("from".to_owned(), from.as_ref().into());
    }
    if !args.json && io::stderr().is_terminal() {
        let _ = human::write_waiting(&mut io::stderr().lock(), name);
    }
    let sent = request(&endpoint, args.after, args.timeout_ms, params);
    let ok = match client::answer_of(sent)? {
        Ok(Reply::Ok(ok)) => ok,
        Ok(Reply::Fault(detail)) => return Ok(ExitCode::from(client::fault(args.json, &detail))),
        Ok(Reply::Refused { .. }) => anyhow::bail!("wait answered with a refusal"),
        Err(during) => return Ok(ExitCode::from(unreachable(during))),
    };
    if args.json {
        client::document(&json!({"v": 1, "ok": ok}));
    } else {
        show(name, &ok, args.timeout_ms);
    }
    run::log_self_exit(0, None);
    Ok(ExitCode::SUCCESS)
}

#[instrument(skip_all, name = "wait.block", fields(after = after, timeout_ms = timeout_ms))]
fn request(
    endpoint: &str,
    after: Option<u64>,
    timeout_ms: Option<u64>,
    params: Map<String, Value>,
) -> Result<Value, ChannelError> {
    Client::connect(endpoint, "cli")?.request("wait", params)
}

/// The result line on stdout (design-system cli Component Patterns 3).
fn show(name: &str, ok: &Value, timeout_ms: Option<u64>) {
    let mut out = io::stdout().lock();
    if ok["timed_out"] == true {
        let _ = human::write_timed_out(&mut out, name, timeout_ms.unwrap_or_default());
        return;
    }
    let event = &ok["event"];
    let kind = event["kind"].as_str().unwrap_or("unknown");
    let cursor = ok["cursor"].as_u64().unwrap_or_default();
    if DIALOG_KINDS.contains(&kind) {
        let dialog = event["data"]["dialog_id"]
            .as_u64()
            .map_or_else(|| "unknown".to_owned(), |id| id.to_string());
        let _ = human::write_woken_dialog(&mut out, kind, name, &dialog, cursor);
    } else {
        let at = clock_part(event["ts"].as_str().unwrap_or("unknown"));
        let _ = human::write_woken(&mut out, kind, name, at, cursor);
    }
}
