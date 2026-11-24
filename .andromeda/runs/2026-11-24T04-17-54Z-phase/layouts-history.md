@history_item n=1 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — `~ contract/cli-verbs/help` (`board:` holds list and focus), `~ contract/cli-exit-codes/21` (focus joins the verbs whose strict-modes refusal prints the fixed state pair), `+ contract/cli-verbs/focus`; all PROVISIONAL on the delegate. This is the nearest precedent for a new fleet verb: `viola up` must take a help row and an exit-21 or exit-1 strict-modes arm in the same way, and the PROVISIONAL marks on these rows still stand.

@history_item n=2 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — `setup:` became `open, run, revive, verify, plugin install` (`~ contract/cli-verbs/help`, `~ contract/viola-help/verb-groups`), and the entry added `+ contract/cli-verbs/open`, `+ contract/viola-open/parent-default` (origin `agent` vs `cli`) and `+ contract/viola-revive/no-window` (revive never places a window; the window step reaches only a start that asks for one). `up` re-opens each instance through revive "in the window `viola open` would give it", so it runs straight into the no-window row. It also needs a help-row placement, and it has to pick the window origin it records.

@history_item n=3 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — `~ contract/viola-revive/usage` (revive gains `--model` / `--effort`, which conflict with `--list`; PROVISIONAL, W2) and `+ contract/run-revive-usage/pair-after-dashes`. This entry is the source of the launch pair recorded at every start. When `up` builds each revive argv it decides whether to replay `--model` / `--effort` and must avoid the pair-after-dashes refusal.

@history_item n=4 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (1 of 2) — `~ decision/board-ordered-by-name`: the board keeps its own name order with each wrapped child drawn under its parent, so a child sorting before its parent still comes after it. It also changed `~ contract/viola-list-board/single-write` and `column-widths`, plus `~ decision/mcp-hides-wheel-and-handoff-verbs` (PROVISIONAL, W1). This is the existing parent-before-child ordering from the lineage record. `up`'s order and its handling of a missing or cyclic parent should match it.

@history_item n=5 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (2 of 2) — `+ contract/viola-list-board/tree`: a row whose parent has no wrapped row is a root, and cycle members are drawn once, from the first in order. This is the standing answer to the scope's premise on a missing, never-live or cyclic parent, which `up`'s ordering should reuse rather than reinvent.

@history_item n=6 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `~ contract/viola-run/start-output`: before the child, only a key-on start's automatic verify line and verify's own output reach the terminal (discarded under `revive --json`). This is a trap for `up`: each revive start it fires can print verify output, which affects the one-line-per-instance output and the `--json` purity.

@history_item n=7 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (the ratification of 2026-11-20-a-new-cli-version-stamped-automatically) — the `contract/viola-run/start-output` promise now stands on the founder's own word, no longer PROVISIONAL. `up` rests on this row as settled.

@history_item n=8 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ contract/cli-verbs/run`, `~ contract/viola-run/start-output` and `~ contract/cli-verbs/revive`: revive writes the trust record keyed by the recorded working directory and adds no refusal, exit code, hint or byte. `up` resumes "in its recorded workspace" through revive, so it inherits this trust step and its recorded-directory key.

@history_item n=9 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (decision / contract / absence) verified against the code, and the fates were set by the founder. This explains the form of every picked row and the rule that `up`'s amendments land as `+` / `~` row operations.

@history_item n=10 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0. Flat verbs: no nested hierarchy, no interactive prompt and no pager in a verb, and the terminal fleet console is no verb's output (`decision/flat-verbs`, `decision/no-prompt-or-pager`). It also made the web surface "viola renders no page" (`absence/web-surface`). So `up` stays one flat, non-interactive verb that prints lines, not a console.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — the `.cmd`/`.bat` refusal leaves revive (its start codes became `pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused`, `internal-error`). The entry also made per-verb `--help` usage name the program by its executable (`contract/viola-verb-help/usage`) and made Viola Linux-only. These are the current revive start codes that `up` would relay, and its usage follows the same per-verb help rule.

@history_item n=12 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory refusal of the `run` and `revive` starts) — exit 1 before any snapshot, child or bind, with `socket-dir-refused` added to revive's `--json` start codes (`contract/run-revive-start/socket-dir-refused`, `contract/run-revive-start-json/socket-dir-refused`). This is a per-instance start refusal that `up` can meet mid-fleet and must report in its line or document.

@history_item n=13 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (exit 21's sixth cause, `socket-dir-refused`) — the exit-21 `<cause>` list grew to six, and every exit-21 cause has its own hint. This list is where `up` would add an exit-21 cause if it gets one (`contract/instance-unreachable-detail/*`, `decision/hint-for-every-cause`).

@history_item n=14 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document carries its cause in `detail`) — `viola list --json`'s and plugin install's strict-modes refusals became `"detail":"strict-modes-failed"`, while exit 1's documents keep `"detail":null`. These are the precedent rows for `up`'s `--json` refusal of a refused home or instances/ (`contract/viola-list-json/strict-modes-failed`, `contract/viola-plugin-install/strict-modes-refused`).

@history_item n=15 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's `internal-error` document under `--json`) — with `--json` before a `--`, the catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, and a revive start stays outside the catch site. This is the internal-error form `up` takes (`contract/cli-catch-site/internal-error`, `contract/cli-internal-error/json`). It also leaves open how `up`'s own catch site relates to the revive starts it drives.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder's code under `viola revive --json`) — `already-live` covers a live holder or a fresh replayed beat, and `stale-heartbeat` covers a stale holder (`contract/viola-revive-json/already-live`, `stale-heartbeat`). `up`'s "skip an instance still live" sits right on this distinction, and a stale holder is neither skipped nor cleanly revivable.

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `viola revive --json` post-preflight gap closed) — a start refusal that revive shares with run prints one exit-1 document; a `pump_child` error prints no document because the child has held the terminal, and a document from `main` was rejected for that reason. This is a trap for `up`: a resumed child holding the terminal breaks one-document `--json`, which bears on whether `up` drives revive in-process or spawns it in a window.

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the unwrapped-name and endpoint-lost hints) — each new cause got its own hint as the last stderr line. This is a precedent for `decision/hint-for-every-cause` and `decision/hint-last-on-stderr`: any refusal cause `up` adds needs its own hint.

@history_item n=19 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder ratified the PROVISIONAL records of the four 2026-10-31-exit-cause-code-catalogue entries. The exit-21 detail, the catch-site document and the revive `--json` codes that `up` rests on are therefore settled.

@history_item n=20 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of verify and the run and revive starts) — revive's start checks the home, `ledger/`, `bin/`, `plugin/` and the pinned copy before its log opens: the home pair for a refused tree, the state-files pair for a refused instance, exit 1, with `strict-modes-failed` under `--json` (`contract/run-revive-start/home-tree-refused`). A refused check writes no line. `up` re-opens through this start, so the home-tree refusal reaches it, and it has to choose between checking once up front or per instance.

@history_item n=21 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six frame verbs' exit-21 instance pair) — the strict-modes refusal is made before any role file or log opens, the directory's mode is left as found, and exit 21 carries the fixed state pair (`contract/cli-exit-codes/21`). This is the strict-modes shape `up`'s board read must follow.

@history_item n=22 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the strict-modes refusal of the driven instance) — exit 21 as list and plugin install use it, not revive's exit 1, was the founder's choice. The trap: opening a role file narrows an existing home to 0700, so a check placed after `client::start` passes a 0770 home. Both bear on `up`: it must choose exit 21 or exit 1 for its strict-modes refusal, and it must put the check before any log opens.

@history_item n=23 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the `handoff:  link, unlink` help row built) — the help table reached five rows with served verbs only, pinned by `help_lists_only_served_verbs` and the `HELP` literal (`contract/viola-help/verb-groups`). This is the precedent for how a new verb joins a group row and its tests.

@history_item n=24 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink` join the exit-21 `unable` lines, the server check and the catch site) — new verbs were added to the `error: internal error` catch-site verb list and the shared `unable` word column. `up` joins both lists in the same way (`contract/cli-catch-site/internal-error`, `decision/shared-refusal-word-column`).

@history_item n=25 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — the exit-21 hint is keyed by cause, with the not-running hint only for an instance not running. This is a precedent for `decision/hint-for-every-cause` that `up`'s refusals must respect.

@history_item n=26 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch ratified served-verbs-only `--help` and the board's width from `host_size()`. `up` therefore appears in `--help` only once it is served, and any width-bound line follows `decision/truncate-only-unwrapped-names`.

@history_item n=27 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola --help` was built as the grouped verb table (served verbs only, the `HELP` literal, no `Commands:` list, per-verb usage left to clap). It also defined the styled versus uncoloured conditions (`--json`, a pipe, `NO_COLOR`, `TERM=dumb`). `up`'s help placement and its per-instance lines follow these (`contract/viola-help/verb-groups`, `decision/plain-output-modes`).

@history_item n=28 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is hidden: its tools are the driver's machine view of the verbs, and no tool names release, pause, link or unlink. The scope keeps `up` CLI-only with no MCP tool, so this entry marks where a tool would have gone and why `up` does not add one.

@history_item n=29 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — plugin install joined the `setup:` row and got its exit-21 home pair plus a catch-site entry, with `--json` refusals (`contract/viola-plugin-install/strict-modes-refused`, `internal-error`). This is the closest model for a board-scope verb's strict-modes refusal and internal error, which `up` would copy.

@history_item n=30 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — revive gained `--json` (refusal codes, one document, exit 1, stderr empty), and `--json` became the machine view of every data verb (`decision/json-on-every-data-verb`, `contract/viola-revive-json/refusal-document`). `up` must offer `--json` under the same one-document rule.

@history_item n=31 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — revive's fifth pair (session open in another claude) and its fixed preflight order: holder, logged session, `session-live` (one `claude agents --json` read, skipped under `--fork`, a failed read passes), then the recorded directory (`contract/viola-revive/preflight-order`, `session-live-check`, `contract/viola-revive-refusals/session-open-elsewhere`). These are the per-instance outcomes `up` meets when it drives revive, and a source for its closed outcome words.

@history_item n=32 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder ratified the list refusal pairs, `list --json`'s exit-21 refusal with no log opened, and the exit 1 for an unlistable `instances/`. These are the board-read refusals `up` inherits when it scans the instances.

@history_item n=33 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — viola list as built: exit 21 on a refused home, instances/ or instance, with no log and the mode left as found; exit 1 at the catch site for an unlistable `instances/` or a failed log start; no row for a per-instance read error (`contract/viola-list/strict-modes-refused`, `contract/viola-list-json/strict-modes-failed`). These are the precedents for `up`'s whole-board refusal versus a per-instance skip.

@history_item n=34 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — the revive output structure: a passed start prints nothing of its own while the child holds the terminal, each refusal is exit 1 with an `unable:` / `hint:` pair last on stderr, the five pairs, and revive in the `setup:` group (`contract/viola-revive/*`, `contract/viola-revive-refusals/*`). This is the base for `up`, which re-opens each instance "through the existing revive path".

@history_item n=35 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — every `cli` verb prints exactly `error: internal error` once from the one catch site, and verbs join the shared `unable` word column. This is the origin of `contract/cli-internal-error/human` and `decision/shared-refusal-word-column`, which `up` joins.

@history_item n=36 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1 of 2) — `~ contract/cli-exit-codes/11`: exit 11 is retired and never reused, and the list payload lost its `budget` member (`~ contract/viola-list-json/document`). `up`'s exit codes must not reuse 11.

@history_item n=37 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the web surface was cut and the cli surface stands alone; `list --json` shapes are the list payload contract. The `absence/web-surface` lineage: `up` has no web counterpart, and its `--json` sits beside the CLI machine contract only.
