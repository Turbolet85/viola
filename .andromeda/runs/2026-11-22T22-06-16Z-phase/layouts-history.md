@history_item n=1 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — registry migration: the Decisions Log left the body; the semantic roles of racks, strips and rack separators, the cock playing only on a WRAPPED-rack strip, one token vocabulary on every surface. Matters because the fleet tree is a new drawing inside the racks, and it must stay within the one token vocabulary (`decision/one-token-vocabulary-on-cli`) and the rack order.

@history_item n=2 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — created the `viola revive` output structure: a passed start prints nothing, the `--list` row `<ts>  <cause>  <id>`, the refusal pairs and the help group. Matters because "parent kept by revive" extends this verb, and its passed-start silence (`contract/viola-revive/start-output`) and preflight order (`contract/viola-revive/preflight-order`) bind how the recorded parent is carried.

@history_item n=3 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board as built: the widths (NAME the longer of 4 and the longest name, LIVE 5, STATUS 7, WHEEL 23, DIALOG 17, CLI unpadded, no trailing space), one stdout write, `escape_row` covering DEL and C1, the `list --json` refusal document, and the exit-1 catch-site paths. Matters because fleet trees must fit these fixed columns (`contract/viola-list-board/column-widths`, `single-write`, `decision/board-row-escaping`) without a new column (`decision/human-output-is-contract`).

@history_item n=4 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder ratified the board's planning forms, WHEEL 23, the list exit-21 refusal and the unlistable `instances/` exit 1. Matters because the tree drawing must not change ratified board wording (`decision/board-planning-forms-wording`). Any new word or glyph is a fork for the founder.

@history_item n=5 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the web surface was cut to a stub. `list --json`'s item and skipped shapes became the `list` payload contract (architecture §Standard Contracts). "Shared vocabulary" fixed the six captions, words and slot order. Matters because the scope ties the fleet trees to `decision/shared-captions-fixed`, and what `list --json` gains must extend that payload contract.

@history_item n=6 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added the unwrapped rack under `-- UNWRAPPED - READ-ONLY --` (sorted by name, NAME the escaped sessionId), the STATUS join by `claude agents --json`, and NAME width over both racks. It also added revive's session-live pair and fixed the preflight order. This is the latest precedent for a new grouping on the board within fixed columns (`contract/viola-list-board/unwrapped-rack`, `status-join`). Trees must coexist with both racks and with the name ordering (`decision/board-ordered-by-name`).

@history_item n=7 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `viola revive` took `--json`: refusal documents `{"v":1,"error":"<code>","detail":null}`, the `--list --json` document, and `--json` as the machine view of every data verb. Matters because a revived parent, or any new revive refusal, must be stated in both forms (`decision/json-on-every-data-verb`, `contract/viola-revive-json/*`).

@history_item n=8 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola list` was styled on a TTY: bold NAME, amber DIALOG, stale rows dim. It is uncoloured under `--json`, a pipe, `NO_COLOR` or `TERM=dumb`. Unwrapped NAMEs are cut by `host_size()`, never inside an escape. Matters for the scope's "plain when piped". Tree prefixes interact with NAME width, truncation (`contract/viola-list-board/name-truncation`) and the styled spans (`decision/viola-list-styling`). A glyph must respect `decision/plain-output-modes`, which allows no non-ASCII glyph.

@history_item n=9 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp`'s tools are a driver's machine view of the CLI verbs, each result the verb's `--json` document less `v`, with no tool naming `link` or `unlink`. Matters because anything `list --json` gains for lineage reaches the mcp `list` tool (`contract/mcp-tools/list`). Exposing link data there runs into the no-`link`-naming rule.

@history_item n=10 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch ratified the board's width from `host_size()`, `list`'s exit 1 for a failed log start, and no row on a per-instance read error. Matters because a lineage read that fails per instance must follow the "no row" precedent, or else be a priced fork.

@history_item n=11 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the `link` / `unlink` result lines as built) — the verbs print `link    <driver> -> <driven>  since …` / `unlink  <driver> -> <driven>`, plain in every mode. The printed names are the verb's own arguments, never read back from the reply. The self-pair is refused by the wrapper only. Matters because "links drawn apart, name-checked" shows these observed links beside parent edges. The never-read-back rule and `decision/no-link-column` (links visible only in link/unlink output) are direct constraints, possibly a fork.

@history_item n=12 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the `handoff:  link, unlink` help row; exit 21, the server check and the catch site; the driven instance's strict-modes refusal) — link/unlink act on the driven instance, refuse exit 21 before any log, and moved to the catch site. There is a trap: opening a role file narrows the home to 0700, so a check placed after `client::start` passes a widened home. Matters because the link store and its reads are where the links come from, and the strict-modes order applies to any new lineage read.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of the run and revive starts) — the run and revive starts check the home tree before the wrapper log opens: exit 1, no log, no child, and `strict-modes-failed` under revive `--json`. Matters because the lineage event is written on this start path (`contract/run-revive-start/home-tree-refused`). It must land after these checks.

@history_item n=14 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's `detail`; revive's `stale-heartbeat`; the revive `--json` post-preflight gap closed) — `list --json`'s strict-modes refusal carries `"detail":"strict-modes-failed"`. Revive's start refusals shared with run got documents (`pinned-hash-mismatch`, `squatted-name`, `internal-error`). A `pump_child` error prints no document. Matters because any new launch-time parent refusal at run/revive must join this closed code catalogue (`contract/run-revive-start-json/*`) as a priced fork.

@history_item n=15 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory refusal of the run and revive starts) — `socket-dir-refused` runs at the start before any snapshot, child or bind, and its code entered revive's list. Matters because it fixes the order of the start path where the lineage line is written, and the precedent for adding a start code.

@history_item n=16 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows VT enable leaves the cli colour surface; the .cmd/.bat refusal leaves revive) — `list`'s uncoloured conditions are now exactly `--json`, pipe, `NO_COLOR` and `TERM=dumb`, and revive's start codes went from 11 to 10. Matters because the tree's plain/styled split keys on these conditions (`contract/viola-list-board/uncoloured-modes`). It also shows the current revive code set.

@history_item n=17 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0: no web surface, no TUI or prompt in a verb, and the terminal fleet console (matrix#v2-14) reusing the six captions, words and slot order. Matters because fleet trees on `list` must keep the captions the console reuses (`decision/shared-captions-fixed`, `decision/no-prompt-or-pager`, `absence/web-surface`). The console itself is Epoch 6, out of scope.

@history_item n=18 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (226), each checked against the code. Matters because every row this chunk changes or adds (a lineage column or glyph, a `list --json` field, a revive promise) must be named by handle with `+`, `~` or `−` in the same form.

@history_item n=19 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1 of 2 and 2 of 2) — trimmed `decision/board-planning-forms-wording` to bare skip counts and the empty board's three lines, made `contract/viola-list-json/document` `{items, skipped}`, retired the BAY budget cells, and retired exit 11 for good. This is the latest change to the board and list payload rows this chunk extends. It shows the `~` promise form for growing `list --json`, and exit 11 must not be reused.

@history_item n=20 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `contract/cli-verbs/run`, `contract/cli-verbs/revive` and `contract/viola-run/start-output` gained the start's trust step: no refusal, exit code, hint, `--json` code or terminal byte, keyed by revive's recorded directory. This is a precedent for a silent start-time write. The lineage event at launch must keep the passed start's output unchanged, or state the change in these rows.

@history_item n=21 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `contract/viola-run/start-output` now says only a key-on start's verify line reaches the terminal before the child, discarded under `revive --json`. This is the current wording of the start-output row that a launch-time parent declaration (a `viola run` flag) must respect.

@history_item n=22 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (the delegate's confirm ratified — 2026-11-20-a-new-cli-version-stamped-automatically, contract/viola-run/start-output) — the founder ratified the start-output row. Matters because the row this chunk's launch path rests on now stands on his own word, so changing it needs his fork.
