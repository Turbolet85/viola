@history_item n=1 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — made `viola mcp` a hidden verb with no `--help` row (so the `HELP` literal stays unchanged) and gave it five tools that are a driver's machine view of the CLI verbs. No tool, description or result names `release`, `pause`, `link` or `unlink`. This is the source of the rule behind v1-13's unprovable clause (an MCP result for `link` and the budget release). The disposition must be a deferral or a re-worded acceptance, or a design change on the founder's word.

@history_item n=2 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the `link` / `unlink` result lines as built) — states that `link` / `unlink` each print one plain stdout line (`link    <driver> -> <driven>  since …`, `already linked`, `unlink  <driver> -> <driven>`) and the `{"v":1,"ok":…}` document. This is where links show only as verb output, never as a board column. It is the built CLI half of v1-36 and v1-13 that item 4 cites, and the base for re-wording v1-36's «link set on the board» clause.

@history_item n=3 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the `handoff:  link, unlink` help row built) — took `viola --help` to five rows (board, traffic, wheel, handoff, setup), with `handoff:` between `wheel:` and `setup:`. Pinned by `help_is_the_grouped_verb_table`, `help_lists_only_served_verbs` and the `HELP` literal. This is the five-row literal that item 17's remnant removal in `tests/cli_output_plain.rs` must leave byte-identical.

@history_item n=4 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink` join the exit-21 `unable` lines, the server check and the catch site; the strict-modes refusal of the driven instance) — the frame verbs became eight, and `link` / `unlink` gained exit-21 refusals and catch-site exit 1. The CLI halves of v1-13 and v1-36 are built and tested, so only the board and MCP clauses are open.

@history_item n=5 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built `viola --help` as the grouped table over served verbs only, with the override usage line and the `HELP` literal pin. Styled `viola list` on a terminal, with the width read from `viola_pty::host_size()` and never from `COLUMNS`. With no width, nothing is truncated. Item 12's `portable-pty` bump must keep this width source. The bump is the `crates/viola-pty` binding.

@history_item n=6 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified served-verbs-only `--help` and the board's width from `host_size()`, removing their PROVISIONAL marks. Changing either rule is now a change to his own word, not to a delegate's lean. This is also a precedent for how a batch of founder words discharges the history's provisional marks without pruning any entry.

@history_item n=7 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — put `plugin install` in the `setup:` row as one entry (a bare `plugin` is unserved), read by `help_lists_only_served_verbs`. The acceptance «`plugin install` as one `setup:` entry» after item 17's edits rests on this.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the per-OS wording of help and plugin install) — dropped «on every OS» from the usage-line override and `viola.exe` from the per-verb usage, and made plugin install's `<dir>` quoting Linux-only. This is the precedent for item 17's remaining Windows sites (`tests/cli_output_plain.rs:104,116`): re-word the master with its claim kept, and leave the `HELP` literal untouched.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows VT enable leaves the cli colour surface) — removed the failed-VT-enable rung from `viola list`'s uncoloured conditions and `windows-sys` from the Tooling context, keeping Linux output byte-identical. This is the template for item 17's `plain_drive` / `src/cmd/mod.rs` remnant: remove it, and have the master follow, with the plain-output checks unchanged.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the .cmd/.bat refusal leaves revive and verify; the server-check hint without its OS scope; verify's relied count and stop on one OS) — removed Windows-only words and arms because «a closed word no build can emit leaves with its emitter». The «on Unix» socket-directory qualifiers were kept as true on Linux. This is the precedent and the limit for item 17's «removed or kept with its reason, the masters following».

@history_item n=11 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board as built: six columns with fixed widths (NAME, LIVE 5, STATUS 7, WHEEL 23, DIALOG 17, CLI unpadded), wrapped rows only, `escape_row`. There is no link column. The acceptance `viola list | cat` keeps the six captions `NAME LIVE STATUS WHEEL DIALOG CLI` rests on this, as does the anti-pattern against adding a column for v1-36.

@history_item n=12 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder confirmed WHEEL 23, the board's planning forms and its refusals, and `release --budget`'s two lines and date form. The board's columns are his ruled design, so adding a link column for v1-36 is a design change on his word.

@history_item n=13 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `viola list` (piped) gained the `-- UNWRAPPED - READ-ONLY --` separator and the unwrapped rows, with `n/a` cells, within the same six columns. `viola list | cat` in the live session may show this rack. The check must expect it, and it adds no column.

@history_item n=14 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — cut Surface: web-spa to a stub (the frontend is deferred to 0.2.0) and fixed the six captions, words and slot order as the shared vocabulary that a future page reuses. This bears on the scope's boundary that caps naming a frontend surface are not provable in 0.1.0, and on why the board's captions cannot grow.

@history_item n=15 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — made `--json` the machine view of every data verb, `viola revive` included. This backs the IA rule that new fields go into `--json` first. If a v1-36 re-wording needs a link set somewhere readable, the machine view, not a human column, is the place the master allows.

@history_item n=16 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — states `viola release <name> --budget`'s two stdout lines (`budget gate lifted until <end>` / `budget gate open`). This is the built CLI half of v1-13's budget release. The MCP half stays unprovable while no tool names `release`.

@history_item n=17 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a start that passes its preflight prints nothing of its own while the child holds the terminal, and refusals are an `unable:` / `hint:` pair. The 0.0 expression list names the `viola run` / `viola revive` passthrough. The acceptance that `viola run` prints nothing of its own in the live overseer-drives-builder session (and v1-53's live `viola run`) rests on this.

@history_item n=18 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — `[RB] read back …` prints when the send is confirmed, by the matching `prompt-submitted` or, for `/clear`, by its post-condition. This defines what the live session's confirmed `viola send` must end with on stdout, exit 0.

@history_item n=19 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` is read back by its compacted-session post-condition inside its 40 s PROVISIONAL window. `[  ] unconfirmable …` prints for `/remote-control` and for an unverified CLI. If the live driver session sends a local command, these readback forms apply.

@history_item n=20 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the `input-not-ready` hint reads `builder was not ready for input; send again, and if it repeats a human must look at the session` (the founder's wording). This is the precedent from the first live self-drive, and the refusal a live overseer-drives-builder session can meet.

@history_item n=21 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — `wait` / `last` print one result line, with `waiting:` only on a terminal stderr without `--json`. It also added the exit-21 pair and the one catch site. This is the pattern for the driver's `wait` in the live session, and for keeping the stdout and stderr streams apart.

@history_item n=22 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the `viola wait` wireframe gained `dialog-expired  <name>  dialog <id>  cursor <n>`, printed when a dialog hold expires. Item 2's live read of a hold near the 3600 s ceiling would end on this line. Its wording was ratified at 2026-10-20-t06-58-31-wrap.

@history_item n=23 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory refusal of the `run` and `revive` starts; exit 21's sixth cause, `socket-dir-refused`) — the socket-directory pair's wording is a planning form on the overseer's P5 word as delegate, PROVISIONAL, with the founder's wording owed at the Epoch 7 boundary batch. This is exactly item 7's first owed word. When he gives it, this history's provisional record is superseded (as at 2026-11-01-t02-46-44-wrap), and the pair's text is otherwise unchanged.

@history_item n=24 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`server-verify-failed` reaches macOS, as an euid mismatch alone) — `connect` reads `peer_admitted(client.server_euid())` ahead of `verify_server`, and a refused euid reads `server-verify-failed`. This is the client seam behind item 8's mutation survivor (always-admit `peer_admitted`). The confirm was PROVISIONAL, delegate the overseer, owed in the Epoch 7 batch.

@history_item n=25 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — superseded the Epoch 6 provisional records in this history with the founder's own word, listing each heading in `marks-discharged.md`, with none pruned and no body mark. This is the form for discharging item 7's Epoch 7 PROVISIONAL marks in this master's history.
