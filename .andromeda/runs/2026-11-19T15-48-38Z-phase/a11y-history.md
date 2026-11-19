@history_item n=1 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — ~ contract/viola-verify/step-line: verify's step line went from `pass|fail` to `pass|absent|fail`, `absent` only on the browser-tools-prompt row when no run met the prompt (PROVISIONAL, delegate overseer). This is the precedent if the StopFailure settlement lands a ledger row with its `viola verify` probe: a new row adds a `[NN/NN]` step line, the NN count moves, and a probe that may not meet a StopFailure on every run has to decide whether it may print `absent`. The row's promise allows that word only on the browser-tools-prompt row today.

@history_item n=2 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — ~ decision/no-screen-content-parsing except: the test-only standing live rig may read the trust selector's place, the input box (empty, holds-text, absent) and modal dialogs, as closed words only, its raw frames kept local and gitignored. This chunk mends that rig (C5, the record dir keyed by CLI version under `target/live-rig/<label>/record/<version>/`) and records through it. The mend must keep the frames local and gitignored and the readings closed words, and it must not widen what the rig reads of the vendor screen.

@history_item n=3 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — ~ decision/no-screen-content-parsing rule: the vendor screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row only for its opening char, a boundary widening ratified by the founder (card confirm PROVISIONAL). A StopFailure probe or reading that tried to read error text off the rendered TUI would be a new widening of this rule. The payload is meant to come through the hook, not the screen.

@history_item n=4 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (wrapped-tui-reads-as-unwrapped) — ~ decision/wrapped-tui-reads-as-unwrapped except: in an untrusted folder a wrapped `claude` opens on its session box, because viola's start wrote the CLI trust record (the flag alone) and typed no key. A contained live start under the rig's private Claude config goes through this start path. Its first screen is the session box, not the trust form, and no key is typed into any CLI dialog (scope boundary).

@history_item n=5 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (run-diagnostics-off-terminal) — ~ decision/run-diagnostics-off-terminal: the start's trust step reports its word in one `trust-record` line in `diagnostics/`, never as a refusal, exit code or hint. Whatever the StopFailure turn end writes (its `cause`, any `parse-rejected` `unknown-value` line) goes to the log and diagnostics only. Nothing reaches the terminal, in line with contract/tui-passthrough/run-diagnostics and viola-terminal-bytes.

@history_item n=6 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (time bounds) — ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver: the start's Claude config lock wait joined the list of bounds, falling on viola's own start. These are the rows that list every time bound viola holds. If containment adds a bound viola itself holds (a lost-connection wait, a probe timeout in a verify row), the rule says it must fall on the driver, the child or the start, never on a human key, and the list must name it.

@history_item n=7 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — ~ decision/dialog-hold-bounds-the-driver: the state went from provisional to standing after a live measurement. This is the precedent for this chunk's settlement: a PROVISIONAL exception settled by a live reading changes a row's state only once the reading stands. The StopFailure exception (C1/C2) is settled on the founder's word, either as a ledger row or kept.

@history_item n=8 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — ~ decision/time-bounds-fall-on-the-driver and ~ decision/time-bounds-never-on-human: these dropped the budget pause, and contract/cli-exit-codes/11 was retired and never reused (an older wrapper's budget-paused reads unknown, exit 14). This is how a picked time-bound row is edited when a bound leaves. A retired exit code is never reused, which matters if the settlement maps a StopFailure cause to an exit.

@history_item n=9 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver dropped the status-line deadline. This is the history of the two picked time-bound rows that any new bound from this chunk would extend.

@history_item n=10 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — this added verify's interrupted pair (`unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, exit 1, no step line or summary; wording later the founder's, 2026-10-20), now contract/viola-verify/interrupted-pair. The pair also says the stop path adds no time bound. A new StopFailure verify probe must keep that: its waits end at the stop.

@history_item n=11 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (strict-modes refusals of `run`, revive's start and `verify`) — this made `run`, a revive start and `verify` refuse a home tree that fails strict modes before any log: exit 1, stdout empty, the home-state pair, no child, and `verify` prints no step line or summary. It is now contract/start-refusal/home-strict-modes. The rig's contained start under a private Claude config and a test-owned home must pass this check, and a verify probe row must sit after it.

@history_item n=12 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — this added the Linux-only platform line (decision/linux-only) and put the tui boundary on openpty alone, each boundary clause its own nextest case on `ubuntu-latest`. A11y cases run in the `coverage` suite there. Any containment means (a no-network namespace via `unshare`, `ANTHROPIC_BASE_URL` to a closed port) and any new case are Linux and `ubuntu-latest` only, with no other-OS arm.

@history_item n=13 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — this moved the tui boundary driver to portable-pty `=0.9.0`, inherited and never re-pinned (decision/tui-boundary-driver-pin, portable-pty-outer-pty-driver). It also re-worded the CLI checks to `std::process::Command` with literal pins (decision/cli-a11y-by-literal-tests, fixed-cli-strings-pinned). Any new fixed string a verify probe prints (its step label) must be pinned by literal, and the rig's PTY is not re-pinned.

@history_item n=14 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — this added the takeover clause's second live reading on the product over a live `claude`, supplemental and never gating (decision/live-readings-never-gate). It is the precedent for this chunk's live StopFailure reading: it informs the settlement but gates no CI case.

@history_item n=15 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — this recorded the first live reading of the takeover clause on Linux, supplemental and never gating. It also widened the closed non-editing list (decision/non-editing-sequences) by exact grammar. This is the origin of the live-readings-never-gate rule that this chunk's contained live starts fall under.

@history_item n=16 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — this made the non-editing set the closed list of focus and mouse reports and terminal replies, with every other byte editing, and stepped case (3) on receipts (decision/non-editing-sequences, wheel-ignores-terminal-events). A contained start that types nothing into the CLI must still leave the wheel untouched. Any rig input stays inside the closed list or is a driver send.

@history_item n=17 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — this sequenced the outer-PTY boundary case on the wrapper's `channel-request` line and the `wheel` record, never a timer (decision/boundary-tests-sequenced-on-records). Any test this chunk adds around a StopFailure turn end must be sequenced on log records, not timers, in line with the scope's 20 s wait lint.

@history_item n=18 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — this made a passed `viola revive` enter the TUI passthrough boundary, writing no line of its own while the child holds the terminal, and added it to the zero-SGR list (contract/tui-passthrough/own-bytes, decision/no-sgr-in-plain-modes, single-vendor-content-boundary). A StopFailure turn end inside a revived or run start writes nothing of viola's to the terminal.

@history_item n=19 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (feed-panic boundary) — this added the boundary after a contained vt100 feed panic: none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal. This chunk reads that no `parse-rejected` `unknown-value` line is written for a word of the CLI's 13. Any such line is a log line only, never terminal bytes.

@history_item n=20 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (a peer's refusal screened to a code word; a word outside the list) — this made a code word outside the closed refusal list visible by design, in the document and in `refused: <word>`, taking `unknown`'s exit 14, and set the code-word format (1–64 bytes of `[a-z0-9-]`, a lower-case first). These are now decision/unlisted-code-words-stay-visible and contract/code-word/format. They are the precedent for how a live StopFailure `error` word outside the nine-word cause map shows. It also matters whether the CLI's 13 words all meet the code-word format.

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's `internal-error` document) — this added the internal-error document under `--json` where a verb prints `error: internal error`, with `run`, revive's start, mcp and hook staying silent (contract/json-fault/internal-error). A StopFailure turn end inside `run` or a hook stays silent on the terminal. Any new verify probe failure follows the catch-site form.

@history_item n=22 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — this made the dialog hold configurable and the bound fall on the driver; at the hold's end one `dialog-expired` line wakes `wait` (contract/event-line/dialog-expired). It is the precedent for a wrapper-sourced event line appended at a turn's or dialog's end. A StopFailure turn end's `cause` line extends the same event-line register.

@history_item n=23 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — this put the CLI output-discipline invariants (P4, P6) in crate tests under the epoch-boundary mutation audit (decision/mutation-audit-covers-crate-invariants). The scope says no mutation run this chunk, so new cases wait for the epoch audit.

@history_item n=24 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — this put the cli and tui cases into the `coverage` suite gated `coverage,doctest`, with `#[ignore]` banned (decision/zero-flakiness). C4's intermittent rig rehearsal red is handled under that rule: no retry and no loosened entry, the cause read at its next red.

@history_item n=25 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the version census for 0.2.0 retired the frontend stubs, leaving no web or graphical frontend. §5, §6 and §7 and the WCAG, focus, contrast and violation keys now read N/A, and the absences are picked here. This chunk adds no rendered surface, so those absences stand.

@history_item n=26 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — this turned the master into rows: a master holds what a planner must respect and the code holds the rest. It is the baseline of every picked row's handle that this chunk's amendments (a possible ledger-row and verify-probe row edit) would `~` or `+`.
