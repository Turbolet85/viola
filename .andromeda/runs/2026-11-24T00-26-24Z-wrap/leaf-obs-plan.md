@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `.c
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=67 class=omission
leaf-text: and from `viola revive`'s preflight `no-session|session-live|cwd-missing`
source: ctr-aful5fws «already-live, stale-heartbeat, squatted-name, pinned-hash-mismatch, internal-error, socket-dir-refused, from revive's preflight no-session, session-live, cwd-missing, and from viola open terminal-not-found and not-live»
replace-with: and from `viola revive`'s preflight `no-session|session-live|cwd-missing`, and from `viola open` `terminal-not-found|not-live` (each one `process-exit` line with no child started, bar `not-live`'s terminal)
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=67 class=omission
leaf-text: or `revive`'s `strict-modes-failed` document)
source: ctr-4y43s52d «run, revive's start, verify and open exit 1 on a strict-modes refusal; revive and open under --json print the strict-modes-failed document.»
replace-with: or the `strict-modes-failed` document of `revive` and of `viola open`, which also exits 1 on a strict-modes refusal)
@leaf_edit n=3 leaf=.claude/docs/obs-summary.md line=72 class=omission
leaf-text: (the start's chain runs `channel.bind` › `state.snapshot_write`)
source: ctr-6f6hckro «run.start (INTERNAL, root) › state.snapshot_recover › run.collision_check › run.pin_copy › run.version_gate › channel.bind › state.snapshot_write › state.heartbeat_start › pty.spawn (CLIENT); under --origin, run.window follows the trust step, before state.snapshot_write.»
replace-with: (the start's chain runs `channel.bind` › `state.snapshot_write`; under `--origin`, `run.window` follows the trust step, before `state.snapshot_write`, carrying the field `outcome` alone, never an address, pid or workspace)
@leaf_edit n=4 leaf=.claude/docs/obs-summary.md line=59 class=omission
leaf-text: and the MCP `list` result).
source: dec-zbecdqti «Window line has no trace of its own — The window line a start under --origin writes after its launch line is log-only product audit data: it wakes no wait, opens no span and is never copied into a process-log line.»
replace-with: and the MCP `list` result); NEVER copy the window line `{address, workspace, placement, origin}` a start under `--origin` writes into `events.ndjson` after its launch line into a process-log line — it is log-only audit data, waking no wait and opening no span.
@leaf_edit n=5 leaf=.claude/docs/obs-summary.md line=70 class=omission
leaf-text: their request inside `cli.link` / `cli.unlink`.
source: ctr-n7m2srv7 «CLI process-log file · viola open — Logs as cli into the opened instance's cli-<name>.ndjson, its work inside the cli.open dispatch span.»
replace-with: their request inside `cli.link` / `cli.unlink`. `viola open` logs as `cli` into the opened instance's `cli-<name>.ndjson`, its work inside the `cli.open` dispatch span, with a `subject:"terminal"` `process-start` / `process-exit` pair for the foot terminal window it spawns.
