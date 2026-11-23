@history_item n=1 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** — `+ decision/start-verify-stop-grace-off-the-human`: the two time-bound lists (`decision/time-bounds-fall-on-the-driver`, `decision/time-bounds-never-on-human`) are full at their bound, and the operator's ruling lands a new time bound as a row of its own beside them. This is the precedent for this chunk's new bounds: `HYPRCTL_DEADLINE` 5 s (carry `cry-2oejf2md`), open's 30 s liveness wait and its 5 s window-appearance wait each become their own row, never an edit to the full lists.

@history_item n=2 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — (rows) `~ decision/time-bounds-fall-on-the-driver` / `~ decision/time-bounds-never-on-human` gained «paste write», which filled both lists, and the verify stop grace was sent to a row of its own. `~ contract/tui-passthrough/own-bytes`, `~ decision/zero-viola-bytes-on-the-terminal`, `~ decision/wheel-ignores-terminal-events` and `~ decision/portable-pty-outer-pty-driver` each name the keyed start's automatic-verify lines as the one exception to zero viola bytes. The `viola run` that open starts inside foot must keep these, and open's own output must not reach the child's terminal. The entry also ratified the Epoch 4 PROVISIONAL marks.

@history_item n=3 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — `~ decision/time-bounds-fall-on-the-driver` and `~ decision/time-bounds-never-on-human` gained «MCP call deadline», and `+ contract/instance-unreachable-detail/call-deadline` was added. This is the last time a bound was added to the lists before they filled. It shows the form a driver-only bound takes (it ends the driver's call and never touches a key), which open's 30 s / 5 s waits and the compositor read bound must follow.

@history_item n=4 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — The time-bound rows were widened to bounds that fall on «viola's own start» (the Claude config lock wait). `~ decision/run-diagnostics-off-terminal` gained the `trust-record` line, which goes only to `diagnostics/` and is never a refusal, exit code or hint. `~ decision/wrapped-tui-reads-as-unwrapped` gained the trust-form exception. Open's placement outcome (`unplaced`, off-Hyprland skip) is recorded the same way, in the wrapper's state and log and never on the terminal. Its waits fall on open's own start, not on the human.

@history_item n=5 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ contract/cli-exit-codes/11`: exit 11 is retired and never reused, and an old `budget-paused` reads `unknown` exit 14. The budget pause also left the time-bound lists. The scope says «no exit 11», so open's exit map must not reuse it.

@history_item n=6 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — The 5 s statusline deadline and the bound on the user's command were dropped from `decision/time-bounds-*`. This shows that a retired bound leaves the lists whole. It is also the earlier 5 s bound precedent beside the new 5 s `HYPRCTL_DEADLINE`.

@history_item n=7 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — §8 Timeout extensions: the plan lists every bound viola holds and whom it falls on. The 5 s `STATUSLINE_DEADLINE` was added as a bound on the user's command, never on a key, with the wheel untouched. This is the original rule that each new bound (here the compositor read, the liveness wait and the window wait) must be listed with its bearer. The bound was later retired (n=6).

@history_item n=8 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — The `--help` group labels were built over the served verbs only (`contract/cli-help/served-verbs-only`, `contract/cli-help/group-labels`) and pinned by `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs`. The SGR spans sit beside their word (`decision/sgr-beside-its-word`), with none under `--json` or a passthrough role (`decision/no-sgr-in-plain-modes`). Open joins `setup:` (`setup:  open, run, revive, verify, plugin install`), so the help literal and both cases move, and open's output must carry no SGR in plain modes.

@history_item n=9 marker=2026-10-20T06-58-31-wrap
**2026-10-20T06-58-31-wrap** — The founder ratified «served verbs only» for `--help`'s rows (the PROVISIONAL marks were removed). A row for `open` appears only once the verb is served.

@history_item n=10 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — `plugin install` was built into the `setup:` row and a bare `plugin` is unserved, exit 2 (`contract/cli-help/served-verbs-only`, `contract/cli-usage-error/exit-2`). Its planning-form lines were pinned by literal. This is the precedent for adding a verb to `setup:` and for a clap usage error exiting 2, which open's allowlist refusal reuses.

@history_item n=11 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — The last time a help group row was added (`handoff:  link, unlink`), with the help literals updated. The new verbs' result lines were pinned by literal in one write with no SGR. Names printed in result lines are the verb's own parsed arguments (`decision/printed-names-are-parsed-arguments`). Open's result line and `--json` ok document should print `<name>` / `--parent` as parsed, never read back from the log.

@history_item n=12 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — A second verb hosting the wrapped TUI through `run`'s start was added. The passthrough boundary binds it, and a passed start writes no line of its own while the child holds the terminal (`contract/tui-passthrough/own-bytes`). Its exit-1 refusals take the fixed-message form (`contract/cli-fixed-message/form`, `contract/cli-refusal/unable-line`). Open is a third entry into `run`'s start inside a foot window, so the same boundary applies. Open's own refusal lines also need a choice between the `unable  <name>  <reason>` form and the fixed-message form.

@history_item n=13 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — revive gained `--json`: each exit-1 refusal is one `{"v":1,"error":"<code>","detail":null}` document on stdout with empty stderr and no hint, and the human pair stays byte-identical (`contract/revive-json-refusal/document`). This is the shape open's `--json` error document copies.

@history_item n=14 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — Every refusal of the start shared with `run` now gives one exit-1 document (`contract/start-json-refusal/document`). A start against a holder reads `stale-heartbeat` for a stale holder and `already-live` for a live one (`contract/start-holder-refusal/*`). Under `--json` a fault prints the `internal-error` document (`contract/json-fault/internal-error`). One hint per code (`no_two_codes_share_a_hint`) and no hint naming `release` (`no_hint_names_release`) apply. `tests/cross_exit_causes.rs` is the exit-cause table. Open's `already-live` / `stale-heartbeat` check before any spawn, its new codes `terminal-not-found` and `not-live` with their hints, and its `--json` documents all have to join this catalogue and the table.

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — Each `--json` failure is one clean document: a literal exit, empty stderr, no ESC byte, and no path or upstream text (`decision/json-failure-documents-are-clean`, `cli_error_documents_hold_no_path_or_upstream_text`). The exit-2 paths write one fixed stderr line and no document (`decision/json-one-document-no-hint`, `contract/cli-exit-codes/2`, `NOT_ERROR_BODIES`). Every error writer is named in the census, and no thiserror message interpolates a value. A code word outside the list stays visible and exits 14. Open's fixed line for a word after `--` other than `claude` (exit 2, beside `PAIR_AFTER_DASHES`) falls under the exit-2 exception. Its new error writers must enter the census, and its documents must hold no cwd or foot path.

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `viola run` and revive's start refuse a home tree that fails the strict-modes check before any log opens: exit 1, stdout empty, the home-state pair with `hint:` last, and no child (`contract/start-refusal/home-strict-modes`, `decision/strict-modes-refusal-before-any-write`, `decision/unstattable-home-is-strict-modes-refusal`). Open's `strict-modes-failed` exit 1 «before any log» must refuse before foot is spawned, following this pattern.

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — `run` and revive's start refuse a bad socket directory before any snapshot, bind or child, with exit 1. That refusal takes the fixed-message form (`decision/socket-dir-refusal-before-any-frame`, `contract/cli-fixed-message/form`), and `revive --json` gained the `socket-dir-refused` code. The started `run` inside foot can hit this refusal, and it is open's question whether it surfaces as `not-live` or as its own code.

@history_item n=18 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — `decision/linux-only` was set: the core runs on any distribution, CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland. The tui boundary runs on openpty alone, and the a11y cases are in the `coverage` suite on the one `ubuntu-latest` leg. The manual SR pass for the tui is Orca with foot on Hyprland, the founder's one terminal. Open's foot and Hyprland placement are the first 0.2.0 window feature under this row. Its CI cases run on `ubuntu-latest` over the fake and stubs, with no live compositor.

@history_item n=19 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — The 0.2.0 census found no web or graphical frontend (the N/A stubs of §5–§7). The terminal fleet console's a11y scope belongs to the route entry «Fleet a11y verification». Open adds a window but no viola-rendered UI, so §5–§7 stay N/A and console keyboard reach stays out of this chunk.

@history_item n=20 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `~ contract/tui-passthrough/viola-terminal-bytes` and `~ contract/tui-passthrough/human-keystrokes`: zero viola bytes and never-blocked keys hold from the child's start. The keyed start's automatic verify lines L1–L5 come before the child. The `run` that open starts in the new foot window inherits these. Open adds nothing to that terminal (its own output goes to the caller's stdout).

@history_item n=21 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — `~ contract/tui-passthrough/human-keystrokes` and `~ decision/human-keystrokes-never-blocked`: a key waits behind at most the paste's in-flight chunk and closing marker. This is the latest form of the keystroke promise, which a `run` started by open must keep unchanged.

@history_item n=22 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — `~ contract/viola-list-board/order`, `~ decision/terminal-structural-equivalents` and `~ decision/cli-structural-landmark-equivalents`: the board draws wrapped rows as a tree with a plain-space NAME indent, then a `-- LINKS --` block. Open's `--parent` sets the parent-child lineage that the tree shows. The scope keeps the six-column board unchanged and puts `window?` / `origin?` in `list --json` only, so no structural-equivalent row moves.

@history_item n=23 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — The board's caption order is pinned by the `CAPTION` literal, and its fixed strings by literals (`decision/fixed-cli-strings-pinned`). Each state is carried by its word. Open adds `window?` / `origin?` to `list --json` only, so the human board's caption and row literals must stay byte-identical.

@history_item n=24 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — The 5 s `claude agents --json` read (direct spawn, size cap, kill) is listed as a bound that falls on viola's own child and fails open. This is the precedent for a bounded external read with a 5 s kill, as with open's `hyprctl` / `clients` polling under `HYPRCTL_DEADLINE`, recorded as falling on viola, never on a key.

@history_item n=25 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — The cli checks are literal pins over `std::process::Command` (`decision/cli-a11y-by-literal-tests`, `decision/cli-output-discipline-assertion-tools`, `decision/refusal-layout-literal-pins`). The outer-PTY driver is portable-pty `=0.9.0` (`decision/portable-pty-outer-pty-driver`). Every fixed CLI string is pinned by literal. Open's refusal pairs, hints, result line and the exit-2 fixed line need literal pins in this form.

@history_item n=26 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — The a11y cases run as nextest cases in the `coverage` suite, with no stage, budget or second driver of their own (`decision/no-separate-a11y-driver`, `decision/no-a11y-performance-budget`). cli and tui emit no a11y violation rows (`contract/structured-violation-json/cli-tui-a11y-rows`). Open's a11y cases must use the same harness, with no new `suite` value.

@history_item n=27 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — The CLI output-discipline evidence reports under the CI `coverage` suite, gated by `gate --require coverage,doctest` (`decision/cli-invariants-pass-in-ci`). This is where open's output-discipline cases land.

@history_item n=28 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — The takeover clause's second live reading was taken on the product binary over live `claude`, as a supplemental reading that never gates (`decision/live-readings-never-gate`). This is the precedent for R2, the one live `viola open`: it is the operator's live reading and never part of the CI gate.

@history_item n=29 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — The first live takeover reading was a `wtype` key into a live session's window on a compositor the test started, with the reply probe on foot 1.28.0. It was supplemental and never gating. This is a prior live reading in a foot window on the compositor, and the gating posture of R2's window reading must follow it.

@history_item n=30 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's interrupted pair (one `unable:` line, one `hint:` line last, exit 1) is one of the fixed-message exceptions named in `contract/cli-refusal/unable-line`. Any fixed-message form open adopts must be added to that row's list.
