@history_item n=1 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** — (Input Validation; Authentication & Authorization) added `+ decision/console-key-reader-bounded` (64-byte raw reads, closed key classes, no key byte on any log line; provisional on a delegate word), `+ decision/console-terminal-and-home-check` and `+ contract/strict-modes-refusal/console`. This chunk builds directly on these rows. The view must keep the scaffold's keys (q, Ctrl-C, unbound), its terminal-then-home-read-rule check before any log, and its fixed refusal pair with no line written.

@history_item n=2 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — made `viola list` a strict-modes entry point: `board::scan` checks the home, `instances/` and each instance before anything is opened and before its log, and the first failure refuses the whole board with no line. It also added `board::log_pass` to the one events reader and `escape_row`, which escapes every control char, DEL and C1 included, as `\xHH`. The console calls `scan`/`read` in process and reuses `escape_row`, so it takes on this order, this whole-board refusal and this escaping (`decision/whole-board-refusal`, `decision/list-scan-checks-before-open`, `decision/human-output-escapes-controls`).

@history_item n=3 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder took the board's whole-board refusal and its no-line shape as his own word. Was the operator's answer. A console re-read once a second that refuses differently, or logs a refusal, would rework a ruled shape.

@history_item n=4 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`check_instance`'s set; `viola list`'s home-check inconsistency CARRY; the snapshot readers on `read_snapshot_or_replay`; the diagnostics opens) Held the home, instance and `diagnostics/` to the read rule. Moved `board::read` and the other snapshot readers onto the replay after their strict-modes check. Recorded the 0750-home inconsistency of the board scan as a CARRY. Made `open_role_file` refuse a symlinked or widened diagnostics path before narrowing. These bind the console's repeated board read and its `console` role file.

@history_item n=5 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the board's one rule on the home) `board::scan` switched to `check_read` on the home, so a home another user may read is `ScanRefused::Home` with or without an instance. This closed the CARRY above. It is the origin of `decision/list-home-read-rule` and `decision/board-home-read-rule`, which the console's board source inherits.

@history_item n=6 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the board's read rule confirmed) The founder's word removed the OPEN mark on the board's read rule: a readable home is refused with or without an instance. The console's per-second board read must not loosen it.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the home strict-modes check is owner and mode alone) Strict modes became Unix owner + mode only (`mode & 0o077` read rule). The Windows arms were retired, an unstattable home became a refusal, and the board's read rule lost its every-OS qualifier. This defines `decision/strict-modes-check`, `decision/strict-modes-read-rule` and `decision/unstattable-home-refused` as the console meets them.

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added the `claude agents --json` child of `list`: a direct spawn under the R8 strip, stdout capped at `MAX_FRAME`, killed at 5 s, and parsed tolerantly into rows or `oversize`/`malformed`. A failed read leaves the board as `board::read` gave it. Each `agents-probe` pair holds no row text, id, cwd or path. An unwrapped name is never a target and is printed through `escape_row`. The console's agents read every 5 s off the loop (`read_here`) inherits all of this (`decision/claude-agents-read-fail-open`, `decision/agents-probe-log-content-free`, `decision/unwrapped-name-never-target`).

@history_item n=9 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the mcp process spawns claude agents --json) Precedent: a second process gained the agents read through `list`'s board path, ratified as a boundary widening, with its `agents-probe` pair in that process's own role file under the same redaction. The console is a third such caller writing the pair into the `console` role file. The MCP list item (`contract/mcp-tools/list`) is a surface `LAST` must stay out of.

@history_item n=10 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** — (Input Validation: `decision/claude-agents-resolved-from-path`; Threat Model, Data Protection and Secret Management trust-flag exceptions) Added adopt's live-elsewhere check to the named callers that resolve `claude` for `claude agents --json` from the caller's PATH. Also set the current excepts of `decision/nothing-written-under-claude-home`, `decision/no-writes-under-claude-home`, `decision/viola-writes-nothing-in-claude-dir`, `decision/env-not-config-channel` and `decision/no-stored-secrets`. If the console resolves `claude` for the agents read, it widens that named caller list. It must add no write under `~/.claude` and no env read.

@history_item n=11 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (Threat Model Summary) reworded `~ decision/child-output-tolerant` and `~ decision/child-output-tolerant-unknown` to «a failure becomes unknown or a closed failure word». This is the rule the console's agents-read failure display rests on.

@history_item n=12 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — admitted `NO_COLOR`/`TERM`/`COLORTERM` for `viola list`'s human mode only, as a ratified widening. `COLUMNS`, `CLICOLOR`, `CLICOLOR_FORCE` and `CI` are not read, and the width comes from `viola_pty::host_size()`. Any further env read needs its own Decisions Log entry. This is the origin of `decision/terminal-colour-facts`, `decision/colour-env-only-compared`, `decision/no-width-or-colour-override-env` and `decision/new-env-read-needs-log-entry`. The console's quarter-screen width and row cut must come from the terminal size, with no env read. Colour in the console would be a new widening.

@history_item n=13 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — added `+ decision/board-lineage-and-links-name-checked`, `+ decision/lineage-parent-authenticates-nothing` and `+ decision/lineage-kept-to-snapshot-and-event-log`. The last keeps lineage to the snapshot and one event-log line, «with no process-log line, span or terminal byte». The console draws lineage trees on a terminal from the board's re-parsed rows. This is a trap: check how the board's own human list squares with "no terminal byte" before the view prints parents. The view also must not log them on `console-frame`.

@history_item n=14 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — added `+ decision/board-launch-pair-re-parsed` and `+ decision/launch-pair-kept-to-snapshot-and-event-log`. The launch pair leaves only through `list --json` and the MCP list item, with no terminal byte. Precedent and constraint for any board cell the console shows. The pattern for a new board field: re-parse it on read, and name where it may cross.

@history_item n=15 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** — (Data Protection) `~ decision/launch-pair-kept-to-snapshot-and-event-log`: one launch line at start and one per switch, never in the user's Claude settings. A board row the console re-reads each second may change its launch pair mid-life, which counts as a drawn-row change.

@history_item n=16 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — added `+ decision/board-window-and-origin-re-parsed` (window and origin only when they re-parse) and `+ decision/window-record-kept-to-snapshot-and-event-log`: address, workspace and origin leave only through `list --json` and the MCP list item, and no address, pid or workspace enters a process-log line. This is the closest precedent for `LAST`. It is a board-row field whose crossing surfaces are named. Here the founder's F2 inverts them: `LAST` is in neither `list --json` nor the MCP item. The console's `console-frame` lines must carry none of these values.

@history_item n=17 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** — the operator's own word set standing the board rows the console reads through: `board-launch-pair-re-parsed`, `board-lineage-and-links-name-checked`, `board-window-and-origin-re-parsed`, plus `launch-pair-kept-…`, `window-record-kept-…` and the provisional marks of 2026-11-22/23/24. The view rests on ratified rows. Changing any of them is his to rule.

@history_item n=18 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (the link lines the served methods write, and the names they keep out) Defined `events.ndjson`'s `link`/`unlink` lines as `{driver, driven}` names only (`contract/events-link-lines/data`). Verbs print only their own parsed names, never a name read back from the log. It also recorded a trap: opening a role file narrows an existing home to 0700, so a check placed after it passes a 0770 home. The console's home check must stay before its role file opens.

@history_item n=19 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — (the two instance checks ahead of any log) measured that a log opened before or after a refused check narrows the home to 0700 and erases the state the check refuses. The checks moved before any log, with a refused check leaving no trace. This is the reason behind `decision/strict-refusal-leaves-no-trace` and `decision/strict-check-before-narrowing`, which the console's start and re-reads must keep.

@history_item n=20 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — set the one events reader's contract: counts per read (`unknown_kinds`, `unknown_fields`, `torn_lines`), not logged; a reader never writes; the next append heals a torn last line with one `state-recovered` line. Last activity must come from the board's existing pass over `events.ndjson`, under the same bounds and with no write. The `parse-rejected`/`state-recovered` lines the board read writes land in the console's role file (`decision/readers-tolerate-torn-lines`, `decision/state-readers-never-write`, `decision/events-reader-bounded-lines`, `decision/torn-last-line-healed-on-append`).

@history_item n=21 marker=2026-10-10-session-links
**2026-10-10-session-links** — precedent for adding a pass over the events reader: the link-set restore became a second `read_from` pass, and the master row named it. Taking the newest line's age "in the pass the board read already makes" should be named in the same way and add no second unbounded read.

@history_item n=22 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the bounded-reader census) `tests/contract_bounded_readers.rs`'s `product_readers_are_each_bounded_or_named` requires every product reader call to be bounded in its statement or named in `EXCEPTIONS` (`decision/bounded-readers-census`). Any new reader in `src/console/` or a last-activity read must pass this census.

@history_item n=23 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the error bodies' census) `tests/contract_error_bodies.rs` holds `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing` (`decision/every-error-writer-named`, `decision/error-messages-interpolate-nothing`). A new console error or refusal writer must be enumerated there with fixed messages.

@history_item n=24 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (exit 21 names its cause in detail) `src/cmd/list.rs`'s `refused_document` writes the board's `strict-modes-failed` detail. This is `contract/strict-modes-refusal/list-json`'s current document. The console's in-process board source must not alter `list`'s refusal documents while it reuses `scan`.

@history_item n=25 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — (Threat Model Summary, the correction at amd-3w3vmtkj) A trap: `focus` was added to `decision/state-tree-strict-modes-before-use`'s home-tree verb list although it runs only `check_instance`, and the entry had to be undone. The console holds the home to the read rule through its own row (`decision/console-terminal-and-home-check`). It must not be added to the home-tree list unless it runs `check_home_tree`.

@history_item n=26 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — (Threat Model Summary) widened `~ decision/cwd-only-in-snapshot` and `~ decision/spawn-cwd-snapshot-only` to the recorded cwd on `viola up`'s terminal argv, never on a log line, error body or stdout. It is ratified at amd-kp64tfg2 and 2026-11-27. The console view must draw no cwd, since a host path carries the OS username.

@history_item n=27 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — (Threat Model Summary) gave the recorded cwd its second home as the trust-record key in the Claude config, keeping it off every log line, error body and stdout. Together with the entry above, it bounds what the console may show of a row.

@history_item n=28 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Input Validation 2 of 2; Threat Model Summary) set `~ decision/replayed-snapshot-inert`: a replayed snapshot carries no endpoint, pid or start time. It also set `~ contract/cli-state-root/home-flag` and `~ contract/cli-state-root/viola-dir` to «state tree, snapshot, endpoint and ledger stamps». The console's board source reads replayed rows as inert and is chosen by the same home resolution.

@history_item n=29 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — (the CLI home resolution as landed) Every CLI verb resolves `--home`, else the `VIOLA_NAME`/`VIOLA_DIR` instance's home, else `~/.viola`, and `canonical_home` creates nothing. This is the origin of `decision/cli-home-resolution-order`, which the console verb follows.

@history_item n=30 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — (the VIOLA_DIR home step ratified) The founder ratified the `VIOLA_DIR` step and the canonicalisation as a widening of what chooses a CLI verb's home. The console inherits it as ratified and must not widen it further.

@history_item n=31 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — (Data Protection) `~ decision/logs-owner-only`: the event, heartbeat and diagnostics owner-only append open never follows a symlink and sets 0600 through the handle. This binds the `console` role file whose `console-frame` gains `shown`/`hidden` counts.

@history_item n=32 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — (registry migration) moved into the body `scripts/release-check.sh`, which refuses any executable other than `viola` and any `test-support`/`fake-agent` artifact, and G2's single exact-path exemption with its `--probe`. These are the origins of `decision/release-build-no-test-binaries`, `decision/release-build-without-test-features`, `decision/g2-single-panic-exemption` and `decision/g2-zero-panics-single-exemption`. T4's booted fake-agent wrappers must stay behind the test-only feature.

@history_item n=33 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — admitted the `FAKE_AGENT_HOOK_PANIC` seam and G2's one whole-string exemption, a founder-ratified widening. Any further seam or exemption needs its own Decisions Log entry (`decision/test-seams-fake-agent-only`, `decision/test-seams-compiled-out`). No console test may add a seam or panic exemption to drive its size cases.

@history_item n=34 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (`MAX_FRAME` consumers extended, verify's CI split) the real CLI runs only locally, and CI runs only the fake agent (`decision/real-cli-local-only`, `decision/ci-no-secrets-no-real-claude`). This chunk's directive forbids a real `claude`, and the console's agents read in tests must go against the fake.

@history_item n=35 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — limited test-side removal under `target/e2e-home` to dirs the test side created. It recorded the tmpfs home link as a ratified widening and a trap about keepers with relative targets. T4's planted instance dirs and test-owned homes fall under it.

@history_item n=36 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — (Security Anti-Patterns) `~ decision/test-removal-own-dirs-only` added a fourth way, another build's stamp dir taken with `try_lock`, ratified at 2026-11-16T06-05-33-wrap. This is the current rule for the planted homes and booted wrappers of the console's size tests.
