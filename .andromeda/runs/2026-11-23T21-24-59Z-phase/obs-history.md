@history_item n=1 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (6. Log Coverage) added `+ contract/diag-subject/hyprctl-probe` and `+ contract/parse-rejected/hyprctl-json`, and widened `~ contract/diag-line-fields/process-start`, `~ contract/diag-line-fields/parse-rejected` and `~ contract/parse-rejected-detail/oversize` to name them. These are the seam's own lines, a `hyprctl-probe` process-start/process-exit pair around each hyprctl spawn and one codes-only `parse-rejected{parser:"hyprctl-json"}` for an oversize or malformed reply. This chunk makes the seam's first product caller, so the wrapper's `clients` polling and its one move dispatch must write exactly these lines, and no new subject or parser, into `run-<name>.ndjson`.

@history_item n=2 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (1. Obs Scope Summary, 3 rows) changed `~ decision/per-parser-parse-failure-counts` (hyprctl replies keep their own count), `~ decision/instrument-call-sites-not-os-internals` (the hyprctl spawn is instrumented at viola's call site) and `~ decision/parse-failures-surface-as-unknown` (a hyprctl parse failure reads the closed `oversize`/`malformed`, never `unknown`). Placement words (`unplaced`, the seam's `unsupported`/`unreachable`) must likewise stay closed words, never `unknown`. Instrumentation stays at the call site in `src/run/`, not inside the seam.

@history_item n=3 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (1. Obs Scope Summary, 2 rows) `~ decision/os-apis-at-call-sites` now names the hyprctl spawn among the call-site-only operations, and `~ decision/parse-failures-unknown-counter` gained the hyprctl exception. The foot spawn and the `clients` reads of this chunk fall under the same call-site rule.

@history_item n=4 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (2. Telemetry Strategy) added `hyprland` to the closed span-area list of `~ decision/span-naming`, for the manual `skip_all` span `hyprland.read`. Any span this chunk opens must use a listed area, for example `cli.open` as the verb's dispatch span or `hyprland.*`. A new area would need a row edit.

@history_item n=5 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (2. Telemetry Strategy) `~ decision/span-naming` dropped `statusline` from the closed span areas, which leaves the list the current `decision/span-naming` row holds. The entry also cut the shell spawn: `~ contract/child-spawn-log/subject` now reads "child spawns" only, because viola runs no shell. `open` must start `foot` directly with no shell, which is consistent with this.

@history_item n=6 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — (6. Log Coverage) added `+ contract/diag-subject/auto-verify` and put it in the process-start subject list. This is the precedent for a new child-spawn subject: a closed `subject` word joins `schemas/diag-line.v1.json` with its emitter and gets its own contract row. If `open`'s `foot` spawn is logged as a child pair, it needs its own subject word and row, unless it is logged by no pair by design and a row says so.

@history_item n=7 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — added `+ contract/diag-subject/verify-sender` and widened `~ contract/process-log/child-and-probe-lifecycle` to name the new sender. It also rewrote `~ contract/diag-line-fields/process-start` to name the subject "one word of the diag-line subject enum" instead of listing the words. A new spawn subject from this chunk therefore needs only its own subject row and the lifecycle row, not an edit to the process-start list.

@history_item n=8 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — every child spawn is logged as a process-start/process-exit pair at its call site, and "a new closed value takes a Decisions Log entry" (D-35). This is the constraint behind `decision/event-values-closed` and `decision/diag-schema-additive-only` for any subject, event or detail word `open` adds.

@history_item n=9 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (D-05 `mcp-stdin`) a new closed `parser` value takes a Decisions Log record, and that log now lives in this amendments file, with §12 of the body pointing here. Any new closed word for `open` (detail codes `terminal-not-found` and `not-live`, a subject, a parser) is recorded the same way.

@history_item n=10 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — `~ contract/run-start-events/order` gained the lineage line, `~ decision/product-order-from-events-log` gained lineage, and `+ decision/lineage-line-untraced` was added: a log-only start line that wakes no wait and writes no process-log line, span or terminal byte. The new log-only `window {address?, workspace?, placement, origin}` kind directly extends this register. Its place in the start order, and whether it is untraced like lineage and launch, must be stated in `contract/run-start-events/order`, in `decision/product-order-from-events-log` and in a sibling row.

@history_item n=11 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — (8. PII) added `+ decision/lineage-names-confined`: parent and role names appear only in `events.ndjson`, the snapshot, the board, `list --json` and the MCP `list` result. Each parent is re-parsed as a viola name, each role as a code word, and neither appears in any process-log or detail line. It also narrowed `~ decision/link-names-never-in-process-log`. `open`'s `--parent`/`--role` and the new `window?`/`origin?` fields on `list --json` and MCP `list` fall under this confinement. `open`'s own `cli` lines and the wrapper's lines must carry neither name.

@history_item n=12 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — `~ contract/run-start-events/order` now reads wheel, then lineage, then launch before session-start. `+ decision/launch-line-untraced` was added, PROVISIONAL. `~ decision/config-observed-at-load` adds one malformed line per refused `default_model`/`default_effort` value, and `~ contract/config-json/known-keys` now holds seven keys. `open --model/--effort` hands this pair on, and a `window` line would be the fifth start record, the next in this order register.

@history_item n=13 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ contract/run-start-events/order` adds `wheel{human-input}` before session-start when trust keys pass through. `~ decision/product-order-from-events-log` says the order is read from that start's own lines. `~ decision/events-stream-never-rotated` says a reused name keeps the earlier starts' lines at the head. A `window` line replayed "last line winning" must be read from the start's own lines, and because the log is never rewritten, an earlier start's window line stays in the log.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the snapshot readers over a replayed snapshot) a replayed `snapshot.json` carries no `endpoint`, `pid` or `started_at`. Over a replay, `live_endpoint` is `None`, and the collision check reads the heartbeat alone. These rows are now `contract/replayed-snapshot/live-fields`, `decision/replayed-snapshot-unreachable` and `contract/run-span-attributes/run-collision-check`. `open` waits on `live_endpoint` and reads `already-live`/`stale-heartbeat` before any spawn, and the new snapshot `window?` must state how it behaves under replay.

@history_item n=15 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (1. Obs Scope Summary) `~ contract/replayed-snapshot/live-fields` dropped its statusline clause. The row now lists what a replayed snapshot lacks and what each reader then does. It is the row to extend if `window?` survives replay, or is deliberately not live-only.

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (E5: replay's product readers two to six, `state.snapshot_recover` placed) `read_snapshot_or_replay` is the only product snapshot read, and each call writes one `state-recovered` line on a replay. This now stands as `decision/no-raw-snapshot-reads`. If `open` reads the parent's recorded window from the snapshot, it must use the replaying read and accept its `state-recovered` line into the caller's log.

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (a stale holder's code is `stale-heartbeat`) a `stale` holder reads `stale-heartbeat` and `already-live` means a live holder or a fresh replayed beat. Both are logged by `collision_check` as `process-exit{subject:"self", exit_code:1, detail}`, the rows `contract/run-start-failure-detail/*` and `contract/run-exit-1-detail/*`. `open` reports these two words "read before any spawn", so it must reuse the run words and add no new ones for the same cause.

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (Founder Direction 4's codes and named carve-outs) every exit-1 and exit-21 cause gets a distinct `detail` code with no path or pid, and a strict-modes refusal made before any log is the named exception. These are `decision/distinct-detail-code-per-exit-cause` and `decision/exit-1-detail-codes`. `open`'s new causes `terminal-not-found` and `not-live` each need their own distinct code on a logged `process-exit`. Its `strict-modes-failed` exit 1 before any log stays lineless.

@history_item n=19 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (Founder Direction 4: the socket-directory carve-out retired) the precedent for giving a new exit-1 cause its own code once it is built, here `socket-dir-refused` on `run` and on revive's start. The companion entry (`socket-dir-refused` in the log catalogue) moved the `process-exit` detail enum from 19 to 20 and recounted it by the enum's members. Adding `open`'s codes means recounting the same way.

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the closed words that left `diag-line.v1.json` with their emitter) the `process-exit` `detail` enum is back to 19, counted by its members, `os` is closed to `linux`, and a new §6 rule says a closed word no build emits leaves the schema with its emitter. Any `detail` word added for `open` must have an emitter in this chunk.

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the `cli` catch site's document under `--json`) under `--json` the `cli` catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, and `json_asked` reads `--json` before any `--`. These are the rows `contract/cli-internal-error/*` and `contract/panic-outcome-by-role/cli-json`. `open --json` takes `-- claude` after it, so this argv rule applies directly.

@history_item n=22 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (every refusal of `revive --json`'s start prints its document) the revive `--json` start refusals print `{"v":1,"error":"<code>","detail":null}`, the logged line is unchanged, and nothing is printed after the child holds the terminal. This is the precedent for `open`'s `--json` error document `{"v":1,"error":"<code>","detail":null}` alongside its logged codes.

@history_item n=23 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — (a refused instance check writes no line, on both arms) revive's start arm runs its instance check before the log opens. A refusal is the stderr pair and exit 1, with no log opened, no `process-start` and no `process-exit`, and `strict-modes-failed` left the exit-1 detail codes. This is the origin of `decision/strict-modes-refusal-writes-no-line` and `contract/viola-revive/strict-modes-failed`, the model for `open`'s "exit 1 `strict-modes-failed` before any log".

@history_item n=24 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified the no-line shape of every refused strict-modes check (revive, statusline, `budget.json`, list, plugin install) as built. A new verb that refuses before any log follows a shape the founder has already ratified, not an open question.

@history_item n=25 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the pre-init window and the panic exemptions gain the chunk's checks) each check runs before its first open, because the open narrows the home to 0700. Init step 3 runs the home check there, and exemption 8 covers a refused check with no role file and no line. These are `decision/strict-modes-before-diagnostics`, `decision/instance-check-before-log-init` and `decision/process-init-order`. `open`'s check must sit before its `cli` log, and its refusal must join the bounded panic exemptions.

@history_item n=26 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the home-tree check of `run` and `viola revive`'s start, before the wrapper log) `checked_exe` and `check_home_tree` run before `open_wrapper_log`, and a refusal writes no line and has no `detail` code. The `run` that `open` starts with hidden flags goes through this same gate, so a refusal there shows nothing in the wrapper's log. `open` sees only that the instance never comes live (`not-live`).

@history_item n=27 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the refusal-telemetry record: two strict-modes lines, every other refusal none) a strict-modes failure writes a line only on the `ledger-stamps` read and on verify's stamp write, now `decision/strict-modes-failure-lines` and `contract/diag-detail/strict-modes-failed`. `open` must not add a third line site.

@history_item n=28 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive's start arm is role `run`, logs into `run-<name>.ndjson`, and adds exit-1 details there, while `human::refuse` gained its second caller. This is the precedent for a verb that drives a start (`contract/cli-log-file/revive-start`). `open` instead spawns a separate `viola run` under `foot`, so its own lines belong to the `cli` role while the wrapper logs as `run`.

@history_item n=29 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `revive --list` became role `cli`. It logs `process-start`/`process-exit` into the caller's `cli-<VIOLA_NAME>.ndjson` only under a parsed `VIOLA_NAME` and only after its check passes, and with no name it writes no log. This is the shape of `decision/cli-log-only-under-viola-name` and `decision/no-process-log-without-instance`. `open`'s log instance must be decided between the named target, `<name>`, and the caller's `VIOLA_NAME`.

@history_item n=30 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (the link verbs log into the driven instance's `cli` file, inside `cli.link` / `cli.unlink`) a cli verb's instance can be its target argument rather than the caller's `VIOLA_NAME`, with `cli.<verb>` spans added to the span kinds. This is the precedent for which `cli-<name>.ndjson` `open` writes and for its dispatch span (`decision/cli-manual-dispatch-span`, `contract/span-kinds/client`).

@history_item n=31 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — a new cli verb's span, log shape and no-line refusal: the self pair goes only under a parsed `VIOLA_NAME` after the check passes, and the refused check opens no role file (exit 21). The `human::refuse` census was re-derived. This is the template for landing `open` as a new short-lived verb.

@history_item n=32 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the `.cmd`/`.bat` start refusal retired; the refusal writer's site counts) the latest `human::refuse(` census is `run` 7 and `verify` 5, re-derived by `grep -rcE 'human::refuse\(' src`. `open`'s refusal lines in `src/human/` change this count, and the §3/§11 rows that hold it must be re-measured.

@history_item n=33 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — `run`'s pre-spawn refusals go through `src/human.rs` `refuse` → `write_refusal`, two fixed stderr lines in one write. An output module may carry a local print `#[allow]` only where it uses a print macro. `open`'s fixed refusal lines (the word after `--`, `terminal-not-found`) must use this writer, under `decision/no-print-macros` and `decision/stderr-fixed-strings`.

@history_item n=34 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — (3. Observability Harness Contract) `~ contract/run-terminal-output/child-output`: `run` writes nothing to the terminal except the child's output and, before the child, a keyed start's automatic-verify lines. Under `open` the started `run` owns a `foot` window, and the placement work (polling `clients`, the move) must write no terminal byte, under `decision/run-silent-terminal`.

@history_item n=35 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — (4. Span / Trace Coverage) `+ contract/run-process-log/auto-verify-stop`: a stop in the start's verify window ends the start with no `process-exit` of its own. A `run` started by `open` may run the automatic verify before its child, which delays when the instance comes live inside `open`'s 30 s wait.

@history_item n=36 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1. Obs Scope Summary) `~ decision/exit-code-log-field`: exit 11 is retired and never reused. `~ decision/diag-schema-additive-only` was narrowed to "must not remove a field the product still emits". This bears on the scope's "no exit 11" and on any `diag-line` additions this chunk makes.

@history_item n=37 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — (the `ui` process and four events) `process` is closed to `run|hook|mcp|cli`, no process logs a liveness transition, `run` touches `heartbeat` every 1 s, and the flip comes after 5 s. `open` adds no process role. Its liveness wait (`live_endpoint`, 30 s) must not log per poll, under `decision/liveness-transitions-not-logged` and `decision/no-per-heartbeat-logging`.

@history_item n=38 marker=2026-10-10-session-links
**2026-10-10-session-links** — the start's steps with no span or line went from two to three, recorded explicitly with the span chain unchanged. The later socket-directory entry made it four. If the wrapper's window lookup and move add a step to the start, it must be placed relative to `contract/run-start-spans/span-chain` and stated as either spanned or deliberately unspanned.

@history_item n=39 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the start's socket-directory step, no span and one line on a refusal) this adds a fourth start step that has no span and writes a line only when it refuses. It is the latest precedent for a start step outside the span chain, relevant to where the wrapper's window step sits.

@history_item n=40 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ decision/env-vars-not-configuration` and `~ decision/r8-strip-logged-as-names-only` name each env value viola reads (`CLAUDE_CONFIG_DIR`), and `~ decision/claude-boundary-only-instrumentation` adds the trust-record line. `open` reads `PATH` to find `foot` and `VIOLA_NAME` for the parent default. Any such bounded env read must be named so the "env is not configuration" rows stay true.

@history_item n=41 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `list` opens `cli-<name>.ndjson` only under `VIOLA_NAME`, `board::read` is a replay caller with its `state-entry`/`state-recovered` lines, and a refused scan writes no line. `list --json` gaining `window?`/`origin?` must add no new log line and no content to the board's lines.

@history_item n=42 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the MCP `list` tool's lines (the `agents-probe` pair, `state-entry`, `state-recovered`, `config-json`) go into `mcp.ndjson`. When MCP `list` items gain `window?`/`origin?`, they ride the result only, with no new line, under `contract/viola-mcp-tools/list`.

@history_item n=43 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (version census 0.2.0) there is no frontend scenario. The keyboard-driven terminal fleet console tiled on Hyprland, and its scenarios, are owed to the route entry «Console scaffold». The scope excludes console work, so `open`'s window telemetry must not reach into that console's scenarios.
