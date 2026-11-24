@history_item n=1 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — landed `revive`'s exit-1 fixed `unable:` / `hint:` pairs read in a fixed first-refusal order, none showing a path, pid, recorded directory or logged id, and no hint naming `viola release`. It put `revive` in the `setup:` help group and made a passed `revive` print nothing once the child starts. A restart-based switch that rides `revive`'s start (scope premise fct-fweyeqnp) inherits all of this. It is the precedent for decision/start-refusals-show-no-path-or-pid, decision/run-prints-nothing-while-wrapped and decision/no-hint-names-release.

@history_item n=2 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (pattern 2) — added `revive`'s `session-live` pair (`<name>'s session is open in another claude` → `close it there first, or start a copy: viola revive <name> --fork`), read after the logged session. A switch that ends and resumes a session through `revive` meets this refusal and the fixed order it sits in.

@history_item n=3 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `revive` its `--json` exit-1 documents `{"v":1,"error":"<code>","detail":null}`, no hint, stderr empty; every other exit 1 had no document. This is the precedent for how a new verb's exit-1 refusals get a machine form (contract/viola-revive-output/json-refusal, decision/json-one-document-on-stdout).

@history_item n=4 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola --help` lists the served verbs only, in groups. The width comes from `viola_pty::host_size()`, and `NO_COLOR` is read as present even when empty (contract/cli-colour-decision-order/no-color). A new verb joins the help only once it is served, and its group row must change (contract/help-output/verb-groups, contract/viola-help/verb-groups).

@history_item n=5 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified the width source and "served verbs only" as built, removing the delegate's PROVISIONAL marks. This is the standing basis for how the help groups grow with each new verb.

@history_item n=6 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap is built without its `color` feature, so help and usage output is plain in every mode and every styled byte is viola's own SGR module's (decision/clap-without-colour-feature, decision/hand-written-sgr-module). A new verb's clap args and usage text inherit this; no dependency may reintroduce colour.

@history_item n=7 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is a hidden verb (`#[command(hide = true)]`) that appears in no `--help` row. This bears on the scope's out-of-bounds MCP tool for the switch (fct-lucgxm5j): an MCP arm would add a tool there, not a help row.

@history_item n=8 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (strict-modes pair; eight frame verbs; `--json` and exit 20; `handoff:` row) — `link` / `unlink` refuse a failing driven instance with the fixed instance-state pair at exit 21 and join the fixed-form lists. The server-verification cause and hint grew from six to eight frame verbs. Their `--json` ok documents are `{"v":1,"ok":{…}}`, and a wrapper `-32602` refusal reads exit 20 `wrapper fault`. A help row was added when the verbs were served. This is the closest precedent for a new frame-writing verb against a running instance: every closed list of frame verbs (six or eight) must be widened if the switch writes a frame.

@history_item n=9 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (six frame verbs' pair; run/revive home-tree check; home under a regular file) — the six frame verbs run the instance strict-modes check before any frame and print the instance-state pair at exit 21 with no line written. `run`'s and `revive`'s start check the home tree before any log (home pair or instance pair, exit 1; under `revive --json`, `strict-modes-failed`). A home under a regular file is a strict-modes refusal. Rows contract/strict-modes-refusal/*, contract/frame-verb-refusals/strict-modes, contract/viola-run-start-refusals/home-tree-check and contract/viola-revive-refusals/home-tree-check stand on it. The switch verb must pick which of these orders and pairs it follows.

@history_item n=10 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (code word; `unknown`; exit 20 closed detail; exit 2 lines; json failures witness) — both clients screen a peer's `refusal` / `detail` through `viola_core::is_code_word`. An unlisted code word exits 14 as `unknown` with no hint. Exit 20's `--json` detail is closed by `viola_channel::sanitised_fault`. Exit-2 paths print a fixed stderr line and no document, whatever `--json` says. Each `--json` failure is one clean document (guard `tests/cli_error_bodies.rs`). A new wrapper refusal word for the switch must be a code word and join the closed refusal list, or it reads `unknown`. A new exit-2 usage refusal follows the fixed-line form.

@history_item n=11 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21 `detail`; `endpoint-lost` hint; stale-heartbeat; D-20 codes; catch-site `internal-error`; `revive --json` codes; no-endpoint reading) — exit 21's document names one closed cause, with one hint per cause. A frame verb with no live endpoint reads `instance-dead` or `unwrapped-name` by its instance directory, and a lost call reads `endpoint-lost`. The catch site prints the `internal-error` document under `--json` and counts `--json` only before `--`. `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site. `revive --json` holds closed codes for every refusal of the start it shares with `run`, and a `pump_child` error prints no document. A switch, whether a frame verb or a `revive` restart, reuses these closed sets. Any new word widens a closed set, which is a founder-ruled boundary.

@history_item n=12 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (board's home rule; socket directory at frame verbs; run/revive socket-dir pair; `socket-dir-refused`; euid; `revive --json` code; refused-peer drop) — added the socket-directory refusal at the eight frame verbs (exit 21, `socket-dir-refused`) and at `run`'s / `revive`'s start (exit 1, the fixed pair). Exit 21 gained its sixth cause, and `revive --json` its socket-dir code. The server check now also compares the peer euid, the wrapper drops a refused peer silently, and the board holds the home to the read rule. These are the refusal paths a new frame verb or restart path inherits (contract/frame-verb-refusals/socket-dir, contract/wrapper-socket/refused-peer, contract/viola-list-board/strict-modes-scan).

@history_item n=13 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (`.cmd`/`.bat` retired; server check without the macOS carve-out; strict-modes without an OS arm) — `revive --json`'s closed codes are ten, and `batch-script-child` has left. The server check has no macOS arm, and a home under a regular file is a strict-modes refusal with no OS lead. This is the current closed code set a restart-based switch's `--json` would print.

@history_item n=14 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — contract/refusal-hints/refusal-line: `send` and `answer` print the same hint for the same reason, and an opaque or absent detail prints no hint. contract/exit-21-json-cause/unwrapped-name: a symlinked instance directory reads `strict-modes-failed`. A new verb's hints key by reason in the same way.

@history_item n=15 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Brand Identity; Iconography; Surface: cli) — `budget-paused` left decision/fixed-phraseology-vocabulary. Exit 11 is retired and never reused, and decision/no-hint-names-viola-release now names the human-typing refusal alone. A new verb must not reuse exit 11 and must keep to the fixed vocabulary. Its hints must never name `viola release`.

@history_item n=16 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — contract/cli-exit-codes/2 gained the fixed `PAIR_AFTER_DASHES` line: `run` and `revive` refuse a model/effort pair after `--` at exit 2, with no document. This is the direct predecessor of this chunk: the switch's pair arguments and their usage refusals extend this row and its closed fixed lines.

@history_item n=17 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (help, exit 1, fixed-unable-form, name register; viola-help twin, fixed-message twin) — a new verb added `name/command/open`, widened the `setup:` row, contract/cli-exit-codes/1 (its refusals and `--json` documents), and both twins of the fixed `unable:` form list. It also added closed-word rows one row an item. This is the full row set a new CLI verb touches.

@history_item n=18 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (help board row; fixed-form lists exit 1 and 21; exit-1 codes; exit-21 refusals; strict-modes-failed list) — `focus` widened both help twins, both fixed-form twins (its exit-1 and exit-21 lists), contract/cli-exit-codes/1 and contract/exit-21-json-cause/strict-modes-failed. It added contract/viola-focus-refusals/exit-21 and closed exit-1 code rows. The help group and the fork words stand PROVISIONAL on the delegate's word. This is the most recent precedent for a verb that acts on one named instance, refuses with exit 21 before touching it, and carries closed exit-1 codes.

@history_item n=19 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (help, list-is-the-board, fixed-form, outcome words, line order, dry run, boot-time-unread pair, not-live hint, not-taken) — `up` resumes many instances, so decision/list-is-the-board was reworded. It added closed outcome words, among them `session-live`, `no-session` and `cwd-missing`, which reuse `revive`'s preflight. It also widened the help, fixed-form and strict-modes-failed lists; most rows are PROVISIONAL on the delegate. The switch must record the new pair so that `up`'s resume replays it (scope fct-sn6muqua); `up`'s rows and outcome words are where that replay surfaces.

@history_item n=20 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (help, child-started, start-refusals-show-no-path-or-pid, fixed-form; cli-start-refusal, silent-catch-sites, 1-adopt-start, session-live; twins; run-prints-nothing) — a new start role that rides `run`'s start widened decision/start-refusals-show-no-path-or-pid, decision/silent-catch-sites, decision/run-prints-nothing-while-wrapped, contract/cli-catch-site/silent-verbs and decision/no-json-after-child-held-terminal. contract/cli-exit-codes/1 had no room under its bound, so adopt's exit-1 refusals went into a row of their own (contract/cli-exit-codes/1-adopt-start). This is the trap and the precedent for a switch that restarts through `run`'s start: it widens these same lists, and needs its own exit-1 row.

@history_item n=21 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `run` and `revive` start print fixed automatic-verify stderr lines (contract/auto-verify-line/*), silent under `--json`. A restart-based switch through the shared start would print these lines too, so its output contract must account for them.

@history_item n=22 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback fills on a measured post-condition as well as on `prompt-submitted`; a local command without one stays open (`unconfirmable`). This bears on the "command typed into the live session" channel for a switch (fct-bzkj2tdh) and on decision/no-success-before-confirmation and decision/no-optimistic-send-confirmation: a switch must not announce success before it is read back.

@history_item n=23 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — contract/send-readback/unconfirmable lists the local commands with no post-condition. `verified` now needs a stamp that reads every relied row `pass`. A switch confirmed by a local command or a ledger row would extend this list and the relied set.

@history_item n=24 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (brand, surface and bans scoped to the CLI) — the spinner, progress-bar and live-redraw ban now reads "in a verb's output", and the console's redraw rule is «Console scaffold»'s. "No interactive prompt or pager" is scoped to a verb. The switch verb is bound by these (decision/no-spinners-in-verb-output, decision/no-interactive-prompt-or-pager); the console leg (v2-31) is out of scope.

@history_item n=25 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — spelled out message-mode escaping: every control character except `\n` and `\t` prints as `\xHH`, and `--json` stays serde-escaped (contract/cli-message-escaping/*). Any upstream-derived text the switch prints, such as a model word read back, would go through this escaper.

@history_item n=26 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — added `list`'s exit-21 whole-board strict-modes refusal in the fixed `unable:` / `hint:` form, extending the fixed-message list beyond the exit-1 start refusals. This is the origin of the closed fixed-form lists that each new verb has to widen since (decision/list-refuses-board-on-strict-modes-failure).
