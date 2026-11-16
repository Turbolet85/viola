@history_item n=1 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — (the wheel scenario as landed) set `run.wheel_transition.cause` to `human-input|manual-pause|release` (a point-in-time span with a static name, opened once per change on the wheel's worker thread), traced `pause` / `release` as CLIENT spans, and admitted `release-from-driver`. This is the wheel span and cause vocabulary that `contract/wheel-span-attributes/run-wheel-transition` and `contract/wheel-spans/span-chain` hold. A survey key that keeps the wheel must open no `run.wheel_transition` and append no `wheel` record. Every other key keeps the existing `human-input` transition.

@history_item n=2 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — (`contract/run-start-events/order`, `decision/product-order-from-events-log`, `decision/events-stream-never-rotated`) A live run on 2.1.325 measured folder-trust keys passing through the wrapper and writing `wheel{human-input}` before `session-start`, with a later human key adding no wheel line. The start order is read from that start's own lines. This is the newest live precedent of a CLI prompt's keys reaching the wheel classifier. Moving a survey key onto the non-editing list must keep these start-order promises true. If the survey ever shows during a start, the start's `wheel` lines change.

@history_item n=3 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (feed overflow, send.client exits) `vt100-feed` gained the `oversize` detail (a dropped feed copy poisons the model, one line per episode). The wrapper `send-refused`'s `wheel` field was kept required (CARRY until the wheel landed). If the survey's screen state has to reach the wheel decision through the pump's vt100 model, it inherits the model's poisoning behaviour and the `parse-rejected{vt100-feed}` lines. `contract/wheel-log-line/human-typing` and `contract/send-refused/human-typing` carry the `wheel` field that a kept wheel must leave correct.

@history_item n=4 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — Moved the vt100 panic catch onto `run`'s feed thread (`src/run/gate.rs`) as a contained worker: one `parse-rejected{vt100-feed, panicked}` per poisoning, and `run` continues. This is `decision/vt100-panics-contained` / `decision/vt100-feed-panic-contained`. A wheel classifier that newly reads screen state must decide what a poisoned model means for a survey key. The trap: no telemetry may be derived from that read (`decision/no-telemetry-from-screen`).

@history_item n=5 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (the G2 question answered: F4) `tests/chaos_feed_panic.rs` witnesses that a typed key still reaches the child after a forced feed panic. Its home is the outside-scan carve-out. This precedent bears on the boundary «the human's key still reaches the child unchanged» once the classifier consults screen state.

@history_item n=6 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (1. Obs Scope Summary; 11. Obs Anti-Patterns; 10. SLO Invariants; 4. Span / Trace Coverage) This is the most recent chunk to make a CLI-observed signal a deciding input. It added named excepts to `decision/claude-internals-not-a-contract`, `decision/transcripts-not-a-contract` and `decision/deterministic-emission` / `decision/deterministic-event-emission`, and the new `decision/transcript-echo-read-uninstrumented` (a read inside an existing span with no new line). All of these were PROVISIONAL on a delegate's confirm. A screen-state read feeding the wheel decision likely needs the same kind of named except, now on `decision/no-telemetry-from-screen` / `decision/no-telemetry-from-screen-content` (vt100 for gate outcomes only) and on `decision/deterministic-emission`, which today names only the readiness gate.

@history_item n=7 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — Set `verify`'s stdout to one `[NN/MM]` step line per relied ledger row, the summary summing to `MM` (`contract/verify-stdout/step-line`, `contract/verify-stdout/summary-line`). It also set the capture arm uninstrumented. A new survey-grammar ledger row raises the relied count `MM`.

@history_item n=8 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — Introduced `verify-pty-probe` and one spawn pair per interactive PTY run of verify (`contract/verify-spawn-subject/verify-pty-probe`, `contract/diag-subject/verify-pty-probe`). In CI, verify drives the fake agent's interactive runs by flag. This is the register a survey `viola verify` probe extends: a probe inside an existing run adds no pair, and a new run adds one.

@history_item n=9 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — Verify went from four to six spawn pairs (Runs C and D) and CI's fake-agent flag list gained `--dialogs`. This is the precedent for adding an interactive verify run with its fake-agent flag. The `subject` enum was left unchanged.

@history_item n=10 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — CI's fake-agent verify flag list gained `--framing`, bound to test-plan §3 boot step 4. A fake-agent survey screen for CI likely adds such a flag, and the bound list moves on both sides.

@history_item n=11 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Seven verify spawn pairs (Run E), `MM` recounted, and a killed PTY child's `child_exit_status` (`contract/verify-process-exit/child-exit-status-and-duration`). This is a precedent for a new ledger row's spawn-pair and count effects.

@history_item n=12 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — Eight verify spawn pairs (Run B's `agents-probe`), `MM` 23. A probe reusing an existing subject keeps the closed `subject` enum unchanged. This is the current spawn-pair count that a survey probe must keep or bump.

@history_item n=13 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `MM` became 24 with the 24th row `local-command-compact`. Run B's sixth paste added no spawn pair or subject. This is the precedent of a new ledger row probed inside an existing run with no new pair.

@history_item n=14 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the Windows host arms … retired) The relied count reads all 24 rows of `ALL` on the one OS. Before this change it was 24 / 21 per OS. A new ledger row now moves one `MM` only.

@history_item n=15 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the OS target, Linux only) Added `decision/linux-only-os-target`: the core on any current distribution, CI on `ubuntu-latest`. The live measurement and the fake-agent CI case run on Linux alone.

@history_item n=16 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — In CI, verify runs only against the fake agent, and the real-`claude` verify runs only through the local live run, refused under `CI` (`decision/ci-verify-fake-agent-only`, `decision/real-cli-verify-never-in-ci`). The survey probe's CI half must therefore be a fake-agent survey screen. Its real-CLI reading stays a local live run, which matches the scope's capped live starts.

@history_item n=17 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — Added `strict-modes-failed` on the `ledger-stamps` read (`contract/run-log-parse-rejected/ledger-stamps`, `contract/parse-rejected-detail/strict-modes-failed`). CI verify runs at the recorded CLI version. A new ledger row is stamped through this same stamps read, where an unreadable ledger reads as unverified (`decision/unreadable-ledger-is-unverified`).

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (`verify` writes its refusal's code) verify's `process-exit` carries `unclean-fixture` among its refusal codes, and the fixture's `<file>` enters no code (`contract/process-exit-detail/verify-refusal`). A recorded survey screen under `fixtures/claude/<version>/` is scrub-required data (`decision/claude-fixtures-scrub-required`). An unclean one refuses verify with this code.

@history_item n=19 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `detail:"interrupted"` on verify's `process-exit`: a stopped run starts no further spawn and closes every pair it opened (`contract/verify-process-exit/interrupted`, `contract/process-exit-detail/interrupted`). A new survey probe run must honour the stop.

@history_item n=20 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (4. Span / Trace Coverage: 1 row(s)) `decision/verify-lines-carry-no-content` now reads no row text, prompt, session id, probe command or path. A survey probe's lines must carry none of these, and no survey key or screen text.

@history_item n=21 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (2. Telemetry Strategy) `decision/span-naming`'s closed `<area>` list lost `statusline`, after 2026-11-14-hyprland-seam added `hyprland`. Any new span for the survey reading must fit an existing area (`run`, `pty`) or amend this closed list.

@history_item n=22 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (6. Log Coverage) StopFailure was registered with no ledger row and no verify probe, ratified by the founder on 2026-11-16 (2026-11-16T06-05-33-wrap), its live reading owed to a fixture-set route entry. This is the contrasting precedent: this chunk's scope requires a ledger row with a probe for the survey grammar under the CLAUDE.md invariant.

@history_item n=23 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (the send trailing-only tolerance ratified by the founder) A leading trim waits for the live reading carried on «Standing live rig with a screen model». This is the precedent that a CLI behaviour not yet measured live stays out until a capped live reading lands. The survey grammar joins the non-editing list only once measured.

@history_item n=24 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — (fake agent's print-ban exemption) The fake agent is a root-package `[[bin]]` exempted by a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]` in `src/bin/viola-fake-agent.rs`. A survey screen drawn by the fake agent rides this exemption, and no new lint carve-out is needed.
