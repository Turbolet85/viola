@history_item n=1 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — ~ contract/viola-verify/step-line: the verify step line now reads `pass|absent|fail`, `absent` only on the browser-tools-prompt row when no run met the prompt (PROVISIONAL, delegate overseer). This chunk's clean `viola verify --record` on 2.1.325 prints these step lines over 24 rows, and C9/C11 (the 8/24 exposure, the `bt-absent` control) rest on that row's `absent` word.

@history_item n=2 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — ~ decision/no-screen-content-parsing gained its except: the test-only standing live rig reads the trust selector's place, the input box as empty / holds-text / absent and modal dialogs as closed words only, raw frames local and gitignored. C7 (`box-text: holds-text` as placeholder hint, a cell attribute unmeasured) and the box-only screen read on the rig sit inside this exception; any wider read of the vendor screen needs a new amendment.

@history_item n=3 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — ~ decision/no-screen-content-parsing: the vendor screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row for its opening char only (PROVISIONAL card confirm). The recorded fixture set's screens and the drift suites must read against these signatures only; recorder hygiene (no host text kept) is the same boundary.

@history_item n=4 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (wrapped-tui-reads-as-unwrapped, time-bounds rows, run-diagnostics-off-terminal) — ~ decision/wrapped-tui-reads-as-unwrapped gained the except that a wrapped `claude` in an untrusted folder opens on its session box (trust record written, flag only); ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver add the start's Claude config lock wait falling on viola's own start; ~ decision/run-diagnostics-off-terminal: the trust step reports one `trust-record` diagnostics line, never a refusal, exit code or hint. The live starts of this chunk (C1/C5/C8, the trust-record row on 2.1.325) run through this start and must keep these rows true.

@history_item n=5 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's interrupted pair: a `verify` stopped by Ctrl-C/TERM prints no step line or summary, only `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI`, exit 1, pinned by the stop table in `tests/cli_verify.rs`; the stop path adds no time bound. Source of the picked contract/interrupted-verify/* and contract/viola-verify/interrupted-pair rows a killed or aborted live `viola verify --record` falls under.

@history_item n=6 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's own word took the provisional mark off the interrupted pair's wording (and the dialog hold, board width, served verbs); the pair is his wording, not to be re-worded by this chunk.

@history_item n=7 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (strict-modes entities, refusal text, witnesses, unstattable home) — `viola run`, a revive start and `viola verify` over a home tree failing strict-modes refuse with exit 1 before any log opens, the home-state pair, `hint:` last, no child, `verify` printing no step line or summary; `update_stamps`'s own refusal gives the same pair; pinned by `HOME_PAIR` / `INSTANCE_PAIR` literals and `verify_refuses_a_widened_ledger_before_any_step`. Origin of contract/start-refusal/home-strict-modes and decision/strict-modes-refusal-before-any-write / unstattable-home rows; the record home a live `verify --record` takes (C6, no `home` component) must still pass this check.

@history_item n=8 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (socket-dir start refusal; client euid limit of proof) — `viola run` and a revive start over a failing socket directory refuse with exit 1 before any snapshot, bind or child, one `unable:` / `hint:` pair, directory left as found (origin of contract/start-refusal/socket-dir); and the client euid refusal is tested over injected euids, each seam's wiring read only by forced refusals in remove-the-guard runs (decision/client-euid-refusal-test-strategy) — the precedent for C11's remove-the-guard `bt-absent` control read on a filtered run.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (CI home, tui boundary on openpty, strict-modes, platform, screen reader) — a11y cases run in the `coverage` suite on the one `ubuntu-latest` leg; the tui boundary is openpty only, each boundary clause its own nextest case, zero-viola-bytes compared byte for byte with an unwrapped fake-agent run; strict-modes Windows arms retired; Linux-only platform; the manual SR pass is Orca alone, founder-owned. The fake agent replaying the new 2.1.325 set in CI lands under these rows (decision/a11y-ci-github-actions, decision/zero-viola-bytes-on-the-terminal).

@history_item n=10 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — cli / tui a11y cases are nextest cases in the `test` job's `coverage` suite, gated by `gate --require coverage,doctest`; a breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`) fails the build; `#[ignore]` banned. Origin of decision/a11y-gate-breach-fails-build, decision/gate-requires-coverage-doctest and decision/ci-test-gate-no-skips: the new drift suites over 2.1.325 may not land skipped or ignored.

@history_item n=11 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — the CLI output-discipline tests report in CI inside the `coverage` suite under `gate --require coverage,doctest` (decision/cli-invariants-pass-in-ci).

@history_item n=12 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — `ci.yml` is the one push/PR workflow carrying a11y; the scheduled `nightly.yml` carries no a11y step (decision/a11y-ci-github-actions). Any fixture-drift check this chunk adds belongs in the push/PR workflow, not a scheduled one (the scope also bans timers).

@history_item n=13 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CLI output-discipline invariants meet mutation only at the epoch-boundary audit, no per-chunk mutation gate; consistent with this chunk's «no mutation run» boundary.

@history_item n=14 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (literal pins, portable-pty =0.9.0, dialog hold owner) — cli checks are `std::process::Command` with every fixed CLI string pinned by literal (decision/fixed-cli-strings-pinned*, decision/refusal-layout-literal-pins, decision/cli-a11y-by-literal-tests); the tui boundary driver is portable-pty `=0.9.0`, never re-pinned by a11y; the dialog hold bounds are PROVISIONAL, owned by 0.2.0's first entry. Any new verify / step-line string this chunk's set touches takes a literal pin.

@history_item n=15 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — the non-editing set became the closed list of focus, mouse reports and terminal replies, every other byte editing; case (3) probes the wheel by an `answer` to no pending dialog. Origin of decision/non-editing-sequences and decision/stdin-classifier-non-editing-list, which bind any live start this chunk records.

@history_item n=16 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the closed list widened by seven `CSI` reply grammars (ratified), and the first live takeover reading recorded as supplemental, never gating (decision/live-readings-never-gate): this chunk's live readings on 2.1.325 likewise never gate CI.

@history_item n=17 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's second live reading, the founder's own key on the product over live `claude` 2.1.296; supplemental and never gating — precedent for how a live reading on the product binary is recorded.

@history_item n=18 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — a verified send waits in the readiness gate up to 8.5 s, then `input-not-ready`; a human key during the wait takes the wheel and the send is refused `human-typing`; the outer-PTY case is sequenced on records, never a timer. Origin of decision/human-key-takes-the-wheel, decision/boundary-tests-sequenced-on-records and contract/send-refusal/input-not-ready.

@history_item n=19 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — a confirmed send's 10 s confirm window and `/compact`'s 40 s window (PROVISIONAL), unconfirmed → `no-prompt-submitted`, a bound on the driver only (contract/send-refusal/no-prompt-submitted). C4's live `/context` post-condition measurement sits on this local-command confirm path.

@history_item n=20 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the dialog hold `dialog_hold_s` 60–3600 s, default 60, one `dialog-expired` line at its end, any human key handing the dialog back at once (contract/config-json/dialog-hold-s, contract/event-line/dialog-expired, decision/dialog-hold-bounds-the-driver).

@history_item n=21 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — ~ decision/dialog-hold-bounds-the-driver: provisional → standing after the ceiling was measured live. The same chunk is the source of this chunk's C1 carries (the tools-list re-record and the clean `verify --record`).

@history_item n=22 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the MCP `wait` tool clamps `timeout_ms` to 50 000 ms, also its default, a driver-only bound (contract/mcp-wait-tool/timeout-ms); the `Mcp.tools-list-1.json` re-record (C1) captures this server's tools list.

@history_item n=23 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — exit 11 retired and never reused (an older `budget-paused` reads `unknown`, 14); the budget pause left decision/time-bounds-fall-on-the-driver and decision/time-bounds-never-on-human. A recorded set must not reintroduce a budget-era word or exit.

@history_item n=24 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — the status-line deadline left both time-bounds rows and the statusline passthrough case was removed; `viola hook statusline` is an unknown hook event. Lineage of the time-bounds rows the live starts must keep.

@history_item n=25 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `list` and `revive` each make one 5 s `claude agents --json` read, failing open, the bound on viola's own child (the «agents read» in both time-bounds rows); the board's captions and slots (decision/critical-path-cli-board-output).

@history_item n=26 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (failure documents, exit-2 exception, `unknown`'s detail, peer screening) — each `--json` failure is one clean document (decision/json-failure-documents-are-clean); `unknown` (exit 14) has a code word or nothing as detail and no hint; a peer's refusal that is no code word reads `unknown`, a code word outside the closed list stays visible (decision/peer-replies-screened-to-code-words, decision/unlisted-code-words-stay-visible, contract/code-word/format). C2's live `StopFailure` `error` word must be read against this code-word discipline and the cause map without new `parse-rejected` noise.

@history_item n=27 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (stale vs live holder, exit-cause table) — a start against a holder tells `stale-heartbeat` from `already-live` (contract/start-holder-refusal/*); `tests/cross_exit_causes.rs`, a `fake-agent` binary, holds each cause's exit, document and hint, one hint per code. The fake agent replaying the new fixture set drives this table.

@history_item n=28 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough boundary like `viola run` (contract/tui-passthrough/entry, decision/single-vendor-content-boundary); C10's Run E (resume) reading of whether a revived start spawns lives inside this boundary.

@history_item n=29 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — the tui passthrough boundary holds after a contained vt100 feed panic: the driver's send is refused `input-not-ready`, no viola literal reaches the outer terminal. Bears on contract/tui-passthrough/vendor-content (viola parses the vendor screen through a vt100 feed) when new 2.1.325 screens are replayed.

@history_item n=30 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the colour decision reads `NO_COLOR`, `TERM`, `COLORTERM` and terminal state, no SGR under `--json` or a passthrough role (decision/no-sgr-off-terminal); `viola wait` and `--help` carry no ESC or lone CR (decision/linear-static-cli-output).

@history_item n=31 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the frontend stubs retired to N/A: no web or graphical frontend, the terminal fleet console's a11y scope at «Fleet a11y verification»; origin of the absences and N/A lines in the picked rows file.

@history_item n=32 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (decision · contract · absence), each verified against the code; every picked row's handle and its relied/guard fields date from this conversion, so later row edits are the `~`/`+`/`−` entries above.
