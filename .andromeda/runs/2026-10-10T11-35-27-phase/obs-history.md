## Relevant amendment history

- **2026-09-24-three-os-ci-headless-harness-skeleton** — the fake agent is a `[[bin]]` of the root package and
  is exempt from the print ban only by its own crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]`
  (§11 carve-out). Matters: this chunk extends the fake agent (`--resume <id>`, a SessionStart with source
  `resume`); its prints stay inside that one bin-level allow and no product file borrows it.
- **2026-09-24-diagnostics-plane** — `obs_event!` attaches `event`, `process` and `instance` only; `corr` is a
  caller-supplied typed field with no macro arm. Default-deny is one top-level `unevaluatedProperties: false` in
  the schema, not per-event `additionalProperties`. Matters: every new revive line or field (a session id, a
  new `detail`) is refused by G4 unless the schema names it, and `corr` is never implied on the revive lines.
- **2026-09-24-log-redaction-and-never-log-floor** — anyhow lives at the root-bin dispatch edge and its
  catch-site reporter `viola::obs::report_internal_error`; the chain goes only to `detail-<process>.ndjson` and
  only when an instance resolves. Matters: a revive fault (not one of the four refusals) takes this path, and
  the recorded cwd may ride an anyhow chain into the detail file only, never a home-level line.
- **2026-09-24-observability-gates** — raw `tracing` level macros are banned by `clippy.toml` path, raw `event!`
  by the fail-closed grep in `scripts/lint-probes.sh`; a probe failure is a build failure. Matters: every revive
  log line goes through `obs_event!`; the gate commands for the chunk include the lint probes.
- **2026-09-25-pty-wrapper-on-windows** — `process-start{subject:"claude-child"}` gained `env_kept` (names only)
  and its field list stays argv-free; `batch-script-child` is decided by program resolution before the strip
  plan and `pty.spawn`; `run` writes two fixed stderr lines on that refusal. Matters: revive re-resolves the
  program at revive time, so the `.cmd`/`.bat` refusal is reachable from revive too (a fifth refusal the scope's
  four do not name), and `--resume` / the id stay off the child's start line.
- **2026-09-27-instance-state-and-start-order** — `already-live` is logged for a `live` AND a `stale` holder; a
  `gone` holder is taken over with no exit-1 line (`run.collision_check` outcome `free`); `run` writes `wheel` +
  `budget-gate` as the first product records. Matters: the scope's `instance-live` is this catalog's
  `already-live` (one name must win, and it covers `stale` too); a revive of a dead name is the `gone` case as
  worded here; the closed `outcome` set and the first-records order are what the revived start reuses, and the
  founder's open wheel question touches the first of those records.
- **2026-09-27-wrapper-channel** — `schemas/diag-line.v1.json` requires `corr` on a named list of events
  (dialog, channel, `send-*` wrapper side, `hook-decision`) and nowhere else. Matters: `process-start`,
  `process-exit` and `state-recovered` are outside that list, so the revive lines carry no `corr`; a revive line
  that reused a corr-bearing event would fall under the `required` rules and their negative test.
- **2026-09-27-hooks-to-normalised-events** — `session-start{source:"hook"}` is record three of a start, sent
  through `hook.event`; the harness `boot` check reads it. Matters: the E2E's `session-start{cause:"resume"}`
  arrives the same way (the fake agent's hook, not the wrapper), and on a revived instance it is not line 3 of
  `events.ndjson`: the log already holds the earlier life's records.
- **2026-09-28-hook-perf-gate** (the G2 entry) — G2 runs as `scripts/g2-zero-panics.sh` (`--probe` first) and
  exempts exactly one panic location, `src/cmd/hook/seam.rs:<digits>`. Matters: the revive E2E kills a wrapper
  by pid; neither the killed wrapper nor the reviving process may leave a counted `panic` line, and no second
  exemption exists to lean on.
- **2026-09-28-cli-output-tokens** — `run`'s one pre-spawn exception became every start refusal (`.cmd`/`.bat`
  child, live or stale name, tampered pinned copy, squatted endpoint): two fixed stderr lines through
  `src/human.rs` `refuse` → `write_refusal`, one `write_all`, no print macro and no `#[allow]` in that module
  (five callers at that chunk). Matters: revive's refusals use the same writer; the §3 intro and §11 name `run`
  as its path, so a second verb calling it is an amendment owed at the wrap; the tampered-pin and
  squatted-endpoint refusals also apply to a revive that runs the start order.
- **2026-09-28-capability-ledger-and-viola-verify** (verify's outcome, the capture arm, the readiness owner) —
  refusals are the stderr `unable:` + `hint:` pair and a fault exactly `error: internal error`; kept at that wrap: §6's
  `process-start` / `process-exit` requirement for every child spawn stood although verify's two spawns logged
  none (overseer ruling: the spec stays right, the fix was carried). Matters: a `claude agents --json` read
  built here without its `agents-probe` pair is a gap against the spec, not a reason to amend it; the hidden
  `hook --capture` arm is the only uninstrumented arm and revive may not be a second.
- **2026-09-28-capability-ledger-and-viola-verify** (the panic backtrace, the `cli` role's internal-error line)
  — a short-lived `cli` verb with a resolved instance writes `process-exit{subject:"self", exit_code:1,
  detail:"internal-error"}` to `cli-<name>.ndjson` "as `run` does", the chain only in `detail-cli.ndjson`, then
  `error: internal error\n` in one write; panic backtraces are raw frames. Matters: this is the `cli`-role shape
  `revive --list` falls under, and it is one side of the role decision the extract leaves open for the wrapper
  arm (`cli-<name>.ndjson` vs the `run` role file).
- **2026-09-28-mutation-testing-to-the-epoch-boundary** — obs code meets cargo-mutants only at `agent-run run
  --mutants` at the epoch boundary or on demand; a surviving obs-code mutant is a failure there (§10). Matters:
  the chunk plans no mutation run, so each new revive log line needs an asserting test now or it surfaces as a
  survivor at the Epoch 4 boundary audit.
- **2026-09-29-verify-stamped-test-homes-and-harness** — `subject` is the closed set
  `self|claude-child|version-probe|verify-probe|agents-probe|statusline-shell` (later `verify-pty-probe`), spawns
  are logged at the call site with `child_exit_status` + `duration_ms`, `run`'s version gate keeps one
  `version-probe` pair, and a new closed value takes a Decisions Log entry (D-35). The harness `boot` stage
  `start_records` checks `events.ndjson` lines 1-3 with the missing code `<name>:events`. Matters: `agents-probe`
  already stands in the enum, so a `claude agents --json` read needs no new subject; the revived wrapper's
  version gate writes its own `version-probe` pair; `start_records` cannot describe a revived start.
- **2026-09-29-sideloaded-conpty** — the start chain became `run.pin_copy` › `run.conpty_sideload` (Windows) ›
  `run.version_gate`; `pty_backend` gained `conpty-sideload`; `process-start{claude-child}` gained the closed
  `sideload_fallback`; D-36 is the template for adding a closed value under default-deny. Matters: the revived
  wrapper inherits this chain and these fields unchanged on all three CI OSes, and any closed value revive adds
  follows the same schema + catalog + Decisions Log route.
- **2026-09-29-t15-07-57-wrap** — the obs Decisions Log left the body for `obs-plan-amendments-archive.md` (40
  entries); in-force lifts landed in the body: spawn `Err` → `internal-error`, handle-wait `exit_source`,
  `$defs.subject.enum` closing `subject`, `sideload_fallback` closed and `pty_backend` open in the schema.
  Matters: a decision this chunk needs (the role `revive` logs under, a new closed code) is recorded as an
  amendment with its fact in the body, not as a new in-body D-entry; a failed `--resume` spawn is
  `internal-error`.
- **2026-10-02-epoch-2b-cleanup** — CI keeps every home until G2, G4, the secret scan and the uploads ran
  (`AGENT_RUN_KEEP_HOMES=1`); a root test home drops through `remove_owned`. Matters: the kill-and-revive E2E's
  home must survive to the gates under that mechanism, and this is the chunk whose P5 evidence (the child ends
  with its wrapper) the E2E stands on.
- **2026-10-03-mutation-scoring-completion** (the "§1 kept current" entry) — §1 is a labelled verbatim copy that
  is kept current; until a wrap brings a difference current, §3, §6 and §12 win. Matters: new `detail` codes or
  a new §4 flow for revive have §1 twins the wrap must bring along; planning reads §3/§4/§6, not §1, where they
  differ.
- **2026-10-04-readiness-gate-and-timing-constants** — `run`'s vt100 feed thread is a contained worker (a caught
  panic writes one `parse-rejected` and the hook's `panic` lines; nothing goes to `main`); the "no
  `process-exit`" ban covers the PTY pump and the handle-wait thread. Matters: if the revived wrapper logs as
  `run`, these per-role panic rules are the ones it inherits; a `cli`-role reading would not carry them.
- **2026-10-04-confirmed-send-with-cl-1-records** (the F4 entry, the chaos home outside the scans) — integration
  homes live under `target/e2e-home/` except one founder-ruled carve-out, `TestHome::outside_scan()`, used by
  `tests/chaos_feed_panic.rs` because it writes a counted panic line. Matters: the revive E2E takes the default
  in-scan home; a kill that produced a panic line would need the founder's ruling for a third carve-out, never
  a quiet `outside_scan()`.
- **2026-10-04-wait-and-last** — exit 21's `detail` is `instance-dead` + `during` only; `strict-modes-failed`
  and `server-verify-failed` are impossible on the client verbs until the Epoch 6 entries land. Matters: revive
  is a new reader of `snapshot.json`; whether it logs `strict-modes-failed` is bound to the same dated gap, and
  the scope holds the strict-modes question as the founder's, so no revive line may assume the check ran.
- **2026-10-04-dialog-answers-by-dialog-id** — `parse-rejected{parser:"ledger-stamps"}` gained
  `strict-modes-failed`; the plan's own spelling `strict-modes` "has no place in the closed schema", so the §6
  list was matched to `diag-line.v1.json`'s enum; CI verify runs at 2.1.287. Matters: `parse-rejected` details
  (and parsers) are closed in the schema, so `parser:"claude-agents-json"` and its detail must be read from the
  schema before use; the revived start's version gate goes through `read_stamps_strict` and can emit this line.
- **2026-10-04-the-wheel** — `run.wheel_transition.cause` is the closed `human-input|manual-pause|release`; the
  span `<area>` list was extended with `pause`, `release` when those verbs landed (`pause.client` /
  `release.client`); `release-from-driver` was admitted by the schema. Matters: the precedent for giving a new
  verb an area is an amendment of §2 Naming conventions, so revive either reuses `run.*` / `state.*` / `cli.*`
  or the wrap adds `revive`; a revived instance starting at `human` (the founder's open question) would need a
  start-time wheel record whose cause is outside this closed span set.
- **2026-10-04-running-turn-refusal** — the running turn is in-memory wrapper state that writes no event, span
  or line, ended by `turn-ended` / `session-start` / `session-end`. Matters (weak): a killed wrapper loses it
  and no record restores it; the revived wrapper starts with none, and the resume's `session-start` is the
  record that would end one.
- **2026-10-05-real-cli-verify-probes** — `subject` gained `verify-pty-probe` at three sites together (the §6
  event table, the child-spawn set, `$defs.subject.enum`); verify's CI runs drive the fake agent with a fixed
  flag list. Matters: the three-site pattern for any closed-value addition, and a `viola verify` probe for
  `--resume` or `claude agents --json` (if the chunk mints a ledger row) adds a spawn pair to the count below.
- **2026-10-05-dialog-rows-and-re-probe** — verify logs six spawn pairs (`version-probe`, `verify-probe`, four
  `verify-pty-probe`); the subject enum unchanged; CI adds `--dialogs`. Matters: the count "six" and the run
  list are stated in §4 Edge flows and §6; a new probe run for this chunk moves both, with the rule that
  produced the count.
- **2026-10-06-local-command-and-paste-framing-rows** — CI's fake-agent verify flag list is `--screens
  --turn-stop --trusted-root <workspace root> --dialogs --framing`, bound to test-plan §3 boot step 4; seventeen
  rows are stamped. Matters: a fake-agent flag added for revive that verify or `boot` must pass extends this
  bound list on both masters; `--resume` on the fake agent alone does not.
- **2026-10-06-local-command-send-outcomes** — a post-condition `/clear` is confirmed by the `session-start`
  with `cause` `clear` and a string `agent_session_id` differing from the remembered id; every hook invocation
  logs `hook-invoked` with its own `hook_event`. Matters: the log's id chain revive reads grows at each `/clear`
  (the newest id is not the first), the "remembered id" is wrapper memory a revived wrapper must re-seed from
  the log or the resume's `session-start`, and the resumed start writes
  `hook-invoked{hook_event:"session-start", corr:null}` to the hook role file.
- **2026-10-07-test-homes-off-the-contended-volume** — G2's `find` start point is `target/e2e-home/` with the
  trailing slash because `target/e2e-home` is a link to a tmpfs directory on the dev host; a kept home's
  diagnostics live in memory there. Matters: the revive E2E's home must sit under that scope for G2, G4 and the
  secret scan to read it, and a local red reading's home is gone at a reboot.
- **2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — E2's R8 sentence: the strip always removes the 11
  `IDENTITY_FLOOR` names; a differing Linux dev-host reading on 2.1.287 stands beside it; the logged fields
  (`env_stripped_count`, `env_stripped_known`, `env_kept`) are names only. Matters (weak): the revived child is
  spawned under the same strip and its `process-start` carries the same fields; an E2E asserting them on the
  Linux host reads the Linux values.
- **2026-10-09-epoch-3-cleanup** (the `empty-text` entry) — a new refusal detail was added to the §6 `detail`
  code catalog and the §1 list in one amendment; no schema moved only because `send-refused.detail` is an open
  string in `diag-line.v1.json`. Matters: the direct precedent for `cwd-missing`, `session-live` and
  `no-session`; whether `process-exit.detail` is open or closed in the schema decides if the schema moves too,
  and that fact was established per event, not assumed.
- **2026-10-09-epoch-3-cleanup** (the "route entries cited by title" entry) — the dated Epoch 6 gap is cited by
  entry title ("Server verification before any frame", "Home and code-bearing file integrity"), never a bare
  route number. Matters: plan text bound for a master names "Exit-cause code catalogue" and "The board: viola
  list" by title; the scope's `working-route.md:115` / `:123` numbers must not travel into a master.
- **2026-10-10-self-healing-state** — `state-recovered` got its two emitters: `read_snapshot_or_replay` writes
  one line per call at WARN with no `corr` (`snapshot-replayed` for an unreadable snapshot,
  `snapshot-unsupported-v` with `v_seen` for a newer one; none for a present or absent snapshot or a failed
  replay) under `state.snapshot_recover` / `state.replay`, and the healing append writes
  `torn-line-healed{file:"events.ndjson", offset}` (none from `try_append_event` under a held lock). The §6 row
  scopes `file` / `offset` / `v_seen` per detail; the reader's three counts have no process-log carrier;
  tracing-subscriber is a product dependency of the root bin only. The text names "viola revive" as the first
  reader. Matters: this chunk turns the library sentence into a product fact (the "no product run writes these"
  clause goes stale at this wrap); the enum, schema and fields were left unchanged and must already admit a
  product writer; a revive that appends to a killed wrapper's log may write `torn-line-healed` first, into its
  own role file.
