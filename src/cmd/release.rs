//! `viola release <name> [--json]`: the human hands the named instance's wheel back to the driver
//! ("you have control"). A human verb: run with `VIOLA_NAME` set it carries `from`, and the
//! wrapper refuses it `-32602` `release-from-driver` (exit 20). The same liveness-only pre-check
//! and reply shapes as `viola pause`.

use std::io;
use std::path::Path;
use std::process::ExitCode;

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
    /// Print one JSON document on stdout instead of the result line
    #[arg(long)]
    json: bool,
}

pub(crate) fn release(home: &Path, args: &ReleaseArgs) -> anyhow::Result<ExitCode> {
    let home = client::start(home, &args.name)?;
    let code = wheel_call(&home, &args.name, args.json, request, |name, holder| {
        human::write_released(&mut io::stdout().lock(), name, holder)
    })?;
    Ok(ExitCode::from(code))
}

#[instrument(skip_all, name = "release.client")]
fn request(endpoint: &str, params: Map<String, Value>) -> Result<Value, ChannelError> {
    Client::connect(endpoint, "cli")?.request("release", params)
}
