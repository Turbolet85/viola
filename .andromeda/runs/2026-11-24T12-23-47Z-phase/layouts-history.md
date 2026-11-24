@history_item n=1 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — `~ contract/viola-revive/usage` gave `viola revive` `--model <MODEL>` / `--effort <EFFORT>`, both conflicting with `--list`; `+ contract/run-revive-usage/pair-after-dashes` made a launch pair after the dashes of `run` / `revive` a usage refusal, exit 2, one fixed stderr line echoing none of the caller's words. This is the pair surface a restart-based switch would ride and the closed-word precedent the new verb's arguments extend; the usage widening stands PROVISIONAL (delegate overseer, founder-forks W2).

@history_item n=2 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — `+ contract/cli-verbs/open` (a closed flag allowlist holding `--model`, `--effort`, `--json`), `+ contract/viola-revive/no-window`, and the `setup:` help row gaining `open` (`~ contract/cli-verbs/help`, `~ contract/viola-help/verb-groups`). A precedent: a new verb taking the model/effort pair as closed words, and the help rows a new verb has to update.

@history_item n=3 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (1 of 2) — `~ contract/cli-verbs/help` (`setup:` gains `up`), `~ contract/viola-revive/no-window` (hidden `--origin`), `~ contract/viola-revive/usage` (`--list` conflicts with `--origin`). `up` replays the recorded pair through revive, and this chunk's switch must make a later `up` / `revive` replay the NEW pair. The revive rows were touched here last, PROVISIONAL on the delegate.

@history_item n=4 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (2 of 2) — `~ contract/cli-exit-codes/21` (the board refusal of `up` joins the fixed-state-pair exceptions), `~ decision/board-vs-strip` (`up` acts on the fleet), `+ contract/cli-verbs/up` (CLI-only, no MCP tool, no channel method). Precedent for the exit-21 row a new strict-modes-checking verb extends, and for a CLI-only verb with no MCP tool (the scope's conservative arm). Both are PROVISIONAL on the delegate.

@history_item n=5 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (verb-groups) — `~ contract/viola-help/verb-groups` was rewritten to the HELP literal in src/cmd/mod.rs (`board:    list, focus`, `setup:` with `up`). The trap: the help rows go stale whenever a verb joins a group.

@history_item n=6 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (3 rows) — `~ contract/cli-verbs/help` (`setup:` holding `adopt`), `+ contract/cli-verbs/adopt` (the verb list holds one row per verb), `+ contract/viola-help/setup-row` (the printed `setup:` literal split into its own row because verb-groups passed its bound). A new verb needs its own `cli-verbs/*` row, and the setup-row literal (or the help row of whichever group the verb joins) must change.

@history_item n=7 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (2 rows) — `~ contract/viola-help/verb-groups` now names the setup row without its verbs. `− contract/cli-catch-site/internal-error` was retired: its promise stands under contract/cli-internal-error/human and /json, and adopt's start is silent at the catch site like revive's start. Settle where the new verb sits relative to the catch site, and which internal-error rows cover it.

@history_item n=8 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — `~ contract/cli-verbs/help` (`board:` holding list and focus), `~ contract/cli-exit-codes/21` (focus's strict-modes refusal prints the fixed state pair), `+ contract/cli-verbs/focus`. This is the latest precedent for a one-strip verb that joins the exit-21 row and a help group, all PROVISIONAL on the delegate (forks F-A, F-B).

@history_item n=9 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (1 of 2) — `~ decision/mcp-hides-wheel-and-handoff-verbs`: the MCP list tool's result carries held links as data, PROVISIONAL (delegate overseer, founder-forks W1). Bears on the scope's MCP-tool-out boundary: any MCP reach of the switch must respect this rule.

@history_item n=10 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ contract/cli-verbs/run`, `~ contract/viola-run/start-output`, `~ contract/cli-verbs/revive`: the run/revive start writes the CLI's trust record, keyed by the recorded directory. A restart-based switch would pass through this start step. The precedent: a start step adds no refusal, exit code, hint, `--json` code or terminal byte, and writes into the CLI's own records only through measured channels.

@history_item n=11 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `~ contract/viola-run/start-output`: before the child, a key-on start's automatic verify line and verify's output reach the terminal (discarded under `revive --json`). This is ratified 2026-11-22 (amd-qfze2g56). If the switch restarts through revive, this start output is what a restart prints.

@history_item n=12 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1 of 2) — `~ contract/cli-exit-codes/11` (retired, never reused) and `~ contract/viola-send-output/unable` (exit 13 or 10/12/14, never 11). A trap: the new verb's exit codes must not reuse 11.

@history_item n=13 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's `detail`) — the exit-21 `--json` document carries its cause in `detail` (`strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`), and `error` / `v` stay fixed. This is the source of the contract/instance-unreachable-detail/* and contract/cli-exit-codes/json-form rows the new verb's machine view must use.

@history_item n=14 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`internal-error` under `--json`) — at the catch site, `--json` before `--` prints `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty. The human line stays, with no hint, and the revive start stays outside the catch site. This rules the new verb's exit 1.

@history_item n=15 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (stale holder / post-preflight gap) — revive `--json` gained `stale-heartbeat` beside `already-live`. Post-preflight start refusals gained exit-1 documents (`pinned-hash-mismatch`, `squatted-name`, `internal-error`), and a `pump_child` error prints no document because the child held the terminal. These are the closed revive codes a restart-based switch inherits, plus the no-document trap after the child starts.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (unwrapped-name / endpoint-lost hints) — gave each exit-21 cause its own hint (contract/frame-verb-hints/not-wrapped, /wrapper-went-away), so every cause has a hint. If the new verb is a frame verb, it inherits the full hint table.

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (socket-dir) — `socket-dir-refused` became exit 21's sixth cause at the frame verbs, with its own hint and `--json` detail, and the run/revive start's exit-1 socket-directory pair was added, `socket-dir-refused` among revive's `--json` codes. These are the sources of contract/frame-verb-hints/socket-dir-refused, contract/instance-unreachable-detail/socket-dir-refused and contract/run-revive-start/socket-dir-refused.

@history_item n=18 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — `.cmd`/`.bat` left revive's codes (closed refusal codes 11 to 10), and the server-check hint is no longer scoped by OS. Help per-verb usage names the program by its executable (contract/viola-verb-help/usage). This is the current closed revive code set and the hint wording a frame verb inherits.

@history_item n=19 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — the run/revive starts check the home tree before the log opens (home pair / state-files pair, exit 1, no log, no child). The frame verbs' exit-21 strict-modes refusal runs before any role file or log opens. The trap from 2026-10-26: opening a role file narrows the home to 0700, so a check placed after `client::start` passes a widened home. A new verb must check first.

@history_item n=20 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — `link` / `unlink` joined the exit-21 `unable` lines, the server check (frame verbs six to eight), the catch site, and the strict-modes refusal before any log, and the `handoff:` help row was built. Precedent for the full set of tables a new frame verb must join (contract/frame-verbs/strict-modes-refused, contract/frame-verb-hints/*).

@history_item n=21 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — a frame verb whose server check fails writes no frame, exits 21 and takes the server-verify hint. This is the source of contract/frame-verb-hints/server-verify-failed, which a switch sent over the channel must honour.

@history_item n=22 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — `pause` / `release` joined the catch-site verbs (`error: internal error` once, no hint). This is the wheel verbs' origin (contract/wheel-verb-output/*). Whose wheel a switch needs is an open premise of the scope.

@history_item n=23 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — `answer` joined the catch-site verbs. A precedent for a new frame verb joining the catch site, and the source of decision/argument-order-mirrors-channel's `answer` form.

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the `unknown` refusal's detail is a code word or nothing, and `unknown`, `wrapper fault` and `internal error` take no hint. This is the exception clause of decision/hint-for-every-cause that the new verb's hint table follows.

@history_item n=25 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — created revive's output structure: a passed start prints nothing, refusals are exit 1 with an `unable:` / `hint:` pair last on stderr (contract/viola-revive/refusal-shape, /start-output), and revive was added to `setup:`. This is the base contract a restart-based switch rides.

@history_item n=26 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — revive's `session-live` pair and its fixed preflight order: holder, logged session, `session-live` (one `claude agents --json` read, skipped under `--fork`, a failed read passes), then the recorded directory. This is the source of contract/viola-revive/preflight-order and /session-live-check, which a restart-based switch would run through.

@history_item n=27 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — revive takes `--json` (exit-1 document `{"v":1,"error":"<code>","detail":null}`, stderr empty, human pairs byte-identical), and `--json` became the machine view of every data verb. This is the source of decision/json-on-every-data-verb and contract/viola-revive-json/refusal-document.

@history_item n=28 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is hidden, and its tools are the CLI verbs' `--json` documents less `v`, with no tool naming release, pause, link or unlink. This matters for the scope's out-unless-a-fork MCP tool for the switch (decision/mcp-server-without-human-surface, decision/mcp-hides-wheel-and-handoff-verbs).

@history_item n=29 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola --help` was built as the grouped table over served verbs only, pinned by the `HELP` literal, with `viola <verb> --help` staying clap's per-verb usage. This is the origin of contract/viola-help/usage-line, /verb-groups and contract/viola-verb-help/usage, which a new verb must keep true.

@history_item n=30 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — `plugin install` joined `setup:` and the catch-site and exit-21 exception lists. A precedent for adding a served verb and the help test `help_lists_only_served_verbs`.

@history_item n=31 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified help's served-verbs-only rule. A switch verb appears in `--help` only once it is served.

@history_item n=32 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — flat verbs: no nested noun hierarchy, no interactive prompt and no pager in a verb, and the fleet console is no verb's output (decision/no-prompt-or-pager, decision/flat-verbs). The switch's keyboard reach belongs to «Console actions», not to this verb.

@history_item n=33 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (decision/contract/absence), each verified against the code. This is the origin of every picked row's current wording and of the row-handle form this chunk's amendments will take.
