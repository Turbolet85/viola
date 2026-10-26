//! `viola last <name> [--json]`: the newest turn's `last_assistant_message` and `ts`, held by the
//! instance's wrapper (architecture §Standard Contracts, `last`). In human mode the message is
//! upstream text, so its control characters show as `\xHH` (design-system cli Component Patterns
//! 3); `--json` carries it serde-escaped.

use std::io::{self, IsTerminal as _};
use std::path::Path;
use std::process::ExitCode;

use serde_json::{Map, Value, json};
use tracing::instrument;
use viola_core::ViolaName;

use viola_state::liveness::{LiveEndpoint, live_endpoint};

use super::client::{self, CallError, Reply, Unreachable, clock_part, own_name};
use crate::{human, run};

#[derive(clap::Args)]
pub(crate) struct LastArgs {
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = super::send::parse_name)]
    pub(super) name: ViolaName,
    /// Print one JSON document on stdout instead of the message
    #[arg(long)]
    json: bool,
}

pub(crate) fn last(home: &Path, args: &LastArgs) -> anyhow::Result<ExitCode> {
    if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
        return Ok(ExitCode::from(code));
    }
    let home = client::start(home, &args.name)?;
    let name: &str = args.name.as_ref();
    let instance_dir = home.join("instances").join(name);
    let unreachable = |cause| {
        client::unreachable(args.json, cause, || {
            client::unable_unreachable(name, cause);
        })
    };
    let Some(live) = live_endpoint(&instance_dir) else {
        return Ok(ExitCode::from(unreachable(Unreachable::Dead("connect"))));
    };
    let mut params = Map::new();
    if let Some(from) = own_name() {
        params.insert("from".to_owned(), from.as_ref().into());
    }
    let ok = match client::answer_of(request(&live, params))? {
        Ok(Reply::Ok(ok)) => ok,
        Ok(Reply::Fault(detail)) => return Ok(ExitCode::from(client::fault(args.json, &detail))),
        Ok(Reply::Refused { .. }) => anyhow::bail!("last answered with a refusal"),
        Err(cause) => return Ok(ExitCode::from(unreachable(cause))),
    };
    if args.json {
        client::document(&json!({"v": 1, "ok": ok}));
    } else {
        show(name, &ok);
    }
    run::log_self_exit(0, None);
    Ok(ExitCode::SUCCESS)
}

#[instrument(skip_all, name = "last.client")]
fn request(live: &LiveEndpoint, params: Map<String, Value>) -> Result<Value, CallError> {
    Ok(client::connect(live)?.request("last", params)?)
}

/// The context line on stderr when it is a terminal, then the message on stdout, or `no message`
/// on stderr.
fn show(name: &str, ok: &Value) {
    if let Some(ts) = ok["ts"].as_str()
        && io::stderr().is_terminal()
    {
        let _ = human::write_last_turn(&mut io::stderr().lock(), name, clock_part(ts));
    }
    match ok["last_assistant_message"].as_str() {
        Some(message) => {
            let _ = human::write_message(&mut io::stdout().lock(), message);
        }
        None => {
            let _ = human::write_no_message(&mut io::stderr().lock());
        }
    }
}

#[cfg(test)]
#[cfg(not(target_os = "macos"))]
mod tests {
    use super::*;

    /// A server that is not the recorded wrapper is refused inside `last.client`, before any frame
    /// (obs-plan §4, the must-trace spans).
    #[test]
    fn request_refuses_an_unverified_server_inside_its_dispatch_span() {
        client::refused_inside::assert_refused_inside("last.client", |live| {
            request(live, Map::new())
        });
    }
}
