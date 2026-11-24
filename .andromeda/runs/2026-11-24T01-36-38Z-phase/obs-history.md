@history_item n=1 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — (6. Log Coverage) `~ contract/process-exit-detail/exit-1` gained `terminal-not-found` and `not-live` from `viola open`, and `+ contract/diag-subject/terminal` joined the closed subject enum. This is the most recent precedent for a Hyprland-facing cli verb. Its exit-1 words went into the closed `process-exit` `detail` enum, and the subject enum holds one row per word. Any refusal word `viola focus` adds (no window, unsupported outside Hyprland) has to go into `schemas/diag-line.v1.json` and get a row in the same way.

@history_item n=2 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — (4. Span / Trace Coverage) Added `run.window` to the start span chain, which runs only under `--origin`. It also added `+ contract/cli-log-file/open`: `viola open` logs as cli into the opened instance's `cli-<name>.ndjson` inside the `cli.open` dispatch span. And it added `+ decision/window-line-untraced`. These form the template for `viola focus`'s log file and its `cli.focus` dispatch span. They also show where the window record that focus reads (arm a) is written, and that the window line has no trace of its own.

@history_item n=3 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — (1. Obs Scope Summary) `~ contract/process-log/child-and-probe-lifecycle` added the terminal `viola open` spawns. `+ contract/run-start-events/window-line` added the `window {address, workspace, placement, origin}` line, written with source wrapper after launch under `--origin` only. This is the recorded identity source focus reads. It also confirms that a hand-started `viola run` writes no window line, so the ancestry arm is needed.

@history_item n=4 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — (2. Telemetry Strategy) `~ contract/strict-modes-refusal-exit/1` added `open` to the exit-1 strict-modes refusals, with the `strict-modes-failed` document under `--json`. This is the exit-1 versus exit-21 fork focus must settle for its own pre-log strict-modes refusal. `open` took exit 1; the frame verbs and `link` / `unlink` take 21.

@history_item n=5 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (6. Log Coverage) `+ contract/diag-subject/hyprctl-probe` and `+ contract/parse-rejected/hyprctl-json` were added. `~ contract/diag-line-fields/parse-rejected` and `~ contract/parse-rejected-detail/oversize` now include `hyprctl-json` past `MAX_FRAME`. The seam already writes a `hyprctl-probe` pair around each hyprctl spawn and one `parse-rejected` per bad reply. Focus's dispatches and client-list read go through the same seam and inherit these lines; focus must not duplicate them.

@history_item n=6 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (1. Obs Scope Summary, 3 rows) `~ decision/per-parser-parse-failure-counts` now names the hyprctl replies. `~ decision/instrument-call-sites-not-os-internals` now names the hyprctl spawn. `~ decision/parse-failures-surface-as-unknown` gained an except: a hyprctl reply failing its parse reads `oversize` or `malformed`, never `unknown`. Focus's window lookup over the compositor client list must surface a bad reply as that closed error, not as `unknown`.

@history_item n=7 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (1. Obs Scope Summary, 2 rows) `~ decision/os-apis-at-call-sites` now covers the hyprctl spawn, instrumented at viola's call sites only. `~ decision/parse-failures-unknown-counter` got the same hyprctl except. These are the rules focus's dispatch path rests on.

@history_item n=8 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (2. Telemetry Strategy) `~ decision/span-naming` added `hyprland` to the closed `<area>` list, for the seam's manual `skip_all` span `hyprland.read`. Focus's new span names, such as `cli.focus` and any new `hyprland.*` dispatch span, must fit this closed list. A new area would need an amendment of the row.

@history_item n=9 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — (1. Obs Scope Summary, `decision/parse-failures-unknown-counter`) Widened the except to a second case, an outside StopFailure word read as `api-error`. This is the current wording of a picked row that focus's hyprctl parse handling must respect.

@history_item n=10 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — (1. Obs Scope Summary, `decision/parse-failures-surface-as-unknown`) Applied the same widened except to the twin row. Both twins must stay in step if focus touches parse-failure handling.

@history_item n=11 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (2. Telemetry Strategy) `~ decision/span-naming` removed `statusline` from the closed area list, giving its current form: run, pty, channel, state, hook, send, wait, last, answer, pause, release, cli, mcp, hyprland. This is the list focus's spans must fit.

@history_item n=12 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (1. Obs Scope Summary) `~ contract/replayed-snapshot/live-fields` now reads: a replayed snapshot carries no endpoint, pid, started_at or statusline_command. Focus reads instance state through the snapshot. A replayed snapshot has no `pid`, so the ancestry arm's starting pid (premise P3) may be missing.

@history_item n=13 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (4. Span / Trace Coverage, 5 rows) `~ decision/no-endpoint-over-replayed-snapshot` now says a replayed or absent snapshot read gives no endpoint and sends no frame. This bounds what focus may do with a replayed snapshot.

@history_item n=14 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (4. Span / Trace Coverage, 7 rows, 2 of 3) `~ contract/span-kinds/client` dropped `statusline.shell_out`, leaving the current closed CLIENT span list. If focus adds a CLIENT-kind span (a dispatch to hyprctl), this list grows.

@history_item n=15 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (6. Log Coverage, 6 rows) `~ contract/diag-line-fields/process-start` and `~ contract/diag-line-fields/process-exit` dropped `statusline-shell` and `shell_exit_status`. A separate one-row entry changed `contract/child-spawn-log/subject` to child spawns only. These are the current field sets focus's lines must validate against.

@history_item n=16 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — (6. Log Coverage, `contract/diag-line-fields/process-start`) Rewrote `subject` as «one word of the diag-line subject enum» instead of a list. The enum is held one row per word (`contract/diag-subject/*`). A new subject word for focus needs its own row, but no edit to the process-start row.

@history_item n=17 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — (1. Obs Scope Summary) `~ contract/process-log/child-and-probe-lifecycle` named verify's print-mode sender. This is the lifecycle row `viola open` later extended with the terminal. Any process focus spawns (hyprctl via the seam) must be covered there or by the seam's `hyprctl-probe` pair.

@history_item n=18 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — (6. Log Coverage) `+ contract/diag-subject/auto-verify` set the pattern of a new closed subject word added with its emitter. The same route applies if focus logs a spawn of its own.

@history_item n=19 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — (1. Obs Scope Summary) `~ contract/mcp-iserror/instance-unreachable` brought the closed cause set to seven with `call-deadline`. The scope keeps focus CLI-only; an MCP `focus` tool would be a boundary widening, PROVISIONAL. If one is ruled in, its rejects follow this closed cause set and write no line.

@history_item n=20 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the `cli` catch site's document under `--json`) Under `--json` the cli catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout, with stderr empty and exit 1. `json_asked` reads `--json` before `--`. Focus inherits this catch-site behaviour as a cli verb (`contract/cli-internal-error/*`).

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (exit 21's document names its cause) Exit 21 prints one document per cause, with the `UnreachableCause` closed words. A refused instance check reads `strict-modes-failed` with no line. If focus refuses on instance reachability with exit 21, it uses this document and its closed words. A new cause word widens the enum.

@history_item n=22 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (Founder Direction 4's codes and named carve-outs) Every exit-21 and exit-1 cause gets a distinct `detail` code with no path or pid. The one exception is a strict-modes check refused before any log opens. This is the origin of `decision/distinct-detail-code-per-exit-cause`: each new focus refusal cause (no window, unsupported, a gone recorded window) needs its own code.

@history_item n=23 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (`verify` writes its refusal's code) `strict-modes-failed` has a log line only on `verify`'s refused `update_stamps` and on run's `ledger-stamps` read. Every other refused strict-modes check writes none. This is the origin of `decision/strict-modes-failure-lines` and `contract/verify-process-exit/strict-modes-failed`. Focus's refused check falls on the no-line side.

@history_item n=24 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (exit 21's closed causes, five to six) Added `socket-dir-refused` as the sixth `UnreachableCause`. This is the current exit-21 detail set (`contract/exit-21-document/detail`) that focus would pick from or extend.

@history_item n=25 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`Refused::NotADirectory`'s fixed message) Set the fixed, pathless refusal message «a trusted state path is not a directory». This is the row `decision/not-a-directory-refusal-message-fixed`. Focus's strict-modes read of instance state reuses these fixed messages.

@history_item n=26 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the refusal-telemetry record) A strict-modes failure has a line in only two places; every other refused check writes none. Founder Direction 4 excepts strict-modes refusals made before any log opens (exit 21 at frame verbs and link / unlink, exit 1 at run, revive and verify). Focus's pre-log check joins this no-line set.

@history_item n=27 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the pre-init window and the panic exemptions) Every role that checks the home runs that check before its first open, because the open narrows an existing home to 0700. A refused check writes no line, and the panic exemptions name such sites. Focus must run its check before `cli-<name>.ndjson` opens and must be added to the pre-init window and exemption lists.

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the snapshot readers over a replayed snapshot) A replayed snapshot carries no endpoint, pid, started_at or statusline_command. Readers over it treat the instance as unreachable. This is the origin of `contract/replayed-snapshot/live-fields` and `decision/replayed-snapshot-unreachable`. Focus's snapshot read, including the window record and the ancestry starting pid, must handle the replayed case.

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (E5: the replay's product readers two to six) Every product reader uses `read_snapshot_or_replay`, and none uses `read_snapshot` raw. Each writes one `state-recovered` per call. This is the origin of `decision/no-raw-snapshot-reads`. As a new reader, focus writes `state-recovered{snapshot-replayed|snapshot-unsupported-v}` into its cli file.

@history_item n=30 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the `diagnostics/` opens behind `check_diagnostics`) A role or detail file opens only after `check_diagnostics`, and a refusal counts as a failed open. This is the origin of `decision/diagnostics-checked-at-open` and `decision/refused-diagnostics-drops-line`, which focus's cli log open inherits.

@history_item n=31 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (the link verbs' instance check before their log) `check_instance` runs before `client::start`. A refusal is exit 21 with no log and no line, held by a mode-reading test that reads red with the check moved. This is the order trap and precedent for a cli verb acting on a named instance; focus faces the same order.

@history_item n=32 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (the link verbs log into the driven instance's `cli` file) The verb's log goes to the instance its argument names, never the caller's `VIOLA_NAME`, and its request runs inside `cli.link` / `cli.unlink`. This is the origin of `contract/cli-log-file/link-unlink`. Focus takes `<name>` the same way (`decision/instance-never-from-working-directory`).

@history_item n=33 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — (viola list's lines, its no-line refusal and the replay's second caller) `cli.list` opens `cli-<name>.ndjson` only after its scan passes, and a refused scan writes no line. `state-recovered` from a board read lands in the cli file. This is the precedent for a cli verb reading instance state, and it set the cli-log-only-under-a-name rule focus's logging follows.

@history_item n=34 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — (revive --list logs as cli; state-recovered names the scanned instance) `revive --list` logs as `cli` after its check passes, and gets no log without a name. `state-recovered` gained `scanned`. This is the origin of `contract/cli-log-file/revive-list` and of the `scanned` field on `contract/diag-line-fields/state-recovered`, both relevant to focus's state read.

@history_item n=35 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — (plugin install's span, log shape and no-line refusal) `cli.plugin_install` is not a dispatch span. The self pair goes only under a parsed name after the check. A refused check is exit 21 with no line. This is the origin of `contract/plugin-install-exit-codes/21` and a precedent for where focus's dispatch span opens relative to its check.

@history_item n=36 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — (the founder's Epoch 5 batch) The founder accepted as his own every refused check's no-line shape (revive, list's scan, plugin install, and others). This is why `decision/refused-check-writes-no-line` and `decision/strict-modes-refusal-writes-no-line` stand ratified. Focus's refused check can follow them without a new ruling.

@history_item n=37 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the OS target, Linux only) The OS target is Linux only, with CI on `ubuntu-latest` (`decision/linux-only-os-target`). Focus needs no per-OS arm, and its tests run on the one leg.

@history_item n=38 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the closed words that left `diag-line.v1.json` with their emitter) Set the rule that a closed word no build can emit leaves the schema with its emitter, with `os` closed to `linux` (`decision/unemittable-schema-words-removed`, `contract/diag-line-schema/os`). Focus's new closed words enter the schema with their emitter.

@history_item n=39 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the `.cmd`/`.bat` start refusal retired; the refusal writer's site counts) This is the latest stated census of `human::refuse(` call sites (run 7, verify 5, and the rest), re-derived by `grep -rcE 'human::refuse\(' src`. If focus refuses through `human::refuse`, the census moves and must be re-derived by its rule, not copied.

@history_item n=40 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1. Obs Scope Summary, `decision/diag-schema-additive-only`) Narrowed the rule to «must not rename or remove a field the product still emits». This is the current form of the schema rule focus's additions must meet: add fields or event values through a Decisions Log entry.

@history_item n=41 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (6. Log Coverage, `decision/log-schema-additive-only`) Narrowed to «removes only a field no build emits». This is the twin of the schema rule above, and it binds any change to the log schema that focus makes.

@history_item n=42 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1. Obs Scope Summary, 6 rows) `~ decision/exit-code-log-field` set the exit codes to 0, 1, 2, 10, 12–14, 20 and 21, with 11 retired and never reused. A new focus exit code, or a cause mapped to an existing one, must avoid 11.

@history_item n=43 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the thiserror derive census on the product sources) The census reads 9 `thiserror::Error` derives over the product sources, re-measured by grep. A new typed focus error with a fixed Display (`decision/typed-errors-fixed-display`) moves that census, which must be re-counted.

@history_item n=44 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (the panic backtrace as raw frames; the `cli` role's internal-error line) After a panic, cli writes `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` when an instance resolves, keeps the chain in the detail file only, and prints exactly `error: internal error`. This is the origin of `contract/panic-outcome-by-role/cli`, which focus inherits.

@history_item n=45 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — (the `ui` process and four events leave) Set the process roles to `run|hook|mcp|cli` (four) and the role files to `run-<name>`, `hook-<name>`, `mcp` and `cli-<name>`. Focus is a `cli` role and adds no process role (`contract/process-log/process`).

@history_item n=46 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (version census 0.2.0) There is no frontend scenario because viola has no web or graphical frontend. The terminal fleet console tiled on Hyprland replaces it, and its scenarios are owed by «Console scaffold». This is the context for `absence/web-or-graphical-frontend`: focus is a CLI verb, not a GUI surface.

@history_item n=47 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — (4. Span / Trace Coverage) `+ decision/lineage-line-untraced` and the start product order gaining lineage set the precedent for product lines kept untraced. The window line (`decision/window-line-untraced`) followed it. If focus reads or writes a product line, the same untraced-by-rule choice applies.

@history_item n=48 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — (4. Span / Trace Coverage) `+ decision/launch-line-untraced` is the sibling rule the window line was declared «kept untraced as the launch line» against. It names the chain of untraced start lines that focus's identity read consumes.
