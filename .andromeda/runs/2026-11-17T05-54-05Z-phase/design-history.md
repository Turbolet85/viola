@history_item n=1 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — Brand Identity: `~ contract/send-readback/prompt` widened the match that fills the readback box to the sent text or the sent text with its trailing run of ASCII space, TAB, CR or LF removed; a prompt trimmed at the start does not match (then provisional). This chunk's carried live reading (cry-vdpx2jqz) measures which whitespace the CLI trims at submit; the row is what that reading confirms or widens, and the scope keeps it trailing-only until the reading lands.

@history_item n=2 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (Brand Identity: 1 row) — `~ contract/send-readback/prompt` lost its «provisionally»: the trailing-only tolerance stands on the founder's own word. Any leading-trim change this chunk's reading suggests is a new amendment to a ratified row, which the founder places.

@history_item n=3 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (the send trailing-only tolerance ratified) — names this chunk outright: a leading trim («either end») waits for the live reading carried on «Standing live rig with a screen model» («Только хвостовые, ведущие — после измерения»). This is the binding precedent for the trim reading: measure first, and change the row only on the founder's word.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — the verify ledger gained `long-paste-wrapper` (a long paste unwraps to the text as pasted), `tag-escaping` and `local-command-clear`. These are prior readings of how the CLI frames a paste. The trim reading's «inside a paste» case and the rig's screen model rest on them.

@history_item n=5 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — added the ledger row `paste-newline` (an LF inside a paste stays inside the one prompt submitted), and the step counter now derives from `LedgerRow::ALL`. This is the measured precedent for in-paste whitespace that the rig's trim reading extends.

@history_item n=6 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box also fills on a measured post-condition (`/clear`'s new session), not only on the matching `prompt-submitted`. Under decision/no-success-before-confirmation and decision/no-optimistic-send-confirmation, a live reading may confirm only by a measured `prompt-submitted` or a post-condition.

@history_item n=7 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` is read back by its same-session post-condition on a verified CLI and stays `unconfirmable` on an unverified one; verify went to 24 relied rows. This is a precedent for live readings that move readback and ledger rows, tied to contract/cli-verification/verified and unverified-cli.

@history_item n=8 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `~ contract/send-readback/unconfirmable` and `+ contract/send-readback/mcp`: a local command with no post-condition stays `unconfirmable` (row contract/viola-send-mirror/unconfirmable). It is the latest change to the readback register that the rig's readings rest on.

@history_item n=9 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `viola verify`'s static step-line counter and summary `stamped <ver>  <n> pass  <m> fail` and its fixed exit-1 `unable:` refusals, each with a hint. The rig drives `viola verify` (`--record`) live, and its output is rows contract/viola-verify-output/step-line and summary.

@history_item n=10 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — `viola verify --help` gained a static paragraph on a trusted folder (the external-import blocker), and the fixed refusal for a recorded payload became the named `a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`; closed codes; never the content). This bears on reading the trust dialog live and on keeping screens as evidence: kept screens must pass the clean check (contract/viola-verify-refusals/unclean-fixture) and leak no host path or username.

@history_item n=11 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — the four dialog ledger rows (`question-answer`, `plan-approve-revise`, `question-notes`, `dialog-concurrency`) landed. These are prior live readings of the CLI's modal dialogs, which the rig must read by their drawn literals and selector rows.

@history_item n=12 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify prints one step line per relied ledger row, and `MM` is `LedgerRow::relied()`'s count. A row added after a live reading (for example v2-36's wake row) grows `MM` (contract/viola-verify-output/step-line).

@history_item n=13 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `viola verify` stopped by Ctrl-C or TERM before its rows are checked prints only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, with no step line or summary, and exits 1 (contract/viola-verify-output/stopped-or-refused). The rig's kill deadline and its exit-path cleanup hit this path when they stop a live verify.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `run`'s and `revive`'s start, and `verify` before its log, check their home tree (the home, `ledger/`, `bin/`, `plugin/`, the pinned copy) and refuse with the home-state or instance-state pair, exit 1, with no child. A `stamps.json` update refused by strict modes prints the home pair. The rig's private scratch home and its stamped-home copy must pass these checks, or every live start ends in contract/viola-run-start-refusals/home-tree-check or contract/viola-verify-refusals/home-tree-check.

@history_item n=15 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — added the socket-directory pair at `run`/`revive` start (`unable: the socket directory for <name> cannot be trusted`), the log code and exit-21 cause `socket-dir-refused`, `revive --json`'s `socket-dir-refused` code, and the board's read rule on the home. The rig's sockets under the short `/tmp` root and its scratch home must be owner-only directories. These are rows contract/viola-run-start-refusals/socket-dir, contract/exit-21-cause/socket-dir-refused and contract/log-detail-codes/socket-dir-refused.

@history_item n=16 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — retired the Git Bash warning: prompt text comes only from stdin or `--file` (decision/prompt-text-from-stdin-or-file), and the socket-directory pair's wording is confirmed as the founder's. The rig feeds sends through stdin or `--file` only.

@history_item n=17 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — the cli Platform narrowed to Linux terminals (CI on `ubuntu-latest`, decision/linux-terminal-platform); the width is read by `TIOCGWINSZ` alone; the Windows `.cmd`/`.bat` refusal and `batch-script-child` were retired; `revive --json` has ten codes; verify's relied count is 24 on every build, and `shim-resolution` passes when `claude --version` answers. The rig targets Linux PTYs only, and the codes it may meet are this closed set.

@history_item n=18 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — the precedent for an outer-PTY witness: `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`tests/cross_list_rows.rs`) runs `viola list` under an outer PTY with `TERM=xterm-256color` and reads exact SGR sequences. This is prior PTY/screen tooling in the tree that a standing rig and screen model can reuse or must not duplicate.

@history_item n=19 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour layer (colour decision order, `NO_COLOR` read, dim/bold spans) and the width from `viola_pty::host_size()`. Under a PTY the rig owns, `TERM`, `NO_COLOR` and the PTY size decide whether viola prints SGR, which the screen model must handle; `viola run` and `viola hook` print none (contract/cli-colour-decision-order/viola-run and viola-hook).

@history_item n=20 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap is built without `color`, so help and usage output is plain in every mode. Every styled byte is viola's own SGR module's, which narrows what the rig's screen model must expect from viola itself.

@history_item n=21 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — `revive` prints nothing once the child starts (contract/viola-run-output/child-started, decision/run-prints-nothing-while-wrapped). It also gained its fixed-order start refusals, none showing a path, pid or logged id (decision/start-refusals-show-no-path-or-pid), and the `setup:` help group. Whatever the rig sees on the PTY while viola wraps `claude` is the child's screen alone.

@history_item n=22 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — gave the revive `--json` start refusals their closed codes and the catch site's `internal-error` document; `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site (decision/silent-catch-sites); a `pump_child` error prints no document; a stale holder logs `stale-heartbeat`. A rig's interrupted or killed wrapper meets these silent paths and logged codes, not printed text.

@history_item n=23 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `revive` its `--json` refusal document `{"v":1,"error":"<code>","detail":null}`, with no hint and stderr empty (contract/viola-revive-output/json-refusal). This is the first form of the code set row contract/revive-json-refusal-code/* holds.

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `--json` failure documents hold no ESC, path or upstream text (decision/json-failure-documents-clean, guard `tests/cli_error_bodies.rs`). It also defined exit 2's fixed `UNUSABLE_TEXT` / `UNUSABLE_RESPONSE` stderr lines with no document, and the screen on peer replies. This is the NEVER-log floor (decision/no-internals-in-errors) that the rig's records share: no host path or upstream text.

@history_item n=25 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the first live test reworded the `input-not-ready` hint to `<name> was not ready for input; send again, and if it repeats a human must look at the session` (contract/refusal-hints/not-delivered-input-not-ready). This is the precedent for live rounds that move a hint's wording; the founder words such changes.

@history_item n=26 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — gave `viola wait`/`last` their event form (`session-end` takes the `turn-ended` form, `dialog unknown`) and message-mode `\xHH` escaping (contract/viola-wait-output/event-form, contract/cli-message-escaping/control-characters). The rig's live readings (for example a fired wake's `turn-ended`) read through these forms.

@history_item n=27 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `dialog-expired` takes the dialog form `<kind>  <name>  dialog <id>  cursor <n>` (contract/viola-wait-output/dialog-form). It sits beside the modal dialogs the rig reads on screen.

@history_item n=28 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — an API-error turn end shows like any turn end: `wait` reads `turn-ended`, and the only trace is one closed cause word of nine in `--json` and MCP data. This is the latest precedent for a turn-end field added with no human-output change, as the scheduled-wake reading (`scheduled_wakes`) carried here is. A `turn-ended` never fills a readback box (decision/turn-ended-never-fills-readback).

@history_item n=29 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified decision/api-error-turn-end-shown-as-turn-end (provisional to standing) and the delegate's confirm. This is the boundary wrap that relayed this chunk's isolation ruling and its carried readings, and no design row moved for the rig.

@history_item n=30 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `mcp` is a hidden verb, run by the plugin and absent from `viola --help` (decision/mcp-hidden-verb). The scope says the product binary gains no verb or surface from this chunk, so a rig entry point must not appear in the help's verb groups (contract/help-output/verb-groups).

@history_item n=31 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — built the five `viola --help` groups (`handoff:` row) and extended the fixed `unable: <text>` form's lists to `link`/`unlink`'s exit-21 strict-modes refusal (contract/help-output/verb-groups, contract/cli-refusal-line/fixed-unable-form). A rig that adds a verb or an output would move these closed lists.

@history_item n=32 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — retired exit 11 for good (contract/cli-exit-codes/10-14) and rewrote decision/no-hint-names-viola-release's why around the human-typing refusal alone. These are the current exit-code and hint constraints the rig's live sends can meet.

@history_item n=33 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ contract/refusal-hints/refusal-line`: `send` and `answer` share hints, and a `not-delivered` refusal with an opaque detail or none prints no hint. This is the hint register a live send's refusal is read against.

@history_item n=34 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — added the `empty-text` refusal: its hint `the text is empty once its trailing newlines are removed; send a text with content` (contract/refusal-hints/not-delivered-empty-text) and the exit-13 detail list. Trailing-whitespace handling on the send side is the neighbour of the trim reading.

@history_item n=35 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (decision, contract, absence and name). Any change this chunk forces lands as `+`/`~`/`−` row entries against the picked handles, not as prose sections.
