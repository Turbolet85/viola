//! `viola pause <name> [--json]`: the human takes the named instance's wheel without a keystroke
//! ("I have control"); automated `send` and `answer` are then refused `human-typing` /
//! `manual-pause` until the human hands it back. The instance must be live by its snapshot pid +
//! start time and heartbeat (the seventh dated gap), then one `pause` request; the reply is one
//! line or one `--json` document, and a typed exit code.

use std::io;
use std::path::Path;
use std::process::ExitCode;

use serde_json::{Map, Value, json};
use tracing::instrument;
use viola_channel::{ChannelError, Client};
use viola_core::ViolaName;

use super::client::{self, Reply, live_endpoint, own_name};
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
    let home = client::start(home, &args.name)?;
    let code = wheel_call(&home, &args.name, args.json, request, |name, _| {
        human::write_paused(&mut io::stdout().lock(), name)
    })?;
    Ok(ExitCode::from(code))
}

/// One wheel request (`pause` or `release`) after the liveness-only pre-check: exit 0 with the
/// `ok` payload printed (`--json`, or `line` with the reply's holder), 20 on a wrapper fault, 21
/// when the instance cannot be reached. A refusal or an `ok` without a holder is no known reply.
pub(super) fn wheel_call(
    home: &Path,
    name: &ViolaName,
    json: bool,
    request: fn(&str, Map<String, Value>) -> Result<Value, ChannelError>,
    line: impl FnOnce(&str, &str) -> io::Result<()>,
) -> anyhow::Result<u8> {
    let name: &str = name.as_ref();
    let instance_dir = home.join("instances").join(name);
    let unreachable =
        |during| client::unreachable(json, during, || client::unable_unreachable(name));
    let Some(endpoint) = live_endpoint(&instance_dir) else {
        return Ok(unreachable("connect"));
    };
    let mut params = Map::new();
    if let Some(from) = own_name() {
        params.insert("from".to_owned(), from.as_ref().into());
    }
    let ok = match client::answer_of(request(&endpoint, params))? {
        Ok(Reply::Ok(ok)) => ok,
        Ok(Reply::Fault(detail)) => return Ok(client::fault(json, &detail)),
        Ok(Reply::Refused { .. }) => anyhow::bail!("a wheel request was refused"),
        Err(during) => return Ok(unreachable(during)),
    };
    let Some(holder) = ok["wheel"].as_str() else {
        anyhow::bail!("malformed reply");
    };
    if json {
        client::document(&json!({"v": 1, "ok": ok}));
    } else {
        let _ = line(name, holder);
    }
    run::log_self_exit(0, None);
    Ok(0)
}

#[instrument(skip_all, name = "pause.client")]
fn request(endpoint: &str, params: Map<String, Value>) -> Result<Value, ChannelError> {
    Client::connect(endpoint, "cli")?.request("pause", params)
}
