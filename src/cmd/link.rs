//! `viola link <driver> <driven> [--json]`: the driven instance's wrapper records that `driver`
//! drives it. The verb's instance is the driven one: its home and state files pass the
//! strict-modes check before any log opens or any snapshot value is read, then the instance must
//! be live by its snapshot pid + start time and heartbeat, and the process serving its endpoint
//! its recorded wrapper, then one `link` request; the reply is one line or one `--json` document,
//! and a typed exit code. `viola unlink` shares the call.

use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;

use serde_json::{Map, Value, json};
use tracing::instrument;
use viola_core::ViolaName;
use viola_state::liveness::{LiveEndpoint, live_endpoint};

use super::client::{self, CallError, Reply, Unreachable, own_name};
use crate::{human, run};

#[derive(clap::Args)]
pub(crate) struct LinkArgs {
    /// The driving instance's name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = super::send::parse_name)]
    pub(super) driver: ViolaName,
    /// The driven instance's name, whose wrapper records the link
    #[arg(value_parser = super::send::parse_name)]
    pub(super) driven: ViolaName,
    /// Print one JSON document on stdout instead of the result line
    #[arg(long)]
    pub(super) json: bool,
}

pub(crate) fn link(home: &Path, args: &LinkArgs) -> anyhow::Result<ExitCode> {
    let code = link_call(home, args, Verb::Link, request)?;
    Ok(ExitCode::from(code))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Verb {
    Link,
    Unlink,
}

/// A reply's `ok`, once it holds the verb's keys: `link`'s string `since` and boolean
/// `already_linked`, `unlink`'s object. Any other shape is no known reply.
fn checked(verb: Verb, ok: Value) -> anyhow::Result<Value> {
    let known = match verb {
        Verb::Link => ok["since"].is_string() && ok["already_linked"].is_boolean(),
        Verb::Unlink => ok.is_object(),
    };
    anyhow::ensure!(known, "malformed reply");
    Ok(ok)
}

/// What the wrapper's reply comes to: an `ok` of the verb's shape, or a wrapper fault. A refusal
/// is no reply either method makes.
fn answered(verb: Verb, reply: Reply) -> anyhow::Result<Result<Value, Value>> {
    match reply {
        Reply::Ok(ok) => Ok(Ok(checked(verb, ok)?)),
        Reply::Fault(detail) => Ok(Err(detail)),
        Reply::Refused { .. } => anyhow::bail!("a link request was refused"),
    }
}

/// The result line, built from the verb's own parsed names and the reply's `since` /
/// `already_linked` alone.
fn write_line(
    out: &mut impl Write,
    verb: Verb,
    driver: &str,
    driven: &str,
    ok: &Value,
) -> io::Result<()> {
    match (verb, ok["already_linked"].as_bool()) {
        (Verb::Unlink, _) => human::write_unlinked(out, driver, driven),
        (Verb::Link, Some(true)) => human::write_already_linked(out, driver, driven),
        (Verb::Link, _) => {
            let since = ok["since"].as_str().unwrap_or_default();
            human::write_linked(out, driver, driven, since)
        }
    }
}

/// One `link` or `unlink` request to the driven instance's wrapper: exit 0 with the line or the
/// `ok` document, 20 on a wrapper fault, 21 when the instance's state files fail the strict-modes
/// check, it cannot be reached, or its server fails the check.
pub(super) fn link_call(
    home: &Path,
    args: &LinkArgs,
    verb: Verb,
    request: fn(&LiveEndpoint, Map<String, Value>) -> Result<Value, CallError>,
) -> anyhow::Result<u8> {
    let (driver, driven): (&str, &str) = (args.driver.as_ref(), args.driven.as_ref());
    if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
        return Ok(code);
    }
    let home = client::start(home, &args.driven)?;
    let unreachable = |cause| {
        client::unreachable(args.json, cause, || {
            client::unable_unreachable(driven, cause);
        })
    };
    let instance_dir = home.join("instances").join(driven);
    let Some(live) = live_endpoint(&instance_dir) else {
        return Ok(unreachable(Unreachable::without_endpoint(&instance_dir)));
    };
    let mut params = Map::new();
    params.insert("driver".to_owned(), driver.into());
    params.insert("driven".to_owned(), driven.into());
    if let Some(from) = own_name() {
        params.insert("from".to_owned(), from.as_ref().into());
    }
    let ok = match client::answer_of(request(&live, params))? {
        Ok(reply) => match answered(verb, reply)? {
            Ok(ok) => ok,
            Err(detail) => return Ok(client::fault(args.json, &detail)),
        },
        Err(cause) => return Ok(unreachable(cause)),
    };
    if args.json {
        client::document(&json!({"v": 1, "ok": ok}));
    } else {
        let _ = write_line(&mut io::stdout().lock(), verb, driver, driven, &ok);
    }
    run::log_self_exit(0, None);
    Ok(0)
}

#[instrument(skip_all, name = "cli.link")]
fn request(live: &LiveEndpoint, params: Map<String, Value>) -> Result<Value, CallError> {
    Ok(client::connect(live)?.request("link", params)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    /// A server that is not the recorded wrapper is refused inside `cli.link`, before any frame
    /// (obs-plan §4, the must-trace spans).
    #[cfg(not(target_os = "macos"))]
    #[test]
    fn request_refuses_an_unverified_server_inside_its_dispatch_span() {
        client::refused_inside::assert_refused_inside("cli.link", |live| request(live, Map::new()));
    }

    #[derive(Debug, PartialEq)]
    enum Read {
        Ok(Value),
        Fault(Value),
        Internal,
    }

    /// The verb's reply table: each accepted and refused `ok` shape, a refusal and a fault.
    #[rstest]
    #[case::link_new(Verb::Link, Reply::Ok(json!({"since": "t", "already_linked": false})), Read::Ok(json!({"since": "t", "already_linked": false})))]
    #[case::link_held(Verb::Link, Reply::Ok(json!({"since": "t", "already_linked": true})), Read::Ok(json!({"since": "t", "already_linked": true})))]
    #[case::link_no_since(Verb::Link, Reply::Ok(json!({"already_linked": false})), Read::Internal)]
    #[case::link_since_not_a_string(Verb::Link, Reply::Ok(json!({"since": 1, "already_linked": false})), Read::Internal)]
    #[case::link_no_flag(Verb::Link, Reply::Ok(json!({"since": "t"})), Read::Internal)]
    #[case::link_flag_not_a_bool(Verb::Link, Reply::Ok(json!({"since": "t", "already_linked": "no"})), Read::Internal)]
    #[case::link_empty(Verb::Link, Reply::Ok(json!({})), Read::Internal)]
    #[case::unlink_empty(Verb::Unlink, Reply::Ok(json!({})), Read::Ok(json!({})))]
    #[case::unlink_with_keys(Verb::Unlink, Reply::Ok(json!({"x": 1})), Read::Ok(json!({"x": 1})))]
    #[case::unlink_not_an_object(Verb::Unlink, Reply::Ok(json!([])), Read::Internal)]
    #[case::unlink_null(Verb::Unlink, Reply::Ok(Value::Null), Read::Internal)]
    #[case::link_refused(Verb::Link, Reply::Refused { reason: "not-delivered".to_owned(), detail: None }, Read::Internal)]
    #[case::unlink_refused(Verb::Unlink, Reply::Refused { reason: "unknown".to_owned(), detail: None }, Read::Internal)]
    #[case::link_fault(Verb::Link, Reply::Fault(json!({"code": -32602, "message": "invalid params", "data": null})), Read::Fault(json!({"code": -32602, "message": "invalid params", "data": null})))]
    #[case::unlink_fault(Verb::Unlink, Reply::Fault(json!({"code": -32603, "message": "internal error", "data": null})), Read::Fault(json!({"code": -32603, "message": "internal error", "data": null})))]
    fn link_reply_is_read_by_its_verbs_shape(
        #[case] verb: Verb,
        #[case] reply: Reply,
        #[case] want: Read,
    ) {
        let got = match answered(verb, reply) {
            Ok(Ok(ok)) => Read::Ok(ok),
            Ok(Err(detail)) => Read::Fault(detail),
            Err(_) => Read::Internal,
        };
        assert_eq!(got, want);
    }

    fn line(verb: Verb, ok: &Value) -> String {
        let mut out = Vec::new();
        write_line(&mut out, verb, "overseer", "builder", ok).expect("written");
        String::from_utf8(out).expect("utf-8")
    }

    #[test]
    fn link_line_is_built_from_the_names_and_the_reply() {
        let new = json!({"since": "2026-10-26T14:03:09.512Z", "already_linked": false});
        assert_eq!(
            line(Verb::Link, &new),
            "link    overseer -> builder  since 14:03:09Z\n"
        );
        let held = json!({"since": "2026-10-26T14:03:09.512Z", "already_linked": true});
        assert_eq!(
            line(Verb::Link, &held),
            "link    overseer -> builder  already linked\n"
        );
        assert_eq!(
            line(Verb::Unlink, &json!({})),
            "unlink  overseer -> builder\n"
        );
    }
}
