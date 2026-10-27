## 2026-10-26-home-and-code-bearing-file-integrity — the `diagnostics/` opens behind `check_diagnostics`
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: the panic-record group, a new bullet) · §1 Obs Scope Summary (obs-scope-summary.md: the log files' 0600-in-0700 rule; the zero-unlogged-panics signal) · §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: verification, the content sink) · §10 SLO Invariants & Telemetry Budgets (slo-invariants-telemetry-budgets.md: bounded exemption 4) · §2 Telemetry Strategy (telemetry-strategy.md: the panic exemptions)
**Change:**
- A role file or a detail file opens only past `check_diagnostics` (`src/obs.rs`, in `open_role_file` and `write_detail`), which runs `strict::check_diagnostics` on the existing `diagnostics/` folder and on the file before `ensure_private_dir` narrows anything: a symlink refused, a folder held to the read rule (Unix: no group or other bit; Windows: no read grant to another SID), a file to the read rule on Unix and the write rule on Windows, an absent path passing.
- A refusal is exactly a failed open: the role writer ends at `std::io::sink` and the detail line is dropped, so High-class content is never written through a symlink or into a folder another user may read.
- The 0600-in-0700 rule: was enforced by the tests' secret scan alone; now the product also holds it at open.
- Exemption 4 and §2's list: a failed step-4 role-file open now includes a `check_diagnostics` refusal; the panic signal adds that such a process has no panic sink, so a panic in it writes no record.
**Why:** the chunk put `check_diagnostics` ahead of both opens in `src/obs.rs`, held by its unit cases; a refused check writes no line by the founder's own answer to P4 fork Q2, live by dialog, relayed by the overseer (`inputs#I3`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the home-tree check of `run` and `viola revive`'s start, before the wrapper log
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: the main-thread catch site) · §6 Log Coverage (log-coverage.md: `process-exit` exit 1) · §1 Obs Scope Summary (obs-scope-summary.md: the `run` wrapper's required log fields) · §4 Span / Trace Coverage (span-trace-coverage.md: Scenario 1's failure line) · §3 Observability Harness Contract (obs-plan.md: the `run` terminal bullet) · §3 → OTel SDK init (registries/contracts/obs-plan/otel-sdk-init.md: the `run` anchor)
**Change:**
- Before the log init, `run` and `viola revive`'s start arm (the latter after its `check_instance`) run `checked_exe`: the exe's bytes read once, their `pin_key`, then `strict::check_home_tree` over the home (the read rule), the `check_instance` set, `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned copy and `plugin/<key>/` with its `hooks/` and `.claude-plugin/`, each that exists.
- A refusal (`TreeRefused`) opens no log, writes no `process-start`, no `process-exit` and no other line, starts no child and leaves the modes as found: exit 1 with the home pair (`HomeLevel`) or the instance pair (`InstanceLevel`) through `refuse_tree`, and under `viola revive --json` its `strict-modes-failed` document. That exit-1 cause has no `detail` code.
- §3: the start refusals add a home tree another user could write or read; `viola revive` adds three refusals to `run`'s five (was four).
**Why:** the chunk calls `checked_exe` / `refuse_tree` before `open_wrapper_log` in both starts, and its acceptance met `run` refusing a 0770 home, a widened `instances/<name>`, `bin/` or pinned copy with exit 1 before any log; no line on a refusal is the founder's own answer to P4 fork Q2, live by dialog, relayed by the overseer (`inputs#I3`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — `viola verify`'s home-tree check and the catch site's narrowed reach
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: Catch-site reach, a new bullet) · §1 Obs Scope Summary (obs-scope-summary.md: the short-lived verbs' exporter) · §4 Span / Trace Coverage (obs-plan.md: Edge flows, the `verify` / `plugin install` line; span-trace-coverage.md: the `verify` edge flow) · §3 → OTel SDK init (registries/contracts/obs-plan/otel-sdk-init.md: the `cli` anchor, `verify`)
**Change:**
- `viola verify` runs `strict::check_home_tree` with no name and no key (the home to the read rule, `ledger/`, `bin/`, `plugin/`, each that exists) before its `cli` log opens; a refusal is exit 1 with the home pair (`refuse_home`), no step line, no summary, no log opened and no line.
- `update_stamps` runs `check_stamps` before `create_private_dir(ledger/)` and again under its lock, reading and writing nothing on a refusal; verify maps `StampsError::StrictModes` to the same home pair, exit 1 through its own `process-exit`.
- Catch-site reach (new): on Unix a home under a regular file is a strict-modes refusal with no line — `viola verify` exit 1 with the home pair, the frame verbs exit 21 with the instance pair — so it never reaches the `cli` catch site; on Windows such a path stats `NotFound`, the check skips it as absent, and `process-exit{detail:"internal-error"}` stands.
**Why:** the chunk built the check before verify's log and the pre-read refusal of `update_stamps`, retiring the dated gap of `update_stamps` reading `ledger/stamps.json` without strict-modes; the per-OS reach is measured — the chunk's CI found the Windows leg on the catch-site line where Unix reads the home pair.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the pre-init window and the panic exemptions gain the chunk's checks
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: the pre-step-4 window) · §10 SLO Invariants & Telemetry Budgets (slo-invariants-telemetry-budgets.md: bounded exemptions 1 and 8) · §2 Telemetry Strategy (telemetry-strategy.md: the panic exemptions) · §1 Obs Scope Summary (obs-scope-summary.md: init order, step 3)
**Change:**
- The pre-step-4 window and exemption 1 add the hook event and dialog paths' instance check (`canonical`, then `trusted`, before `read_diagnostics_level` and obs init), the six frame verbs' `client::refuse_untrusted`, `viola verify`'s home-tree check and the home-tree check of `viola run` and `viola revive`'s start arm; each refusal writes no line.
- Exemption 8 (new): a refused check on the hook event and dialog paths (exit 0, nothing printed), on `send`, `wait`, `last`, `answer`, `pause` or `release` (exit 21, the instance pair or the `instance-unreachable` document), on verify's home tree (exit 1, the home pair) or on the home tree of `run` or revive's start (exit 1, the home or the instance pair) opens no role file and writes no line, so a panic there writes none. §2's bounded list adds the same sites.
- Init step 3: where a role checks the home before its first open, that check runs there — every `viola hook` arm but the two `--capture` arms, the eight `refuse_untrusted` verbs, `verify` (`check_home_tree`), `run` and revive's start (`checked_exe` + `check_home_tree`); a refusal ends the process with nothing under the home opened and no line.
**Why:** each check precedes its first open, since that open narrows an existing home to 0700 and a check made after it passes a widened home; a refused check writes no line, as `link` / `unlink`, by the founder's own answer to P4 fork Q2, live by dialog, relayed by the overseer (`inputs#I3`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the MCP frame tools' instance check before the liveness read
**Section:** §7 Error Capture & Reporting (obs-plan.md: Error classes captured, the MCP rejects bullet; error-capture-reporting.md: a new MCP-rejects group) · §1 Obs Scope Summary (obs-scope-summary.md: the MCP surface's required telemetry signal) · §4 Span / Trace Coverage (span-trace-coverage.md: the viola-mcp server check; the `send`, `wait` / `last` and `answer` scenarios) · §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: verification)
**Change:**
- `viola mcp`'s frame tools (`send`, `wait`, `last`, `answer`) run `check_instance` on the home and the instance directory in `frame` before `live_endpoint`; a refusal returns `instance-unreachable` with `detail: null`, writes no frame and no line.
- The MCP rejects bullet: `instance-unreachable`'s no-line causes were the liveness pre-check's and a frame tool's server-verification mismatch, per «2026-10-26-server-verification-before-any-frame — MCP's `instance-unreachable` reject writes no line»; now a frame tool's refused instance check, made before the liveness pre-check, joins them, the `isError` code its only record.
**Why:** the chunk retired the dated gap of the MCP frame tools writing their frames without strict-modes, its acceptance met by `binary(handlers)`; no line on a refusal is the founder's own answer to P4 fork Q2, live by dialog, relayed by the overseer (`inputs#I3`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — `sideload_fallback` gains `strict-modes-failed`
**Section:** §6 Log Coverage (log-coverage.md: the `process-start` fields) · §4 Span / Trace Coverage (span-trace-coverage.md: Scenario 1's `run.conpty_sideload` attribute, its success line, the diag-line schema note)
**Change:**
- `sideload_fallback`: was four words (`hash-mismatch|unreadable|load-failed|not-built`); now five, `strict-modes-failed` added — the write-rule check `pin_companions` makes before its held open refusing `bin/<key>/conpty/` or a companion in it (`PinError::StrictModes`), nothing written, the child on the inbox ConPTY with its exit unchanged.
- `run.conpty_sideload`'s `outcome` gains the same word (still never a refusal or an exit change), and `schemas/diag-line.v1.json` closes `sideload_fallback` in `process-start` to the five.
**Why:** the chunk built the companions' owner/DACL leg, retiring the dated gap of Windows x64 loading them under the re-hash alone, and the schema's enum gained the word; its acceptance met on `windows-2025`, the kept homes holding exactly one `strict-modes-failed`, the companion test's own.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the hook event and dialog paths' instance check before obs init
**Section:** §1 Obs Scope Summary (obs-plan.md: the hook surface's exporter; obs-scope-summary.md: the hook's service identity and its no-instance note, the fail-open logging note, the dialog scenario's required fields) · §6 Log Coverage (log-coverage.md: `hook-decision` fail-open codes) · §10 SLO Invariants & Telemetry Budgets (slo-invariants-telemetry-budgets.md: exemption 3) · §4 Span / Trace Coverage (span-trace-coverage.md: the `viola hook` row, the dialog path, its fail-open paths) · §2 Telemetry Strategy (telemetry-strategy.md: the Vector 4 row) · §3 → OTel SDK init (registries/contracts/obs-plan/otel-sdk-init.md: the `hook` anchor)
**Change:**
- Every `viola hook` arm but the two `--capture` arms runs the instance check (`instance_of(…)`, then `canonical` and `trusted`: `check_instance` on the canonicalised instance) before `read_diagnostics_level` and obs init; was `hook statusline` the one arm with a check.
- A refused or absent instance exits 0 at once: no `config.json` read, no role file, no stdin read, no `hook-invoked`, no `hook-decision`, no line, no frame, no SessionEnd append; no hook arm writes `strict-modes-failed`. The `--capture` arms run ahead of the check, `FAKE_AGENT_HOOK_PANIC` after obs init.
- A `VIOLA_DIR` naming an instance directory that does not exist resolves none, so the hook writes nothing (exemption 3; the exporter's silent no-op, was `VIOLA_NAME` missing alone).
**Why:** the chunk retired the dated gaps of `hook.event` reading its snapshot and `hook.dialog` writing its frame without strict-modes; no line by the founder's own answer to P4 fork Q2, live by dialog, relayed by the overseer (`inputs#I3`). The standing «one arm» lines were rewritten with their companions, as the overseer answered at this wrap's halt, founder-delegated (`inputs#I6`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the snapshot readers over a replayed snapshot
**Section:** §6 Log Coverage (log-coverage.md: `channel-unreachable`) · §1 Obs Scope Summary (obs-scope-summary.md: the persistent stores' scrub rules, a new sub-bullet) · §4 Span / Trace Coverage (span-trace-coverage.md: the hook server check; `run.collision_check`'s attribute; `send`'s server check and cleanup)
**Change:**
- A `snapshot.json` rebuilt from `events.ndjson` (`read_snapshot_or_replay`) carries no `endpoint`, `pid`, `started_at` or `statusline_command`.
- Hook `connect` goes on only on a snapshot read whole: a replayed or absent one is `NotSent::Unreachable`, the hook failing open with `hook-decision{detail:"channel-unreachable"}`, a SessionEnd taking its direct append.
- `live_endpoint` over a replayed snapshot is `None`: the frame paths end as unreachable (CLI exit 21 `instance-dead` `during:"connect"`, a replayed snapshot included); `hook statusline` runs nothing and prints nothing.
- `run.collision_check`: over a replayed snapshot the heartbeat alone decides (a beat no older than `STALE_AFTER` is `already-live`, an older one `free`); an absent snapshot or a failed read is `free`.
**Why:** the four readers moved onto the replay by the founder's own answer to P4 fork Q1, live by dialog, relayed by the overseer, his boundary-widening ratification (`inputs#I3`); the escalated proposals were confirmed at this wrap's halt by the overseer, founder-delegated, PROVISIONAL (`inputs#I6`), the ratification staying his `inputs#I3`.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the refusal-telemetry record: two strict-modes lines, every other refusal none
**Section:** §1 Obs Scope Summary (obs-plan.md: the Vector 7 and Founder Direction 4 required signals; obs-scope-summary.md: the OS-API reason, the Founder Direction 4 note, the refusal-telemetry trigger) · §6 Log Coverage (log-coverage.md: Filesystem refusals, Vector 7) · §2 Telemetry Strategy (telemetry-strategy.md: the Vector 7 and Founder Direction 4 rows)
**Change:**
- Trigger: was a line only on the `ledger-stamps` read, five refusals leaving none, per «2026-10-26-explicit-session-links — the link verbs' instance check before their log: a refused check writes no line»; now a line in two places, the `ledger-stamps` read and on Windows x64 a refused `conpty/` companion (`sideload_fallback` `strict-modes-failed`); a refused `update_stamps` leaves only verify's own `process-exit`; none from revive's instance check, run's and a revived start's home tree, verify's home tree, every hook arm but the two `--capture` arms, the six frame verbs, `link` / `unlink`, the MCP frame tools, the `diagnostics/` opens (a failed open), the `budget.json` read and `list`'s scan. obs-plan.md §1 states the same.
- Founder Direction 4: was a distinct `detail` code for each exit-21 and exit-1 cause; now except a strict-modes check refused before any log opens (the six frame verbs, `link`, `unlink`: exit 21; `run`, `revive`, `verify`: exit 1), carried by the exit code, the stderr pair or the `--json` document.
- §6 Vector 7: was «the nine hook events run no instance check»; now their refused check, made before obs init, leaves no line, and a closing bullet lists the other no-line refusals. The OS-API reason names `check_instance`, `check_home_tree` and `check_diagnostics` as call sites.
**Why:** the founder's own answer to P4 fork Q2, live by dialog, relayed by the overseer (`inputs#I3`): a refused check at every new entry point writes no line. The Vector 7 line was rewritten with its companion, as the overseer answered at this wrap's halt, founder-delegated (`inputs#I6`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the six frame verbs' instance check through `client::refuse_untrusted`
**Section:** §5 Metric Coverage (metric-coverage.md: the `wait` outcome) · §1 Obs Scope Summary (obs-scope-summary.md: the short-lived verbs' exporter; the dialog scenario's required fields) · §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: verification) · §4 Span / Trace Coverage (obs-plan.md: Scenario `wait` / `last`; span-trace-coverage.md: the short-lived verbs' server check, a new instance-check bullet, `send`, `wait` / `last`, `answer`) · §3 → OTel SDK init (registries/contracts/obs-plan/otel-sdk-init.md: the `cli` anchor)
**Change:**
- `send`, `wait`, `last`, `answer`, `pause` and `release` take the one `client::refuse_untrusted` that `link` / `unlink` share (`check_instance` on the absolute home and `instances/<name>`, before `client::start`; `send` after its text read): a refusal exits 21 with the fixed instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, opens nothing under the home, sends no frame and writes no line.
- The eight verbs' order: the instance check, then the liveness pre-check, then the server check.
- `wait` / `last`: was «`strict-modes-failed` cannot occur on them», owed to «Home and code-bearing file integrity», per «2026-10-26-server-verification-before-any-frame — `wait` / `last`: «`server-verify-failed` cannot occur» retired»; now a refused instance check ends them first, exit 21 with no line.
- A refused CLI or MCP `wait` writes no line, so neither `viola.wait.outcome` nor the Counter row counts it. The refusal carries no pid, path or endpoint; `tests/security_negatives_strict_modes.rs` reads every output with `digit_runs` on three OSes.
**Why:** the chunk retired the dated gaps of CLI `send`, `wait` / `last`, `answer` and `pause` / `release`; no line and `link`'s instance pair are the founder's own answers to P4 forks Q2 and Q3, live by dialog, relayed by the overseer (`inputs#I3`); the `wait` / `last` rewrite took its companion at this wrap's halt, the overseer's answer, founder-delegated (`inputs#I6`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the `human::refuse` census, 15 to 18
**Section:** §11 Obs Anti-Patterns → Logs (obs-anti-patterns.md: the print-macro ban's `src/human.rs` exception) · §3 Observability Harness Contract (obs-plan.md: the `run` terminal bullet)
**Change:**
- Was 15 (`run` 5 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1 · `link` 1), per «2026-10-26-explicit-session-links — the `human::refuse` census, 14 to 15»; now `run` 7 · `verify` 6 · `list` 2 · `revive` 1 · `plugin install` 1 · `client` 1 — `client::refuse_untrusted`, the one site `link` / `unlink` and the six frame verbs share, in place of `link` 1 — re-derived by `grep -rcE 'human::refuse\(' src` at chunk 2026-10-26-home-and-code-bearing-file-integrity; the rule stays the `human::refuse(` call sites under `src/`.
- §11's refusals written through `refuse` / `write_refusal` add the six frame verbs' instance-check refusal beside `link` / `unlink`'s.
**Why:** the report's count by the master's rule, 15 → 18, the instance-check refusal moved from `link_call` into `client::refuse_untrusted` and the home-tree refusals added to `run` and `verify`.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the hook's instance check inside the timed path
**Section:** §1 Obs Scope Summary (obs-scope-summary.md: the hook surface's latency note) · §10 SLO Invariants & Telemetry Budgets (slo-invariants-telemetry-budgets.md: the spine-hook row, the obs-overhead row)
**Change:**
- The instance check ahead of logger init (`canonical` + `trusted`: stat and, on Windows, owner and DACL reads) is in the timed path; `run --perf` keeps the five hook rows under `SPINE_DEADLINE` with it, max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms.
- Spine row: the timed path also holds the instance check, and its one `snapshot.json` read is `read_snapshot_or_replay` (a replayed or absent snapshot failing open as `channel-unreachable`), inside the same gate.
- Obs-overhead row: beside a missing `VIOLA_NAME`, the gate passes vacuously when `VIOLA_DIR` names no existing instance directory or the instance fails the check, so the timed instance must pass it; logger init's one `open` follows the instance check's and `check_diagnostics`' metadata reads.
**Why:** the chunk's obs acceptance, met: the entry's `run --perf` green with the check in the timed path, at the maxima above.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — E5: the replay's product readers two to six, `state.snapshot_recover` placed
**Section:** §4 Span / Trace Coverage (obs-plan.md: Edge flows E5; span-trace-coverage.md: Scenario 1's must-trace chain, `viola revive`'s preflight, the hook server check, E5's snapshot readers) · §1 Obs Scope Summary (obs-scope-summary.md: the `run` start's and the dialog scenario's must-trace spans)
**Change:**
- E5's product readers of `read_snapshot_or_replay`: was two, `revive` and `list`; now six, with hook `connect`, statusline `serve`, `run`'s start and `liveness::live_endpoint`, each writing its one `state-recovered` line per call.
- `read_snapshot` product callers: was three, per «2026-10-26-server-verification-before-any-frame — E5: the other snapshot readers, five to three»; now none (the rule: `read_snapshot(` under `src/`, outside test modules).
- `state.snapshot_recover` now sits under `run.start` before `run.collision_check` (the start's one read; an unreadable snapshot writes one `state-recovered{detail:"snapshot-replayed"}` in `run-<name>.ndjson`) and under `hook.handle` before `channel.request(hook.dialog)`.
- `viola revive`'s preflight makes one snapshot read for its collision check and its recorded-directory reading: one replay, one `state-recovered` line.
**Why:** the readers' move is the founder's own answer to P4 fork Q1, live by dialog, relayed by the overseer, his boundary-widening ratification (`inputs#I3`); one read per revive keeps one replay line. The escalated proposals and their companions were confirmed at this wrap's halt by the overseer, founder-delegated, PROVISIONAL (`inputs#I6`), the ratification staying his `inputs#I3`.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/
