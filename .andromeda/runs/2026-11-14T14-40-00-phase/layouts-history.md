@history_item n=1 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — added both `viola release <name> --budget` stdout lines (`budget gate lifted until <end>  wheel <holder>` / `budget gate open  wheel <holder>`, exit 0, the `<end>` date/time forms and escaping) to the wheel, handoff and dialog verbs block; these are exactly contract/release-budget-output/gate-lifted and gate-open, which this chunk retires with `write_budget_lifted` / `write_budget_open` while the plain `release` line (contract/wheel-verb-output/release) stays.

@history_item n=2 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder confirmed WHEEL 23, the board's planning forms (including `human - budget-paused`), the list refusal and the two `release --budget` lines as his wording; the source of decision/board-planning-forms-wording, whose `budget-paused` form leaves with this chunk, so removing a founder-ratified wording is a row change to name at wrap, not a silent cut.

@history_item n=3 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `viola list` as built: one stdout write, column widths (NAME/LIVE 5/STATUS 7/WHEEL 23/DIALOG 17/CLI unpadded), plain lines, `list --json` refusal, the BAY header; the origin of contract/viola-list-board/single-write, column-widths and plain-lines that this chunk's BAY-line and board-cell budget removal (`src/human/board/bay.rs`, `cells.rs`, `rows.rs`) must keep byte-stable for every non-budget cell.

@history_item n=4 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — made `list --json`'s item, `budget` and `skipped` shapes the `list` payload contract (contract/viola-list-json/document); the `budget` field this chunk drops from `--json` and MCP `list` is that contract's, so its removal amends the payload contract and the MCP success shape (contract/mcp-tool-results/success).

@history_item n=5 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the styled TTY board and the unwrapped NAME cut from `host_size()` (contract/viola-list-board/name-truncation, decision/words-carry-every-state); the board's styling and cut sit beside the BAY line this chunk edits, and the decision/bay-line-unstyled / only-header-bay-line rows rest on it.

@history_item n=6 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified the board width from `host_size()`, served verbs only and `viola list`'s exit 1 / no-row behaviour as built; the board rows this chunk touches stand on his word, so any byte change outside the budget cells is a fork.

@history_item n=7 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` tools are the CLI verbs' `--json` documents less `v`, and no tool, description or result names `release`, `pause`, `link` or `unlink` (decision/mcp-hides-wheel-and-handoff-verbs); this chunk edits the MCP `list` tool description naming the budget (`crates/viola-mcp/src/tools.rs`) and its result, and must keep the driver `release` guard and the no-release-naming rule intact.

@history_item n=8 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — added `viola pause` / `viola release` to the catch-site verbs; `release` is the verb whose `--budget` option leaves while the plain wheel hand-back and its exit-1 catch site stay.

@history_item n=9 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the verify wireframe gained `[21/21] statusline-rate-limits` with the statusline rows, and `MM` became `LedgerRow::relied()`'s length; this chunk retires the `statusline-rate-limits` row (relied 24 → 23), so the verify step-line contract (contract/viola-verify-output/step-line) and its relied-count rule are what it re-counts.

@history_item n=10 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — set the rule that the verify count carries its rule (`MM` from the ledger, not a literal); the precedent that removing a ledger row changes `MM` through the rule, not a hand-edited number.

@history_item n=11 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — verify at 24 relied rows, `[24/24] local-command-compact` last, failing example `22 pass  2 fail`; the current wireframe numbers this chunk's row removal moves to 23.

@history_item n=12 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — verify's relied count and stop on one OS: `MM` all 24 rows on every build (the Windows set that skipped the three statusline rows gone); the latest statement of the count this chunk reduces when `statusline-rate-limits` leaves.

@history_item n=13 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — the .cmd/.bat refusal leaves revive and verify: a closed word no build can emit leaves with its emitter, and revive's code count was recounted; the removal precedent for `budget-paused` and its exit 11, with the difference that this chunk's exit 11 is retired and never reused.

@history_item n=14 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — the Windows VT enable leaves the cli colour surface: a removal that kept Linux human output byte-identical, recorded as was/now on the board's uncoloured conditions; the form for recording this chunk's board budget-cell removal.

@history_item n=15 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `viola send`'s client-side `control-character` / `empty-text` refusals before any frame, exit 13, mirrored by the wrapper (contract/send-refusals/*, decision/wrapper-repeats-send-refusals, contract/viola-send-output/unable); these are send refusal rungs adjacent to the `budget-paused` rung this chunk removes from `send`'s refusal order, and must stay intact.

@history_item n=16 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the founder's `input-not-ready` hint wording (contract/viola-send-output/input-not-ready-hint); a send hint in the hint table (`src/human/hints.rs`) this chunk edits to drop the budget hints, kept word for word.

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — the unwrapped-name and endpoint-lost hints: every cause has its own hint, the no-hint list unmoved (decision/hint-for-every-cause); removing the `budget-paused` cause and its hint must leave every remaining cause with its distinct hint and none naming `viola release` (decision/no-hint-names-release).

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — exit 21's document carries its cause in `detail`: «moving no exit», `error` and `v` unchanged; the exit-code typed tail (contract/cli-exit-codes/*, json-form) this chunk edits by retiring 11, keeping every other code's number.

@history_item n=19 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — exit 21's sixth cause, `socket-dir-refused`, at the frame verbs: the latest change to the exit-codes typed tail («the exit set does not move»); the register this chunk's exit-11 retirement amends.

@history_item n=20 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the `unknown` refusal's detail is a code word or nothing, and `unknown` / `wrapper fault` / `internal error` take no hint (the except of decision/hint-for-every-cause); a `budget-paused` refusal word from an old wrapper after the removal reads through this screening, so it bears on mixed-version readers.

@history_item n=21 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — the six frame verbs' exit-21 instance pair: `send` and `release` among the frame verbs refusing strict modes before any log (contract/cli-exit-codes/21); the refusal order of `send` and `release` that the budget rung removal edits.

@history_item n=22 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — revive's start preflight order (holder, logged session, session-live, recorded directory; contract/viola-revive/preflight-order) and the board's unwrapped rack; this chunk removes budget references in `src/cmd/revive.rs` and the board, and must keep that order and the rack.

@history_item n=23 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 version census (`doc:viola-0.2.0/census`) amended the cli surface; the census whose F-08 rows route every budget passage (and the `statusline-rate-limits` ledger row) to v2-04, i.e. this chunk, retired at its wrap.

@history_item n=24 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (the picked rows' source), verified against the code; the budget rows this chunk retires (`budget-paused`, `release --budget`, the BAY cells, exit 11) are rows of that conversion, retired by `−`/`~` at wrap.

@history_item n=25 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — added contract/viola-answer-output/not-delivered-hints, the answer hint table carrying the send hints; the shared hint table this chunk edits when the budget hints leave.
