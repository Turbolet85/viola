# obs · Log Coverage — category read

## Carried
- (.andromeda/obs-plan-detail/log-coverage.md:28) A CLI verb (`send`, `wait`, `last`, `answer`, `pause`, `release`) whose server verification fails or cannot be made must end in `process-exit` exit 21 with the closed `detail` `server-verify-failed`. That value is already in the catalog, and the chunk must write it rather than invent a new code or reuse `instance-dead`.
  > - `process-exit` exit 21: `instance-dead`, `strict-modes-failed`, `server-verify-failed`.
- (.andromeda/obs-plan-detail/log-coverage.md:11) The exit-21 line for a failed verification must carry only the catalogued `process-exit` fields. Because the check runs before the first frame, `during` is the catalogued `connect` / `call` value and no new field is added.
  > | `process-exit` | `subject`, `exit_code` (self), `child_exit_status` (child/probes), `shell_exit_status` (statusline), `exit_source` (`handle-wait|kill-fallback`), `detail`, `during` (`connect|call`), `duration_ms` |
- (.andromeda/obs-plan-detail/log-coverage.md:30) `viola hook`'s fail-open path on a verification mismatch, for `hook.event` and `hook.dialog` alike, must log a `hook-decision` whose `detail` is the catalogued fail-open code `server-verify-failed`, kept distinct from `channel-unreachable`.
  > - `hook-decision` fail-open codes (security Vector 4): `oversize-stdin`, `malformed-json`, `channel-unreachable`, `server-verify-failed`, `strict-modes-failed`, `unverified-cli`, `deadline`.
- (.andromeda/obs-plan-detail/log-coverage.md:16) The hook's fail-open line must use only the catalogued `hook-decision` fields: the code goes in `detail`, and no frame was sent, so no decision is emitted.
  > `stdin_bytes`, `invoked_at`, `decision_emitted`, `deadline_hit`, `budget_written`, `detail`, `duration_ms` |
- (.andromeda/obs-plan-detail/log-coverage.md:33) Today no hook arm writes `strict-modes-failed`, because the nine hook events run no instance check. If this chunk takes the strict-modes half with the verification, a hook arm starts writing that word, and the plan must state it. If it leaves that half to "Home and code-bearing file integrity", the statement stays true.
  > on `hook-decision` the schema's `detail` enum keeps the word and no arm writes it, since `viola hook statusline`'s refused check leaves no line and the nine hook events run no instance check
- (.andromeda/obs-plan-detail/log-coverage.md:33) Today no arm writes `strict-modes-failed` on `process-exit` exit 21. That holds unless the chunk adds a strict-modes check to the CLI clients, and then the master text moves at the wrap.
  > so on `process-exit` exit 21 no arm of this build writes the word
- (.andromeda/obs-plan-detail/log-coverage.md:19) `state-recovered{detail:"snapshot-replayed"|"snapshot-unsupported-v"}` is written only by the replaying reader. A client moved to the classified or replayed snapshot read (CARRYs 7 and 8, held for the founder) would start writing these lines, so the line set follows that hold's outcome.
  > | `state-recovered` (D-03) | `detail` (`torn-line-healed|snapshot-replayed|snapshot-unsupported-v`), `file` (basename only: `events.ndjson` on `torn-line-healed`, `snapshot.json` on the two snapshot details)
- (.andromeda/obs-plan.md:394) The other snapshot readers, including the five client readers this chunk touches, must keep writing no `state-recovered` line unless they move to `read_snapshot_or_replay`.
  > - **E5** (snapshot corruption): `state-recovered{detail:"snapshot-replayed"|"snapshot-unsupported-v"}` at WARN, written by `viola_state::replay::read_snapshot_or_replay`, one line per call; the other snapshot readers write no `state-recovered` line.
- (.andromeda/obs-plan.md:361) This entry owns the debt the `wait` / `last` scenario records: after it lands, `server-verify-failed` must be reachable on `wait` / `last`. The `strict-modes-failed` half belongs to whichever entry builds the strict-modes check.
  > `strict-modes-failed` / `server-verify-failed` cannot occur on `wait` / `last` until client-side server verification lands, owned by the route entries "Server verification before any frame" and "Home and code-bearing file integrity".
- (.andromeda/obs-plan.md:460) A verification failure must be logged at `warn`, both as the CLI's `process-exit` exit 21 and as the hook's fail-open `hook-decision`.
  > | `warn` | `state-recovered`; `parse-rejected`; `hook-decision` with a fail-open `detail`; `process-exit` exit 21; `channel-response` with `-32600`/`-32602` |
- (.andromeda/obs-plan.md:735) The check compares pids and start times. No pid, start time or path may appear in its `detail` code.
  > - NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.
- (.andromeda/obs-plan.md:253) Exit 21's verification cause must stay its own distinct, path-free and pid-free `detail` code, separate from `instance-dead`.
  > - **Required telemetry signal:** `release-from-driver` is its own logged event, and each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid.
- (.andromeda/obs-plan.md:557) The endpoint the verification inspects, and the snapshot's recorded values, must not reach a home-level line.
  > Home-level lines stay path-free; `endpoint` / `pinned_bin` are never logged;
- (.andromeda/obs-plan.md:576) Under default-deny, any new field the check might want to log (a server pid, a start time, a verification reason) is a build failure under G4 unless it is first added to the §6 catalog. The chunk must therefore reuse the existing `detail` codes.
  > **Default-deny posture:** a field is logged only if it is named in this plan's Section 6 catalog. Unknown fields are a build failure (§10), caught by gate G4 (§9) against `schemas/diag-line.v1.json`.
- (.andromeda/obs-plan.md:715) The chunk must add no new `event` value, such as a dedicated verification event, without a Decisions Log entry.
  > - NEVER rename tests fields (for example `corr` → `correlation_id`, `event` → `event_type`) or add `event` values without a Decisions Log entry.
- (.andromeda/obs-plan.md:200) The hook's mismatch path must log only to `hook-<name>.ndjson`, exit 0 and leave stderr empty.
  > - **Required telemetry signal:** fail-open paths log to `hook-<name>.ndjson` only, exit 0 with stderr empty, and drift reports go to `diagnostics/` only.
- (.andromeda/obs-plan.md:205) The `viola mcp` frame tools' `instance-unreachable` result on a mismatch must be logged with codes only.
  > - **Required telemetry signal:** rejected tool calls are logged with codes only, with no upstream text, absolute paths or type names.
- (.andromeda/obs-plan.md:111) The CLI's stderr refusal for a failed verification must be a fixed string with no path or pid.
  > refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text.
- (.andromeda/obs-plan.md:709) Every new line on the verification paths must go through `obs_event!`, never a raw tracing macro.
  > - NEVER call `tracing::{event,info,warn,error,debug,trace}!` directly outside `viola_core::obs`. Only `obs_event!` guarantees `event`, `process` and `instance`; the caller sets `corr` explicitly as a typed field on the events that carry one.

## Bindings
- **exit-21 `server-verify-failed` ↔ security.md §Local trust boundary and the scope's CLI outcome**: the scope says a CLI verb reports the instance unreachable with exit 21, and the obs side gives that exit its distinct `detail` `server-verify-failed` at `warn` (.andromeda/obs-plan-detail/log-coverage.md:28) (.andromeda/obs-plan.md:460)
- **hook fail-open `server-verify-failed` ↔ security Vector 4 and the universal hook invariant**: exit 0, nothing on stdout or stderr and no frame on the security side, one `hook-decision{detail}` in `hook-<name>.ndjson` on the obs side (.andromeda/obs-plan-detail/log-coverage.md:30) (.andromeda/obs-plan.md:200)
- **strict-modes half ↔ the route entry "Home and code-bearing file integrity"**: whether a client arm starts writing `strict-modes-failed` depends on which entry takes the strict-modes check, and the master's "no arm writes it" text moves only if this chunk takes it (.andromeda/obs-plan-detail/log-coverage.md:33) (.andromeda/obs-plan.md:361)
- **`state-recovered` snapshot details ↔ CARRYs 7 and 8 (the founder's hold)**: a client moved to `read_snapshot_or_replay` starts writing `state-recovered`, so the log line set follows the held fork (.andromeda/obs-plan-detail/log-coverage.md:19) (.andromeda/obs-plan.md:394)
- **default-deny field catalog ↔ `schemas/diag-line.v1.json` (G4) and test-plan §3 Log format**: no new field or `event` value without a catalog amendment and a Decisions Log entry (.andromeda/obs-plan.md:576) (.andromeda/obs-plan.md:715)
- **MCP `instance-unreachable` ↔ security.md §Error handling (codes-only `isError`)**: the tool result and its log line carry codes only (.andromeda/obs-plan.md:205)
