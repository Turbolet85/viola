proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 17
    section: "## Established Decisions"
    change: "In the [PTY] row's fail-open sentence name the fifth cause: an absent, tampered, unreadable or unloadable sideload, or one whose `bin/<key>/conpty/` dir or a companion fails its owner/DACL check, fails open to kernel32's inbox ConPTY with no byte, refusal or exit change (`sideload_fallback` on `process-start`: `hash-mismatch` · `strict-modes-failed` · `unreadable` · `load-failed` · `not-built`; `strict-modes-failed` since chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "[PTY]: the sideload fail-open causes gain the companions' owner/DACL refusal, `sideload_fallback` `strict-modes-failed` (4 → 5 words)."
    rationale: "Report Schema / config: diag-line.v1.json line 70, the `process-start.sideload_fallback` enum gains `strict-modes-failed`; Symbols pin.rs (`PinError::StrictModes`) and run.rs:510 (`sideload_outcome`); Counts 7 names established-decisions.md:17 stale."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:24"
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 79
    section: "## Established Decisions"
    change: "- A verified client meets an unreadable or newer snapshot through `replay::read_snapshot_or_replay` (§Standard Contracts, Snapshot envelope), a replayed or absent one met as an absent one: the readers moved onto the replay at chunk 2026-10-26-home-and-code-bearing-file-integrity, on the founder's own boundary-widening ratification (P4 Q1, `inputs#I3`), and no product reader under `src/` stays on `read_snapshot`."
    sidecar: "[Message Broker / IPC]: the owed move of the snapshot readers onto `read_snapshot_or_replay` is done; the dated owner retired."
    rationale: "Report Retired: `read_snapshot` as a product reader under `src/` (3 → 0); Symbols liveness.rs 92-94, hook.rs 224-226, statusline.rs 128-132, run.rs 425-432; Decisions & corrections: the founder's P4 Q1 (`inputs#I3`); Counts 5 names established-decisions.md:79 stale."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 79
    section: "## Established Decisions"
    change: "- On the replay, `viola_state::liveness::live_endpoint` proceeds only on a real snapshot (a replayed one carries no `endpoint`, `pid` or `started_at`, so it gives no endpoint and no frame is written), hook `connect` reads a replayed or absent snapshot as unreachable (fail open, `channel-unreachable`), and `hook statusline`'s `serve` runs nothing; the `state.snapshot_recover` span now sits under `run.start` before `run.collision_check`, and under `hook.handle` on the hook path (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "[Message Broker / IPC]: what each moved reader does with a replayed snapshot, and where the `state.snapshot_recover` span now sits."
    rationale: "Report Symbols liveness.rs 92-94, hook.rs 224-226, statusline.rs 128-132; Deviations 4 (the span under `run.start` / `hook.handle`); task New facts F11."
    fact: F11
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 101
    section: "## Established Decisions"
    change: "In the SessionEnd tier, after «else nothing», add: a hook whose instance its instance check refuses, or which names no instance, appends nothing, neither by the channel nor directly ([Hook Contract]); a replayed snapshot is an endpoint not reached, so the direct `try_lock` append stands there as before."
    sidecar: "[Hook Transport]: SessionEnd appends nothing on a refused or absent instance; a replayed snapshot keeps the direct append."
    rationale: "Report Symbols src/cmd/hook.rs: a refused or absent instance returns exit 0 at once with no SessionEnd append; hook `connect` (224-226) maps `Replayed` / `Absent` to `NotSent::Unreachable`, SessionEnd's direct append as before."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 110
    section: "## Established Decisions"
    change: "- The event and dialog paths, once their instance check has passed ([Hook Contract]), share one connect: `replay::read_snapshot_or_replay` (a replayed or absent snapshot is unreachable: fail open, `channel-unreachable`), then `Client::connect_by`, then `verify_server` (§Standard Contracts, Session liveness). A serving process whose pid or OS start time is not the snapshot's, or a check that cannot be made, gets no frame: the hook records `server-verify-failed` and exits 0 with no body, the event lost as for an unreachable endpoint."
    sidecar: "[Hook Transport]: the shared connect reads the replay after the instance check; `read_snapshot` no longer read there (dated gap 1 retired)."
    rationale: "Report Retired (1) `viola hook`'s `hook.event` snapshot read without strict-modes, and `read_snapshot` 3 → 0 (hook.rs:204 gone); Symbols hook.rs 224-226; Counts 5 names established-decisions.md:110 stale."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 116
    section: "## Established Decisions"
    change: "- Beside the absent-`VIOLA_NAME` exit: the event and dialog paths take `instance_of` → `canonical` → `trusted` (`viola_state::strict::check_instance`) before `read_diagnostics_level` and obs init; a refused or absent instance exits 0 with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append. The `--capture` arms stay ahead of it, unchanged (chunk 2026-10-26-home-and-code-bearing-file-integrity; no line on a refused check, the founder's own word, P4 Q2, `inputs#I3`)."
    sidecar: "[Hook Contract]: the event and dialog paths' instance check before obs init, failing open with nothing done."
    rationale: "Report Symbols src/cmd/hook.rs (`canonical` 75-81, `trusted` 83-86, before `read_diagnostics_level` / `viola_obs_init`); Retired (1) and (6); Decisions & corrections, P4 Q2; task New facts F8."
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 151
    section: "## Established Decisions"
    change: "- The explicit verbs run `viola_state::strict::check_instance` on the home and `instances/<driven>` through `client::refuse_untrusted` (`src/cmd/client.rs`, shared since chunk 2026-10-26-home-and-code-bearing-file-integrity with the six frame verbs; behaviour unchanged) before `client::start`, so ahead of any role file, log or snapshot read, because opening a log narrows an existing home to 0700 and a check made after it passes a home at 0770; a refusal is exit 21. Then come the liveness-only pre-check (`live_endpoint`), the server identity check through `cmd::client::connect`, and one request."
    sidecar: "[Links]: the verbs' instance check now sits in `client::refuse_untrusted`, shared with the six frame verbs."
    rationale: "Report Symbols src/cmd/client.rs (`refuse_untrusted` 20-41; `link_call` link.rs:94-95 moved onto it unchanged in behaviour) and the remaining-caller facts (`src/cmd/link.rs:98` → `src/cmd/client.rs:31`)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 203
    section: "## Established Decisions"
    change: "After the pid + start time sentence add: `run`'s `collision_check` takes the caller's one read (`Option<&Recovered>`); over a replayed snapshot (no `pid` or `started_at`) it decides from the beat alone — at most `STALE_AFTER` old refuses `already-live`, older is free — and an error or an absent snapshot reads as free (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "[Session Liveness]: the collision check over a replayed snapshot reads the beat alone."
    rationale: "Report Symbols src/cmd/run.rs (`collision_check` 425-432; unit `collision_check_over_a_replayed_snapshot_reads_the_beat_alone` 1173-1224); task Touches, [Session Liveness] established-decisions.md:203-207."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 203
    section: "## Established Decisions"
    change: "- `viola revive`'s preflight reads the snapshot once and hands that read to both its collision check and its recorded-directory reading: one replay, one `state-recovered` line (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "[Session Liveness]: revive's preflight takes one snapshot read for two readings."
    rationale: "Report Symbols src/cmd/run.rs (revive's preflight reads once) and Deviations 4 (`cli_revive`'s one replay, one line); task New facts F12."
    fact: F12
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 205
    section: "## Established Decisions"
    change: "In the as-landed step chain add: before the wrapper log opens, `run` and a revived start check the home tree (`checked_exe`: the exe's bytes read once, `pin_key`, `check_home_tree(home, Some(name), Some(key))`; a refusal exit 1, no log, no line, no child), and the ConPTY sideload's `outcome` list becomes `loaded` · `hash-mismatch` · `strict-modes-failed` · `unreadable` · `load-failed` · `not-built`, never a refusal (`sideload_outcome` maps `PinError::StrictModes`)."
    sidecar: "[Session Liveness]: the start's home-tree check before its log, and the sideload outcome `strict-modes-failed`."
    rationale: "Report Symbols src/cmd/run.rs (`checked_exe` 189-200, `run` 166-172 and revive.rs 125-135 before `open_wrapper_log`; `sideload_outcome` run.rs:510)."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 207
    section: "## Established Decisions"
    change: "- The snapshot's `pid` and `started_at` are also the server identity: `viola_state::liveness::live_endpoint` returns them with the endpoint from one `replay::read_snapshot_or_replay` (`LiveEndpoint`, only on a real snapshot; a replayed or absent one gives none), and every frame-writing client first runs the strict-modes instance check (`client::refuse_untrusted`; the `viola mcp` frame tools' `check_instance`; the hook's `trusted`), then the liveness pre-check, then, on Linux and Windows, compares the connected peer's pid and its OS start time with them before its first frame (`verify_server`; §Standard Contracts, Session liveness)."
    sidecar: "[Session Liveness]: no frame-writing client runs liveness alone; the instance check comes first, and `live_endpoint` reads the replay."
    rationale: "Report Retired (4)-(8): each frame client now runs the strict-modes check before its first read or open; Symbols liveness.rs 92-94, client.rs 20-41, tools.rs 337-339, hook.rs 83-86; Counts 5 names established-decisions.md:207 stale."
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 213
    section: "## Established Decisions"
    change: "Name a third open divergence beside `viola-mcp`'s `ServeError`: `viola-state` holds `StampsError` (`StrictModes`, `State`; fixed messages, returned by `update_stamps`, its sole product caller `viola verify`'s `measure`) beside `StateError` (chunk 2026-10-26-home-and-code-bearing-file-integrity); the rule stands, and its fold or ratification is open."
    sidecar: "[Error Handling]: `viola-state`'s `StampsError` named as an open divergence from one enum per crate."
    rationale: "Report Symbols stamps.rs: new `pub enum StampsError { StrictModes(Refused), State(StateError) }` (37-44, fixed messages) with `From<io::Error>`; `update_stamps` returns it (was `StateError`); the row's rule is one enum per workspace crate, its divergences named."
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture.md
    near: 65
    section: "## Established Decisions"
    change: "In [Error Handling]: «and two open divergences, `viola-agent-claude`'s `Refusal` and `viola-mcp`'s `ServeError`» becomes «and three open divergences, `viola-agent-claude`'s `Refusal`, `viola-mcp`'s `ServeError` and `viola-state`'s `StampsError`»."
    sidecar: "[Error Handling] broad line: the open divergences 2 → 3 (`StampsError`)."
    rationale: "The broad line names the divergences its detail row names; report Symbols stamps.rs (`StampsError`)."
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture.md
    near: 106
    section: "## Conventions"
    change: "In Rust error types: «the two open divergences named there» becomes «the three open divergences named there», adding `viola-state`'s `StampsError` beside `StateError` to the parenthesis."
    sidecar: "Conventions, Rust error types: the open divergences 2 → 3 (`StampsError`)."
    rationale: "The convention line counts the [Error Handling] divergences; report Symbols stamps.rs (`StampsError`)."
    dependent-of: D-arch-decisions
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 225
    section: "## Established Decisions"
    change: "- As landed (chunk 2026-10-26-home-and-code-bearing-file-integrity): `viola_state::pin::pin_key(bytes)` names the key `<VERSION>-<16 hex>` once, and `pin_bytes(home, bytes)` pins already-checked bytes; `pin_exe` keeps its signature and delegates to it. `run`'s start reads its exe's bytes once, checks the home tree with that key (`bin/<key>/`, the pinned exe, `plugin/<key>/`, its `hooks/` and `.claude-plugin/` included) before its log, and pins those same bytes through `pin_bytes`; `viola verify` calls `pin_exe`."
    sidecar: "[Deployment / Distribution]: `pin_key` / `pin_bytes`, the start pinning the bytes its home-tree check hashed."
    rationale: "Report Symbols pin.rs (`pin_key` 71-74, `pin_bytes`, `pin_exe` delegating) and run.rs (`checked_exe` 189-200); Deviations 3; task New facts F13."
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 225
    section: "## Established Decisions"
    change: "- On Windows x64, before the held open, `pin_companions` runs `viola_state::strict::check_present` on `bin/<key>/conpty/` and each companion that exists: a refusal is `PinError::StrictModes`, writes nothing, and the child runs on the inbox ConPTY with `sideload_fallback` `strict-modes-failed`, the exit code unchanged (chunk 2026-10-26-home-and-code-bearing-file-integrity; the founder's dated re-hash-alone exception of 2026-09-29 retired)."
    sidecar: "[Deployment / Distribution]: the ConPTY companions' owner/DACL leg before the held open."
    rationale: "Report Symbols pin.rs (Windows `pin_companions` runs `strict::check_present`); Retired (3); Outcome (a refused companion falls back with `strict-modes-failed`, MET on `windows-2025`); Counts 18 names established-decisions.md:225 stale; task New facts F14."
    fact: F14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 23
    section: "## Conventions"
    change: "Add to exit `1`: `viola run` and `viola revive`'s start check the home tree before `open_wrapper_log` (`checked_exe` / `refuse_tree`); a refusal exits 1 with the home-state pair, or the instance-state pair for an instance-level refusal, no log opened, no line written and no child started; `revive --json` keeps `strict-modes-failed` (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "Exit codes: exit 1 gains the start's home-tree refusal (`run`, `revive`)."
    rationale: "Report Symbols run.rs (`checked_exe` 189-200, `refuse_tree` 202-215; `run` 166-172, revive.rs 125-135); Outcome (security) `run` refuses a 0770 home, exit 1 with its pair before any log; task New facts F4."
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 25
    section: "## Conventions"
    change: "Add to `viola verify`'s refusals: `check_home_tree(home, None, None)` (the home, `ledger/`, `bin/`, `plugin/`) runs before its `cli` log; a refusal exits 1 with the home-state pair, no step line, no summary and no line, and `measure` maps `StampsError::StrictModes` to the same pair (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "Exit codes: `viola verify`'s home-tree refusal, exit 1, the home pair."
    rationale: "Report Symbols verify.rs (64-70, `refuse_home` 95-99, `measure` 288-296); Outcome (security) `verify` refuses a widened `ledger/`; task New facts F5."
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 34
    section: "## Conventions"
    change: "Exit `21`'s refused-check list gains the six frame verbs: `send`, `wait`, `last`, `answer`, `pause` and `release` call `client::refuse_untrusted` first (`send` after its text read); a refused instance exits 21 with `link`'s instance-state pair, or under `--json` exactly the `instance-unreachable` document with `detail` null; nothing under the home is opened and no line is written (the founder's own word, P4 Q2 and Q3, `inputs#I3`)."
    sidecar: "Exit codes: exit 21 covers the six frame verbs' refused strict-modes check."
    rationale: "Report Symbols client.rs (`refuse_untrusted` 20-41, its six call sites); Decisions & corrections (P4 Q2, Q3); Counts 8; task Touches (conventions.md exit-code rows); task New facts F6."
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 100
    section: "## Conventions"
    change: "«`21` instance unreachable, and a refused strict-modes check of `viola list`, `viola plugin install`, `viola link` or `viola unlink`» becomes «`21` instance unreachable, and a refused strict-modes check of `viola list`, `viola plugin install`, `viola link`, `viola unlink` or a frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`)»."
    sidecar: "Conventions broad line: exit 21's strict-modes list gains the six frame verbs."
    rationale: "Report Symbols client.rs (`refuse_untrusted`, `Some(21)`, called by the six verbs); task Touches names architecture.md:100."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 40
    section: "## Conventions"
    change: "The last sentence becomes: the client's own refusals (exit 13) and the strict-modes instance check (exit 21, `client::refuse_untrusted`: the target's instance for the six frame verbs, `send` after its text read, the driven instance for `link` / `unlink`) come first, before any connect, and the server-check cause has no exit code, `--json` code or `RefusalReason` of its own."
    sidecar: "Exit 21 (server check): the instance check now precedes the connect on all eight verbs."
    rationale: "Report Symbols client.rs (`refuse_untrusted` called first by `send` 190-192, `wait` 43-45, `last` 30-32, `answer` 94-96, `pause` 32-34, `release` 35-37, and `link_call` 94-95)."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 53
    section: "## Conventions"
    change: "- A frame tool (`send`, `wait`, `last`, `answer`) first runs `check_instance` on the home and the instance, before `live_endpoint`; a refusal returns the `instance-unreachable` `isError` result with `detail: null`, text `refused: instance-unreachable`, and no frame or line is written (chunk 2026-10-26-home-and-code-bearing-file-integrity; the founder's dated exception of 2026-10-17 retired)."
    sidecar: "MCP results: the frame tools' strict-modes refusal is `instance-unreachable`, no frame, no line."
    rationale: "Report Symbols tools.rs `frame` (337-339); Retired (8); Outcome (security) `viola mcp` frame tools; task New facts F7."
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 24
    section: "## Standard Contracts"
    change: "The fallback's product readers go from two to six, each after a strict-modes check has passed: beside `viola revive`'s preflight (one read handed to its collision check and its recorded-directory reading) and `viola list`'s `board::read`, `viola_state::liveness::live_endpoint` (a replayed snapshot gives no endpoint), hook `connect` (replayed or absent: unreachable, fail open), `hook statusline`'s `serve` (no `statusline_command`, nothing runs) and `run`'s `collision_check` (the beat alone) (chunk 2026-10-26-home-and-code-bearing-file-integrity, the founder's P4 Q1, `inputs#I3`)."
    sidecar: "Snapshot envelope: the replay's product readers 2 → 6."
    rationale: "Report Counts 5 (replay product readers 2 → 6); Symbols liveness.rs 92-94, hook.rs 224-226, statusline.rs 128-132, run.rs 425-432; task Touches names standard-contracts.md:25."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 25
    section: "## Standard Contracts"
    change: "«`read_snapshot` keeps its results, and its three product callers (…) still read every such snapshot as absent» becomes: `read_snapshot` keeps its results but has no product caller (the rule: `read_snapshot(` under `src/`, outside test modules — 0, as measured at viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md); its three former callers and `live_endpoint` read the replay."
    sidecar: "Snapshot envelope: `read_snapshot`'s product callers 3 → 0, by its rule."
    rationale: "Report Retired (`read_snapshot` as a product reader under `src/`, 3 → 0) and the remaining-caller facts (hook.rs:204, statusline.rs:142, run.rs:383 at the base; the only HEAD hit inside `mod tests`)."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 35
    section: "## Standard Contracts"
    change: "`update_stamps` returns `StampsError` (`StrictModes` · `State`, fixed messages) and runs `strict::check_stamps` before creating `ledger/` and again under its lock before the capped read; a refusal reads and writes nothing (chunk 2026-10-26-home-and-code-bearing-file-integrity; the founder's dated stamps-read exception of 2026-09-28 retired)."
    sidecar: "Ledger stamps envelope: the writer's strict-modes check before `ledger/` is made, `StampsError`."
    rationale: "Report Symbols stamps.rs (`StampsError` 37-44, `update_stamps`); Deviations 1 (the check before `create_private_dir` as well as under the lock); Retired (2); task New facts F10."
    fact: F10
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 51
    section: "## Standard Contracts"
    change: "Add: before that comparison each client first runs the strict-modes instance check (`client::refuse_untrusted` for the six frame verbs and `link` / `unlink`; the hook's `trusted` for `hook.event` / `hook.dialog`; `check_instance` in the MCP frame tools), a refusal writing no frame and no line; its one snapshot read is `replay::read_snapshot_or_replay`, and only a real snapshot proceeds."
    sidecar: "Channel methods: the instance check precedes every client's server check; the one read is the replay."
    rationale: "Report Symbols client.rs 20-41, hook.rs 83-86 and 224-226, tools.rs 337-339, liveness.rs 92-94; Retired (1), (4)-(8)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 75
    section: "## Standard Contracts"
    change: "«sent id-less by `hook` once its server check has passed» becomes «sent id-less by `hook` once its instance check (`trusted`) and its server check have passed (a refused or absent instance, or a replayed or absent snapshot, sends nothing and the hook fails open)»."
    sidecar: "`hook.event`: sent only after the instance check too."
    rationale: "Report Symbols src/cmd/hook.rs: `canonical` / `trusted` before obs init (no frame on a refused or absent instance); `connect` 224-226 (`Replayed` / `Absent` → `NotSent::Unreachable`)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 93
    section: "## Standard Contracts"
    change: "Add: the six frame verbs' (`send`, `wait`, `last`, `answer`, `pause`, `release`) refused strict-modes check, made by `client::refuse_untrusted` before anything under the home is opened, prints the same `instance-unreachable` document on stdout (exit 21), with stderr empty and no line in any log."
    sidecar: "CLI `--json`: the `instance-unreachable` document is also the six frame verbs' strict-modes refusal."
    rationale: "Report Symbols client.rs (`refused_document(instance-unreachable)` under `--json`, `Some(21)`, no line); task Touches (CLI `--json` output, standard-contracts cli-json rows)."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 101
    section: "## Standard Contracts"
    change: "In the arm's order «reads the snapshot» becomes «reads the snapshot through `replay::read_snapshot_or_replay` (a replayed snapshot carries no `statusline_command`, so nothing runs or prints; chunk 2026-10-26-home-and-code-bearing-file-integrity)», and the canonicalisation and instance check it runs first are `canonical` / `trusted`, now in `src/cmd/hook.rs`, shared with the event and dialog paths."
    sidecar: "Hook statusline: the arm reads the replay; its `canonical` / `trusted` pair moved to `hook.rs`."
    rationale: "Report Symbols statusline.rs `serve` (128-132) and hook.rs (`canonical` 75-81, `trusted` 83-86 moved from `statusline.rs`); Retired (`read_snapshot`, statusline.rs:142 at the base)."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 109
    section: "## Standard Contracts"
    change: "«all from its one `read_snapshot` call, so a frame-writing client takes the liveness-only pre-check first and the server check after it» becomes «all from its one `replay::read_snapshot_or_replay` call, only on a real snapshot (a replayed or absent one, or an error, gives `None`), so a frame-writing client takes the strict-modes instance check first, then the liveness pre-check, then the server check»."
    sidecar: "Session liveness: `live_endpoint` reads the replay; the instance check precedes the liveness pre-check."
    rationale: "Report Symbols liveness.rs (92-94; unit `live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line`); Retired (4)-(8); Counts 5 names standard-contracts.md:109 stale."
note: F1-F3, F9, F15-F20 are occupied-resources facts and F21 a route fact, not this group's; the catch-site's narrowed reach on Unix (Count 13) rides F18, and the body's architecture.md:319 / :323 / :325 lines sit in §Occupied Resources.
