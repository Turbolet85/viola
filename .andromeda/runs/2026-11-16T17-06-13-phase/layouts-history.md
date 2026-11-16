@history_item n=1 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — wait / last lines as landed: every non-dialog kind takes the `turn-ended` line form, `waiting:` only on a terminal stderr without `--json`, one `--json` document per call, the exit-21 pair and the once-printed `error: internal error`. This is the origin of contract/viola-wait-output/turn-ended, contract/cli-verbs/wait and contract/cli-json-mode/one-document; the nine cause words must reach `viola wait --json` inside that one document, and the human `turn-ended  <name>  <ts>  cursor <n>` line (decision/human-output-is-contract) is not widened by a cause column unless it is planned as a design change.

@history_item n=2 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the `viola wait` wireframe gained the `dialog-expired` line and the turn-ended annotation became "every kind without a dialog_id takes the turn-ended form" (the current wording of contract/viola-wait-output/turn-ended). Precedent for a new event-level value reaching `wait`: the line was a planning form with the founder's wording owed, later ratified (n=3); the cause words here are already the founder's (CARRY), so no wording is owed, but any human rendering of them would be.

@history_item n=3 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch ratified the `dialog-expired` line and other planning forms on the wait / list / verify outputs. Shows how a provisional wait-output form is closed (founder's word relayed by the overseer); relevant if the plan touches the wait wireframe beyond the `--json` field.

@history_item n=4 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` as a hidden verb whose tools are a driver's machine view: each success the CLI verb's `--json` `ok` payload less `v`, each refusal `isError: true` with `refused: <code>`, bodies codes only, and no tool naming release / pause / link / unlink. Origin of contract/mcp-tool-results/success, contract/mcp-tool-results/refusal, contract/mcp-tools/wait and decision/mcp-hides-wheel-and-handoff-verbs: MCP `wait` must carry the nine words by mirroring `wait --json`, not by a shape of its own, and the tool description may name the closed set but no error text.

@history_item n=5 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — made `--json` the machine view of every data verb (decision/json-on-every-data-verb), one stdout document with stderr empty. The cause word's widening is a value change inside that existing contract; no new document or field shape outside it.

@history_item n=6 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `reply_of` / `of_reply` screen a peer's reply: a refusal that is no code word reads `unknown`, a string `detail` that is no code word reads as none, so no opaque upstream text crosses. The direct precedent for this chunk's boundary: an `error` value outside the map reads `api-error`, and every reader of the event (wrapper check, event-log readers, `wait`, MCP, list) must refuse any text other than the nine, never pass it through.

@history_item n=7 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document carries its cause in `detail`) — replaced one opaque `detail: null` with a closed list of cause words so a driver can tell causes apart, `error` and `v` unchanged, no exit moved. Same pattern as growing `cause` from one word to nine: a closed set grown in the machine view, the outer shape and exit set fixed; ruled by the founder after the widening was shown to him.

@history_item n=8 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's `internal-error` document under `--json`) — `viola wait` / `viola last` print `{"v":1,"error":"internal-error","detail":null}` on stdout under `--json` for a failure or panic. If a malformed or unknown `cause` reaches the CLI's reader, the plan must say whether it is refused into this path or read as `api-error`; the catch-site behaviour is the standing contract for wait's failure.

@history_item n=9 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — viola list as built: the uncoloured stream form, `escape_row` on upstream cells (one upstream cell, the CLI version), `list --json`'s exit-21 refusal with stderr empty and no log. Origin of contract/cli-verbs/list, contract/viola-list-json/document and the strict-modes refusal rows; the scope names the board / `list` among consumers of the cause word, so any board rendering of it must stay within the board's fixed columns and the list payload contract.

@history_item n=10 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — a joined wrapped row's STATUS reads `idle` or `busy` from the agents read; the board's status words are a fixed set. Relevant if a turn end's cause is considered for the board: STATUS is not a cause column and adding one is a design change (decision/human-output-is-contract).

@history_item n=11 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola list` styled on a colour terminal (bold NAME, `DIALOG` amber, stale dim) and plain under `--json`, a pipe, `NO_COLOR`, `TERM=dumb`. Origin of decision/viola-list-styling, decision/amber-means-dialog and decision/plain-output-modes: any new cause word shown on the board or in `wait` takes no colour (amber is DIALOG only) and stays plain ASCII.

@history_item n=12 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Surface: cli: 6 row(s), 1 of 2) — changed contract/viola-list-json/document (the payload is `{items, skipped}`) and made an older wrapper's retired `budget-paused` reply read as `unknown`. Precedent for the "value outside the map reads the fallback word" rule this chunk applies (`api-error`), and the current text of a picked row.

@history_item n=13 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows, each verified against the code; many divergences settled "as the code shows". The picked rows naming `turn-ended` / `wait` / MCP results are these rows; rows naming the one word `api-error` are the wrap's reconcile once the nine land.

@history_item n=14 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0: no web surface, viola renders no page (absence/web-surface), the fleet console owed to «Console scaffold». The cause words reach only the cli, `--json` and MCP surfaces; no page shows them.
