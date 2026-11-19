@history_item n=1 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify/step-line`: verify's step line became `[NN/NN] … pass|absent|fail`, `absent` only on the browser-tools-prompt row when no run met the prompt (PROVISIONAL, delegate overseer). The `viola verify --record` run on 2.1.328 prints these lines; an `absent` there is that row's own word, not a fault, and the line is pinned by literal.

@history_item n=2 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — set verify's interrupted pair: a verify stopped by Ctrl-C or TERM prints no step line and no summary, only `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI` last on stderr, exit 1, pinned by the stop table in `tests/cli_verify.rs`; the stop path adds no time bound. This is the source of `contract/interrupted-verify/exit`, `/stderr` and `contract/viola-verify/interrupted-pair`. A live recording that gets stopped falls under it, and it must not be read as a stamp.

@history_item n=3 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch made the interrupted pair's wording his (the provisional mark on 2026-10-13-interrupted-verify-cleanup was superseded). The pair's literal stands ratified, so a recording chunk does not reword it.

@history_item n=4 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `viola run`, a `viola revive` start and `viola verify` refuse a home tree that fails the strict-modes check before any log opens: exit 1, stdout empty, the home-state pair with `hint:` last, no child; verify prints no step line or summary (`verify_refuses_a_widened_ledger_before_any_step`, `verify_under_a_regular_file_refuses_the_home`). This is the source of `contract/start-refusal/home-strict-modes`. The rig's private config home for `R4` / `R4M` must pass the check, or the recording stops at that refusal.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — the tui boundary moved to openpty alone, so the zero-viola-bytes case compares byte for byte with an unwrapped fake-agent run. The a11y cases run in the one `ubuntu-latest` `coverage` suite under `gate --require coverage,doctest`; the strict-modes Windows arms and the `verify_under_a_regular_file_refuses_the_home` per-OS oracle were retired; the manual SR pass is Orca alone. Any new drift or recorded-exchange case over the 2.1.328 set lands on that one leg, none skipped.

@history_item n=6 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `~ decision/no-screen-content-parsing`: the vendor screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), and the prompt row only for its opening char (PROVISIONAL card confirm). A moved literal on 2.1.328 (e.g. the trust dialog's decline text) can only be met by changing a compiled signature inside this bound, never by a wider screen read.

@history_item n=7 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — `~ decision/no-screen-content-parsing` gained its except: the test-only standing live rig reads the trust selector's place, the input box state and modal dialogs as closed words only, its raw frames kept local and gitignored. The `R4` / `R4M` live starts run on this rig: its frames stay out of the landed fixture set, and the recorder's scrub binds every landed file.

@history_item n=8 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — four row edits. `~ decision/wrapped-tui-reads-as-unwrapped` except: in an untrusted folder a wrapped `claude` opens on its session box, the start having written the trust record and typed no key. `~ decision/run-diagnostics-off-terminal`: the trust step reports one `trust-record` diagnostics line, never a refusal, exit or hint. `~ decision/time-bounds-never-on-human` and `~ decision/time-bounds-fall-on-the-driver`: both add the start's Claude config lock wait. On 2.1.328 the trust prompt shows only with `--add-dir` and gains a decline text; whether the trust step still holds must be read against these rows.

@history_item n=9 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — added the feed-panic boundary (`tests/chaos_feed_panic.rs`): send refused `not-delivered` / `input-not-ready` (exit 13) while a human key still reaches the child, and no viola literal on the outer terminal. This is the source of `contract/tui-passthrough/vt100-feed-panic` and `/after-feed-panic`. A 2.1.328 screen the vt100 feed cannot parse has to fall under this contained path.

@history_item n=10 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — widened the closed non-editing stdin list (seven more `CSI` replies by exact grammar) and recorded the first live takeover reading on Linux as supplemental, never gating. This is the precedent for `decision/live-readings-never-gate` and `decision/stdin-classifier-non-editing-list`: the chunk's live recording is evidence, never a CI gate.

@history_item n=11 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's second live reading came from the founder's own key on a live `claude` 2.1.296 session (`wheel{human}`, the next send exit 10, release back to the driver). It is supplemental and never gating, and it is the precedent for how a live reading on a new CLI version is recorded.

@history_item n=12 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — the non-editing set became the closed list of focus, mouse (X10, SGR, urxvt) and terminal replies (DA1, DA2, CPR, DECRPM, kitty flags, OSC, DCS), proven by the classifier's unit table; every other byte is editing. This is the source of `decision/non-editing-sequences` and `contract/tui-passthrough/focus-mouse-resize-sequences`, which recorded exchanges must not contradict.

@history_item n=13 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — a verified send waits in the readiness gate for at most 8.5 s and is then refused `not-delivered` / `input-not-ready`; a human key during the wait takes the wheel (`send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, sequenced on records, never a timer). This is the source of `contract/send-refusal/input-not-ready` and `decision/human-key-takes-the-wheel`, both of which the recorded readiness signatures feed.

@history_item n=14 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — added the confirm windows (10 s; 40 s for `/compact` on a verified CLI, PROVISIONAL) with `not-delivered` / `no-prompt-submitted` when unconfirmed, a bound on the driver only. This is the source of `contract/send-refusal/no-prompt-submitted`. It bears on C3: the rig's `L2` `/context` send read exactly this refusal on 2.1.328.

@history_item n=15 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — introduced the dialog deadline as a driver-only bound: at expiry the dialog renders for the human in the `claude` TUI. This is the origin of `decision/dialog-hold-bounds-the-driver` and the critical dialog path, which recorded dialog exchanges replay.

@history_item n=16 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — made the dialog hold configurable: `dialog_hold_s` 60–3600, default 60, a `dialog-expired` line at its end that wakes `wait`, and any human key handing the dialog back at once. This is the source of `contract/config-json/dialog-hold-s` and `contract/event-line/dialog-expired`.

@history_item n=17 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ decision/dialog-hold-bounds-the-driver` went from provisional to standing after the ceiling was measured live. The row is now settled ground for any live dialog reading in the recording.

@history_item n=18 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the MCP `wait` tool clamps `timeout_ms` to 50 000 ms, also its default, a driver-only bound. This is the source of `contract/mcp-wait-tool/timeout-ms`; the `viola-mcp-tap` re-record (`R4M`) replays MCP exchanges under it.

@history_item n=19 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — exit 11 was retired (an older wrapper's `budget-paused` now reads `unknown`, exit 14), the budget pause was dropped from both time-bound rows, and `budget-paused` left the board's state words. A recorded set must carry no `budget-paused` exchange or exit 11.

@history_item n=20 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — the statusline deadline was dropped from both time-bound rows, `− decision/statusline-output-passed-byte-for-byte` and `− contract/viola-hook-statusline/exit`. A 2.1.328 recording has no statusline passthrough case to re-record.

@history_item n=21 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — the cli checks are on `std::process::Command` with literal pins (no trycmd/assert_cmd), which is the source of `decision/fixed-cli-strings-pinned`, `decision/refusal-layout-literal-pins` and `decision/cli-a11y-by-literal-tests`. The outer-PTY driver moved to portable-pty `=0.9.0`, inherited and never re-pinned (`decision/tui-boundary-driver-pin`, `decision/portable-pty-outer-pty-driver`). The dialog hold bounds are owned by 0.2.0's first entry. Any new 2.1.328 literal lands as a literal pin.

@history_item n=22 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master's text into rows, each verified against the code (246 rows). This is where every picked row's handle originates and the baseline later row edits amend.

@history_item n=23 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the cli/tui a11y cases run as nextest cases in the `coverage` suite under `gate --require coverage,doctest`. A breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`) fails the build, and `#[ignore]` is banned. This is the source of `decision/a11y-gate-breach-fails-build`, `decision/zero-flakiness` and `decision/gate-requires-coverage-doctest`: a drift suite red on the landed set cannot be skipped.

@history_item n=24 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — `ci.yml` is the one push/PR workflow carrying the a11y checks, and `nightly.yml` carries none. This is the source of `decision/a11y-ci-github-actions`; the chunk adds no scheduled a11y run.

@history_item n=25 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough boundary and writes no line of its own while the child holds the terminal (`tests/chaos_revive.rs`). This is the source of `contract/tui-passthrough/entry` and `/own-bytes`, which recorded starts replayed by the fake agent must keep.

@history_item n=26 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — built `tests/cross_exit_causes.rs`, the `fake-agent` E2E exit-cause table, checking each cause's exit, document and hint, plus `no_two_codes_share_a_hint`. Any fake-agent change keyed to a version for 2.1.328 must keep this table green.

@history_item n=27 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `unknown` (exit 14) carries a code word or nothing and prints no hint, and a peer's refusal is screened to a code word. This is the source of the `decision/refusal-ends-with-hint` except and bears on how a recorded peer's reply is read.

@history_item n=28 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — `viola run`'s socket-directory refusal took the fixed `unable: <text>` + `hint:` form beside revive's and verify's interrupted pair. This is the source of the exceptions named in `contract/cli-refusal/unable-line`. The rig's private config must also pass the socket-directory check.

@history_item n=29 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the board's colour spans with each state carried by its word, and no SGR under `NO_COLOR`, `TERM=dumb`, `--json`, a passthrough role or non-TTY. This is the source of `decision/no-sgr-off-terminal` / `decision/no-sgr-in-plain-modes` and the board's `unverified-cli` cell (`decision/codes-beside-plain-word`), which a 2.1.328 stamp state shows.

@history_item n=30 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the frontend stubs became N/A (no web or graphical frontend). This is the source of the absences `absence/web-or-graphical-frontend` and `absence/rendered-page` and of the N/A sections: the chunk brings no browser or page a11y tooling.
