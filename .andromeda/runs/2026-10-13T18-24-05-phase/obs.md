# obs extract

## Relevance
partial — the chunk adds a stop path to `viola verify` (a short-lived `cli` verb with six logged child spawns and an owned removal of probe dirs); obs governs what that path writes to the process log and what it must never write, not the signal mechanism or the two literal moves.

## Constraints
- obs-plan §4 Edge flows (`verify` / `plugin install`) requires that `verify`, with `VIOLA_NAME` set, writes `process-start` / `process-exit{exit_code, detail}` to `cli-<name>.ndjson` and logs each of its six child spawns between them as a `process-start{subject}` / `process-exit{subject, child_exit_status, duration_ms}` pair (`version-probe`, `verify-probe`, four `verify-pty-probe`), with no row text, prompt or path in any line; run from a plain terminal with no instance it writes no process-log file. Whether an interrupted run still closes the pair of the child it ends, and still writes its own `process-exit{subject:"self"}`, is research's question.
- obs-plan §6 Log Coverage (`detail` code catalog) requires `process-exit.detail` to be a closed kebab-case code, never a path or pid, and records that `schemas/diag-line.v1.json` closes it at 11 values; no catalogued code names an interrupted run, so a new `detail` (or a new `event` value) for the stop path is an enum extension that needs a Decisions Log entry and a schema change (obs-plan §3 Log format JSON schema: "A new value needs a Decisions Log entry"), never a free string.
- obs-plan §6 Log Coverage (Log levels mapping) requires `process-exit` with `exit_code` 1 or 20 at `error`, exit 21 at `warn`, and every other lifecycle event at `info`; the interrupted run's exit code itself is architecture §Conventions' to fix, and obs logs it as `exit_code` (obs-plan §1 Obs Scope Summary, cli surface notes: exit codes are a log field).
- obs-plan §4 Span / Trace Coverage (Span naming convention) requires any span the stop path adds to be a static `<area>.<operation>` name under the `cli` area beside `cli.verify` / `cli.verify_step`, declared `#[instrument(skip_all, name = "...", fields(...))]` with `err` / `ret` banned and no path or pid in the name.
- obs-plan §8 PII Scrubbing (Data classification rules, Absolute paths row) requires home-level lines to stay path-free: the three removed dir classes (`<cwd>/.viola-verify-<pid>/` and siblings, `<home>/ledger/probes/<pid>/`, Run A's OS-temp `viola-verify-*`) may reach no home-level line, and the raw hook captures held under `ledger/probes/<pid>/` are content that no line carries.
- obs-plan §4 Edge flows (`verify` / `plugin install`) requires the hidden `hook <event> --capture` arm to stay uninstrumented (no `VIOLA_*` read, no obs init, no line) and states that `verify` removes `ledger/probes/<pid>/` on every exit path; the stop path adds no instrumentation to that arm.
- obs-plan §8 PII Scrubbing (Default-deny posture) requires that a field is logged only if the §6 catalog names it; any field the stop path wants (for example a stop cause) is a catalog and `diag-line.v1.json` amendment first.

## Patterns to follow
- obs-plan §6 Log Coverage (Boundary-call wrappers, Child / shell spawns): `verify` logs its spawns at the call site and `run_bounded` stays unlogged — an interrupted spawn's closing `process-exit` belongs at the same call site, not inside the shared helper.
- obs-plan §6 Log Coverage (Boundary-call wrappers, PTY seam): the catalogued way to mark a child ended by a kill rather than its own exit is `process-exit.exit_source:"kill-fallback"`; whether `diag-line.v1.json` admits `exit_source` on a `verify-pty-probe` line is research's question before it is reused.
- obs-plan §4 Edge flows (`verify` / `plugin install`): refusals go to stderr as the fixed `unable:` + `hint:` pair and an internal fault prints exactly `error: internal error`; the agent-readable outcome is the exit code plus `verify`'s stdout step lines.
- obs-plan §7 Error Capture & Reporting (Panic hooks, short-lived `cli`): a cli fault with a resolved instance ends in `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` and the chain in `detail-cli.ndjson` only — the model for a fault met on the stop path.

## Anti-patterns to avoid
- obs-plan §11 Obs Anti-Patterns (Logs): no `print!` / `eprintln!` / `dbg!` and no raw `tracing::{event,info,warn,error,debug,trace}!` outside `viola_core::obs` on the stop path — only `obs_event!`; and no new `event` value without a Decisions Log entry.
- obs-plan §11 Obs Anti-Patterns (PII Scrubbing): no path or pid inside a `detail` code, and no field outside the §6 catalog (default-deny).
- obs-plan §11 Obs Anti-Patterns (Spans / Traces): no bare `#[instrument]`, and no `#[instrument(err)]` / `(ret)` on a function whose error can carry a path or I/O source.

## Contract bindings
- obs ↔ tests: obs owns `schemas/diag-line.v1.json` and `schemas/diag-detail.v1.json`, tests own the G4 check body (obs-plan §9 CI Integration, G4); any new stop-path field or code changes the obs schema first, and the tests that interrupt `verify` against the fake agent keep their homes under `target/e2e-home/` so G2, G4 and the secret scan read their lines (obs-plan §9 CI Integration, Pipeline integration, Integration tests row).
- obs ↔ architecture: the interrupted run's exit code is fixed by architecture §Conventions; obs carries it as `process-exit.exit_code` (obs-plan §1 Obs Scope Summary, cli surface notes).
- obs ↔ security: the path-free home-level rule for the removed dirs and the raw captures binds to security-plan §Data Protection, which this entry names (obs-plan §8 PII Scrubbing, Data classification rules).

## Acceptance criteria contributions
- (obs) An interrupted `verify` run under `VIOLA_NAME` leaves `cli-<name>.ndjson` lines that all pass G4 against `schemas/diag-line.v1.json`, including its `process-start` and `process-exit{subject:"self", exit_code}` (per obs-plan §9 CI Integration, G4).
- (obs) Every `process-start{subject:"verify-probe"|"verify-pty-probe"}` written before the stop has its `process-exit` carrying `child_exit_status` and `duration_ms` (per obs-plan §6 Log Coverage, Child / shell spawns, D-35).
- (obs) No home-level `diagnostics/*.ndjson` line from an interrupted run holds `.viola-verify-`, `ledger/probes`, an absolute path or a pid inside `detail` (per obs-plan §8 PII Scrubbing, Data classification rules).
- (obs) An interrupted run writes zero `event:"panic"` lines (G2 reads it clean) (per obs-plan §9 CI Integration, G2).
