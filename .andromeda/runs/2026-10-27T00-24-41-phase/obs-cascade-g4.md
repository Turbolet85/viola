# obs · cascade read · group 4 of 5

## Carried
- (.andromeda/obs-plan-detail/span-trace-coverage.md:40) The fixed pin step must stay `run.pin_copy` in its place in the start chain, between the collision check and the sideload and version gate, whatever the absent-copy write's new concurrency handling adds inside it.
  > `run.collision_check` › `run.pin_copy` › `run.conpty_sideload` (Windows) › `run.version_gate` (spawns the `version-probe`)
  (.andromeda/obs-plan.md:345)
  > Never the endpoint name, the `pinned_bin` path or env values in a span, and never the sideload companions' paths or hashes; the child's `--settings` argument, the statusline source path and the command appear in no line or span; a sideload outcome is never a refusal or an exit change; a failure's `detail` carries no path or pid
- (.andromeda/obs-plan-detail/span-trace-coverage.md:44) `run.pin_copy`'s `outcome` must stay the closed pair, so a start that loses the race and then finds a matching present copy reads `ok`, and a new outcome value is a plan amendment, never an ad hoc word; no span field may carry the pinned path.
  > `run.pin_copy`: `outcome` (`ok|pinned-hash-mismatch`);
  (.andromeda/obs-plan-detail/span-trace-coverage.md:50)
  > Never the endpoint name, `pinned_bin` path or env values.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:45) If P3 finds the Windows x64 `pin_companions` arm shares the race and the chunk edits it, its outcomes, its no-refusal, no-exit-change rule and its no-path, no-hash rule must hold unchanged, and `sideload_fallback` on the child's `process-start` must keep its closed values at info level.
  > `run.conpty_sideload` (Windows): `outcome` (`loaded|hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built`; `strict-modes-failed` when the owner/DACL check refuses the `conpty/` dir or a companion, `PinError::StrictModes`, nothing written), `search_restricted` (bool) — never a refusal and never an exit change: any outcome but `loaded` leaves the child on the inbox ConPTY; never the companions' paths or hashes (D-36);
  (.andromeda/obs-plan-detail/span-trace-coverage.md:56)
  > plus, on Windows when the `run.conpty_sideload` outcome is not `loaded`, `sideload_fallback` (`hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built`, D-36). A degraded sideload adds no `process-exit{exit_code:1}` and no panic line.
  (.andromeda/obs-plan-detail/log-coverage.md:10)
  > `sideload_fallback` (claude-child, Windows, present only on a degrade: `hash-mismatch|unreadable|load-failed|not-built`, or `strict-modes-failed`, the last when the write-rule check `pin_companions` makes before its held open refuses `bin/<key>/conpty/` or a companion in it, the child then on the inbox ConPTY with its exit unchanged and nothing written; D-36)
- (.andromeda/obs-plan-detail/span-trace-coverage.md:55) Any new exit-1 cause the absent-copy write adds (a lost race that cannot be settled) must be a distinct closed code added to the start's failure `detail` set, with no path or pid, and the exit-1 `detail` catalog and the schema's closed count must move with it by their stated rule.
  > On failure, `process-exit{subject:"self", exit_code:1, detail}` with `detail` ∈ `already-live|squatted-name|pinned-hash-mismatch|batch-script-child` (Founder Direction 4; no path or pid).
  (.andromeda/obs-plan-detail/log-coverage.md:11)
  > | `process-exit` | `subject`, `exit_code` (self), `child_exit_status` (child/probes), `shell_exit_status` (statusline), `exit_source` (`handle-wait|kill-fallback`), `detail`, `during` (`connect|call`), `duration_ms` |
  (.andromeda/obs-plan-detail/log-coverage.md:26)
  > `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, `internal-error`, and from `viola revive`'s start-arm preflight
  (.andromeda/obs-plan-detail/log-coverage.md:28)
  > `schemas/diag-line.v1.json` closes the `process-exit` `detail` at 13 values (the enum's members; 12 before chunk 2026-10-16-unwrapped-sessions-on-the-board, 11 before chunk 2026-10-13-interrupted-verify-cleanup, 9 before chunk 2026-10-10-viola-revive).
  (.andromeda/obs-plan.md:451)
  > **`detail` code catalog** (closed, kebab-case, never a path or pid; Founder Direction 4):
  (.andromeda/obs-plan.md:459)
  > | `error` | `panic`; `process-exit` with `exit_code` 1 or 20; `channel-response` with `error_code` -32603 |
- (.andromeda/obs-plan-detail/log-coverage.md:36) A concurrent start that finds a present copy and re-hashes it must keep Vector 7's `pinned-hash-mismatch` on `process-exit` exit 1 as its only refusal line, and the start's refused home-tree check must stay lineless; the tampered-copy refusal stays the fixed stderr pair, and the `human::refuse(` site counts move only by their grep rule if the fix adds or removes a site.
  > `pinned-hash-mismatch` (on `process-exit` exit 1), `symlink-ignored`.
  (.andromeda/obs-plan-detail/log-coverage.md:38)
  > and the home-tree check of `viola run` and `viola revive`'s start arm (exit 1, the home or the instance pair).
  (.andromeda/obs-plan.md:311)
  > the root bin's human-stderr refusal writer, called by `run` (7 sites), `verify` (6), `list` (2), `revive` (1), `plugin install` (1) and `client` (1, `client::refuse_untrusted`, the one site `viola link` / `viola unlink` and the six frame verbs `send`, `wait`, `last`, `answer`, `pause` and `release` share), as re-derived by `grep -rcE 'human::refuse\(' src` at chunk 2026-10-26-home-and-code-bearing-file-integrity
- (.andromeda/obs-plan-detail/span-trace-coverage.md:19) Any instrumentation the fix adds around the pin write must be a manual start-sequence or `state.*` span, since the start's spans are manual by the `viola run` row.
  > Manual spans on the start sequence, `pty.*` seam, `channel.dispatch`, `run.readiness_gate` (vt100 has no tracing), `run.wheel_transition`, `run.budget_eval`, `state.*`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:58) The two-sided witness must read a failed boot as the `start_records` readiness stage over `events.ndjson` lines 1-3 (a missing `session-start{source:"hook"}` reading `<name>:events`) through the harness, never from process logs, sleeps or the screen.
  > Product order `wheel{cause:"start"}` → `budget-gate` → `session-start` is verified from `events.ndjson` through `agent-run logs --kind`, not duplicated into process logs
  (.andromeda/obs-plan-detail/span-trace-coverage.md:58)
  > the harness `boot` readiness stage `start_records` checks `events.ndjson` lines 1-3 — `wheel{cause:"start"}` → `budget-gate` → `session-start{source:"hook"}` — with the missing code `<name>:events`
  (.andromeda/obs-plan-detail/metric-coverage.md:12)
  > | Gauge | Current value | Harness snapshots: `budget.read_at`, `liveness` via `agent-run status` |
- (.andromeda/obs-plan-detail/span-trace-coverage.md:20) The race breaks the pinned path's hook spawn; a `viola hook` that does run must keep its fail-open exit 0 and sink fallback, and the witness reads the session-start hook's `hook-invoked` / `hook-decision` lines (present after the fix, absent for the failed spawn) by their catalogued fields.
  > `std::panic::catch_unwind` around dispatch keeps exit 0. The writer falls back to `std::io::sink` on open failure
  (.andromeda/obs-plan-detail/log-coverage.md:16)
  > | `hook-invoked` / `hook-decision` | `hook_event` (`session-start|user-prompt-submit|pre-tool-use|permission-request|stop|session-end|notification|post-tool-use|post-tool-use-failure|statusline`), `stdin_bytes`, `invoked_at`, `decision_emitted`, `deadline_hit`, `budget_written`, `detail`, `duration_ms` |
- (.andromeda/obs-plan-detail/span-trace-coverage.md:218) `viola verify`'s edited `pin_exe` call must stay inside its dispatch spans, its `process-start` / `process-exit` must go to `cli-<name>.ndjson` only under an instance, and its home-tree check must stay ahead of its log with a refused tree writing no line.
  > Dispatch spans: `cli.verify`, with `cli.verify_step` per step and `state.ledger_write` for the stamp.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:219)
  > With `VIOLA_NAME` set they log `process-start` / `process-exit{exit_code, detail}` to `cli-<name>.ndjson`;
  (.andromeda/obs-plan-detail/span-trace-coverage.md:222)
  > `verify` runs the home-tree check `check_home_tree` on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log opens: a refused tree exits 1 with the home pair on stderr, no step line and no summary, and opens no log and writes no line.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:18)
  > A dispatch span per verb (`send.client`, `wait.block`, `last.client`, `answer.client`, `pause.client`, `release.client`, `cli.<verb>`). `process-start` / `process-exit{exit_code, detail}` into `cli-<name>.ndjson` when an instance resolves (D-06);
  (.andromeda/obs-plan.md:400)
  > before its log `viola verify` holds the home, `ledger/`, `bin/` and `plugin/` to the strict-modes check, and a refused tree is exit 1 with the home pair and no line
- (.andromeda/obs-plan-detail/span-trace-coverage.md:225) The race's test-side workaround (a stamped home pinned by `viola verify` first) stands on the harness `boot` step 4 and the `stamped_home` fixture, so the witness's unstamped two-instance boot must be the `--unstamped` form and the stamped path keep verify as the first pinner.
  > the tests harness `boot` step 4 (unless `--unstamped`) and the rstest `stamped_home` fixture, with `verify` the only writer of `ledger/stamps.json` (test-plan Decisions Log)
- (.andromeda/obs-plan-detail/log-coverage.md:17) Any path remap, linker flag or profile key the reproducible-build measurement leads the founder to rule must keep `panic_location` workspace-relative, so G2's single exemption still compares it as a whole string.
  > | `panic` | `panic_location` (workspace-relative, or `<crate>-<ver>/src/...` for dependencies), `thread`. Payload and backtrace go to `detail-<process>.ndjson` only |

## Bindings
- **span `run.pin_copy` outcome ↔ log Vector 7 / exit-1 detail**: `pinned-hash-mismatch` is both `run.pin_copy`'s refusing outcome and the start's `process-exit` exit-1 `detail`, so a concurrent start's re-hash keeps the two in step (.andromeda/obs-plan-detail/span-trace-coverage.md:44) ↔ (.andromeda/obs-plan-detail/log-coverage.md:36), (.andromeda/obs-plan-detail/span-trace-coverage.md:55).
- **span start failure set ↔ log `detail` catalog and schema count**: a new exit-1 cause enters the start's failure set, the exit-1 catalog and the schema's closed enum together, its count amended by the enum's members (.andromeda/obs-plan-detail/span-trace-coverage.md:55) ↔ (.andromeda/obs-plan-detail/log-coverage.md:28).
- **span `run.conpty_sideload` ↔ log `sideload_fallback`**: a non-`loaded` outcome is recorded only as `sideload_fallback` on the child's `process-start`, never as a refusal or exit change (.andromeda/obs-plan-detail/span-trace-coverage.md:45) ↔ (.andromeda/obs-plan-detail/log-coverage.md:10).
- **span start readiness ↔ metric `liveness` gauge ↔ log hook lines**: the witness's per-boot verdict is the harness `start_records` stage and `agent-run status`, with the session-start hook's lines as the wait point (.andromeda/obs-plan-detail/span-trace-coverage.md:58) ↔ (.andromeda/obs-plan-detail/metric-coverage.md:12) ↔ (.andromeda/obs-plan-detail/log-coverage.md:16).
- **log tampered-copy refusal ↔ body §3 refusal writer**: the pinned-copy refusal's exit-1 line and its fixed stderr pair move together, the `run` / `verify` site counts by their grep rule (.andromeda/obs-plan-detail/log-coverage.md:36) ↔ (.andromeda/obs-plan.md:311).
- **span verify row ↔ span cli row**: `viola verify`'s pin runs under its `cli.verify` spans and logs by the cli row's instance-resolved rule (.andromeda/obs-plan-detail/span-trace-coverage.md:218) ↔ (.andromeda/obs-plan-detail/span-trace-coverage.md:18).
