//! `viola release <name> [--budget] [--json]`: the human hands the named instance's wheel back to
//! the driver ("you have control"), or with `--budget` lifts that instance's budget pause and
//! leaves the wheel where it is. A human verb: run with `VIOLA_NAME` set it carries `from`, and
//! the wrapper refuses it `-32602` `release-from-driver` (exit 20). The same liveness-only
//! pre-check and reply shapes as `viola pause`.

use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;

use chrono::Utc;
use serde_json::{Map, Value};
use tracing::instrument;
use viola_channel::{ChannelError, Client};
use viola_core::ViolaName;

use super::client;
use super::pause::wheel_call;
use crate::human;

#[derive(clap::Args)]
pub(crate) struct ReleaseArgs {
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = super::send::parse_name)]
    pub(super) name: ViolaName,
    /// Lift this instance's budget pause and leave the wheel where it is
    #[arg(long)]
    budget: bool,
    /// Print one JSON document on stdout instead of the result line
    #[arg(long)]
    json: bool,
}

pub(crate) fn release(home: &Path, args: &ReleaseArgs) -> anyhow::Result<ExitCode> {
    let home = client::start(home, &args.name)?;
    let code = wheel_call(
        &home,
        &args.name,
        args.json,
        params(args.budget),
        request,
        |name, ok| write_line(&mut io::stdout().lock(), name, ok, args.budget),
    )?;
    Ok(ExitCode::from(code))
}

/// The verb's own params: `budget: true` only when the flag is given.
fn params(budget: bool) -> Map<String, Value> {
    let mut params = Map::new();
    if budget {
        params.insert("budget".to_owned(), true.into());
    }
    params
}

/// The result line, read from the `ok` payload: a plain release names the holder; `--budget`
/// names the override's end when the reply holds one, and an open gate when it holds none.
fn write_line(out: &mut impl Write, name: &str, ok: &Value, budget: bool) -> io::Result<()> {
    let holder = ok["wheel"].as_str().unwrap_or_default();
    if !budget {
        return human::write_released(out, name, holder);
    }
    match ok["override_until"].as_str() {
        Some(end) => {
            let end = human::override_end(end, Utc::now().date_naive());
            human::write_budget_lifted(out, name, &end, holder)
        }
        None => human::write_budget_open(out, name, holder),
    }
}

#[instrument(skip_all, name = "release.client")]
fn request(endpoint: &str, params: Map<String, Value>) -> Result<Value, ChannelError> {
    Client::connect(endpoint, "cli")?.request("release", params)
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn release_args_take_the_budget_flag() {
        let plain = parse(&["builder"]);
        assert!(!plain.budget && !plain.json);
        let budget = parse(&["builder", "--budget", "--json"]);
        assert!(budget.budget && budget.json);
    }

    #[test]
    fn release_params_carry_budget_only_when_asked() {
        assert_eq!(Value::Object(params(false)), json!({}));
        assert_eq!(Value::Object(params(true)), json!({"budget": true}));
    }

    fn line(ok: &Value, budget: bool) -> String {
        let mut out = Vec::new();
        write_line(&mut out, "builder", ok, budget).expect("written");
        String::from_utf8(out).expect("utf-8")
    }

    #[test]
    fn release_line_of_a_plain_release_names_the_holder() {
        let ok = json!({"wheel": "driver", "budget_paused": true});
        assert_eq!(
            line(&ok, false),
            "builder  wheel driver  you have control\n"
        );
    }

    /// The override's end falls on a far later date than any run's, so the line carries the date.
    #[test]
    fn release_line_of_a_lifted_pause_is_derived_from_override_until() {
        let ok = json!({"wheel": "human", "budget_paused": false,
            "override_until": "2099-01-01T21:00:00.000Z"});
        assert_eq!(
            line(&ok, true),
            "builder  budget gate lifted until 2099-01-01 21:00:00Z  wheel human\n"
        );
    }

    #[test]
    fn release_line_with_no_pause_standing_says_the_gate_is_open() {
        let ok = json!({"wheel": "driver", "budget_paused": false});
        assert_eq!(line(&ok, true), "builder  budget gate open  wheel driver\n");
    }
}
