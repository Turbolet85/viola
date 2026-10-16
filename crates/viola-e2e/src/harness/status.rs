//! `status`: the test-plan §3 Status endpoint shape. `list` is `null` until `status` reads
//! `viola list`; `instances` carries pid + start-time liveness meanwhile.

use serde_json::{Value, json};

use super::{Outcome, Workspace, load_session};

pub fn status(ws: &Workspace, session: &str) -> Outcome {
    let record = match load_session(ws, "status", session) {
        Ok(r) => r,
        Err(out) => return out,
    };
    let instances: Vec<Value> = record
        .instances
        .iter()
        .map(
            |i| json!({"name": i.name, "wrapper_pid": i.wrapper_pid, "alive": i.wrapper().alive()}),
        )
        .collect();
    let ready = instances.iter().all(|i| i["alive"] == true);
    let state = if ready { "ready" } else { "degraded" };
    Outcome::new(
        json!({
            "v": 1, "cmd": "status", "ok": ready, "state": state,
            "pid": null, "uptime_ms": null, "last_error": null,
            "list": null,
            "instances": instances,
        }),
        ready,
    )
}
