@history_item n=1 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the revived child: the recorded cwd ratified, the session id on its argv recorded and not ratified) — made revive the second spawner of the wrapped child. It spawns in the snapshot's recorded cwd, which is read only after strict-modes, used only as the spawn directory, and refused `cwd-missing` when absent. It also added the row for the session id as a child argv value: the closed 36-char shape, one argv element of a direct spawn built only by `viola_agent_claude::resume_args`, placed after `--plugin-dir` and before the words after `--`. The cwd crossing was ratified live, but the id crossing landed unratified. This matters because adopt hands an operator-given id onto `--resume`, a new source for a ratified crossing. Where adopt's cwd comes from when no snapshot recorded one is exactly the kind of crossing that needs the founder's own word (rows revive-recorded-cwd-spawn-dir, session-id-argv-closed-shape, session-id-argv-no-shell).

@history_item n=2 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (revive reads its own state: the instance check as an entry point, the replay's one caller, its arguments) — revive's two arms run `check_instance` before any snapshot or log value is used; a refusal is `strict-modes-failed`, exit 1, with no child. The classified read and the replay got revive's preflight as their caller. `--id` is a clap parser of the closed session-id shape (anything else is exit 2), and `--list` conflicts with `--id`, `--fork` and the trailing words. Adopt reuses this preflight and argument shape (rows revive-strict-modes-before-use, state-reads-after-strict-modes, contract/revive-arguments/id).

@history_item n=3 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (revive: the session id ratified …) — the founder ratified the logged session id as `--resume`'s value on a revived child's argv. He was shown two options: ratify it, or allow only an id the human types. Adopt is that second option: an operator-typed id. Ratifying the logged id does not ratify a typed one. A typed id from outside viola's log is a new id source, which needs its own showing as a crossing; the 2026-10-14 Run E entry is the precedent for a third source.

@history_item n=4 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — added a third session-id source (Run B's first SessionStart capture, accepted only through `is_session_id`) and gave `resume_args` its second caller. This was ratified as a widening of the session-id crossing. It is the precedent that each new id source, adopt's operator-given id included, is a widening needing its own word.

@history_item n=5 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added the `claude agents --json` child of revive's preflight, skipped under `--fork`. It is a direct spawn under the R8 strip, its stdout capped at `MAX_FRAME`+1 and killed at 5 s, parsed tolerantly into rows or `oversize`/`malformed`. A live id elsewhere is the closed refusal `session-live` with no child. A failed read passes revive (fail open). Each `agents-probe` pair holds no row text, id, cwd or path, and an unwrapped name is never a `ViolaName` or target. This is the live-elsewhere guard adopt rests on. Trap: the read fails open, so a malformed or timed-out agents read lets revive start; adopt's guarantee that it never takes over a running process cannot lean on a fail-open read without a ruling (rows claude-agents-read-fail-open, agents-probe-log-*, unwrapped-name-never-target, contract/revive-json/session-live).

@history_item n=6 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — revive `--json` prints exactly `{"v":1,"error":"<code>","detail":null}` with a closed code. `--list --json` prints only the escaped ts, the cause word and a session-shaped id, never a path or pid. A refused check opens no log and writes no line. Home resolution order is `--home`, then the `VIOLA_NAME`/`VIOLA_DIR` instance home, then `~/.viola`, canonicalised and never created. That widening was not ratified here; see the 2026-10-18 entry. Adopt's `--json` and home choice follow this (rows cli-home-resolution-order, contract/revive-json/error-document, contract/revive-json/output-fields).

@history_item n=7 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (the VIOLA_DIR home step ratified) — the founder ratified the CLI verbs' `VIOLA_DIR` home step and the canonicalisation. A new verb such as adopt inherits this ratified home resolution and must not widen it (row cli-home-resolution-order, contracts cli-state-root/home-flag and viola-dir).

@history_item n=8 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the mcp process spawns claude agents --json) — the MCP `list` tool's own `claude agents --json` read was recorded as a boundary widening, a new process spawning claude, that needed the founder's answer. This bears on the scope's open fork over whether adopt is an MCP tool. The conservative arm is CLI-only; exposing adopt over MCP would be a widening of the same class (row claude-agents-resolved-from-path).

@history_item n=9 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the two instance checks ahead of any log) — revive's check was moved ahead of the wrapper log. A log opened before or after a refused check narrowed the home to 0700 and erased the state the check refuses. Trap for adopt's start: the strict-modes check must precede every log or role-file open (rows strict-refusal-leaves-no-trace, strict-check-before-narrowing, start-checks-home-tree-before-log-opens).

@history_item n=10 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** (the founder's Epoch 5 batch) — the founder's own word covers revive's agents read and its `session-live` refusal, and revive's refused strict-modes check opening no log and writing no line, as built. These are the ratified shapes adopt inherits through revive's preflight.

@history_item n=11 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`viola link` / `viola unlink`'s instance check before any log) — trap for later verbs: opening a role file narrows an existing home to 0700, so a check placed after `client::start` passes a 0770 home. A new verb must run its check before `client::start` (row strict-check-before-narrowing).

@history_item n=12 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (run's collision check: the server check retired as moot) — the start's collision check judges a recorded wrapper by its pid, OS start time and heartbeat, and opens no connection. A second adopt onto a held name meets this check (row collision-check-needs-no-server-verify).

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the collision check reads through the replay; the home tree checked at run and revive's start; the snapshot readers onto read_snapshot_or_replay) — revive's preflight reads the snapshot once, through `read_snapshot_or_replay`, and hands that read to both the collision check and the recorded-cwd reading. A replayed snapshot is judged by its heartbeat alone and yields no endpoint. `check_home_tree` runs through `checked_exe` before the wrapper log; a refusal exits 1 with the home or instance pair and no log, line or child. Adopt's start, and its first snapshot for a name never recorded, sit on this order (rows replayed-snapshot-fallbacks, replayed-snapshot-inert, start-checks-home-tree-before-log-opens, home-tree-strict-modes-before-log).

@history_item n=14 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — the pinned viola is placed once by a no-clobber placement with its write handle closed first, and never replaced. A lost placement re-hashes the standing copy. An exec of a copy still open for write fails `ETXTBSY`. Adopt's start goes through the same pin (rows pinned-binary-placed-once, pinned-viola-never-replaced, pinned-copy-no-clobber-placement, lost-placement-rehash).

@history_item n=15 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (revive --json documents every refusal of its start; a stale holder's code; the catch site's document) — revive `--json` documents every refusal of the start it shares with run, from a closed code list, and an internal error of the start path prints `internal-error`. The collision check reads `stale-heartbeat` for a stale holder and `already-live` for a live one. The catch site's `--json` gives `{"v":1,"error":"internal-error","detail":null}`. These are the closed outcomes adopt would extend for a second adopt of the same id or an adopt onto a held name (contracts revive-json/*, collision-check/*, internal-error-output/json, cli-json-error/internal-error).

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the no-endpoint reading, dead against unwrapped) — a name with no live endpoint is read `instance-dead` or `unwrapped-name` from the instance directory alone. Adopt turns an unwrapped session into a named instance, so the reading of a name never started bears on its outcomes (rows unwrapped-name-never-target, unwrapped-session-name-not-target).

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`socket-dir-refused` and a refused socket directory at a start) — `socket_dir_ready` checks, makes the directory only when absent, and checks again, all before any snapshot, child or bind. Run and revive's start exit 1 `socket-dir-refused` with one `process-exit` line. The revive `--json` codes gained it. Adopt's start shares this (rows start-checks-socket-dir-before-bind, run-refuses-socket-dir-before-start, contract/socket-dir-refusal/run-and-revive-start, contract/revive-json/socket-dir-refused).

@history_item n=18 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (child program resolution; the R8 strip exact) — `viola run` resolves a given path as given and a bare name on PATH, and hands an absolute path. `batch-script-child` left the revive `--json` codes (11 to 10). Every inherited `CLAUDE*` name is matched exactly and removed except those in `claude_env_keep`, and the identity floor is always stripped (rows run-resolves-absolute-program, claude-resolved-from-path, r8-claude-env-strip, contract/config-json/claude-env-keep).

@history_item n=19 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — on a verified CLI, run and revive's start set the CLI's trust flag (flag only) for an untrusted spawn folder, and the flag stays. They skip the home folder and folders already trusted. The recorded cwd became a trust-record key, and run and revive's start read `CLAUDE_CONFIG_DIR` only when it is kept, to choose the config. The start's trust step ends in one of 16 closed words. An adopt built on revive's start would write a persistent trust record for the adopted session's cwd. That cwd may come from outside any snapshot, so it is a new key source for this ratified write (rows start-writes-trust-record-flag-only, start-trust-skips-trusted-folders, trust-record-writer-guarded, revive-cwd-spawn-only, cwd-only-in-snapshot, trust-record-closed-words).

@history_item n=20 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (the Epoch 4 delegate confirms ratified) — the founder's «Ратифицировать все» made the start's trust write rows his own word: the R8 and `CLAUDE_CONFIG_DIR` exceptions, the revive cwd rows, the cwd-only-in-snapshot rows and the nothing-under-claude-home exceptions. These ratified rows bound what adopt may reuse without a new ruling.

@history_item n=21 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added the bounded exceptions to "nothing written under the Claude home", the R8 `CLAUDE_CONFIG_DIR` value read, and the trust record writer's mode-keeping exception to explicit-mode-before-write. These are the source rows adopt would amend if its start writes into Claude's config (rows nothing-written-under-claude-home, no-writes-under-claude-home, credential-strip-by-name-only, env-not-config-channel, explicit-mode-before-write).

@history_item n=22 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — `--parent` goes through the viola name check and `--role` must be a code word. A parent equal to the instance's own name is exit 2, before anything is read. The parent comes from `--parent`, else the inherited `VIOLA_NAME`. It is self-reported and authenticates nothing. Lineage is kept only in the snapshot and one event-log lineage line, and the board shows it only when it re-parses. Adopt's name and `--parent` land as exactly this lineage record (rows run-lineage-flags-checked, lineage-parent-authenticates-nothing, lineage-kept-to-snapshot-and-event-log, board-lineage-and-links-name-checked); the parent rows stand PROVISIONAL.

@history_item n=23 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — `--model` and `--effort` are checked as closed words before anything is read. The pair crosses onto the claude argv only as four separate elements of a direct spawn, after `--plugin-dir` and before the caller's words. Revive handing the recorded pair on is a second ratified crossing. The pair is refused after the dashes. It is kept only in the snapshot's launch field and the event log, and it authenticates nothing. `--list` conflicts with `--model` and `--effort`. The effort check lives in the Claude adapter crate. Adopt's model and effort need a defined source where no snapshot recorded them; reading them from another process would be a new crossing beyond these rows (rows launch-flags-checked, launch-pair-argv-crossing, launch-pair-kept-to-snapshot-and-event-log, launch-pair-authenticates-nothing, cli-input-trust-boundary, effort-word-check-in-agent-crate, board-launch-pair-re-parsed); several stand PROVISIONAL.

@history_item n=24 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — `viola open` spawns its terminal directly from PATH, never through a shell, with a closed argv. Run's hidden `--origin`/`--place` flags are checked. Open was added to the verbs holding the home tree to strict-modes before any log. Origin authenticates nothing, and the window record is kept to the snapshot and event log. These are the precedents if the fork has adopt open a terminal (rows state-tree-strict-modes-before-use, window-record-kept-to-snapshot-and-event-log).

@history_item n=25 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — `viola up` spawns the pinned `viola revive <name> --origin` in the recorded cwd through open's direct spawn. Revive gained a hidden `--origin` that conflicts with `--list`. The recorded cwd on up's terminal argv was written into cwd-only-in-snapshot and spawn-cwd-snapshot-only as a separate boundary widening. It landed PROVISIONAL and was then discharged on the operator's W1 answer. Up checks before its log and before a spawn, and exit 21 covers up's board-scan refusal. This is the closest precedent for adopt running revive in a new terminal: an adopted cwd on a terminal argv would widen those rows again (rows up-checks-before-log-and-spawn, cwd-only-in-snapshot, spawn-cwd-snapshot-only, contract/revive-arguments/id).

@history_item n=26 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (state-tree row corrected) — focus was first listed in state-tree-strict-modes-before-use, then removed in the same wrap because it runs `check_instance`, not `check_home_tree`. Trap: list adopt in that row only if it truly runs the home-tree check at its start.

@history_item n=27 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** — the capture arm's SessionStart env-name file (names only, never a value), the recorded env fixture (names only; an env name carrying the user word refuses the recording), the verify stamp's names-and-counts env probe, and the four non-identity hook names off the identity floor. These rows' guards include `record_refuses_an_env_name_carrying_the_user_word` and `kept_name_reaches_no_stamp_or_record`, the `security_negatives_env_capture` cases in the folded CI item. Any margin change must keep these controls intact, never weakening the refusal (rows env-fixture-names-only, env-probe-names-and-counts-only, capture-arms-isolated, non-identity-hook-names).

@history_item n=28 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — readers count torn lines, unknown kinds and unknown fields, never write, and the next append heals a torn last line. Adopt's first snapshot and its lineage and launch lines go through these readers and appenders (rows readers-tolerate-torn-lines, torn-last-line-healed-on-append, state-readers-never-write, events-reader-bounded-lines).

@history_item n=29 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — replayed-snapshot rows and the `--home`/`VIOLA_DIR` state-root contracts were narrowed to endpoint, pid and start time; the snapshot holds no command. This is the current form of the snapshot facts adopt's first snapshot rests on (rows replayed-snapshot-yields-nothing, replayed-snapshot-fallbacks, contract/cli-state-root/*).

@history_item n=30 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — child output parsing reads a failure as unknown or a closed failure word. This applies to adopt's agents read and to any compositor read if adopt opens a terminal (rows child-output-tolerant, child-output-tolerant-unknown, shared-validators-in-core).

@history_item n=31 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader census) — every product reader must be bounded in its statement or named as an exception, enforced by `product_readers_are_each_bounded_or_named`. Any new adopt reader, such as a read of a session's cwd or model from outside viola, must pass it (rows bounded-readers-census, max-frame-cap).

@history_item n=32 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (handle inheritance measured) — channel handles never leak into the claude child, witnessed by the child's live fd table. An adopt start spawning a resumed child keeps this property (row no-handle-inheritance).

@history_item n=33 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the clients' screen of a peer's reply) — a client takes only closed values from a server's reply into CLI `--json` or MCP results. If adopt talks to a wrapper, its outputs follow this (row client-reads-closed-values-only).

@history_item n=34 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** (the rig's keys) — the founder's word, given anew per chunk, binds the live rig's closed argv key grant, while its no-key stop needs no word. Adopt's gated live reading (ending and resuming a real session) falls under the same rule: it needs the operator's grant for this chunk and cannot borrow an earlier one.
