@history_item n=1 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — put `revive` among the CLI verbs and made a passed revive enter the TUI passthrough boundary (contract/tui-passthrough/entry, decision/single-vendor-content-boundary). The revive refusal is a fixed-message `unable:` line with `hint:` last, and while the child holds the terminal a passed revive writes no line of its own. `viola up` re-opens each instance through this revive path, so every start it makes is bound by that boundary, and up's own per-instance lines must never reach a terminal a child already holds.

@history_item n=2 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — revive's preflight makes one `claude agents --json` read with a 5 s kill, and the read fails open (decision/claude-agents-read-fails-open). The bound falls on viola's own child, never on the human. When up runs revive's preflight once per instance, that read repeats N times, which bears on the wall-time premise. The fail-open rule stays: a failed read passes revive.

@history_item n=3 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave revive `--json` on both arms: each exit-1 refusal is one `{"v":1,"error":"<code>","detail":null}` document with empty stderr, and the human pair stays byte-identical. This is the precedent for the `--json` form of up's per-instance outcomes (contract/revive-json-refusal/document). The document carries codes only, with no hint.

@history_item n=4 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the `--help` group labels over the served verbs only, and the colour decision (NO_COLOR, TERM, COLORTERM, TTY; no SGR under `--json`), with each SGR span beside its word. A new human verb `up` needs a row in a help group (contract/cli-help/group-labels, served-verbs-only, usage-line), pinned by `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs`. Any styling of up's outcome words must follow decision/no-sgr-off-terminal and decision/word-beside-every-sgr.

@history_item n=5 marker=2026-10-20T06-58-31-wrap
**2026-10-20T06-58-31-wrap** — the founder ratified that `--help` rows cover the served verbs only, removing the PROVISIONAL mark. Adding `up` to a help group follows this standing rule.

@history_item n=6 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the `handoff:` help row built) — the precedent for a new verb's help row: `handoff:  link, unlink` was placed between `wheel:` and `setup:` and pinned by the two help cases. `up` will change the same pinned table (contract/cli-help/group-labels), and the group labels serve as terminal structural equivalents (decision/terminal-structural-equivalents).

@history_item n=7 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link verbs' result lines among the literal-pinned strings) — each result line is one write with no SGR and is pinned by literal. Names printed in result lines are the verb's own parsed arguments, never names read back from a reply or the log (decision/printed-names-are-parsed-arguments). Up's one line per instance names an instance it read from the board rather than from argv, so this rule's reach needs a decision, and up's fixed lines take a literal pin.

@history_item n=8 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `run`, a revive start and `verify` refuse a home tree that fails the strict-modes check: exit 1 before any log opens, the home-state pair on stderr with `hint:` last, and no child (contract/start-refusal/home-strict-modes, decision/strict-modes-refusal-before-any-write). Up must decide whether it checks the home once before any start or relays each revive start's refusal.

@history_item n=9 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `--json` failures are each one clean document: a literal exit, empty stderr, no ESC byte, and no path or upstream text (decision/json-failure-documents-are-clean). The `answer` and `send` exit-2 paths are the exception (decision/json-one-document-no-hint). Peer replies are screened to code words (decision/peer-replies-screened-to-code-words), and the census of error writers holds each error body to a writer and a witness. Up's `--json` outcome document must hold codes and closed words only and join the census.

@history_item n=10 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — `viola revive --json` gives one exit-1 document for every refusal of the start it shares with `run` (contract/viola-revive-json/start-refusal). A `pump_child` error prints no document because the child has held the terminal. Exit 1 separates a stale holder (`stale-heartbeat`) from a live one (`already-live`) (contract/start-holder-refusal/*). The entry also added `no_two_codes_share_a_hint`, `no_hint_names_release` and the exit-cause table in `tests/cross_exit_causes.rs`. Up's skip of live instances and its relay of revive's closed codes rest on these, and any new code or hint must pass the one-hint-per-code and no-release guards.

@history_item n=11 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's `internal-error` document) — where a verb prints `error: internal error`, under `--json` it prints the internal-error document instead. `run` and revive's start stay silent there (contract/json-fault/internal-error). Up must say which of these forms its own catch site takes, given that each start it makes is silent.

@history_item n=12 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — a refused socket directory at a `run` or revive start gives exit 1 before any snapshot, bind or child, in the fixed-message pair form. Under `revive --json` it gives the one `socket-dir-refused` document, which holds no path, pid or uid (contract/start-refusal/socket-dir, contract/revive-json-refusal/socket-dir-refused, decision/socket-dir-refusal-before-any-frame). Every start up makes can meet this refusal, and up's per-instance line must not leak a path.

@history_item n=13 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — set the closed set of revive `--json` exit-1 codes at ten, with `batch-script-child` gone. It also set the Linux-only platform line (core, ubuntu-latest CI, the 0.2.0 window features on Hyprland; decision/linux-only), made the manual pass Orca alone (GNOME Terminal for cli, foot on Hyprland for tui; nothing gated on it), and moved the a11y cases to the one ubuntu-latest `coverage` leg. Up's outcomes relay that closed code set. Its window placement is Hyprland-only, and its tests run on ubuntu-latest.

@history_item n=14 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — moved the cli output-discipline checks to `std::process::Command` with literal pins, which replaced trycmd and assert_cmd (decision/fixed-cli-strings-pinned, decision/refusal-layout-literal-pins, decision/cli-a11y-by-literal-tests). Up's fixed strings, its outcome lines and its refusal layout are pinned the same way, and a copy change fails CI.

@history_item n=15 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — retired the frontend stubs to N/A: viola has no web or graphical frontend, and the terminal fleet console takes its keyboard a11y scope at «Fleet a11y verification». This explains the N/A rows the picked set holds for §5–§7. `up` stays a cli surface and is not a console entry.

@history_item n=16 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — retired exit 11 (budget-paused), never to be reused (contract/cli-exit-codes/11), and dropped the budget pause from both time-bound lists. Up must not take exit 11. This entry is also part of the record of how the time-bound lists have been edited.

@history_item n=17 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — dropped the statusline deadline from decision/time-bounds-never-on-human and decision/time-bounds-fall-on-the-driver. These are the two time-bound lists that the scope says will not grow for up.

@history_item n=18 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — the start's trust step reports its outcome in one `trust-record` line in `diagnostics/`, never as a refusal, exit code or hint (decision/run-diagnostics-off-terminal). A wrapped TUI in an untrusted folder opens on its session box (decision/wrapped-tui-reads-as-unwrapped except). The start's Claude config lock wait joined both time-bound lists as a bound on viola's own start. Each revive start that up makes runs this trust step and lock wait, which adds to wall time across N starts and does not fall on the human.

@history_item n=19 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — a keyed start's automatic verify window comes before the child, and its lines L1–L5 and verify's own output come before the child's bytes. This window is the one exception to never-blocked keys (contract/tui-passthrough/human-keystrokes, contract/tui-passthrough/viola-terminal-bytes, decision/human-keystrokes-never-blocked). Ratified at 2026-11-22T17-29-41Z. Each start that up makes may enter this window, so up's own lines must not interleave with it.

@history_item n=20 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — added the MCP call deadline to both time-bound lists and `call-deadline` as an unreachable cause. It is the latest growth of the two lists before they reached full. Up is CLI-only with no MCP tool, per the scope's conservative arm, so no MCP bound applies.

@history_item n=21 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — changed the delay bound on a human key to the paste's in-flight chunk and closing marker (contract/tui-passthrough/human-keystrokes, decision/human-keystrokes-never-blocked). Each passthrough that up opens through revive inherits this promise as it now reads.

@history_item n=22 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — gave contract/tui-passthrough/own-bytes, decision/zero-viola-bytes-on-the-terminal, decision/wheel-ignores-terminal-events and decision/portable-pty-outer-pty-driver the automatic-verify exception. It also added paste write to both time-bound lists, which left them full at their bound. Up's starts sit inside these boundary rows, and the scope's full time-bound list cell comes from this entry.

@history_item n=23 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** — because the two time-bound lists are full, the operator ruled that a new time bound lands as a row of its own: decision/start-verify-stop-grace-off-the-human. This is the precedent for any bound up adds, such as a fleet-wide or per-start wait. Under the scope, that row waits on fork F1 of run `2026-11-24T00-26-24Z-wrap` and is listed in Expected amendments.

@history_item n=24 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — the board draws wrapped rows as a tree with a plain-space NAME indent and a `-- LINKS --` block (decision/terminal-structural-equivalents, decision/cli-structural-landmark-equivalents). Up orders parents before children by this lineage record. If up's per-instance lines show depth, they must use the same plain-space indent and no other structure carrier.

@history_item n=25 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — `viola open` added an exit-2 refusal that is one fixed stderr line with no document and echoes none of the refused words (contract/cli-exit-codes/2). It is now among the `--json` exceptions (decision/json-one-document-no-hint, contract/cli-refusal/json). Up opens each window the way `open` would, so any usage path it adds must either fit exit 2 as a clap usage error or extend this exception list by name.

@history_item n=26 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — the precedent for a non-frame verb: `focus` joined the exit-21 strict-modes list and contract/cli-exit-codes/21 for refusals before any compositor read. Both remain PROVISIONAL (delegate, fork F-A). If up refuses an instance or home before any Hyprland read, these rows would have to name up the same way. Up's rows rest on a provisional base.
