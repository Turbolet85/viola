//! `viola unlink <driver> <driven> [--json]`: the driven instance's wrapper removes `driver`'s
//! link to it. The same instance check, pre-check, server check and outputs as `viola link`.

use std::path::Path;
use std::process::ExitCode;

use serde_json::{Map, Value};
use tracing::instrument;
use viola_state::liveness::LiveEndpoint;

use super::client::{self, CallError};
use super::link::{LinkArgs, Verb, link_call};

pub(crate) fn unlink(home: &Path, args: &LinkArgs) -> anyhow::Result<ExitCode> {
    let code = link_call(home, args, Verb::Unlink, request)?;
    Ok(ExitCode::from(code))
}

#[instrument(skip_all, name = "cli.unlink")]
fn request(live: &LiveEndpoint, params: Map<String, Value>) -> Result<Value, CallError> {
    Ok(client::connect(live)?.request("unlink", params)?)
}

#[cfg(test)]
#[cfg(not(target_os = "macos"))]
mod tests {
    use super::*;

    /// A server that is not the recorded wrapper is refused inside `cli.unlink`, before any frame
    /// (obs-plan §4, the must-trace spans).
    #[test]
    fn request_refuses_an_unverified_server_inside_its_dispatch_span() {
        client::refused_inside::assert_refused_inside("cli.unlink", |live| {
            request(live, Map::new())
        });
    }
}
