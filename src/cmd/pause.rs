//! `viola pause <name> [--json]`: the human takes the named instance's wheel without a keystroke
//! ("I have control"); automated `send` and `answer` are then refused `human-typing` /
//! `manual-pause` until the human hands it back. The home and the instance's state files must pass
//! the strict-modes check, the instance must be live by its snapshot pid + start time and
//! heartbeat, and the process serving its endpoint its recorded wrapper, then one `pause` request;
//! the reply is one line or one `--json` document, and a typed exit code.

use std::io;
use std::path::Path;
use std::process::ExitCode;

use serde_json::{Map, Value, json};
use tracing::instrument;
use viola_core::ViolaName;

use viola_state::liveness::{LiveEndpoint, live_endpoint};

use super::client::{self, CallError, Reply, Unreachable, own_name};
use crate::{human, run};

#[derive(clap::Args)]
pub(crate) struct PauseArgs {
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = super::send::parse_name)]
    pub(super) name: ViolaName,
    /// Print one JSON document on stdout instead of the result line
    #[arg(long)]
    json: bool,
}

pub(crate) fn pause(home: &Path, args: &PauseArgs) -> anyhow::Result<ExitCode> {
    if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
        return Ok(ExitCode::from(code));
    }
    let home = client::start(home, &args.name)?;
    let code = wheel_call(
        &home,
        &args.name,
        args.json,
        Map::new(),
        request,
        |name, _| human::write_paused(&mut io::stdout().lock(), name),
    )?;
    Ok(ExitCode::from(code))
}

/// One wheel request (`pause` or `release`) after the liveness pre-check, over a connection whose
/// server is the recorded wrapper, its `params` the verb's own beside `from`: exit 0 with the `ok`
/// payload printed (`--json`, or `line` with the payload), 20 on a wrapper fault, 21 when the
/// instance cannot be reached or its server fails the check. A refusal or an `ok` without a holder
/// is no known reply.
pub(super) fn wheel_call(
    home: &Path,
    name: &ViolaName,
    json: bool,
    mut params: Map<String, Value>,
    request: fn(&LiveEndpoint, Map<String, Value>) -> Result<Value, CallError>,
    line: impl FnOnce(&str, &Value) -> io::Result<()>,
) -> anyhow::Result<u8> {
    let name: &str = name.as_ref();
    let instance_dir = home.join("instances").join(name);
    let unreachable = |cause| {
        client::unreachable(json, cause, || {
            client::unable_unreachable(name, cause);
        })
    };
    let Some(live) = live_endpoint(&instance_dir) else {
        return Ok(unreachable(Unreachable::Dead("connect")));
    };
    if let Some(from) = own_name() {
        params.insert("from".to_owned(), from.as_ref().into());
    }
    let ok = match client::answer_of(request(&live, params))? {
        Ok(Reply::Ok(ok)) => ok,
        Ok(Reply::Fault(detail)) => return Ok(client::fault(json, &detail)),
        Ok(Reply::Refused { .. }) => anyhow::bail!("a wheel request was refused"),
        Err(cause) => return Ok(unreachable(cause)),
    };
    if !ok["wheel"].is_string() {
        anyhow::bail!("malformed reply");
    }
    if json {
        client::document(&json!({"v": 1, "ok": ok}));
    } else {
        let _ = line(name, &ok);
    }
    run::log_self_exit(0, None);
    Ok(0)
}

#[instrument(skip_all, name = "pause.client")]
fn request(live: &LiveEndpoint, params: Map<String, Value>) -> Result<Value, CallError> {
    Ok(client::connect(live)?.request("pause", params)?)
}

#[cfg(test)]
#[cfg(not(target_os = "macos"))]
mod tests {
    use super::*;

    /// A server that is not the recorded wrapper is refused inside `pause.client`, before any frame
    /// (obs-plan §4, the must-trace spans).
    #[test]
    fn request_refuses_an_unverified_server_inside_its_dispatch_span() {
        client::refused_inside::assert_refused_inside("pause.client", |live| {
            request(live, Map::new())
        });
    }
}
