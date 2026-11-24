@history_item n=1 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — `revive` joined the CLI verbs. Its exit-1 refusals use the fixed `unable:` / `hint:` pair, a clap usage error exits 2, and a passed revive enters the TUI passthrough boundary: it writes no line of its own while the child holds the terminal (`tests/chaos_revive.rs`). `adopt` is built on revive's start, so this is the template for its CLI entity and its passthrough entry. Rows: contract/tui-passthrough/entry, contract/revive-human-refusal/pair, contract/tui-passthrough/own-bytes.

@history_item n=2 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — Built the `-- UNWRAPPED - READ-ONLY --` rack from one `claude agents --json` read. That read is a direct spawn with `take(MAX_FRAME + 1)` and a 5 s kill, and it fails open for `list` and for revive's preflight. The bound falls on viola's own child. `adopt`'s live-elsewhere guard reuses this read: decision/claude-agents-read-fails-open is the trap, because a fail-open read would let a live session through. An adopted session also moves from the unwrapped rack to a wrapped row (contract/viola-list-board/unwrapped-separator).

@history_item n=3 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `revive` takes `--json` on both arms. Each exit-1 refusal is one `{"v":1,"error":"<code>","detail":null}` document with empty stderr, and the human pair stays byte-identical. This is the precedent for the `--json` form of `adopt`'s refusals (contract/revive-json-refusal/document).

@history_item n=4 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — Built the `--help` group labels over the served verbs only (`help_is_the_grouped_verb_table`, `help_lists_only_served_verbs`) and the colour decision (`NO_COLOR` / `TERM` / `COLORTERM`, none under `--json` or a passthrough role). A new `adopt` row must land in a group and keep these help pins green. Rows: contract/cli-help/group-labels, contract/cli-help/served-verbs-only, decision/colour-decision-inputs.

@history_item n=5 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — `plugin install` was added to the `setup:` row, a bare `plugin` was made an exit-2 usage error, and its lines were pinned by literal. This is a precedent for adding a served verb to the help table and the verb entity (contract/cli-usage-error/exit-2, contract/cli-help/served-verbs-only).

@history_item n=6 marker=2026-10-20T06-58-31-wrap
**2026-10-20T06-58-31-wrap** — The founder ratified help's served-verbs-only rows and the interrupted-verify pair's wording. A new verb's help row therefore follows a standing rule, not a provisional one (contract/cli-help/served-verbs-only, contract/viola-verify/interrupted-pair).

@history_item n=7 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — Added the `handoff:` help row (`link, unlink`) between `wheel:` and `setup:`. The link verbs' result lines were pinned by literal, and the rule was set that printed names are the verb's own parsed arguments, never read back from a reply or the log. This is the precedent for `adopt`'s help group placement and for printing its `--name` / `--parent` (decision/printed-names-are-parsed-arguments, contract/cli-help/group-labels).

@history_item n=8 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — Defined verify's interrupted pair (`unable: verify was stopped …` / `hint: run viola verify again …`, exit 1). A keyed start that `adopt` shares through revive runs automatic verify, so this pair can reach adopt's terminal (contract/viola-verify/interrupted-pair).

@history_item n=9 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `run`, a revive start and `verify` refuse a home tree that fails the strict-modes check before any log opens. They exit 1 with the home-state or instance-state pair (`hint:` last), start no child, and revive under `--json` gives `strict-modes-failed`. The pairs are pinned by the `HOME_PAIR` / `INSTANCE_PAIR` literals (`tests/run_cli.rs`, `tests/security_negatives_strict_modes.rs`). The entry also covers the unstattable home. `adopt`'s start inherits this ordering and these pins (contract/start-refusal/home-strict-modes, decision/strict-modes-refusal-before-any-write, decision/unstattable-home-is-strict-modes-refusal).

@history_item n=10 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — Established several rules:
- Each `--json` failure is one clean document with no ESC byte, held by `cli_error_documents_hold_no_path_or_upstream_text`.
- The exit-2 exception: a fixed stderr line and no document.
- `unknown`'s detail is a code word or nothing.
- A peer's refusal is screened to a code word, and unlisted words stay visible at exit 14.
- `wrapper fault` is the fault's body.
- There is a census of error writers and witnesses (`tests/contract_error_bodies.rs`).

Any new `adopt` error writer or exit-2 path must be named in that census and the `NOT_ERROR_BODIES` list. Rows: contract/cli-json-failure/document, decision/json-failure-documents-are-clean, decision/error-bodies-have-writer-and-witness, decision/thiserror-messages-interpolate-nothing, contract/code-word/format, decision/unlisted-code-words-stay-visible, contract/human-fault-line/wrapper-fault.

@history_item n=11 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — Changed several refusal and fault forms:
- `viola revive --json` prints one exit-1 document for every refusal of its strict-modes check, its preflight and the start it shares with `run`.
- A `pump_child` error prints no document.
- A stale holder reads `stale-heartbeat` and a live holder reads `already-live`.
- A fault under `--json` prints the `internal-error` document, and `run` and revive's start stay silent.
- New hints are pinned by `send_hint_is_the_design_string`.
- `tests/cross_exit_causes.rs` is the exit-cause table, with `no_two_codes_share_a_hint` and `no_hint_names_release`.

`adopt`'s refusals (malformed id, live elsewhere, name held) must join this table with one hint per code. Rows: contract/viola-revive-json/start-refusal, contract/viola-revive-json/pump-child-error, contract/start-holder-refusal/*, contract/json-fault/internal-error, decision/one-hint-per-code, decision/no-hint-names-release.

@history_item n=12 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — Added `socket-dir-refused` as revive's eleventh exit-1 code, checked at the shared start before any snapshot, bind or child. `run`'s socket-directory refusal takes the fixed-message form, and a peer the wrapper refuses is dropped before any frame, with no terminal bytes. These hold for any start `adopt` makes. Rows: contract/start-refusal/socket-dir, contract/revive-json-refusal/socket-dir-refused, decision/socket-dir-refusal-before-any-frame, contract/tui-passthrough/refused-peer, contract/cli-fixed-message/form.

@history_item n=13 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's detail) — Exit 21's `--json` document names its cause in `detail` (`strict-modes-failed`, `instance-dead`, `unwrapped-name`, …). If `adopt` refuses any instance-level cause with exit 21 rather than exit 1, this closed set binds it (contract/instance-unreachable-detail/*).

@history_item n=14 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — Made these changes:
- revive's `--json` codes read ten, with `batch-script-child` gone.
- The TUI boundary is hosted in openpty alone.
- Each boundary clause is its own nextest case on `ubuntu-latest`.
- The a11y cases run in the `coverage` suite on the one `ubuntu-latest` leg.
- The Windows strict-modes and NotFound arms are retired.

`adopt`'s cases land on that one leg, where the folded CI item's 20 s kill also applies. Rows: contract/revive-json-refusal/*, decision/boundary-tests-sequenced-on-records, decision/cli-output-discipline-assertion-tools.

@history_item n=15 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — The cli / tui a11y cases became nextest cases in the `coverage` suite, gated by `gate --require coverage,doctest`, and `#[ignore]` was banned under zero flakiness. This bears on the folded CI item: the fix is a margin by the case's class, never a skip or retry (decision/zero-flakiness).

@history_item n=16 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — Moved the cli checks to `std::process::Command` with literal pins (trycmd and assert_cmd retired). Every fixed CLI string, hint and result line is pinned by literal, and the outer-PTY driver is portable-pty `=0.9.0`, inherited and never re-pinned. New `adopt` strings and hints take literal pins. Rows: decision/fixed-cli-strings-pinned, decision/refusal-layout-literal-pins, decision/cli-a11y-by-literal-tests, decision/tui-boundary-driver-pin.

@history_item n=17 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — Retired the frontend stubs: N/A, with no web or graphical frontend, and the fleet console's a11y is owned by «Fleet a11y verification». This explains the picked absences and the N/A areas. `adopt` adds no page, and the fleet console is out of scope.

@history_item n=18 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — The master became rows, each verified against the code. Every picked row's current wording dates from this entry or a later edit of it.

@history_item n=19 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — Made three changes:
- The start's trust step writes the CLI's trust record before the spawn, so a wrapped `claude` reads as unwrapped except that it opens on the session box.
- The step reports one `trust-record` diagnostics line, never a refusal.
- The start's Claude config lock wait joined the time bounds that fall on viola's own start.

An adopted session's cwd goes through this same start. Rows: decision/wrapped-tui-reads-as-unwrapped, decision/run-diagnostics-off-terminal, decision/time-bounds-never-on-human, decision/time-bounds-fall-on-the-driver.

@history_item n=20 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — A start's automatic verify window, before the child exists, is the one exception to the human-keystroke and zero-viola-bytes promises (verify lines L1–L5). This applies to any start `adopt` makes. Rows: contract/tui-passthrough/human-keystrokes, contract/tui-passthrough/viola-terminal-bytes, decision/human-keystrokes-never-blocked.

@history_item n=21 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — A key's delay bound became "the in-flight chunk and closing marker" (the paste is chunked on one writer thread), not "the current atomic paste". This is the current wording `adopt`'s passthrough inherits (contract/tui-passthrough/human-keystrokes, decision/human-keystrokes-never-blocked).

@history_item n=22 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — Made these changes:
- The founder ratified the automatic-verify exceptions.
- The own-bytes promise names a keyed start's automatic-verify lines.
- zero-viola-bytes, wheel-ignores-terminal-events and portable-pty-outer-pty-driver gained the before-the-child exception.
- Paste write joined both time-bound lists.

These rows bind `adopt`'s start as they stand. Rows: contract/tui-passthrough/own-bytes, decision/zero-viola-bytes-on-the-terminal, decision/wheel-ignores-terminal-events, decision/portable-pty-outer-pty-driver, decision/time-bounds-*.

@history_item n=23 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** — Added decision/start-verify-stop-grace-off-the-human as a row of its own, because the two time-bound lists were full at their bound. This set the precedent that a new time bound lands as its own row (the trap if `adopt` adds a wait).

@history_item n=24 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — The board draws wrapped rows as a tree, with a plain-space NAME indent for depth, plus a `-- LINKS --` block. The structural-equivalents rows were extended to match. `adopt`'s `--parent` lands in this lineage, and the adopted row shows on that tree. Rows: decision/terminal-structural-equivalents, decision/cli-structural-landmark-equivalents.

@history_item n=25 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — Added `viola open`'s exit-2 path: a fixed stderr line, no document and no echo of refused words. The exit-2 cause list and the `--json` one-document exception list were each extended to name it. This is the precedent if `adopt` gains an exit-2 path. Rows: contract/cli-exit-codes/2, decision/json-one-document-no-hint, contract/cli-refusal/json.

@history_item n=26 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — Added `focus` to the exit-21 strict-modes verbs and to the exit-21 promise (PROVISIONAL, delegate). This is the precedent that each new verb's strict-modes exit is named in decision/strict-modes-refusal-before-any-write and contract/cli-exit-codes/21.

@history_item n=27 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — Made these changes:
- Added the contract/viola-up-output/instance-line row for a new verb's output surface.
- Named `up` in the exit-21 promise.
- Reworded the time-bound lists to «Eight time bounds» (PROVISIONAL), with three new bounds as rows of their own (decision/hyprctl-read-deadline, decision/window-wait-bound, open-live-wait-bound).

If `adopt` opens a window (the P3 fork), these rows bind it, and any new bound takes its own row rather than a ninth list member. A new verb's output surface also gets a row.

@history_item n=28 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — The passthrough boundary still holds after a contained vt100 feed panic: no viola literal reaches the outer terminal (`tests/chaos_feed_panic.rs`). This is part of the boundary that `adopt`'s start enters (contract/tui-passthrough/vt100-feed-panic, contract/tui-passthrough/after-feed-panic).
