@history_item n=1 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — created the `viola revive` output structure: a passed start prints nothing of its own while the child holds the terminal, `--list` rows `<ts>  <cause>  <id>`, each refusal an `unable:`/`hint:` pair with empty stdout, a malformed `--id` clap's exit 2, and `revive` put in the `setup:` help row. This is the source of contract/viola-revive/usage, /start-output, /refusal-shape and /malformed-id. Any model/effort argument this chunk adds to revive extends that usage line, and its start must stay silent.

@history_item n=2 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the 22nd ledger row `resume-session` (`--resume <id> reopens the session under that id`) landed in verify's wireframe, and MM grew with it. This is the precedent for a new relied ledger row: an undocumented model/effort channel or replay-on-resume behaviour becomes one more `[NN/MM]` step line and moves the summary counts. revive's resume rests on this row.

@history_item n=3 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — MM became the relied count (`LedgerRow::relied()`'s length), not a fixed number. A ledger row added for the model/effort channel changes the count through that rule (contract/viola-verify-output/step-line, /summary).

@history_item n=4 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — fixed revive's preflight order (holder, logged session, `session-live` skipped under `--fork`, recorded directory) and its fifth own pair, and built the board's unwrapped rack. This is the source of contract/viola-revive/preflight-order and contract/viola-list-board/unwrapped-rack. A recorded-pair read in revive must not reorder the preflight. An unwrapped row has no recorded pair, so any shown pair is `n/a` there.

@history_item n=5 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `viola revive` `--json` on both forms, each refusal one exit-1 document `{"v":1,"error":"<code>","detail":null}` with the human pairs byte-identical, and put `revive --list` at the cli catch site while the start stays outside it. Any new revive refusal (for example a bad model or effort value) needs a closed `--json` code here, and the human pairs must not move (decision/json-on-every-data-verb).

@history_item n=6 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — closed the revive `--json` post-preflight gap: the start refusals shared with `run` (`pinned-hash-mismatch`, `squatted-name`, `internal-error`; a `pump_child` error prints no document) each print one exit-1 document. It also added `stale-heartbeat`, the catch site's `internal-error` document and exit 21's `detail` causes. This is the source of the contract/run-revive-start-json/* rows: a new start failure of model/effort resolution joins this closed code set.

@history_item n=7 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of verify and the run and revive starts) — the `run`/`revive` start checks the home tree before the wrapper log opens: the home pair or the instance state-files pair, exit 1, no log, no child. This is contract/run-revive-start/home-tree-refused. A config.json default pair or a per-instance record read must sit inside this order: no read of an unchecked file.

@history_item n=8 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — added the socket-directory refusal of the `run`/`revive` starts (exit 1 before any snapshot, child or bind, the directory left as found) and the `socket-dir-refused` code among revive's `--json` start codes. This is contract/run-revive-start/socket-dir-refused and contract/run-revive-start-json/socket-dir-refused: the spawn ordering this chunk changes comes after this check.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — removed `batch-script-child` from revive's start codes (a closed word with no emitter leaves with its emitter), made verify's relied count one set on Linux, and dropped the per-OS wording of help. Precedent: closed code sets track their emitters, so model/effort codes must be ones a build can emit.

@history_item n=10 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `run`'s start writes the claude CLI's trust record for an untrusted session folder, flag only, and revive keys it by the recorded directory. The start step adds no refusal, exit code, hint, `--json` code or terminal byte. This is the nearest precedent for a silent start step that changes what the child gets (contract/cli-verbs/run, /revive, contract/viola-run/start-output). Handing the child a model and effort should follow it: no new terminal bytes, and nothing written to the user's settings.

@history_item n=11 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — reworded contract/viola-run/start-output: before the child, only a key-on start's automatic verify line and verify's own output reach the terminal (discarded under `revive --json`). This is the current start-output promise that a model/effort step must leave intact.

@history_item n=12 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — the founder ratified contract/viola-run/start-output (from 2026-11-20) and the verify step-line, summary and exit-0 rows (from 2026-11-18-browser-tools-prompt-modal). These rows bind this chunk and are no longer provisional.

@history_item n=13 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — verify's step line gained `absent` for the browser-tools-prompt row alone, `n pass` counts it, and exit 0 means every relied row counts. This is the current form of contract/viola-verify-output/step-line, /summary and /exit-0, which a new model/effort ledger row would print through.

@history_item n=14 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — the board drew wrapped rows as a tree, with NAME width including the tree indent (contract/viola-list-board/column-widths), and added a `-- LINKS --` block after the unwrapped rack instead of a link column (decision/no-link-column). The MCP list result now carries links as data (decision/mcp-hides-wheel-and-handoff-verbs, PROVISIONAL, founder-forks W1). This chunk records the pair beside this lineage record. If the pair is shown on `list`, the precedent is a separate block or `--json`/MCP data, never a seventh column.

@history_item n=15 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board as built: widths (NAME, LIVE 5, STATUS 7, WHEEL 23, DIALOG 17, CLI unpadded), one upstream cell (the CLI version) and the `list --json` refusal. These widths and the CLI column stand. Showing a model/effort pair on the board would change these fixed columns, which decision/human-output-is-contract makes a design change; `--json` comes first.

@history_item n=16 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder confirmed WHEEL 23 and the board's planning forms and refusal. The board's column set and widths are founder-ruled, a constraint on any pair display.

@history_item n=17 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built `viola --help` as the grouped table over served verbs only (pinned by the `HELP` literal; `viola <verb> --help` stays clap's per-verb usage) and the styled, width-cut board. This is the source of contract/cli-verbs/help, contract/viola-verb-help/usage, decision/viola-list-styling and contract/viola-list-board/name-truncation. New run/revive flags appear in per-verb help only, and the grouped table does not move.

@history_item n=18 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified served-verbs-only help and the board's width from `host_size()`. These help and truncation rows bind this chunk as founder-ruled.

@history_item n=19 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the `handoff:  link, unlink` help row built) — the help table reached its five rows, the source of contract/cli-verbs/help as picked. The link verbs' argument order is also behind decision/argument-order-mirrors-channel.

@history_item n=20 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is a hidden verb. Its tools are the driver's machine view, each result the CLI verb's `--json` `ok` payload less `v`, and no tool names release/pause/link/unlink. This is the source of contract/mcp-tools/list and contract/mcp-tool-results/success. A pair added to `list --json` reaches the MCP list result unchanged.

@history_item n=21 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — `list --json`'s item and skipped shapes became the `list` payload contract (architecture §Standard Contracts), and the six captions became the shared vocabulary. Any pair added to the list item is a payload-contract change, not only a layout one.

@history_item n=22 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — retired exit 11 (never reused; contract/cli-exit-codes/11) and dropped `budget` from the list document (contract/viola-list-json/document). A new refusal for this chunk must not take exit 11.

@history_item n=23 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 census: no web surface, and the terminal fleet console reuses the six captions, words and slot order (decision/shared-captions-fixed). It also dropped the Unix qualifiers on the run/revive socket-directory refusal. Any board change to show a pair also binds the console.

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the no-hint list (`unknown`, wrapper fault, internal error) stands. Any new model/effort refusal cause outside it needs its own distinct hint (decision/hint-for-every-cause, decision/no-hint-names-release).

@history_item n=25 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows, each verified against the code. This is where every picked row's handle comes from; amendments now name rows by `+`/`~`/`−` handle.
