//! `viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json]`: the named running session
//! moved to the given model and effort, each half not given kept as recorded. Its wrapper ends its
//! own claude child and resumes the session on the new pair in the same terminal; nothing is
//! written to the user's Claude settings. The home and the instance's state files must pass the
//! strict-modes check, the instance must be live by its snapshot pid + start time and heartbeat,
//! and the process serving its endpoint its recorded wrapper, then one `switch` request; the reply
//! is one line or one `--json` document, and a typed exit code.

use std::io;
use std::path::Path;
use std::process::ExitCode;

use serde_json::{Map, Value, json};
use tracing::instrument;
use viola_core::ViolaName;
use viola_state::liveness::{LiveEndpoint, live_endpoint};

use super::client::{self, CallError, Reply, Unreachable};
use super::send::{exit_of, reason_of};
use crate::{human, run};

#[derive(clap::Args)]
#[command(group(clap::ArgGroup::new("pair").required(true).multiple(true).args(["model", "effort"])))]
pub(crate) struct SwitchArgs {
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = super::send::parse_name)]
    pub(super) name: ViolaName,
    /// The model to switch to: an alias or a full name; else the recorded one
    #[arg(long, value_name = "MODEL", value_parser = super::run::parse_model)]
    model: Option<String>,
    /// The effort to switch to: low, medium, high, xhigh or max; else the recorded one
    #[arg(long, value_name = "EFFORT", value_parser = super::run::parse_effort)]
    effort: Option<String>,
    /// Print one JSON document on stdout instead of the result line
    #[arg(long)]
    json: bool,
}

pub(crate) fn switch(home: &Path, args: &SwitchArgs) -> anyhow::Result<ExitCode> {
    if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
        return Ok(ExitCode::from(code));
    }
    let home = client::start(home, &args.name)?;
    let code = switch_call(&home, args)?;
    Ok(ExitCode::from(code))
}

fn switch_call(home: &Path, args: &SwitchArgs) -> anyhow::Result<u8> {
    let name: &str = args.name.as_ref();
    let instance_dir = home.join("instances").join(name);
    let unreachable = |cause| {
        client::unreachable(args.json, cause, || {
            client::unable_unreachable(name, cause);
        })
    };
    let Some(live) = live_endpoint(&instance_dir) else {
        return Ok(unreachable(Unreachable::without_endpoint(&instance_dir)));
    };
    let mut params = Map::new();
    if let Some(model) = &args.model {
        params.insert("model".to_owned(), model.as_str().into());
    }
    if let Some(effort) = &args.effort {
        params.insert("effort".to_owned(), effort.as_str().into());
    }
    let code = match client::answer_of(request(&live, params))? {
        Ok(Reply::Ok(ok)) => {
            let (Some(model), Some(effort)) = (ok["model"].as_str(), ok["effort"].as_str()) else {
                anyhow::bail!("malformed reply");
            };
            if args.json {
                client::document(&json!({"v": 1, "ok": ok}));
            } else {
                let _ = human::write_switched(&mut io::stdout().lock(), name, model, effort);
            }
            0
        }
        Ok(Reply::Refused { reason, detail }) => {
            refused(name, args.json, &reason, detail.as_deref());
            exit_of(reason_of(&reason))
        }
        Ok(Reply::Fault(detail)) => return Ok(client::fault(args.json, &detail)),
        Err(cause) => return Ok(unreachable(cause)),
    };
    run::log_self_exit(code, None);
    Ok(code)
}

/// A refusal: its document, or `unable  <name>  <reason>  <detail>` with the hint of its detail
/// (of its reason for `human-typing` and `unverified-cli`) last.
fn refused(name: &str, json: bool, reason: &str, detail: Option<&str>) {
    if json {
        return client::document(&json!({"v": 1, "refusal": reason, "detail": detail}));
    }
    let shown = match detail {
        Some(detail) => format!("{reason}  {detail}"),
        None => reason.to_owned(),
    };
    let cause = match reason {
        "human-typing" | "unverified-cli" => Some(reason),
        "not-delivered" => detail,
        _ => None,
    };
    let hint = cause.and_then(|cause| human::send_hint(name, cause));
    let _ = human::write_unable(&mut io::stderr().lock(), name, &shown, hint.as_deref());
}

/// The connection, its server checked, then the one `switch` frame.
#[instrument(skip_all, name = "cli.switch")]
fn request(live: &LiveEndpoint, params: Map<String, Value>) -> Result<Value, CallError> {
    Ok(client::connect(live)?.request("switch", params)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A server that is not the recorded wrapper is refused inside `cli.switch`, before any frame.
    #[test]
    fn switch_request_refuses_an_unverified_server_inside_its_dispatch_span() {
        client::refused_inside::assert_refused_inside("cli.switch", |live| {
            request(live, Map::new())
        });
    }
}
