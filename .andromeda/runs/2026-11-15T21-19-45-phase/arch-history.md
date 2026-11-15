@history_item n=1 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** (seven exec-form hooks, `hook.event` served) — registered `plugin/hooks/hooks.json`'s exec-form entries (`"command": "@@VIOLA_BIN@@"`, `"args": ["hook", "<kebab event>"]`; spine tier `"timeout": 5`, async tier `"async": true`) and served `hook.event`, which re-validates the event against the closed hook kinds and object `data` before appending with `source:"hook"`. The template and validation path a StopFailure registration and a new `turn-ended` source must follow; the wrapper's kind re-validation must admit the new `data` key.

@history_item n=2 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (perf arm registry, the `FAKE_AGENT_HOOK_PANIC` seam) — the per-hook hyperfine exports are required rows of `gate --require perf`, and a second fake-agent env seam was a boundary widening ratified by the founder live. A newly registered hook may owe a perf row; making the fake agent emit StopFailure through an env variable would be a new seam needing the same ratification (the later precedent is files and argv, not env).

@history_item n=3 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** ([Hook Transport] constants) — fixed `viola_core::SPINE_DEADLINE` = 1 s (the perf gate's bound) and the hook's crate-private `CONNECT_DEADLINE` = 750 ms (provisional, asserted below it). Binds the tier and deadline a StopFailure hook runs under, whether spine or fire-and-forget.

@history_item n=4 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (ledger rows, `viola verify`'s probe, fixtures) — established the closed ledger in `viola_agent_claude::ledger`, the probe plugin and hidden `hook --capture` arm, and `--record` fixtures written only at 0 fail. The machinery a StopFailure ledger row and probe would extend; the scope's question of whether the dependence on StopFailure firing is a new row starts here.

@history_item n=5 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (relayed fixtures when no probe can record; nine hooks) — when print mode raised no dialog hook so no verify probe could record it, the founder ruled relayed fixtures (`RELAYED.md`, hygiene-walked) and decisions on the existing stamp as a dated gap, rows owed to a later entry; the same entry grew `hooks.json` from seven to nine entries and the perf exports to five rows. The nearest precedent for StopFailure, which no probe can force without an API error and which has no recorded fixture.

@history_item n=6 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (the wheel as landed) — recorded that as built no running-turn state existed beyond the in-flight `send` slot, so `release` returned the wheel alone. The predecessor of the running-turn state this chunk must clear on an API-error end; superseded by the running-turn entry.

@history_item n=7 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (the running-turn state as built) — the state lives in memory in `WheelSlot`, marked by `run::send::append_hook_event` keyed on kind alone: `prompt-submitted` starts it, `turn-ended` / `session-start` / `session-end` end it; only a `release` returning a human-held wheel clears it otherwise (recovery is `pause` then `release`); residual: a SessionEnd the hook appends directly leaves the turn marked. This is the exact mechanism behind the measured defect (no Stop, so no `turn-ended`, so `turn-running` forever) that this chunk must close.

@history_item n=8 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (send kinds, refusals, the relabel) — `send` refuses `turn-running` also for a second send in flight, appends `send-issued` / `send-confirmed` / `send-refused` log-only, and relabels a matching `prompt-submitted` as `driver`. The refusal order and records the "next send accepted" acceptance reads against.

@history_item n=9 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** (wheel and turn read twice) — `send` reads the wheel and then the running turn at arrival and again after the gate returns ready. A turn end from StopFailure must clear the state both reads see.

@history_item n=10 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (send and human split by phase) — moved `append_hook_event` with the driver relabel and the one-in-flight `SendSlot` into `src/run/send/slot.rs`. The file the scope names for the turn-end path.

@history_item n=11 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** (wait and last as built) — `wait` wakes on `EventKind::WAIT_WAKE` through the `WaitFeed` (only hook lines signal), and `last` is the newest `turn-ended`'s `last_assistant_message`, updated under the append's lock and rebuilt at start (`src/run/wait.rs`). A StopFailure-sourced `turn-ended` wakes `wait` for free, but its `last_assistant_message` is the error text and must not flow into `last` or the line.

@history_item n=12 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** (`dialog-expired` and the six wake kinds) — added a normalised kind and fixed the wake set at six (`turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired`), noting the one wrapper-appended kind that wakes. Precedent for adding to the core kind set and the wake contract; reusing `turn-ended` keeps the wake set unchanged.

@history_item n=13 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (`turn-ended` gains `background_tasks`) — the last change to `turn-ended`'s `data`: `{last_assistant_message, background_tasks}`, a count only with no element carried, `null` when absent, additive, no `v` bump, ratified by the founder as a crossing. The direct precedent for adding a `cause` key to the same contract.

@history_item n=14 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (the reader counts three names) — `events::read_from` counts a known-kind line with a key outside its kind's `data` list as `unknown_fields`. A new `cause` key must enter `EventKind::data_keys` or every reader counts it as unknown.

@history_item n=15 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Conventions; Established Decisions) — widened `decision/protocol-version-bump-rule`'s except (removals kept every own-format `v` at 1) and moved the relied ledger to 21 rows, so a new row is the 22nd (`decision/permission-request-question-stays-null`). The bump rule governs whether the `cause` key is additive, and the count tells the cost of a StopFailure row.

@history_item n=16 marker=2026-11-15T18-01-44-wrap
**2026-11-15T18-01-44-wrap** (the widened protocol-version-bump exception ratified) — the founder's own word ratified that widened except; the rule itself (adding a field or kind needs no bump) stands for this chunk's additive key.

@history_item n=17 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** (a ledger row and a probe widening) — adding a row made every older stamp read unverified until re-stamped, and the probe's new paste widened verify's ratified child set, needing the founder's own answer. Precedent for what a StopFailure row costs: re-stamps, homes reading `cli_verified:false`, and a ratification if the probe widens `decision/verify-child-set`.

@history_item n=18 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (the 24th row) — the latest row-and-probe addition: every home stamped before reads `cli_verified:false` until its next verify, the fixture set and the fake agent's `--framing` replay grow with it. The template a new row would follow.

@history_item n=19 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (relied-on dependences with no row) — the founder chose a hand reading with no row, no probe and no fixture, recorded as named exceptions each with an owner, the rule kept. One of the forks the scope leaves open for StopFailure (a row with a probe, or a named exception with an owner).

@history_item n=20 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the ruled limit) — the founder ruled that a CLI shape no `send` relies on needs no probe (`decision/no-probe-for-unrelied-shapes`). The turn-end path is one `send` relies on (the `turn-running` rung), so the limit likely does not exempt StopFailure.

@history_item n=21 marker=2026-10-05-permission-end-to-end
**2026-10-05-permission-end-to-end** (the question stays null) — declined a new row because its probe and re-stamp cost exceeded the live-session cap left. A precedent for weighing a StopFailure row against the directive's no-live-start cap.

@history_item n=22 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (Standard Contracts: the agents read and join) — `viola list`'s `status` (`idle` / `busy`) comes from one `claude agents --json` read joined by pid or session id, not from the wrapper's running-turn state. This explains the measured split (board `idle`, `send` refused `turn-running`) the scope asks P3 to re-read.

@history_item n=23 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (Occupied Resources: the fake agent's agents mode) — the fake agent's modes take files beside its own executable, not a new env seam (founder's answer). The pattern for letting the fake agent emit StopFailure.

@history_item n=24 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the 2.1.302 fixture set and the long-turn screen switch) — the fake agent's argv options are counted by their list (thirteen), and a recorded set is committed only from a verify round. A new emit option moves that count; a synthetic StopFailure payload is not a recorded fixture.

@history_item n=25 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** (diagnostics roots) — `hook` logs codes only to the home-level `diagnostics/hook-<name>.ndjson`, with content-bearing detail only in the instance's `detail-hook.ndjson`. Decides where (if anywhere) `error` / `error_details` / `last_assistant_message` may land.

@history_item n=26 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the pattern and its contract binary) — codes and fixed messages only on CLI `--json`, MCP errors and channel error data, a peer's text reaching them only as a closed value, held by `tests/contract_error_bodies.rs` and the error-body check in `tests/support/events.rs`. The closed-cause / no-error-text rule this chunk extends, and the check its absence probe can reuse.

@history_item n=27 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (bounded readers; serde_json depth ban) — every product reader of external bytes is bounded in its own statement, held by `tests/contract_bounded_readers.rs`'s census, and `serde_json/unbounded_depth` is banned. A new StopFailure payload read must stay under the hook's `MAX_FRAME` take and the census.

@history_item n=28 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the hook's one connect) — the hook's event path is `read_snapshot`, connect, `verify_server`; a failed check records `server-verify-failed` and fails open, the event lost. The StopFailure hook inherits this path.

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook's instance check; the replay readers) — `viola hook` takes its instance only when the `VIOLA_DIR` directory exists and passes `check_instance`, before config, obs or stdin, and reads the snapshot through the replay (a replayed snapshot unreachable, fail open). A new hook arm must keep this order.

@history_item n=30 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a client's refused socket directory) — the hook records one `hook-decision` `socket-dir-refused` and fails open when the endpoint's directory is refused. Part of the hook contract a StopFailure path joins.

@history_item n=31 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the one-enum divergences ratified) — fixed the ratified error-enum exceptions and the census of 9 `thiserror::Error` derives outside `crates/viola-e2e`. A new parse failure for the StopFailure payload must fold into `AgentError`, not a new enum, or move the census.

@history_item n=32 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (Occupied Resources) — added `decision/test-sockets-under-short-tmp-root` and raised the root `WITHIN` bound to 12 s. The scope's directive binds new tests to the short `/tmp` socket root.
