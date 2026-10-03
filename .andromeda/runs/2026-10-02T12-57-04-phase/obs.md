# obs extract

## Relevance
partial — the chunk is test/mutation/lint/measurement work and adds no new telemetry surface, but M2 (homes under `target/e2e-home`), the tempdir-leak fix (home lifetime vs the CI gates), M1 (obs-observable effects and obs-code mutants), P2 (a vacuity-proof exit-code gate) and P5 (child lifecycle read from process logs) all touch obs contracts.

## Constraints
- Integration-test viola homes must stay under `target/e2e-home/` so G2, G4, the secret scan and the `diag-<os>` upload cover them; a home relocated elsewhere hides a `hook` panic (exit 0). Neither M2's fix nor the tempdir-leak fix may move a test home out of that root (per obs-plan §9 Pipeline integration, Integration tests row).
- Homes are kept until the gate steps have run: under `AGENT_RUN_KEEP_HOMES=1` neither harness `cleanup` nor an rstest `TempDir` drop may delete a home before G2, G4, the secret scan and the scan-gated uploads read it; an empty `target/e2e-home/` fails G2's non-empty check. The `.tmp*` leak fix must close the leaking dirs without breaking this keep-homes contract (per obs-plan §9 Step order and conditions, step 1). Whether the leaked `.tmp*` dirs are viola homes, other tempfile dirs, or both is research's question.
- A surviving cargo-mutants mutant in obs code is red at a `run --mutants` (missed, timed out, or unviable outnumbering caught). M1's scoped mutation run is judged by the same rule if any touched unit holds obs code (`obs_event!` call sites, panic hook, `MillisUtc`) (per obs-plan §9 Pipeline integration, Mutation row; §10 Build / deploy failure conditions).
- M1's `search_restricted` survivor sits behind a contracted obs surface: §4 requires the `run.conpty_sideload` span to carry `search_restricted` (bool) and `outcome`, and a non-`loaded` outcome to add `sideload_fallback` on `process-start{subject:"claude-child"}` with no companion path or hash. Whether the code already records `search_restricted` (making it an observable for a killing test) is research's question (per obs-plan §4 Scenario: `viola run` start sequence to child spawn).
- `refuse_stale` borders the `run.collision_check` contract: `live` or `stale` holder → `already-live` (with `process-exit{subject:"self", exit_code:1, detail:"already-live"}`), `gone` holder → `free`, taken over with no exit-1 line. A killing test that asserts on the process log must assert these codes, never a path or pid inside `detail` (per obs-plan §4 Scenario: `viola run` start sequence; §6 `detail` code catalog).
- Evidence written into the chunk's `evidence/` (M2's env/tree diff, P5's measurement) follows the NEVER-log floor: `CLAUDE*` values are never recorded, only names (as R8's `env_stripped_known` / `env_kept` do). `claude agents --json` rows are Low/untrusted, and §8 records them as counts, never verbatim. Pids are Low and may be recorded (per obs-plan §8 PII Scrubbing data classification table).
- Gate verdicts are exit codes, never human-read log review, and there are no retry-once policies: a green re-run never stands in for a cause (per obs-plan §11 CI; §11 SLO; §10 Cross-input parallel).

## Patterns to follow
- M2 diagnosis reads failed runs through the harness: `agent-run logs` (merged `events.ndjson` + `diagnostics/*.ndjson` + `detail-*.ndjson`) filtered with `jq`, and `agent-run status` for the aggregate state; errors and panics once via `select(.record.level=="ERROR" and (.file|startswith("detail-")|not))` (per obs-plan §3 Snapshot / paste-to-AI integration).
- P5 takes the child pid from the wrapper's own role file: `process-start{subject:"claude-child", child_pid}` in `<home>/diagnostics/run-<name>.ndjson`. A clean child end is `process-exit{subject:"claude-child", exit_source:"handle-wait"|"kill-fallback"}` and then `process-exit{subject:"self"}`; the absence of both after `Stop-Process` on the wrapper is part of the reading (per obs-plan §4 Scenario: `viola run` start sequence, Cleanup; §3 Log file location).
- Liveness after the wrapper dies is read from the short-lived readers' `liveness` (`list --json`) or `agent-run status` (`state: degraded` on `stale`), with the flip at `heartbeat_age_ms > 5000`. No per-tick lines (per obs-plan §3 Heartbeat ticks; §10 Standard+ invariants, Liveness signal).
- P2's lint takes the G2 shape: refuse an empty scope before judging, so it cannot pass vacuously, and set an exit code. The G2 `--probe` form (known-red and known-clean inputs proven before the real check) is the precedent for proving the lint fires (per obs-plan §9 Gate commands, G2).

## Anti-patterns to avoid
- NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` or raw `tracing::{event,info,warn,error,debug,trace}!` in product code paths while adding diagnostics or test seams for M1/M2; only `obs_event!` with catalog fields, and any new field outside the §6 catalog fails G4 (per obs-plan §11 Logs; §11 PII Scrubbing default-deny).
- NEVER add a retry-once policy or a retried assertion to green a flaky or host-red test; `retries = 0` parallels the zero-unlogged-panics invariant (per obs-plan §11 SLO; §10 Cross-input parallel).
- NEVER put a path or pid inside a `detail` code, or a `CLAUDE*` value anywhere (detail files included), including in any assertion helper or evidence capture this chunk adds (per obs-plan §11 PII Scrubbing).

## Contract bindings
- obs ↔ tests §3 harness: `target/e2e-home/` layout, `AGENT_RUN_KEEP_HOMES=1` and `cleanup` step 6 decide home lifetime. The tempdir-leak fix and any M2 fix to home handling must keep the G2 / G4 / secret-scan scope intact (obs-plan §9 Step order ↔ test-plan §3 `cleanup`).
- obs ↔ tests §10 mutation gate: obs-code mutants count under the same `missed == 0`, `timeout == 0`, `unviable <= caught` rule M1 is judged by (obs-plan §9 Mutation row ↔ test-plan §10).
- obs ↔ security NEVER-log floor: evidence captures for M2's env diff and P5's real-CLI leg (obs-plan §8 table ↔ security-plan §Bootstrap phases `logging-redaction-wire`).

## Acceptance criteria contributions
- After the tempdir-leak fix, a CI-shaped run with `AGENT_RUN_KEEP_HOMES=1` still leaves a non-empty `target/e2e-home/**/diagnostics/*.ndjson` home-level set, and G2 (`--probe` then the check) and G4 exit 0 over it (per obs-plan §9 Step order and conditions; §9 Gate commands G2/G4).
- No integration-test viola home this chunk adds or changes resolves outside `target/e2e-home/`, shown by a grep over the touched tests' home construction (per obs-plan §9 Pipeline integration, Integration tests row).
- Committed `evidence/` files for M2 and P5 contain no `CLAUDE*` value, GUI token, `Cookie` or `?t=`, and no verbatim `claude agents --json` row; env differences are recorded by name only (per obs-plan §8 PII Scrubbing data classification table; §11 PII Scrubbing).
- M1's scoped mutation run reads zero missed and zero timed-out mutants in any obs-code site it covers (per obs-plan §10 Build / deploy failure conditions).
