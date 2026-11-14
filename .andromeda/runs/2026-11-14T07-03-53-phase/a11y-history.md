@history_item n=1 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Linux-only platform line) — added «Platform (every surface): Linux only — the core on any current distribution, its CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland»; `os` reads `linux` and the multi-platform trigger holds on `[ubuntu-latest]`. This is the origin of `decision/linux-only`, the row the seam rests on: Hyprland is the window features' only target, and nothing off Linux needs an arm.

@history_item n=2 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the a11y cases' CI home, the one ubuntu-latest leg) — §9's matrix became `[ubuntu-latest]`. The cli and tui cases run in the `test` job's single `coverage` suite there, and its `gate --require coverage,doctest` is the verdict; the runner is `scripts/agent-run.sh run` alone. This sets `decision/a11y-ci-github-actions` and `decision/gate-requires-coverage-doctest`. The seam's fake-compositor tests must run green on that one leg, where no compositor exists. They are never skipped there and never get a leg of their own.

@history_item n=3 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the tui boundary on openpty alone) — the zero-viola-bytes case compares the outer-PTY stream byte for byte with an unwrapped fake-agent run, and each of the three boundary clauses is its own nextest case on `ubuntu-latest`. The ConPTY/Windows readings are retired. This is the current form of `decision/zero-viola-bytes-on-the-terminal` and `contract/tui-passthrough/viola-terminal-bytes`. Nothing the seam does (calls, `unsupported` outcome, logging) may put a byte on the outer terminal.

@history_item n=4 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the manual screen-reader pass, Orca on Linux alone) — the tui pairing reads Orca + foot on Hyprland, the founder's host's one terminal (an overseer technical answer, founder-delegated). This is the existing Hyprland reference in the a11y plan. The seam does not touch the manual pass, which stays founder-owned and gates nothing.

@history_item n=5 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — §9: `ci.yml` became "the one push/PR workflow", and the scheduled `nightly.yml` carries no a11y step. This is the source of `decision/a11y-ci-github-actions`. Any seam check belongs in `ci.yml`'s test job, never in nightly.

@history_item n=6 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — in CI, output-discipline tests report inside the `test` job's single instrumented `coverage` suite, gated by `gate --require coverage,doctest`; failures surface only in `suites[].failures[]`. This is the origin of `decision/gate-requires-coverage-doctest` and `decision/ci-test-gate-no-skips`. The seam's replay tests land in that suite.

@history_item n=7 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the browser a11y leg was removed, and cli/tui cases became plain nextest cases in the `coverage` suite. §10's failure condition became a `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`), the zero-flakiness parallel bans `#[ignore]`, and §11 bans a separate a11y driver or `suite` value outside `scripts/agent-run.* run`. This is the source of `decision/a11y-gate-breach-fails-build` and `decision/zero-flakiness`. The fake compositor must be deterministic: no `#[ignore]` for "needs a compositor", no retries, no driver of its own.

@history_item n=8 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the per-chunk and CI mutation gates were removed, and the epoch-boundary audit (`run --mutants`) covers the output-discipline invariants. This matches the chunk's no-mutation-run directive. Guard holding is read by a `gate.py control`, not by a mutation gate.

@history_item n=9 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — recorded the first live reading of the takeover clause on Linux, "supplemental and never gating": a `wtype` key into a live session's window on a compositor the test started itself. This is the origin of `decision/live-readings-never-gate`, and the precedent for live compositor work. The seam's L1–L3 live-desktop measurements are recorded as fixtures and readings and never become a CI gate.

@history_item n=10 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — added the takeover clause's second live reading, on the product binary: a key the founder typed himself, with no agent typing it. It remains supplemental and never gating. This extends `decision/live-readings-never-gate`, and it is the precedent that live steps needing a human key are the founder's. The scope plans no typed key.

@history_item n=11 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — §8: `list` and `revive` each make one `claude agents --json` read (direct spawn, `take(MAX_FRAME + 1)`, a 5 s kill). A failed, oversize, malformed or timed-out read fails open, and the bound falls on viola's own child, never on a key or the human. Unwrapped NAMEs print through `escape_row`. This is the direct precedent for a `hyprctl … -j` read: a capped, killed child read of untrusted output. Its time bound must join §8's list under `decision/time-bounds-never-on-human` and `decision/time-bounds-fall-on-the-driver`.

@history_item n=12 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (a peer's refusal screened to a code word) — `reply_of`/`of_reply` screen a peer's reply with `viola_core::is_code_word` (1–64 bytes of `[a-z0-9-]`, a lower-case letter first). A non-code-word refusal reads `unknown`, and a non-code-word string detail reads nothing. This is the source of `decision/peer-replies-screened-to-code-words` and `contract/code-word/format`. The seam's closed outcome codes (ok, unsupported, unreachable, malformed) must be code words, and no compositor reply text may pass through as a detail.

@history_item n=13 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the census of error writers and the body check) — `tests/contract_error_bodies.rs` holds three guards: `error_bodies_each_name_a_writer_and_a_witness_that_stand`, `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing`. `tests/support/events.rs` holds `assert_error_body_clean` (`absolute-path`, `caused-by`, `serde-path`, `planted`). These are the guards behind `decision/error-bodies-have-writer-and-witness` and `decision/thiserror-messages-interpolate-nothing`. A new seam error type must name its writer and witness in the census, and its `thiserror` messages must interpolate no reply text, socket path or pid.

@history_item n=14 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`unknown`'s detail, a code word or nothing; a fault's body and the `wrapper fault` literal) — `unknown`'s detail became "a code word or nothing". Out-of-set fault codes read the fixed `wrapper fault` through `sanitised_fault`. This is the precedent for mapping untrusted upstream content to closed values. The seam's `malformed` and `unsupported` outcomes should follow the same closed-value shape.

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the `--json` failure documents' witness) — `cli_error_documents_hold_no_path_or_upstream_text` holds each failure to one document with no path or upstream text and no ESC. This is a constraint for any later verb surfacing a seam outcome. Compositor reply text counts as upstream text and must reach no document.

@history_item n=16 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the client euid refusal's limit of proof) — one account can make no foreign peer and "no seam may be added". Each client seam's wiring is read only by forced refusals in remove-the-guard runs. This is a trap for the seam's guard tests: state the limit of proof the fake compositor leaves (real-socket wiring is read only live), and hold guards by `gate.py control` runs.

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the tui boundary and the wrapper's drop of a refused peer; hook and MCP on a refused socket directory: not-assertable) — a refused peer was added as a boundary clause that writes nothing to the outer terminal and moves no wheel. Hook/MCP seam refusals are recorded as not-assertable because they write no human-facing byte. This is the precedent for classing the Hyprland seam: it writes no human-facing byte, so it is not assertable or a boundary-only entity (cf. `decision/no-assertions-for-internal-crates` if it is a new crate). The socket-dir checks are the precedent if the request socket is chosen.

@history_item n=18 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the tui passthrough boundary after a contained vt100 feed panic) — the boundary still holds after a contained parse panic: no viola literal (`panic`, `parse-rejected`, `unable`, `hint:`) reaches the outer terminal. This is the precedent for bounded, untrusted input never leaking to the terminal. A malformed compositor reply must be contained the same way, never as a panic.

@history_item n=19 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — the TUI passthrough boundary binds a passed `viola revive` too: it writes no line of its own while the child holds the terminal. This is the source of `contract/tui-passthrough/own-bytes`. Any seam call made while a child holds the terminal (e.g. a later `up`/console) writes nothing there.

@history_item n=20 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — the statusline start has its own zero-viola-bytes reading. `STATUSLINE_DEADLINE` (5 s, PROVISIONAL) falls on the user's own command, never on the human, and §8 lists every bound with whom it falls on. This is the precedent: a new bound (the seam's call deadline or kill) needs its §8 line naming its bearer.

@history_item n=21 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — the first §8 time bound, the dialog deadline, bounds the driver and never the human. This is the start of the register behind `decision/time-bounds-never-on-human`, which the seam's deadlines extend.

@history_item n=22 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the send readiness gate's `GATE_MAX_WAIT` falls on the driver only. A human key during the wait wins, and its case is sequenced on records, never a timer. This is part of the time-bounds register. It is also the sequencing precedent: seam tests should wait on records or deadlines, not sleeps, to stay flake-free.

@history_item n=23 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — the budget pause was added to §8 as a driver-only bound. It is in the time-bounds register (`decision/time-bounds-fall-on-the-driver`) that a seam bound extends.

@history_item n=24 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the MCP `wait` cap (`WAIT_CAP` 50 000 ms) was added as a driver-only bound. It is in the time-bounds register.

@history_item n=25 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the confirm windows (`CONFIRM_WINDOW_FALLBACK` 10 s, `COMPACT_CONFIRM_WINDOW` 40 s) were added as driver-only bounds. They are in the time-bounds register.

@history_item n=26 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the dialog hold became configurable (`dialog_hold_s` 60–3600). The hold bounds the driver, and any human key hands the dialog back at once. It is in the time-bounds register.

@history_item n=27 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the dialog hold bounds owned by 0.2.0's first entry) — `DIALOG_HOLD_MIN`/`DIALOG_HOLD_MAX` stay PROVISIONAL, owned by "0.2.0's first entry". Check whether this Epoch 1 — Foundation chunk is that owner. If it is not, leave the bounds alone.

@history_item n=28 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the cli output-discipline checks on std::process::Command and literal pins) — cli checks drive the binary through `std::process::Command` with fixed strings pinned by literal; trycmd and assert_cmd are retired. This is the test form a11y expects of any new fixed string or outcome word. The outer-PTY driver is portable-pty `=0.9.0`, which a11y never re-pins.

@history_item n=29 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0 — the frontend stubs retired) — 0.2.0 has no web or graphical frontend. The terminal fleet console (matrix#v2-14) takes its a11y scope at «Fleet a11y verification», and §5–§7 and the web harness keys read N/A. This is the source of the absences and N/A overviews in the rows file. The seam serves window features (v2-07/11/12/14) without adding a graphical a11y surface; the console's keyboard reach stays with that entry.

@history_item n=30 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `decision/no-screen-content-parsing` was narrowed: vendor screen content is parsed only for compiled signatures, never for other text. This is the precedent for treating window titles and classes in compositor replies as content: never parsed as commands and never echoed.

@history_item n=31 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows, each verified against the code, with fates set by the founder. This is the origin of every picked row's current wording. A seam change that moves a row's promise needs a `~`/`+` row amendment, not prose.
