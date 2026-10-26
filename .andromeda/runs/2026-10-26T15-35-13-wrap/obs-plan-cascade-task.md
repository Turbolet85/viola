# Task — obs-plan · 2026-10-26-explicit-session-links

## The session
The driven instance's wrapper now serves the channel methods `link` and `unlink` (before: both answered `-32601`); served channel methods go from seven plus `hook.event` to nine plus `hook.event`.
New CLI verbs `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]`, whose instance is the driven one: `check_instance` on the driven instance BEFORE any role file or log opens (refused: exit 21 `instance-unreachable`, no line in any log, mode as found), then `process-start` in `cli-<driven>`, the liveness pre-check, the server identity check through `cmd::client::connect` inside the dispatch spans `cli.link` / `cli.unlink`, one frame.
The CLI frame verbs go from six to eight (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`); each writes `server-verify-failed` on a server mismatch; only `link` / `unlink` also run `check_instance` first, the founder's form, no dated gap.
`LinkSet::remove` gets its first product caller, so a product path now appends an `unlink` event line (`source: wrapper`, `{driver, driven}`) to `events.ndjson` for the first time.
Retired: the hold stating the explicit `link` / `unlink` verbs and methods are not built, the `-32601` answer for them, and `link` / `unlink` as the unserved-method example in the channel logging test (now `anything`).
Measured: an unserved method's conn-less `channel-response` line carries no `method` key (the channel's closed method-label list); `human::refuse` call sites 14 to 15; `-32602` gains two causes (a bad `link` / `unlink` name or pair).

## Reached files
- src/cmd/link.rs
- src/cmd/unlink.rs
- src/cmd/run.rs
- src/cmd/mod.rs
- src/run/links.rs
- src/human/verbs.rs
- src/human.rs
- tests/channel_links.rs
- tests/cli_links.rs
- tests/channel_endpoint.rs
- tests/cli_output_plain.rs
- tests/security_negatives_server_identity.rs
- .config/nextest.toml
- viola-0.1.0/verification-matrix.json
- viola-0.1.0/working-route.md

## Touches
- §4 Edge flow E3 (obs-plan.md:392 and its detail in span-trace-coverage, line 177): stale — it says only the implicit link exists and the explicit `link` / `unlink` methods are not built; both are now served, and a product `unlink` line is now written.
- The CLI frame-verb count six → eight in span-trace-coverage line 28 (the six frame verbs' server check inside their dispatch span; `cmd::client::connect` callers 6 → 8).
- The six named writers of `server-verify-failed` in log-coverage line 29: now eight, `link` and `unlink` added, each `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN inside its dispatch span.
- Span naming in §2 (obs-plan.md:293) and span-trace-coverage line 18: the `cli.<verb>` dispatch-span rule now also covers `cli.link` and `cli.unlink`; a list of `cli.*` spans or CLIENT spans that enumerates verbs must carry them.
- §3 human-stderr refusal writer census (obs-plan.md:311): "run (5), verify (5), list (2), revive (1) and plugin install (1)" is stale — 15 sites, `link` 1 added (shared by both verbs, the fixed instance-state `unable` / `hint:` pair).
- §5 `viola.compat.v_rejected` over-count (obs-plan.md:418 and metric-coverage line 28): `-32602` now also answers a bad `link` / `unlink` name, a `driven` that is not the serving wrapper and a self-pair.
- §1 scope-surfaces cli verb list (obs-plan.md:109) already names `link · unlink`: re-read true; its exporter line (instance from an argument) now applies to them, the instance being the driven one.
- §1 Vector 7 trigger (obs-plan.md:215, "a strict-modes failure has a line only on the ledger-stamps read"): re-read — the verbs' refused instance check writes no line, so the claim holds, and any list of verbs refused on strict-modes must add `link` / `unlink`.
- The refusing-verb enumerations of instance strict-modes checks in obs-scope-summary (lines 370, 397), slo-invariants-telemetry-budgets line 17 and telemetry-strategy line 20, which name `revive` / `hook statusline` / `list`: extend with `link` / `unlink` (check before any log, exit 21).
- §10 exemption 7 and the §7 pre-init window (obs-plan.md:529, "the instance and home checks that run before any role file is opened"): the verbs' `check_instance` runs inside that window, before `client::start`, so its refusal has no file and no line.
- The `wait` / `last` scenario's "`strict-modes-failed` cannot occur on them, its check owed to Home and code-bearing file integrity" (obs-plan.md:361): still true for `wait` / `last`; any text generalising it to every CLI frame verb is now wrong for `link` / `unlink`.
- The served channel-method set wherever obs-plan enumerates it or the closed `method` label: `link` and `unlink` are now served and their `channel-request` / `channel-response` pair carries `method:"link"|"unlink"`.
- §6 boundary-call wrapper IPC row: re-read — the wrapper's `link` / `unlink` handling opens no span and writes no process-log line beyond the existing channel pair, which holds no names.
- `events.ndjson` writers: a product path now writes the `unlink` kind (already in the closed set); any text saying no product path appends `unlink` is stale.
- The `cli-<name>.ndjson` producers: `viola link` / `viola unlink` write `process-start` / `process-exit` to `cli-<driven>.ndjson` (not the driver's).
- Exit classes at the verbs' edge reach existing `process-exit` levels: a wrapper error (`-32602` included) is exit 20, Dead / ServerUnverified exit 21, a `Refused` or an `ok` of another shape internal error exit 1.
- Test inventory cited by obs: `tests/security_negatives_server_identity.rs` 19 → 25 cases (eight verbs × two modes plus controls); the two `request_refuses_an_unverified_server_inside_its_dispatch_span` cases compiled out on macOS (the recorded gap).
- New obs-assertion tests: `link_verbs_log_the_channel_pair_and_no_names`, `link_verbs_refuse_a_widened_instance_before_any_log`, `channel_link_names_are_validated_by_the_wrapper`.
- §1 Tests E3 coverage line (obs-plan.md:183, E3 "`link` / `unlink` transfer markers" covered by triggers): re-read true; E3 steps 1 and 3 landed.
- Every obs-plan line naming the route entry "Explicit session links" as the pending owner of the hold: stale, the entry is built.

## New facts
- F1 · span-trace-coverage · `viola link` / `viola unlink` each open a CLIENT dispatch span `cli.link` / `cli.unlink` (`#[instrument(skip_all, name = ...)]`) around their one request; a server-verify refusal is logged inside it.
- F2 · span-trace-coverage · E3: the wrapper serves `link` / `unlink`, validating `driver` / `driven` through `ViolaName::try_new`, the own-name rule and the self-pair rule; a refusal is `-32602` with `data: null` and no line written, a record or remove error `-32603`; neither method reads `from`.
- F3 · span-trace-coverage · E3: a served `link` / `unlink` opens no span and writes no process-log line of its own; the wrapper's existing `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no driver or driven name.
- F4 · span-trace-coverage · E3: `unlink` appends a product `unlink` event line (`source: wrapper`, `data: {driver, driven}`) only when a pair is removed; a second `link` of a held pair and an `unlink` of an absent pair append nothing.
- F5 · error-capture-reporting · `viola link` / `viola unlink` run `check_instance` on the driven instance before any role file or log opens; a refusal is exit 21 `instance-unreachable` (human: the fixed instance-state pair; `--json`: `{"v":1,"error":"instance-unreachable","detail":null}`) with no line in any log and the mode as found.
- F6 · slo-invariants-telemetry-budgets · the verbs' refused instance check joins the pre-init no-file window: exit status is its only evidence, no `process-start` precedes it.
- F7 · log-coverage · an unserved method's conn-less `channel-response` line carries no `method` key: the channel logs `method` only for names on its closed method-label list.
- F8 · log-coverage · the `link` event line's `since` is stamped with `Utc::now()` directly in `LinkSet::line`, outside the `Clock` seam; its tests assert `since` by shape only.

## Untouched
- §7 panic-hook mechanics, the one-line panic record, catch sites and the per-role behaviour after the line: no panic path or hook changed.
- §8 PII scrubbing libraries (veil, its banned `toggle`), the default-deny schemas and the secret-scan verification: no field, schema or scrub shape added.
- §9 CI platform, the G1–G4 gate commands, the step order, the uploads and the artifact retention: no CI step, workflow or gate changed.
- §10 performance budgets and the hyperfine hook rows: no hook path changed; `run --perf` rows unchanged.
- §10 liveness signal and §1 heartbeat ticks: the heartbeat and its flip threshold are unchanged.
- §4 Budget governor scenario and the `hook statusline` shell-out: no statusline or budget code changed.
- §4 E1, E2, E5, torn-append and capability-ledger edge flows, and the `verify` / `plugin install` edge flow: none reached.
- The MCP surface: `viola mcp`'s tools and its no-line `instance-unreachable` rule are unchanged; `link` / `unlink` are not MCP tools.
- §6 log-levels mapping, per-module `Targets` and sink configuration: the new lines take existing rows (exit 21 and `-32602` at WARN, exit 20 and 1 at ERROR); no row changes.
- §1 tier, exporter carve-outs, OTel SDK init and service identity: unchanged.
- §11 anti-pattern bans: unchanged; the new spans obey `skip_all` and static names.
- §5 hook duration and other derived metrics besides `viola.compat.v_rejected`: no source changed.
