//! `viola release <name> [--json]`: the human hands the named instance's wheel back to the driver
//! ("you have control"). A human verb: run with `VIOLA_NAME` set it carries `from`, and the wrapper
//! refuses it `-32602` `release-from-driver` (exit 20). The same pre-check, server check and reply
//! shapes as `viola pause`.

use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;

use serde_json::{Map, Value};
use tracing::instrument;
use viola_core::ViolaName;
use viola_state::liveness::LiveEndpoint;

use super::client::{self, CallError};
use super::pause::wheel_call;
use crate::human;

#[derive(clap::Args)]
pub(crate) struct ReleaseArgs {
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = super::send::parse_name)]
    pub(super) name: ViolaName,
    /// Print one JSON document on stdout instead of the result line
    #[arg(long)]
    json: bool,
}

pub(crate) fn release(home: &Path, args: &ReleaseArgs) -> anyhow::Result<ExitCode> {
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
        |name, ok| write_line(&mut io::stdout().lock(), name, ok),
    )?;
    Ok(ExitCode::from(code))
}

/// The result line, read from the `ok` payload: it names the holder.
fn write_line(out: &mut impl Write, name: &str, ok: &Value) -> io::Result<()> {
    human::write_released(out, name, ok["wheel"].as_str().unwrap_or_default())
}

#[instrument(skip_all, name = "release.client")]
fn request(live: &LiveEndpoint, params: Map<String, Value>) -> Result<Value, CallError> {
    Ok(client::connect(live)?.request("release", params)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A server that is not the recorded wrapper is refused inside `release.client`, before any frame
    /// (obs-plan §4, the must-trace spans).
    #[test]
    fn request_refuses_an_unverified_server_inside_its_dispatch_span() {
        client::refused_inside::assert_refused_inside("release.client", |live| {
            request(live, Map::new())
        });
    }
    use clap::Parser as _;
    use serde_json::json;

    #[derive(clap::Parser)]
    struct Line {
        #[command(flatten)]
        args: ReleaseArgs,
    }

    fn parse(words: &[&str]) -> ReleaseArgs {
        Line::try_parse_from(std::iter::once("release").chain(words.iter().copied()))
            .map(|line| line.args)
            .expect("parsed")
    }

    #[test]
    fn release_args_refuse_the_budget_flag() {
        let plain = parse(&["builder"]);
        assert!(!plain.json);
        assert!(parse(&["builder", "--json"]).json);
        let refused = Line::try_parse_from(["release", "builder", "--budget"])
            .err()
            .expect("the flag is refused");
        assert_eq!(refused.kind(), clap::error::ErrorKind::UnknownArgument);
        assert_eq!(refused.exit_code(), 2);
    }

    #[test]
    fn release_line_of_a_plain_release_names_the_holder() {
        let mut out = Vec::new();
        write_line(&mut out, "builder", &json!({"wheel": "driver"})).expect("written");
        assert_eq!(
            String::from_utf8(out).expect("utf-8"),
            "builder  wheel driver  you have control\n"
        );
    }
}
