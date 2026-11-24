@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (MAX_FRAME consumers extended, verify's CI split) — set "the real claude CLI runs only locally; verify runs in CI only against the fake agent" (decision/real-cli-local-only, decision/ci-no-secrets-no-real-claude). `viola up` re-opens real sessions through revive, so every test of it must ride the fake agent, and any real reading stays local and gated.

@history_item n=2 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — readers count torn lines, unknown kinds and fields and never write; the next append heals; the replay is library code. This is the basis of decision/readers-tolerate-torn-lines and decision/state-readers-never-write. Up's reading of logs and snapshots to decide «was live before the reboot» must be a non-writing reader under these bounds.

@history_item n=3 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the recorded cwd ratified, the session id on argv) — revive spawns in the snapshot's recorded cwd, read only after strict-modes, `cwd-missing` otherwise. The session id is one argv element built only by `resume_args`. Revive joined the strict-modes entry points through `check_instance` and became the replay's first product caller. Up re-opens each instance through this same path and inherits these crossings unchanged (decision/revive-recorded-cwd-spawn-dir, decision/session-id-argv-closed-shape, decision/revive-strict-modes-before-use).

@history_item n=4 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (revive: the session id ratified, its arguments, its log before its check) — the founder ratified the logged session id on `--resume`, and the revived child's argv order was fixed. A trap was recorded: opening a log narrows an existing home to 0700, so revive's start must not open its log before the check.

@history_item n=5 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the two instance checks ahead of any log) — revive's check now runs before the wrapper log, and a refusal is exit 1 with no log and no line. This is the precedent for decision/start-checks-home-tree-before-log-opens and decision/strict-refusal-leaves-no-trace. Up starts many revives and must keep this order for each of them.

@history_item n=6 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `list` became a strict-modes entry point. `board::scan` checks the home, `instances/` and each instance in name order before opening anything, and the first failure refuses the whole board (exit 21, no log, no line). The classified read and replay gained `board::read`. Up reads the same board, so it rests directly on decision/whole-board-refusal and decision/list-scan-checks-before-open.

@history_item n=7 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** (budget.json ratified; the board's refusal his) — the founder made the whole-board refusal and its no-line shape his own word. If up reuses the board scan, it inherits a ruled refusal shape, not a delegate's.

@history_item n=8 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — `resume_args` gained its second caller, with the id still gated by `is_session_id`. Up must build any `--resume` argv only through that one builder (decision/session-id-argv-closed-shape) and must not add a third ad-hoc path.

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added the `claude agents --json` child of list and of revive's preflight: a direct spawn under R8, capped at MAX_FRAME, killed at 5 s, parsed tolerantly, failing open. A live id elsewhere is the closed `session-live`. An unwrapped name is never a ViolaName or target. Up's per-instance revive runs this preflight N times; see decision/claude-agents-read-fail-open and decision/agents-row-name-not-a-target.

@history_item n=10 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — revive's `--json` gained closed-code documents (never a path or pid). The CLI home resolution became `--home`, else the `VIOLA_NAME`/`VIOLA_DIR` instance home, else the default, canonicalised. Up's `--json` and home choice must follow decision/cli-home-resolution-order and the revive-json contract rows.

@history_item n=11 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `NO_COLOR`/`TERM`/`COLORTERM` were admitted for list's human mode only, deciding only SGR around escaped rows. Any colour on up's per-instance lines would be a new environment read that needs its own entry; rows stay escaped (decision/human-output-escapes-controls).

@history_item n=12 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the mcp process spawns claude agents --json; the eighth dated gap) — precedent that a new MCP tool or MCP spawn is a boundary widening needing the founder's word. The scope's conservative arm, CLI-only with no MCP tool for up, follows from this.

@history_item n=13 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (the VIOLA_DIR home step ratified) — the CLI verbs' `VIOLA_DIR` step and canonicalisation stand ratified. Up, as a new CLI verb, takes its home through the same `resolved_home` and adds no other source.

@history_item n=14 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder made revive's agents read, and its refused strict-modes check that opens no log and writes no line, his own word. Revive's refusal shapes that up surfaces per instance are ratified.

@history_item n=15 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (run's collision check: the server check retired as moot) — the collision check judges a recorded wrapper by pid, OS start time and heartbeat, and opens no connection. This is decision/collision-check-needs-no-server-verify. Up's «skip pid-and-start-verified live ones» is this liveness judgement, not a server verification.

@history_item n=16 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (link/unlink's instance check before any log) — recorded a trap for later verbs: opening a role file narrows an existing home to 0700, so a check placed after `client::start` passes a 0770 home. Up must check before any role file or log opens.

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — every client checks the instance before the snapshot. The collision check reads through `read_snapshot_or_replay`: a replayed snapshot is judged by the beat alone, and an absent one is free. Run and revive's start hold the home tree (`check_home_tree`) before the wrapper log. Snapshot readers moved to the replay, and a replayed snapshot yields no frame. Up's liveness skip and its «was live» test rest on these (decision/replayed-snapshot-fallbacks, decision/replayed-snapshot-inert, decision/start-checks-home-tree-before-log-opens).

@history_item n=18 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — concurrent first starts raced on the pinned copy (ETXTBSY, 4 of 24 boots). This was fixed by a no-clobber placement that closes its write handle, and a lost placement is re-hashed. Up starting a fleet at once is exactly this case; it relies on decision/pinned-binary-placed-once and decision/pinned-viola-never-replaced.

@history_item n=19 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader census) — every product reader must be bounded in its statement or named in `EXCEPTIONS` (`product_readers_are_each_bounded_or_named`). Any new reader up adds (snapshot, log, runtime-dir probe) must pass the census (decision/bounded-readers-census).

@history_item n=20 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `sanitised_fault` and `is_code_word` screen a peer's reply to closed values, and the error-bodies census `product_error_writers_are_each_named` fails on an unnamed writer. Up's `--json` document and its error writers must be named in that census and carry only codes (decision/client-reads-closed-values-only, decision/every-error-writer-named).

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — the collision check prints `stale-heartbeat` for a stale holder and `already-live` for a live holder or a fresh replayed beat. `revive --json` now documents every refusal of its start. The catch site prints `internal-error` under `--json`. `without_endpoint` reads `instance-dead` vs `unwrapped-name`. Up's closed per-instance outcome words should map onto these codes (contract/collision-check/*, contract/revive-json/*, contract/internal-error-output/json).

@history_item n=22 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — a start (run or revive) runs `socket_dir_ready` before any snapshot, child or bind, and a refusal is exit 1 `socket-dir-refused`. The board scan holds the home to the read rule (`check_read`), so a 0750 home is refused with or without an instance. Up's revives hit these refusals per instance, and its board read rests on decision/list-home-read-rule.

@history_item n=23 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (child program resolution; the R8 strip matches names exactly) — run resolves a given path, or a bare name on PATH, to an absolute path, and not-found is `ProgramNotFound`. `batch-script-child` left the revive codes. R8 strips `CLAUDE*` names exactly, keeping the identity floor. Each child up revives is resolved and stripped this way (decision/run-resolves-absolute-program, decision/r8-claude-env-strip).

@history_item n=24 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the board's read rule confirmed) — the founder confirmed that a home another user may read is refused, with or without an instance. Up reading the board inherits a ratified rule (decision/board-home-read-rule).

@history_item n=25 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — added the hyprctl child under a 5 s deadline, with failures read as closed words. Reply text such as window titles is dropped at the parse, and the Address validator lives in the seam crate. If up reads the compositor to place windows, it reads through this seam alone (decision/hyprctl-read-deadline, decision/hyprland-reply-text-dropped, decision/child-output-tolerant).

@history_item n=26 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — removed the statusline command, so a replayed snapshot carries no endpoint, pid or start time and yields no frame. `--home`/`VIOLA_DIR` choose the state tree, snapshot, endpoint and stamps. These are the current forms of decision/replayed-snapshot-yields-nothing and the contract/cli-state-root rows that up's selection relies on.

@history_item n=27 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — the trust record writer is guarded: it never creates the config, refuses links and other owners, uses closed outcome words, and keeps the file's mode. verify reads `CLAUDE_CONFIG_DIR` only when kept. This is the base of decision/trust-record-writer-guarded and decision/trust-record-closed-words, which each start of up's fleet goes through.

@history_item n=28 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — run and revive's start now write the CLI's trust record for an untrusted spawn folder, flag only, and it persists. The recorded cwd also keys that record, and `CLAUDE_CONFIG_DIR`'s value is read at the start. A fleet up therefore writes one trust record per revived folder under the Claude config, within decision/start-writes-trust-record-flag-only, decision/no-writes-under-claude-home and decision/cwd-only-in-snapshot. No widening beyond that is allowed.

@history_item n=29 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** (identity floor grown; the four non-identity hook names) — the identity floor became thirteen names, and decision/non-identity-hook-names stands. This is the strip every revived child of up receives.

@history_item n=30 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — the founder ratified Epoch 4's provisional marks. Among them: the start's trust record rows, the revive cwd as the trust-record key, and the `CLAUDE_CONFIG_DIR` exceptions. Up's revives rest on these as ratified rows. decision/config-cannot-disable-controls also gained «or lifts the unverified CLI gate».

@history_item n=31 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — the lineage parent and role were checked at `run`, shown on the board only when they re-parse, authenticating nothing, and kept to the snapshot and one event-log line. Up's parents-before-children order reads this self-reported parent, so it must treat it as no identity and only order by it, never join it into a path (decision/lineage-parent-authenticates-nothing, decision/board-lineage-and-links-name-checked, decision/lineage-kept-to-snapshot-and-event-log).

@history_item n=32 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — the launch pair flags are checked before anything is read. The pair crosses onto the child's argv as four elements, and revive hands the recorded pair. Board shows only a re-parsed pair. The pair authenticates nothing, stays provisional, and is kept to the snapshot and event log. Up re-opens each instance with its recorded pair through revive, inside these rows (decision/launch-pair-argv-crossing, decision/board-launch-pair-re-parsed, decision/launch-pair-authenticates-nothing).

@history_item n=33 marker=2026-11-23-seam-requests-for-viola-open
**2026-11-23-seam-requests-for-viola-open** — the hyprctl deadline was settled at 5 s. The test-only recorder may move only a window of a terminal it started (decision/hyprland-recorder-moves-own-window, provisional). Any up test placing windows over Hyprland fakes stays within that bound.

@history_item n=34 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — viola open spawns its terminal directly with a closed argv, starting only the pinned `viola run`. Run's hidden `--origin`/`--place` take only closed words or an integer. The board shows window and origin only when they re-parse. The window step moves only its own terminal's window. The window record is kept to the snapshot and event log, and open's ancestor walk is held in memory. open joined the home-tree strict-modes verbs. Up re-opens each instance «in the window viola open would give it», so it inherits each of these rows, several of them provisional; a new crossing such as revive taking window flags would be a widening.

@history_item n=35 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — focus exits 21 on a strict-modes refusal (contract/cli-exit-codes/21), and its ancestor walk is held in memory. A trap was corrected: the home-tree row first named focus, then was reverted because focus runs the instance check (`refuse_untrusted`) and not `check_home_tree`. When up is added to decision/state-tree-strict-modes-before-use or decision/cli-frame-verbs-strict-modes-first, name it only under the check it really runs, with the exit code it really returns.
