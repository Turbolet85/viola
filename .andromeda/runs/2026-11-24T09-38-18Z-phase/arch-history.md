@history_item n=1 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the `revive` verb registered) — registered `viola revive <name> [--id] [--fork] [-- …]`, its exit-1 preflight refusals, `role_of` filing revive beside `run`, the child flags (`--plugin-dir` first, `--resume <id>` one argv element of a direct spawn, the newest logged id of the session-id shape or an `--id` the log holds) and revive's start arm logging as process `run` into `run-<name>.ndjson`. This is the start path `adopt` builds on: its argv, its log producer and its role filing are what adopt reuses or must decide to diverge from.

@history_item n=2 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the snapshot's `cwd` and the child's second cwd source) — added `cwd?` to the first snapshot (valid UTF-8 only, the 0600 snapshot alone holding it) and made revive spawn in the recorded `cwd`, read only after the instance strict-modes check and refused `cwd-missing` when absent; a replayed snapshot gives no `cwd`. A founder-ratified boundary widening. Trap for adopt: a plain session has no viola snapshot, so the cwd source must be decided anew (scope's P3 measurement); program resolution and the version gate never read the recorded directory.

@history_item n=3 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the replay's first reader, the session chain, revive as the second start) — `replay::session_chain` reads `session-start` ids from `events.ndjson`; revive refuses a `live`/`stale` name through `run`'s collision check, resumes the `gone` instance in place (log appended, never truncated), and its first record is `wheel{holder:"driver", cause:"start"}` (founder's answer). Adopt has no prior log or chain: whether it mints a new instance directory and which wheel/record order it starts with rest on these precedents.

@history_item n=4 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the fake agent's `--resume` / `--fork-session` options, a relied shape with no row) — the fake agent gained `--resume <id>` (launch SessionStart with `source` `resume`) for revive's tests, and `claude --resume <id>` reopening the same session was recorded as relied on with no ledger row (later rowed). Adopt's fakes-only tests ride this fake-agent seam.

@history_item n=5 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (four crossings ratified) — the logged session id as `--resume`'s value on a revived child's argv was ratified by the founder as a crossing. Adopt's session id comes from the operator's argument, not the log: whether that id on the child argv is inside the ratified crossing or a new one is a boundary question for this chunk (the row `decision/logged-session-id-on-child-argv` says «read from the instance's log»).

@history_item n=6 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the instance check before any log, on both arms) — moved revive's start arm `check_instance` before its wrapper log opens (opening a log narrows an existing home to 0700, so a later check passes a 0770 home); a refusal opens no log and writes no line. Standing precedent behind `decision/instance-check-before-any-read` and `decision/strict-check-before-log-open` that adopt's entry point must keep.

@history_item n=7 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (the resume row, Run E) — `resume-session` (`--resume <id> reopens the session under that id`) became a relied ledger row on every OS, probed by verify's Run E through `ledger::resume_target` (session-id shape only) and `resume_args(id, false)`, revive's sibling caller. Adopt relies on the same `--resume` behaviour, now gated by this row and `cli_verified`.

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (Session Liveness: revive refuses a session id live elsewhere) — revive's preflight gained `session-live`: one `claude agents --json` read, a row whose `sessionId` equals the id about to be resumed refuses (exit 1, the «open in another claude» pair, `process-exit{detail:"session-live"}`, no child); `--fork` skips it; a failed read passes (fail open). Measured: a second `claude --resume` of a held id runs two writers on one transcript. This is the live-elsewhere refusal adopt's scope says guards it; its fail-open stance and its `--fork` skip are the precedent adopt must accept or narrow.

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (Standard Contracts: the agents read, the join, the unwrapped rack) — `list` joins one `claude agents --json` read by `child_pid` / `agent_session_id`; an unjoined row is an unwrapped item (`wrapped: false`, named by its `sessionId`, never a valid target); the read is `run::agents::read_here` / `read_logged` (R8 strip, MAX_FRAME cap, 5 s kill, `agents-probe` pair). The unwrapped row is exactly what adopt turns into a named wrapped instance; the reader and its fail-open shape are reusable.

@history_item n=10 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the fake agent's agents mode) — the fake agent answers `agents --json` from files beside its exe (`agents-mode`, planted `agents-rows.json`, a live `sessions/<pid>.json` registry), no env seam added (founder's word). Adopt's fakes-only witness of a «session live in another claude» refusal is built on this mode.

@history_item n=11 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** (revive takes --json on both arms) — revive's refusals print `{"v":1,"error":"<code>","detail":null}` (exit 1, stderr empty, codes only) with human pairs byte-identical without `--json`; `role_of` filing of `revive --list` under `cli`. Precedent for adopt's `--json` documents and its role filing.

@history_item n=12 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (the VIOLA_DIR home step ratified) — every CLI verb, `run` and `revive` included, resolves its home through `resolved_home` (`--home`, else `VIOLA_DIR`'s grandparent when `VIOLA_NAME`+`VIOLA_DIR` pass the shape check, else default) and canonicalises it; ratified by the founder. Adopt, typed inside a wrapped session, inherits this home rule, and `run --parent` takes the inherited `VIOLA_NAME` as parent.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check before the start's log; the pin key taken once) — every start (run's and revive's) calls `check_home_tree` through `checked_exe` before its wrapper log opens and pins the very bytes it hashed (`pin_key` / `pin_bytes`); a refusal is exit 1 with no log, no line, no child, and under `revive --json` the `strict-modes-failed` document. Adopt's start goes through the same shared start and inherits this order (`decision/start-pins-the-bytes-it-checked`, `contract/run-start-refusals/home-tree`).

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (what each replay reader decides; one read per revive) — `run`'s collision check over a replayed snapshot decides from the heartbeat alone; revive's preflight reads the snapshot once and hands that read to its collision check and recorded-directory reading, so one revive writes one `state-recovered` line. Trap for adopt: a second snapshot read in its own preflight would double the line.

@history_item n=15 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the pinned copy placed once with no clobber) — the pinned copy's absent-copy write is `place_private_new` (temp file closed, then `persist_noclobber`); a lost placement re-hashes the standing copy and a mismatch refuses `pinned-hash-mismatch`. Bound by the picked `decision/pinned-copy-*` rows that adopt's start shares.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`stale-heartbeat`, a stale holder's own code) — the collision refusal reads `already-live` for a live holder and `stale-heartbeat` for a stale one, on `run` and revive's first preflight reading. Adopt onto a held name (scope: «a defined closed outcome») inherits these two codes.

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json` documents every refusal of its start) — `Launch.json` makes `refused_with` print `refused_document` for the post-preflight start refusals, a program not found printing `internal-error`, each before `spawn_child`; a `pump_child` error prints no document because the child has held the terminal. Adopt's `--json` over the shared start follows this `Launch` switch and its «no document after the child held the terminal» rule.

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's `internal-error` document) — a `cli` verb with `--json` before `--` prints `{"v":1,"error":"internal-error","detail":null}` at the `main` catch site; `run`, revive's start, `mcp` and `hook` stay silent there. Whether adopt is filed `cli` or beside `run`/`revive`'s start decides which catch-site behaviour it gets (`decision/silent-roles-at-internal-error-catch`).

@history_item n=19 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`socket-dir-refused` and the start's refused socket directory) — `viola revive --json` gained `socket-dir-refused` (codes then eleven); `run` and revive's start refuse through `socket_dir_ready` before any snapshot, child or bind, exit 1 with one `process-exit{detail:"socket-dir-refused"}`. Part of the closed start refusal set adopt's start inherits.

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (child program resolution; `batch-script-child` retired) — `resolve_program` resolves a given path as given and a bare name on `PATH`; `batch-script-child` left with its emitter, so revive's closed `--json` codes are ten. The current closed code list (`contract/viola-revive-json-error-codes/*`) that adopt's documents extend or reuse.

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the R8 strip: exact names, `claude_env_keep` the one pass-list) — every inherited `CLAUDE*` name is removed from the child, matched exactly, except a `claude_env_keep` name; the identity floor always stripped; names only, never a value. Adopt's child gets this strip; it is also the area of the folded CI item (`security_negatives_env_capture`).

@history_item n=22 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Occupied Resources: no `--settings` at start) — the child's argv is now `--plugin-dir` then the caller's args (the `--settings` override and `contract/child-argv/settings` retired; `absence/status-line-interception`). Adopt's child argv must not reintroduce a settings layer.

@history_item n=23 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the sanitised error surfaces pattern and its contract binary) — the error-body census `tests/contract_error_bodies.rs` holds every CLI `--json` document to codes and fixed messages; two new root binaries require `fake-agent` and «both sit in the `ci` profile's kill-line filter». Adopt's new documents become rows of that census; the kill-line filter is the `.config/nextest.toml` precedent for the folded CI item (a class for a case that waits by design, not a raised global kill line).

@history_item n=24 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (Occupied Resources: root waits 12 s, test sockets under /tmp) — the shared root wait bound `WITHIN` moved 7 s → 12 s (`decision/root-waits-share-one-bound`) and test channel sockets bind under a short `/tmp` root. Direct precedent for the folded CI item's timing margin: a bound raised for waits by design, recorded with its value.

@history_item n=25 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** (the Watch bound's waits counted by pattern) — the `WITHIN` wait count is true only with its pattern (`Instant::now() + WITHIN` under `tests/`) beside it; a wait through another form is outside it. Trap for the CI item and adopt's new root tests: state any wait count with its rule.

@history_item n=26 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (verify writes its own trust record; the trust writer's closed words) — verify writes and removes its own trust record in the Claude config file (`CLAUDE_CONFIG_DIR` read only when kept), and the trust writer answers in a closed set of code words. The writer a start reuses when it trusts adopt's spawn folder.

@history_item n=27 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (the start writes the trust record of its spawn folder) — `run` and revive's start, on a verified CLI, write the trust record of an untrusted spawn folder (revive keyed by the recorded `cwd`), read `CLAUDE_CONFIG_DIR` when kept, and read verify-only words (`key-present`, `removed`) as `lost`; the spawn directory lives in the 0600 snapshot and as the trust key. Adopt's start writes the record for its adopted folder by this rule.

@history_item n=28 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (the start's automatic verify and start order) — with `verify_new_cli` on, a start meeting an unstamped CLI version runs one `viola verify` child after the bind under `ledger/verify.lock`, and `cli_verified` is the final verdict; `decision/run-start-order` now reads bind → automatic verify → trust record → first snapshot → spawn. The full start order adopt's start follows.

@history_item n=29 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** (identity-floor ledger row; the capture arm's env-names file) — the identity floor grew (`CLAUDE_CODE_CHILD_NOTES_DIR`, `CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS`), `decision/r8-identity-floor` lost «measured», and the hook `--capture` arm now writes an `Env.<k>.json` of environment names read by name only. This is the env-capture surface the folded CI item's `security_negatives_env_capture` cases exercise; their cost and wait are this probe's.

@history_item n=30 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (the lineage record, `run --parent`, revive keeps it) — added `run --parent` / `--role` (self-parent a usage error, exit 2; the inherited `VIOLA_NAME` taken as parent), the `lineage` wrapper event (log-only, after `wheel`), the snapshot's `lineage?` rebuilt by replay, `list` items' `parent?` / `role?` and `links?`, revive keeping the recorded parent and role, and `decision/lineage-label-only`. Adopt's «name and parent land as the first snapshot and the lineage record the board reads» rests wholly on these rows.

@history_item n=31 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (the launch pair at every start) — every start hands `--model <M> --effort <E>` after `--plugin-dir`; `launch` event and snapshot `launch?`; `run --model` / `--effort`, `config.json` `default_model` / `default_effort`, compiled `DEFAULT_PAIR`; revive resolves the pair from override, else record, else default (`decision/revive-pair-from-record`, PROVISIONAL); the pair is handed, never verified. Adopt, with no record, needs this precedence's fallback for its model and effort source (scope's P3 item).

@history_item n=32 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (`viola open` and its refusals) — the `open` verb landed (`contract/cli-verbs/open`, exit codes 0/1/2, `already-live` / `stale-heartbeat` / `terminal-not-found` / `not-live`), `decision/no-start-over-live-instance` extended to `open`, and the `window` record joined the snapshot, replay and list items. Precedent for adopt's terminal fork (open in a new terminal vs. run in the typed one) and for extending the no-start-over-live rule to a new start verb.

@history_item n=33 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (focus has no driver-facing form; exit 1 list) — `decision/focus-has-no-driver-form` left a driver-facing (MCP) form out on the conservative arm, PROVISIONAL; `contract/cli-exit-codes/1` gained focus. The nearest precedent for adopt's «CLI-only, no MCP tool» conservative arm and for naming the new verb on the exit-1 row.

@history_item n=34 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (up over revive; the cwd readers; lineage read to order) — `viola up` re-opens gone instances through spawned revives (revive's hidden `--origin`), `contract/instance-snapshot/cwd` gained up as a reader, `decision/lineage-label-only` admits one read (up ordering by parent), and `contract/cli-exit-codes/1` gained up. The most recent verb built on revive's start: its pattern of extending revive's surface and the cwd/lineage rows adopt will touch again.

@history_item n=35 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (contract/instance-snapshot/cwd ratified) — the cwd row's two readers (revive's spawn directory and trust folder, up's terminal directory) stand ratified on the operator's own word. Adopting a session adds a writer of `cwd` from a source other than a past start; the row's promise («the directory the start spawned its child in») is what adopt must keep true.
