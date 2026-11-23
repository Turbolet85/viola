@history_item n=1 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Input Validation: 2 rows) — added decision/run-lineage-flags-checked (--parent through the viola name check, --role a code word, a self-parent a usage error exit 2 before anything is read) and decision/board-lineage-and-links-name-checked (the board shows a recorded value only when it re-parses). This is the direct precedent for new `viola run` flags carrying a recorded per-instance pair: each flag is checked to a closed shape before anything is read, and the board re-checks a value read back before showing it. Both rows are still PROVISIONAL (delegate overseer, F8).

@history_item n=2 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Data Protection: 1 row) — added decision/lineage-kept-to-snapshot-and-event-log: the lineage pair stands only in the snapshot and one lineage line of the event log at start, with no process-log line, span or terminal byte. The scope records the model/effort pair "beside the lineage record", so this is the placement rule the new pair will extend or copy.

@history_item n=3 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Authentication & Authorization: 1 row) — added decision/lineage-parent-authenticates-nothing: a self-reported value inherited at start grants no authority. A recorded model/effort pair is in the same position and must decide nothing security-relevant.

@history_item n=4 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — recorded that the identity floor of stripped CLAUDE* names includes `CLAUDE_EFFORT` (absent from the hook env on 2.1.287). This is a trap if the effort channel turns out to be an environment name: the R8 strip always removes floor names whatever claude_env_keep says (decision/r8-claude-env-strip, decision/config-cannot-disable-controls), so effort cannot reach the child through an inherited CLAUDE_EFFORT. Viola writing that name itself would be a new crossing.

@history_item n=5 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** (Secret Management: 2 rows) — the identity floor grew from eleven to thirteen names; decision/identity-floor-eleven-names was retired into decision/non-identity-hook-names. This is the current floor and strip definition that any env-based model/effort channel has to respect.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the R8 strip and the fixture scrub match names exactly) — every inherited CLAUDE* name is removed, matched exactly, unless claude_env_keep keeps it, and the floor is always stripped; claude_env_keep is the R8 pass-list. This is the child-environment rule the start's spawn must keep while adding model/effort.

@history_item n=7 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — added the config.json claude_env_keep row (≤32 names matching `^CLAUDE[A-Z0-9_]*$`, rejected whole when bad, a floor name ineffective). This is the precedent for a config key that shapes the child's spawn and was ratified as a boundary widening, so a default model/effort pair key in config.json belongs to that class.

@history_item n=8 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — the config.json row gained the closed diagnostics_level (info | debug, any other value falls back to info with parse-rejected) under the MAX_FRAME cap. This is the template for a closed-value config key, which is what a default model/effort pair would be (decision/config-tolerant-parse, decision/config-bad-value-handling).

@history_item n=9 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `dialog_hold_s` was validated into `DialogHold` and read once at the wrapper's start through its own capped read. A bad value takes the default with one parse-rejected line, and no env var or flag sets it. Adding the key was ruled a boundary widening that admits a new input (founder's answer). This is the nearest precedent for adding a config.json default-pair key.

@history_item n=10 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Input Validation) — retired the five_hour and seven_day config.json keys and rewrote decision/config-cannot-disable-controls. This is the current config key set and the rule that no configuration value switches off a control; the default pair key joins that set.

@history_item n=11 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (Input Validation: 3 rows) — decision/config-cannot-disable-controls now also says no configuration value (the verify_new_cli key) lifts the unverified CLI gate. A new default-pair key must not touch the gate either. This is the row's latest wording.

@history_item n=12 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive spawns in the recorded cwd (ratified) and puts the logged session id on the child's argv as one element of a direct spawn, built only by `resume_args`. It also recorded that a logged value crossing onto a child's command line is a boundary widening that needs the founder's own word: it was first landed as not ratified. Replaying a recorded model/effort pair onto a resumed child's argv is the same class of crossing (decision/session-id-argv-closed-shape, decision/revive-trailing-words-as-argv).

@history_item n=13 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (revive: the session id ratified, its arguments) — fixed the revived child's argument order (`--plugin-dir`, `--settings` when written, `--resume <id>`, `--fork-session`, the words after `--`) and ratified the session id crossing. A model/effort flag joins this order. The entry also landed a per-instance settings.json override passed by `--settings`, which was ratified as a widening.

@history_item n=14 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Data Protection: 5 rows) — retired the per-instance settings.json override and the child's `--settings`; the plugin files are now the only code-bearing files rewritten at start. If the CLI channel for model/effort turns out to be a settings key, a new settings write or `--settings` brings back a retired surface. The scope also promises nothing is written to the user's settings.

@history_item n=15 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — verify's Run E spawns with `--resume <id>` via `resume_args` as a widening of the ratified verify child set, and the ledger row count moved the stamp. This is the precedent for measuring a resume-time behaviour with a verify probe. A probe for the model/effort CLI channel widens the verify child set (decision/verify-children-direct-spawn) and needs the founder's own word.

@history_item n=16 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — adding a relied ledger row moved the stamp count, and a stamp holding only the older ids then reads cli_verified:false. This is a trap for this chunk: a new capability ledger row for the model/effort channel makes every existing stamp unverified until re-verified (decision/unverified-cli-gate, contract/cli-exit-codes/12).

@history_item n=17 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Authentication & Authorization: 1 row) — decision/unverified-cli-gate now reads "every relied ledger row counts: pass, or absent on the browser-tools-prompt row alone". This is the current gate a new ledger row would join.

@history_item n=18 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (the StopFailure exception settled) — decision/unverified-cli-gate's only exception is StopFailure registered with no ledger row and no verify probe, kept on the founder's word. A model/effort channel relied on with no ledger row would be a second such exception and would need its own word. The Critical Warning in the scope calls for a ledger row with a verify probe.

@history_item n=19 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Authentication & Authorization: 1 row) — first added the StopFailure exception to decision/unverified-cli-gate. It was PROVISIONAL on the delegate's answer and later ratified by the founder. This is the precedent for how a no-ledger-row exception is recorded and ratified.

@history_item n=20 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — the event log reader counts unknown kinds and unknown fields per read without logging them: an unknown kind is not returned, and a known-kind line with an extra key is returned and counted. A new pair field on a snapshot or event line, or a new line kind, is read under this rule by older readers (decision/events-reader-bounded-lines, decision/state-readers-never-write).

@history_item n=21 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the snapshot readers move onto `read_snapshot_or_replay`) — every product snapshot read goes through the classified read or the replay, after its entry point's strict-modes check. A replayed snapshot carries no endpoint, pid or started_at. A pair replayed on resume has to be read the same way. Decide whether a replayed snapshot keeps the pair or drops it, like the inert fields (decision/replayed-snapshot-inert, decision/classified-snapshot-read-only).

@history_item n=22 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home tree checked at `run` and `revive`'s start, before the wrapper log) — run and revive's start run `check_home_tree` before the wrapper log opens, and a refusal exits 1 with no log, line or child. Any new read of config or recorded state for the pair must sit after this check (decision/start-checks-home-tree-before-log-opens, decision/strict-check-before-narrowing).

@history_item n=23 marker=2026-10-10-session-links
**2026-10-10-session-links** — the replay's link rule takes a line's recorded names as strings only when both are strings, without re-parsing them, and a malformed line changes nothing. This is the precedent for replaying a recorded per-instance value from the event log on a start, and for deciding whether to re-validate it.

@history_item n=24 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — list is a strict-modes entry point that refuses the whole board, and every row field comes from viola's own state and is escaped through `escape_row` (control characters as `\xHH`). If the pair is shown on `viola list` or `--json`, it falls under this scan and escaping (decision/whole-board-refusal, decision/human-output-escapes-controls).

@history_item n=25 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the mcp process spawns claude agents --json) — MCP `list` reads the board through CLI list's `board_document`, so a pair shown on the board reaches the MCP `list` tool too (contract/mcp-tools/list).

@history_item n=26 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (revive --json documents every refusal of its start) — `revive --json` prints a closed-code document for every refusal of the start it shares with `run`, and its exit-1 code set is closed (contract/revive-json/*). A new refusal of a bad or unreplayable pair would need a code in that set, or would have to fold into an existing one.

@history_item n=27 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (child program resolution) — `run` resolves a given path or a bare name on PATH, and `ProgramNotFound` is the one refusal. revive's closed codes went from 11 to 10 and verify's from 6 to 5 when batch-script-child left. These are the current closed code counts any new code would extend (decision/run-resolves-absolute-program).

@history_item n=28 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — run and revive's start now write the CLI's trust flag into the Claude config and read CLAUDE_CONFIG_DIR's value when it is kept. The rows decision/no-writes-under-claude-home, decision/viola-writes-nothing-in-claude-dir and decision/nothing-written-under-claude-home now carry these bounded exceptions. The pair must not become another write into the user's Claude config or settings: the scope and v2-31 require the user's settings to stay byte-identical.

@history_item n=29 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — NO_COLOR, TERM and COLORTERM were admitted as the only non-seam environment reads, with a ban on any further environment read without its own Decisions Log entry. If the operator's pair or default is taken from an environment name, that is a new env read needing its own entry (decision/new-env-read-needs-log-entry, decision/env-not-config-channel).

@history_item n=30 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader census) — `product_readers_are_each_bounded_or_named` requires every product reader call to be bounded in its statement or named. Any new read for the pair (config, snapshot, a CLI version probe) must pass this census (decision/bounded-readers-census).

@history_item n=31 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' census) — `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing` require every new error body to be named, with a fixed message. A new refusal of a bad pair must be registered there and must not interpolate the value (decision/every-error-writer-named, decision/error-messages-interpolate-nothing).
