@history_item n=1 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the mutation row left CI: obs code meets cargo-mutants only at `agent-run run --mutants` (the epoch-boundary audit or on demand), and §10 counts a surviving obs-code mutant there as a failure. Items 2, 3 and 6 work from that boundary audit's survivors and the `mutants` nextest profile; the scope bars a mutation run inside the chunk, so the killing tests are proved by the next boundary audit, not by a run here (decision/obs-code-mutation-tested, decision/surviving-obs-mutant-fails).

@history_item n=2 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** — defined what reads red at a `run --mutants`: a timed-out obs-code mutant, a missed one the harness does not leave out as host-excluded, or unviable outnumbering caught; §10's «surviving» is the harness document's `survived`. Item 6 (the `mutants` profile's 15 s kill on the drift replay tests) touches the timed-out class: a kill that times out a mutant changes the audit's verdict, so the profile change must not turn a caught mutant into a timeout or hide one (decision/surviving-obs-mutant-fails except).

@history_item n=3 marker=2026-09-24-epoch-1-cleanup
**2026-09-24-epoch-1-cleanup** — the precedent epoch cleanup's obs touch: the Mutation row gained the per-leg unviable-outnumbering-caught rule, and the entry recorded that the chunk's change added no obs event, field or sink. The same shape applies here: test-only mutation-killing items (2, 3) should move no obs row, and if they do not, the wrap says so.

@history_item n=4 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (§5–§11 entry) — §10's cross-input parallel was set to nextest `retries = 0`. Item 6 edits `.config/nextest.toml`: the scope's «no retry» boundary and decision/no-retry-policies / decision/no-retry-once rest on this; a kill-time change in the `mutants` profile must not add retries.

@history_item n=5 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added verify's eighth spawn pair, one `agents-probe` pair around Run B's single `claude agents --json` read, written by `run::agents::read_logged` (also for `list`, revive's session-live reading and later the mcp `list` tool); `run_bounded` and `run_capped` stay unlogged; `parse-rejected{parser:"claude-agents-json"}` gained `oversize` past `MAX_FRAME`; session-live fails open. Item 4 gives that read (`src/run/agents.rs`) a bound of its own: it is the shared logged reader, so a bound change reaches all four callers' `agents-probe` pairs (contract/verify-spawn-subject/agents-probe, contract/diag-subject/agents-probe, contract/revive-process-log/agents-probe, decision/session-live-check-fails-open), and a new outcome word would be a closed word (the operator's fork).

@history_item n=6 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the mcp list tool's lines) — `viola mcp`'s `list` tool became a fourth writer of the `agents-probe` pair, into `mcp.ndjson`. Item 4's bound on the agents read lands on this caller too.

@history_item n=7 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — the nearest precedent for bounding a call: the MCP call deadline added the closed cause word `call-deadline` to `UnreachableCause` and the instance-unreachable set (seven), with no line written (decision/instance-unreachable-not-logged), and named the CLI's MCP tool-call bound as a per-version bundle reading, never a contract (decision/claude-internals-not-a-contract). Item 4's own bound for the agents read follows that path: if it surfaces a new word it is a new closed word for the operator's fork, and the read's bound must not rest on claude internals.

@history_item n=8 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (D-05 `parse-rejected` gains `mcp-stdin`) — a new closed `parser` value took a Decisions Log record and its schema enum member. If item 4's bounded agents read writes a new `parse-rejected` detail or other closed value, the same record is owed (decision/event-values-closed, decision/per-parser-parse-failure-counts).

@history_item n=9 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `process-exit{detail:"interrupted"}` on a verify stopped by SIGINT/SIGTERM after every pair it opened is closed; the probe dir is removed on a pass, a fail and a stop, while SIGKILL and a crash leave it. Item 7 measures what a verify ended by SIGKILL leaves in the Claude config: this entry is where the SIGKILL residue (probe dir) is already named, and the measured trust-record residue joins it as a named residual or a mend (contract/process-exit-detail/interrupted, contract/verify-process-exit/interrupted).

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows host arms … retired) — a verify stop reads SIGINT/SIGTERM only; SIGKILL and a crash leave the probe dir (the Windows close/logoff clause gone); the relied count reads all 24 rows of `ALL`. Item 7 (SIGKILL residue) and item 9 (relied-row count from one source) both extend this standing statement.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (one CI leg, two workflows, one deny target) — `cargo deny` runs on one target, `x86_64-unknown-linux-gnu`; `nightly.yml` keeps the weekly advisories run. The chrono advisory measurement reads cargo deny on that one target (decision/obs-deps-pass-cargo-deny, decision/cargo-deny-failure-fails).

@history_item n=12 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — verify writes and removes its own temp folder's trust record in the claude config (decision/claude-internals-not-a-contract except), refuses a symlinked, non-regular or foreign config with no line (decision/refusal-telemetry except), reads the kept `CLAUDE_CONFIG_DIR` value (decision/r8-strip-logged-as-names-only, decision/env-vars-not-configuration), and added decision/trust-record-write-uninstrumented; Run F made six PTY runs. Item 7's SIGKILL measurement targets exactly this write/remove pair: a residual found is a project entry this writer left, and any mend must keep the write uninstrumented (no line, no span, no path or key in any line or stamp).

@history_item n=13 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — the run and revive start also write the trust record (flag only) and log exactly one `trust-record` line with a closed start word; the start's trust step opens no span; `CLAUDE_CONFIG_DIR` is read by verify and the start. Item 7 must tell verify's temp-folder record apart from a start's own folder flag when it reads a project entry under a verify scratch root (contract/start-trust-log/trust-record, contract/diag-line-fields/trust-record).

@history_item n=14 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (ratifications) — the founder ratified every Epoch 4 PROVISIONAL mark: direct-chrono-dependency, the trust-record rows of verify and the start (claude-internals-not-a-contract, refusal-telemetry, r8-strip, env-vars-not-configuration, claude-boundary-only-instrumentation), the paste-cut rows, auto-verify-stop and the verify stdout rows. The rows this chunk touches stand on the founder's word, so changing them is no delegate's call.

@history_item n=15 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** — decision/direct-chrono-dependency now keeps chrono though unmaintained: RUSTSEC-2026-0543 in the cargo deny ignore list, re-read at each epoch boundary, jiff and time not adopted. The advisory item re-measures exactly this: an upgrade found removes the ignore and moves the row's words; none found keeps the row and its «re-read at each boundary» (decision/tracing-subscriber-minimal-features keeps the chrono feature off either way).

@history_item n=16 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — the paste writer became a third run PTY thread under `catch_unwind` (decision/panic-caught-at-every-role, decision/run-threads-inside-catch-unwind, decision/run-worker-panic-ends-run); `paste-cut` joined the send details and the event order; the cut mark is uninstrumented (decision/send-cut-mark-uninstrumented). Item 3 writes killing tests for `crates/viola-pty/src/pump.rs` survivors in that paste writer; the tests must hold these rows as built and add no line (contract/send-refused-detail/paste-cut, contract/send-path-log-events/event-order).

@history_item n=17 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — `run.confirm_window` closes on the confirmation its send waits for or on expiry (`no-prompt-submitted`); an unconfirmable send opens no window and logs one `send-issued` plus `send-confirmed{confirmed:false}` under one `corr`. Item 1 gives `SendSlot::confirm` an end for a claimed line whose settle never comes: the end must still close as one of these closed outcomes with the existing details, no new word (contract/send-log-fields/send-confirmed, contract/send-refused-detail/no-prompt-submitted).

@history_item n=18 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the `/compact` claim arm and the per-send window: `window_ms` reads `SendSlot::window()` (40 000 for compact, else 10 000), a claim on the remembered `agent_session_id`. Item 1 touches the slot's claimed-line path (`src/run/send/slot.rs`); the windows are named bounds and must not be raised (contract/confirm-window-close/compact, contract/send-span-attributes/window-ms, decision/compact-confirm-window).

@history_item n=19 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — the ordinary-send claim takes the submitted prompt with the trailing-whitespace tolerance, and the post-gate turn read, link record, `send-issued` and paste run under one flight lock (turn-running bounded to a turn known before the paste). Item 1's fix in the claim/settle path sits inside this flight; it must keep those claim rules and the lock's order.

@history_item n=20 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — an echo-confirmed local command closes on the transcript tag count, awaits no hook line and reads `no-prompt-submitted` at expiry; the echo read is uninstrumented (decision/transcript-echo-read-uninstrumented). A further close path the slot's claimed-line end (item 1) must cover without a new line.

@history_item n=21 marker=2026-10-10-session-links
**2026-10-10-session-links** — a failed link record in the send path is `-32603` with nothing issued and no `send-issued`; the start's link restore has no span or line. The precedent for item 1: a state write that fails inside the send path ends as a channel `-32603` (decision/boundary-errors-end-in-logged-line, decision/module-boundary-error-logging), the shape a failing snapshot write after a claim should be read against.

@history_item n=22 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (where -32603 is proved) — no failpoint or counted panic drives a `-32603` in a scanned home; the unit layer holds that fault. Item 1 measures under a failing snapshot write: by this standing record the measurement and its test belong at the unit layer unless the chunk names a new failpoint, which would be a change to that record.

@history_item n=23 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the vt100 feed thread catches a panic per feed/resize, poisons the model and writes one `parse-rejected{vt100-feed}`. Item 2's `crates/viola-agent-claude/src/screen.rs` survivors sit in the screen reading behind this gate; killing tests are tests only and must not move the gate's telemetry (decision/vt100-panics-contained, decision/no-telemetry-from-screen).

@history_item n=24 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (standing-modal row; run terminal output) — added decision/standing-modal-reading-uninstrumented (a standing modal refuses `input-not-ready`, no line or field of its own) and widened contract/run-terminal-output/child-output for a keyed start's automatic-verify lines. Item 2's `screen.rs` tests must hold the uninstrumented reading; item 4's bound on the agents read also runs inside a start's automatic verify.

@history_item n=25 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — the step line gained `absent`, the summary's pass count holds an absent browser-tools-prompt row, exit 0 means every relied row counted; decision/browser-tools-prompt-reading-uninstrumented. Item 9 (the relied-row count from one source, the stamped-set table held once) rests on these verify stdout rows (contract/verify-stdout/step-line, contract/verify-stdout/summary-line, contract/verify-exit-codes/0).

@history_item n=26 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify's stdout became one `[NN/MM]` step line per relied row, `MM` the relied count, the summary summing to it (was per ledger row). The origin of the relied-row count item 9 wants held in one source.

@history_item n=27 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — CI's fake-agent verify gained `--framing`; the framing replay stamps its rows with no new spawn pair, and no row text, prompt or path reaches a line. Item 8 (one step function for `framing_turns`' per-paste probes) must keep that: no new pair, subject or line (decision/verify-lines-carry-no-content).

@history_item n=28 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — added subject `auto-verify` and contract/run-process-log/auto-verify-stop (a stop in the start's verify window ends the start by the signal with no process-exit). A start can run verify, so item 4's agents-read bound and item 7's SIGKILL residue also apply to a start's automatic verify.

@history_item n=29 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — the `verify-sender` pair joined Run B, and contract/verify-process-exit/child-exit-status-and-duration was restated by rule («each verify spawn pair») instead of a count, because the count went stale. Item 4 works inside Run B beside this pair; the restatement is the precedent for the chunk's wrap freight (a drift-base detector over count literals): state the rule, not the number.

@history_item n=30 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (E5: the other snapshot readers, five to three) — «a count carried from a plan is re-measured at the wrap, never copied». The standing rule behind the scope's re-measure of carry coordinates and counts (29 relied rows, survivor counts) at P3, and behind the count-literal drift detector.

@history_item n=31 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`viola verify`'s home-tree check) — verify checks the home tree before its log opens (a refusal opens no log and writes no line); `update_stamps` runs `check_stamps` before and under its lock, a refusal ending through verify's own `process-exit`. Any verify-path change in items 4, 7 and 8 keeps this order and the no-line refusal (contract/viola-verify/home-tree-refused, decision/refused-home-tree-writes-no-line).

@history_item n=32 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`verify` writes its refusal's code) — verify's `process-exit` under a parsed `VIOLA_NAME` carries a closed refusal code (`program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, `interrupted`); a failing row carries none. If item 4's bound or item 7's mend makes verify end differently, it either fits this closed list or is a new closed word for the operator's fork (contract/process-exit-detail/verify-refusal, contract/process-exit-detail/strict-modes-failed).

@history_item n=33 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** — decision/env-names-only now covers the hook capture arm's env-names file and verify's `env_probe` stamp (names only); decision/identity-floor-always-stripped names a compile-time list of non-identity hook names. Item 2 starts with the `env_names_fault` survivors in `crates/viola-agent-claude/src/ledger.rs`, which sit in this env-names stamp path; killing tests must assert names only, never values (decision/messaging-secrets-never-logged).

@history_item n=34 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — on the dev host `target/e2e-home` is a link to an owner-only tmpfs directory, and G2's `find` start point needs a trailing slash to descend it (trap). Items 4 and 5 both read failures under a neighbour's IO stall; any measurement that kept homes or reads G2 over them needs this layout.
