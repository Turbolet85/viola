@history_item n=1 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — made `viola mcp` a hidden verb (no argument but `--home`, run only by the plugin via `.mcp.json`) whose five tools are a driver's machine view of the CLI verbs: success/refusal = the verb's `--json` document less `v`, refusal `isError: true` with `refused: <code>`, codes-only bodies, `invalid-params` with a closed field word, no tool naming release/pause/link/unlink. This is the surface the chunk bounds (rows contract/mcp-tools/*, contract/mcp-tool-results/*, decision/mcp-*): any new cap on a caller's argument (e.g. `wait`'s timeout) or a timeout outcome must fit these result shapes and codes, and add no human surface or help row.

@history_item n=2 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — fixed the `viola wait` / `viola last` lines as landed: every non-dialog kind takes the `turn-ended` form, `dialog unknown`, the exit-21 pair, `waiting:` only on a terminal stderr without `--json`, one `--json` document. `wait` is the MCP tool whose caller-supplied block the chunk must bound; its result forms (turn-ended, timed-out, dialog) are the rows a capped or split wait must keep (contract/viola-wait-output/*, decision/turn-ended-not-send-outcome).

@history_item n=3 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — added the `dialog-expired  <name>  dialog <id>  cursor <n>` wait line, woken by the wrapper's record of an expired hold; "every kind without a dialog_id takes the turn-ended form". A wait bounded below the client's timeout must still deliver this form unchanged (contract/viola-wait-output/dialog-expired).

@history_item n=4 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — `[RB] read back` gained a second trigger: `/clear` confirmed by its new-session post-condition, not only a matching prompt-submitted. Start of the post-condition waits inside `send`'s confirmation path — part of `send`'s longest block the chunk must measure.

@history_item n=5 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` read back by a `session-start` of cause compact with the session's own id inside its own 40 s PROVISIONAL window, else `not-delivered` / `no-prompt-submitted`; verify grew to 24 rows. The 40 s window is likely `send`'s longest single block — the trap for the chunk: it must sit below the MCP client's tool-call timeout or be split/capped (contract/viola-send-output/read-back-compact).

@history_item n=6 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `/mcp` read back by a new stdout tag in the transcript; `/cost`, `/usage` unconfirmable; `/mcp` unconfirmable when no readable transcript. Another confirmation wait on `send`'s path whose duration the chunk's bound must cover (contract/viola-send-output/read-back-mcp, unconfirmable).

@history_item n=7 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `/context` read back by a new stdout tag after the send, unconfirmable on an unverified CLI or before its transcript exists. A further confirmation wait inside `send`'s block (contract/viola-send-output/read-back-context).

@history_item n=8 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — widened the read-back match to tolerate a trailing run of space/TAB/CR/LF (later ratified, 2026-11-16T06-05-33-wrap). Constrains how `send`'s confirmation decides; a bound must not alter what confirms (contract/viola-send-output/read-back).

@history_item n=9 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (both entries) the founder ratified the trailing-only tolerance on the read-back row; a leading trim waits on the live rig. Current truth of contract/viola-send-output/read-back that `send`'s bounded path keeps.

@history_item n=10 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `viola send` refuses `control-character` and `empty-text` before any frame, exit 13, one document under `--json`; the wrapper repeats both refusals. These are the client-side checks at the head of `send`'s path the scope names (contract/send-refusals/*, decision/wrapper-repeats-send-refusals).

@history_item n=11 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1 of 2) retired exit 11; exit 14 reads an older wrapper's `budget-paused` as `unknown`; send `unable` exits 13 (or 10/12/14); `list --json` is `{items, skipped}`. Fixes the current exit/refusal set a new timeout outcome must not reuse or extend carelessly (contract/cli-exit-codes/14, contract/viola-send-output/unable, contract/viola-list-json/document).

@history_item n=12 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — frame verbs (send, wait, last, answer, …) verify the server before any frame; mismatch or unmakeable check exits 21 with its own hint. Part of `send`/`wait`'s pre-frame path the bound covers (contract/frame-verb-hints/server-verify-failed).

@history_item n=13 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (exit-21 lines / server check / catch site; strict-modes of the driven instance) link/unlink joined the frame verbs' exit-21 not-running hint and server check (eight verbs). Explains the verb lists in contract/frame-verb-hints/instance-not-running and contract/frame-verbs/strict-modes-refused; link/unlink stay hidden from MCP.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the six frame verbs' exit-21 instance pair; verify's home pair) the frame verbs' strict-modes check runs before any role file or log opens; verify checks its home tree before its log. Source of contract/frame-verbs/strict-modes-refused and contract/viola-verify-refusals/home-not-private — the order any new deadline or verify probe must respect.

@history_item n=15 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (exit 21's `detail`; the `internal-error` document; the unwrapped-name and endpoint-lost hints) exit 21's document carries its cause (`strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`); the catch site prints `{"v":1,"error":"internal-error",…}` under `--json`; endpoint-lost has its own hint "went away during the call". A client timeout or a server torn down mid-call is exactly the endpoint-lost/internal-error territory — any timeout outcome the chunk adds must map onto this closed catalogue or extend it deliberately (contract/instance-unreachable-detail/*, contract/frame-verb-hints/wrapper-went-away).

@history_item n=16 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder's word superseded the provisional records of the four exit-cause-code-catalogue entries. The catalogue the chunk rests on is ratified.

@history_item n=17 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — a peer's `unknown` refusal detail is a code word or nothing; `unknown`, wrapper fault and internal error take no hint. Bears on decision/hint-for-every-cause and the MCP refusal bodies (codes only).

@history_item n=18 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (exit 21's sixth cause `socket-dir-refused`; `server-verify-failed` by euid) added `socket-dir-refused` at the frame verbs and the euid check in `connect`. Part of the pre-frame `connect` path inside `send`/`wait`'s block (contract/frame-verb-hints/socket-dir-refused, contract/instance-unreachable-detail/socket-dir-refused).

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count and stop on one OS; the server-check hint without its OS scope) `MM` is `LedgerRow::relied()`'s length on every build, growing as owning chunks land rows; the stop is SIGINT/SIGTERM; the server check unscoped. Rule a 30th relied ledger row would follow (contract/viola-verify-output/step-line, stop).

@history_item n=20 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — landed `viola verify`'s step-line wireframe, summary and failing-row run (exit 1, stamp still written), `MM` growing as owning chunks land rows. Precedent for the fork "a relied ledger row with a verify probe" (contract/viola-verify-output/*, decision/single-progress-pattern).

@history_item n=21 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify to ten rows incl. the timing row `confirm-window`; the named `fixture not clean` refusal. Precedent for a timing/bound row measured by a verify probe, and the fixture-cleanliness rule any recorded fixture of a new probe must pass (contract/viola-verify-refusals/fixture-not-clean).

@history_item n=22 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — `MM` became a rule (`LedgerRow::ALL.len()`) rather than a figure. A new row updates no hard count in the layouts.

@history_item n=23 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` is the relied count `LedgerRow::relied()`; one step line per relied row; exit 0 when every relied row passes. The relied-vs-unrelied fork the chunk's P3 must price.

@history_item n=24 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — verify example at 22 rows, the new row relied on every OS. Precedent of a ledger-row landing's layout amendment.

@history_item n=25 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — one `claude agents --json` read joins the board (STATUS idle/busy, else unknown; a failed read prints the board as with no claude) and verify grew the `agents-join` row. Source of contract/viola-list-board/status-join and agents-read-failure — the `list` tool's own external read whose duration the bound must cover.

@history_item n=26 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a verify stopped by Ctrl-C/TERM prints only `unable: verify was stopped …` / `hint: run viola verify again …`, exit 1, no summary, no stamp. Source of contract/viola-verify-output/stop and contract/viola-verify-refusals/stopped, which a new probe must keep.

@history_item n=27 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — step line gained `absent` for the browser-tools-prompt row alone; summary `n` counts it; exit 0 when every relied row counts (PROVISIONAL, delegate overseer). Current verify output rows a 30th row's verdict word must fit — `absent` is reserved to that one row.

@history_item n=28 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — a key-on start runs `viola verify` after the bind with its output inherited and one fixed stderr line (PROVISIONAL). A new verify probe also runs at such a start, so its cost and output reach that path.

@history_item n=29 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (decision/contract/absence); each row verified against the code. Baseline of every picked row the chunk cites or amends; changes must be stated as `+`/`~`/`−` row amendments.

@history_item n=30 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0: no web surface (absence/web-surface), flat verbs with no interactive prompt or pager, Linux the one target. Constrains the chunk to the CLI/MCP surface only.
