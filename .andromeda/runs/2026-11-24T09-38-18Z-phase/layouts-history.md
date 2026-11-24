@history_item n=1 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — gave `viola revive` its own output structure: a passed start prints nothing while the child holds the terminal, each refusal is exit 1 with one `unable:` line and a `hint:` line last on stderr and stdout empty, a malformed `--id` is clap's usage error (exit 2). It also put `revive` into the `setup:` help group by the operator's answer. This is the precedent for adding a revive-based setup verb. `adopt` is built on revive's start, so it either inherits this refusal shape or must state where it differs, and the help row needs the same kind of decision.

@history_item n=2 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (revive's fifth pair) added the founder-worded live-elsewhere pair `unable: <name>'s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork` and fixed the start's preflight order: holder, logged session, `session-live` (skipped under `--fork`; a failed `claude agents --json` read passes), then the recorded directory. This is the guard `adopt` reuses, and it carries a trap. A failed agents read lets the start go on, which would fail open for a session live elsewhere. The hint also names `viola revive <name>`, which does not fit an adopt of an id that has no viola name yet. The same entry built the board's unwrapped rack, where a plain session's NAME is its `sessionId`. That is the row an adopted session leaves.

@history_item n=3 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `viola revive` `--json`: each refusal is one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, stderr empty, and the human pairs stay byte-identical without `--json`. It moved `--json` to every data verb, revive included, and put `revive --list` at the catch site while a revive start stays outside it. This is the precedent for `adopt --json`'s refusal codes and for where its start sits relative to the catch site.

@history_item n=4 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built `viola --help` as the grouped table with served verbs only: the `Usage: viola [--home <dir>] <verb> [args]` override literal, the rows pinned by the `HELP` literal and `help_lists_only_served_verbs`, and no about line or `Commands:` list. `adopt` joining a group means editing the `HELP` literal and both help rows (`contract/cli-verbs/help`, `contract/viola-help/verb-groups`). It must not appear before it is served.

@history_item n=5 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is a hidden verb whose tools mirror CLI `--json` documents less `v`; a refusal is `isError: true` with `refused: <code>`. This bears on the scope's fork of whether `adopt` is an MCP tool (conservative arm: CLI-only, as `viola up` chose).

@history_item n=6 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified served-verbs-only help and removed the provisional marks on the help and refusal wordings. A new verb's help placement and refusal wording therefore fall under that standing rule. Any new pair wording `adopt` adds is a planning form until the founder words it.

@history_item n=7 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (run/revive home-tree refusals) the run and revive starts check the home tree (home, `ledger/`, `bin/`, `plugin/`, pinned copy) before the wrapper log opens. On a refusal they print the home pair or the instance state-files pair, exit 1, with no log or child, and `strict-modes-failed` under `--json`. A refused check writes no line, by the founder's word. `adopt` must keep this check ahead of any log or snapshot write, its new name's first snapshot included.

@history_item n=8 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the `cli` catch site's `internal-error` document) with `--json` before a `--`, the catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, instead of `error: internal error`. A revive start stays outside the catch site's verb list. `adopt` needs the same decision on which of its paths reach the catch site.

@history_item n=9 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (a stale holder's code) split revive's `--json` collision codes into `already-live` (a live holder or a fresh beat) and `stale-heartbeat` (a stale holder). This applies when `adopt` targets a name that is already held, the scope's defined closed outcome.

@history_item n=10 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the post-preflight gap closed) a refusal of the start revive shares with run, after a passed preflight, prints one exit-1 document (`pinned-hash-mismatch`, `squatted-name`, or `internal-error` for a program not found or a start error). A `pump_child` error prints no document because the child has held the terminal, and a document from `main` was rejected for that reason. `adopt` reuses the shared start and inherits this set and this trap.

@history_item n=11 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (socket-directory refusal of the run and revive starts) `refuse_socket_dir` runs at the start before any snapshot, child or bind. It prints `unable: the socket directory for <name> cannot be trusted` plus its hint, exit 1, `socket-dir-refused` under `revive --json`, and never changes a mode. This shared start refusal also reaches `adopt`. It is not counted among revive's own five pairs.

@history_item n=12 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the .cmd/.bat refusal leaves revive) removed `batch-script-child` from revive's start-refusal codes, a closed word with no emitter left. `adopt`'s `--json` code set should list only the words its build can emit: `pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused`, `internal-error`.

@history_item n=13 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 census put no web surface and no interactive prompt, TUI or pager in any verb; the fleet console is no verb's output. It dropped the Unix qualifiers on the socket-directory refusals. `adopt` is bound to no prompt, so an operator-given name and parent come only from flags.

@history_item n=14 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the layout master into rows (decision/contract/absence), each verified against the code. Every picked row's wording dates from this conversion unless a later entry amended it. `adopt`'s changes land as row operations (`+`/`~`) on the cli surface.

@history_item n=15 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1 of 2) changed `contract/cli-exit-codes/11` to «Retired (was `budget-paused`), never reused; no refusal maps to 11». No new `adopt` refusal may take exit 11.

@history_item n=16 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ contract/cli-verbs/run`, `~ contract/viola-run/start-output`, `~ contract/cli-verbs/revive`: the start writes the claude CLI's trust record for an untrusted session folder, keyed for revive by the recorded working directory. The step adds no refusal, exit, hint or byte. `adopt` has no recorded directory, so its cwd source (a P3 item) also keys this trust write.

@history_item n=17 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `~ contract/viola-run/start-output`: before the child, only a key-on start's automatic verify line and verify's output reach the terminal, discarded under `revive --json`. A start through the shared path prints these lines, so `adopt`'s passed-start output inherits them. The founder ratified this in 2026-11-22T17-29-41Z-wrap.

@history_item n=18 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — (1 of 2 and 2 of 2) `+ contract/viola-list-board/tree`, `~ contract/viola-list-board/single-write`, `~ contract/viola-list-board/column-widths`, `~ decision/mcp-hides-wheel-and-handoff-verbs`: the board draws wrapped rows as a tree under their parents, NAME indented per depth, and a row whose parent has no wrapped row is a root. The parent `adopt` records is what places the adopted session in this tree. The MCP decision is PROVISIONAL (founder-forks.md W1).

@history_item n=19 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — `~ contract/viola-revive/usage` added `--model`/`--effort` to revive, conflicting with `--list`, and `+ contract/run-revive-usage/pair-after-dashes` refuses a launch pair after the dashes. It landed PROVISIONAL (founder-forks.md W2). This bears on `adopt`'s model/effort source where no snapshot recorded them, and on whether it takes these flags.

@history_item n=20 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — `+ contract/cli-verbs/open`, `+ contract/viola-open/parent-default`, `+ contract/viola-revive/no-window`, `~` both help rows. `open` added a closed flag allowlist (`--parent`, `--role`, `--cwd`, `--workspace`, `--model`, `--effort`, `--json`). The parent defaults to the calling agent's ViolaName with origin `agent`, else no parent with origin `cli`. Revive places no window. These are the nearest precedents for `adopt`'s `--parent`, its lineage origin, and the fork on whether it opens a terminal.

@history_item n=21 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — `~ contract/cli-verbs/help` (`board:` holds list and focus), `~ contract/cli-exit-codes/21` (focus's strict-modes refusal prints the fixed state pair), `+ contract/cli-verbs/focus`. This was the latest verb addition by delegate, PROVISIONAL. The help row and exit-21 row now include `focus`, and `adopt` extends these same rows.

@history_item n=22 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — (1 of 2) `~ contract/cli-verbs/help`: `setup:` holds open, run, revive, up, verify, plugin install. `~ contract/viola-revive/no-window` gave revive a hidden `--origin <cli|agent>` for window placement, and `~ contract/viola-revive/usage` made `--origin` conflict with `--list`. Both are PROVISIONAL (delegate overseer). This is how a revive-based verb (`up`) reaches a new window through revive's start, and it is the precedent if `adopt` opens a terminal.

@history_item n=23 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — (2 of 2) `~ contract/cli-exit-codes/21` (up's board refusal prints the fixed state pair), `~ decision/board-vs-strip` (up acts on the fleet), `+ contract/cli-verbs/up` (CLI-only, no MCP tool, no channel method), PROVISIONAL. This is the precedent for the CLI-only arm of `adopt`'s MCP fork. `decision/board-vs-strip` also bears on `adopt`, whose target is a session id plus an operator-given name rather than an existing strip's ViolaName (`decision/targets-are-wrapped-names`).

@history_item n=24 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — (Surface: cli: 1 row) `~ contract/viola-help/verb-groups`: the five rows now read `board:    list, focus` … `setup:    open, run, revive, up, verify, plugin install`, pinned by the `HELP` literal in src/cmd/mod.rs. This is the current wording `adopt` must amend if it joins `setup:`.
