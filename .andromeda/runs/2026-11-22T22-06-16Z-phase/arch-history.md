@history_item n=1 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — (diagnostics roots, `v` exception) named the one exception to "every record carries `v`" (process-log lines version by schema filename) and put codes-only role logs at home level, detail in the instance; a `lineage` event line must carry `v` like every event, and anything it logs follows the codes-only rule.

@history_item n=2 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — landed `viola-state`, the one shared snapshot writer and the start order (resolution → collision → pin → snapshot + heartbeat → events → spawn); the place a launch-time `lineage` line is written sits inside this chain, before the child spawns.

@history_item n=3 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (kinds, refusals) added `send-issued` / `send-confirmed` / `send-refused` as wrapper-appended, log-only kinds; the precedent for a new wrapper-written kind (`source: wrapper`, never waking `wait`) that a `lineage` kind would follow.

@history_item n=4 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — (pause and release served) put the wheel in `src/run/wheel.rs` and every snapshot write through `src/run/snapshot.rs`, and landed the `release-from-driver` refusal (`decision/driver-release-deterred-not-enforced`); a parent edge must touch neither the wheel slot nor that refusal.

@history_item n=5 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — the next append heals a torn line; `events::read_from` counts `unknown_kinds` / `unknown_fields` / `torn_lines`; the replay recovers only log-derived fields; an older reader meets a new `lineage` kind as a counted unknown kind, and the replay is where a crashed instance's lineage must be read back from.

@history_item n=6 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — (the `revive` verb registered) revive's argv (`--resume <id>`, `--fork-session`), its preflight readings and its `run-<name>.ndjson` producer; the verb whose start must carry the recorded parent forward.

@history_item n=7 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — (the snapshot's `cwd`) added the optional `cwd?` to the snapshot, a host path living only in the 0600 snapshot, absent after a replay so a revive over one is refused `cwd-missing`; the precedent and trap for any parent field kept in the snapshot rather than the log.

@history_item n=8 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — (the replay's first product reader) revive resumes the `gone` instance in place through the same start as `run`, appending to `events.ndjson` with no launch replayed and a first record `wheel{driver, start}`; a revived start must keep the parent, not write a new launch's lineage.

@history_item n=9 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — (the instance check before any log) moved revive's `check_instance` ahead of its wrapper log because opening a log narrows the home to 0700; the trap behind `decision/instance-check-before-any-read` that any new lineage read on revive or list must respect.

@history_item n=10 marker=2026-10-10-session-links
**2026-10-10-session-links** — (the implicit link as landed) the driven wrapper records a link on `send` / `answer` before `send-issued`, `link` lines `source: wrapper`, log-only, five shapes on the operator's word; the observed-link register this chunk must draw apart from parent edges.

@history_item n=11 marker=2026-10-10-session-links
**2026-10-10-session-links** — (the link set in the replay, the snapshot and the start) `LinkSet` restored at every start from `events.ndjson` through `replay::apply_link_line`, a `link` line with no `ts` reading `since: "unknown"`; the model for a log-derived set restored at start and replayed, and the place a lineage restore would sit beside.

@history_item n=12 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `list` as landed: a caller-side board (`board::scan` / `board::read`), wrapped rows only, the skip counts first shown, the whole-board exit-21 refusal with no log; the board read model the fleet trees extend.

@history_item n=13 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder ruled the board's whole-board exit-21 refusal his, and held the explicit link verbs to Epoch 6; the refusal shape any tree view of `list` keeps.

@history_item n=14 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** — (send and human split by phase) `src/human.rs` the module root with `src/human/board.rs` holding the board writers; where the tree drawing of `list` lands.

@history_item n=15 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — 0.1.0 a pure CLI, the `list` payload contract (`items`, `skipped`) stands for CLI `list --json` and MCP `list` alike; what `list --json` gains is shared by both surfaces.

@history_item n=16 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (Standard Contracts) `list` joins one `claude agents --json` read, unwrapped rows named by their `sessionId` (`wrapped: false`, never a valid target); a name on the board need not be a viola name, which bears on drawing links only after the name parses as a `ViolaName`.

@history_item n=17 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (Session Liveness) revive's preflight gained `session-live` through one agents read; a reading a revived start's lineage path runs beside.

@history_item n=18 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — (revive takes `--json`) revive's `--json` on both arms with its closed exit-1 codes; the machine form a parent-keeping revive must not disturb.

@history_item n=19 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — (the `VIOLA_DIR` step) every CLI verb, `run` and `revive` included, resolves its home from `VIOLA_NAME` + `VIOLA_DIR` when they pass the shape check; the launching session's identity a nested `viola run` can see, one candidate for how a parent is declared at launch.

@history_item n=20 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `NO_COLOR` / `TERM` / `COLORTERM` read by `src/human/style.rs` for `viola list` only, deciding only SGR bytes, the width cut named in `board.rs`; the colour and plain-when-piped rules the tree drawing inherits; any further environment read needs its own ratification.

@history_item n=21 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (MCP list builds the board) MCP `list` builds the same board through `list::board_document` with one agents read per call; a tree or lineage field in the board document reaches MCP `list` too.

@history_item n=22 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — (the `VIOLA_DIR` home step ratified) the founder ratified the CLI verbs' `VIOLA_DIR` step and canonicalisation; a launch-time parent read through the launching session's variables rests on this ratified step.

@history_item n=23 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the normalised kinds gained `dialog-expired`, the one wrapper-appended kind that wakes `wait`; the precedent for adding a kind in `viola-core` and for which wrapper kinds wake (a `lineage` kind must not).

@history_item n=24 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — (the split modules) `src/human/board.rs` split into `pairs.rs`, `bay.rs`, `cells.rs`, `rows.rs`; the files the fixed-column tree drawing touches.

@history_item n=25 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the snapshot's pid and started_at as server identity) the snapshot's `pid` / `started_at` are the identity every frame-writing client checks; a parent's identity recorded at launch is a separate fact from this server identity.

@history_item n=26 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — ([Links]: the explicit verbs and methods built) `viola link` / `viola unlink` and the served methods, the instance check before any log, the hold retired; the observed-link machinery a parent edge must never change.

@history_item n=27 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (the served link and unlink methods and the link set) `link` answers `{since, already_linked}`, `unlink` `{}`, `-32602` for an invalid name or a self-pair; the link set's change rules that stay untouched by lineage.

@history_item n=28 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (a product path appends the unlink event) `link` and `unlink` are wrapper-appended, log-only, never wake `wait`, and a replay learns the link set from them; the pattern a `lineage` line would follow.

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the snapshot readers moved onto the replay) six product readers read the snapshot through `read_snapshot_or_replay`, a replayed snapshot giving no endpoint; any lineage read from the snapshot meets the same replay.

@history_item n=30 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (one read per revive, one state-recovered line) revive's preflight reads the snapshot once and hands that read on; a parent carried by revive must reuse that one read, not add a second replay line.

@history_item n=31 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (bounded readers) every product reader of external bytes is bounded in its statement or named an exception, held by the `contract_bounded_readers` census; any new read a parent declaration or lineage read adds lands bounded or the census goes red.

@history_item n=32 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the sanitised error surfaces pattern) CLI `--json`, MCP and channel error bodies carry codes and fixed messages only, a peer's text only as a closed value; a parent name or link name reaches no error body but as a checked value.

@history_item n=33 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (`stale-heartbeat`) the collision refusal splits `already-live` and `stale-heartbeat` for `run` and revive; the codes a revived start keeps.

@history_item n=34 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (`viola revive --json` documents every refusal of its start) revive's start shares `run`'s refusals under `--json`, ten codes then; a lineage step in the start adds no code without a fork.

@history_item n=35 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the five implicit link shapes and the `link` line with no `ts` became the founder's; the link register stands as ruled.

@history_item n=36 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`socket-dir-refused`) revive `--json`'s closed exit-1 codes became eleven; the code list a revive change must keep exact.

@history_item n=37 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the separate lock files) the separate `.lock` layout kept as history of the Windows target (`decision/separate-lock-files`); any lineage append goes through the same locked appender.

@history_item n=38 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (`replace_private` is one attempt) the shared replace helper is one attempt, mode set before any byte (`decision/atomic-replaced-state-files`, `decision/private-mode-before-write`); a snapshot carrying a parent field is written only through it.

@history_item n=39 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the `.cmd`/`.bat` refusal retired) revive's `--json` codes 11 → 10 (`contract/cli-json-output/revive-refusal-codes`); the current closed list.

@history_item n=40 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (version census 0.2.0) no web or graphical frontend; the view beyond `viola list` is the Epoch 6 terminal fleet console, what it shows owed to "Fleet console view"; the boundary that keeps the console out of this chunk and the absences the board must respect.

@history_item n=41 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (Conventions) `decision/protocol-version-bump-rule` gained its except (removals kept every own-format `v` at 1); adding a `lineage` kind or a `list --json` field is additive with no `v` bump.

@history_item n=42 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (Established Decisions, 1 of 2) `decision/run-start-order` lost the budget step; the start-order row a lineage step must be written into.

@history_item n=43 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (Standard Contracts) `list` payload became `{items, skipped}`, items `{name, wrapped, liveness, status, wheel?, dialog_pending, cli_version?, cli_verified?}`, and the replay recovers only `links`, `agent_session_id`, `wheel`; the exact shapes a lineage field in `list --json` or the replay would widen.

@history_item n=44 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (Cross-cutting Patterns) `decision/mixed-version-tolerance` says an older writer's state reads whole with unknown keys and kinds skipped; an older wrapper's log without a `lineage` line must still read whole.

@history_item n=45 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (Standard Contracts) `decision/wrapper-events-log-only` lists the wrapper-appended kinds (`wheel`, `send-*`, `link`, `unlink`); a `lineage` kind would join that list.

@history_item n=46 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Conventions) the bump-rule except widened to the snapshot's `statusline_command` and two stamp ids, `v` kept at 1; the precedent for snapshot shape changes with no `v` move.

@history_item n=47 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Standard Contracts, 2 of 2) `contract/instance-snapshot/data` became the current field list; any parent field added to the snapshot changes this row.

@history_item n=48 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Infrastructure Patterns) `decision/board-read-model-shape`: the board read model is sync, std only, Claude-free and holds no endpoint, pinned_bin, cwd, pid, started_at or links; trees and links drawn on the board must keep within that shape or amend it.

@history_item n=49 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `decision/wheel-starts-with-driver` names trust keys typed before session-start taking the wheel; the start-time wheel a parent edge must not change.

@history_item n=50 marker=2026-11-15T18-01-44-wrap
**2026-11-15T18-01-44-wrap** — (the explicit-session-links answer-card confirm ratified) the explicit link verbs' card stands on the founder's word; the [Links] decisions are settled.

@history_item n=51 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — `contract/channel-wait/wake-kinds` and `wait-wake-kinds` pin waking by kind alone and list the wrapper records that never wake; a `lineage` record must be added to the non-waking set.

@history_item n=52 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — (Established Decisions) `decision/run-start-order` gained the trust step; the start order a lineage write is placed in.

@history_item n=53 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — (Standard Contracts) `contract/instance-snapshot/cwd` (revive keys its trust record by the recorded cwd) and `decision/spawn-directory-confined-to-snapshot`; what of a host path lands in a file another session can read, a question this chunk must state for a parent's identity.

@history_item n=54 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `decision/run-start-order` now runs any automatic verify after the bind, `decision/start-verifies-new-cli-version` added; the current full start order, revive's start included.

@history_item n=55 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — (Cross-cutting Patterns) `decision/viola-home-resolution`: `viola run` passes the home to its claude child only through `VIOLA_DIR` and to its automatic verify child only through `--home`; the child-environment channel a launch-time parent declaration would ride or must avoid.
