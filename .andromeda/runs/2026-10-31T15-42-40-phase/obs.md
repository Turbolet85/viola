# obs extract

## Relevance
relevant — the chunk moves the exit-21 and exit-1 cause codes that obs-plan closes on the `process-exit` `detail` list and carries as a Founder Direction 4 signal; it adds no span, metric instrument, sink or `event` value by its scope.

## Constraints
- Obs-plan requires each cause behind exit 21 and exit 1 to reach the process log as a distinct closed kebab-case `detail` with no path or pid (per obs-plan §6 `detail` code catalog; `.andromeda/obs-plan-detail/log-coverage.md`, log-details). Whether every cause in the scope's lists holds such a code in the code is research's question.
- Obs-plan requires a strict-modes check refused before any log opens to write no line, its cause carried only by the fixed stderr pair or the `--json` document: the frame verbs, `link` / `unlink`, `list` and `plugin install` at exit 21; `run`, `revive`'s start and `verify` at exit 1 (per obs-plan §1 Telemetry triggers, creator-explicit Founder Direction 4; per obs-plan §6 Filesystem refusals). A cause moved into a log line here is a change to that rule, for P4 to name.
- Obs-plan requires `process-exit` `detail` to stay closed by `schemas/diag-line.v1.json`, and a field or value outside the §6 catalog is a build failure; a new code joins the catalog and the schema together (per obs-plan §8 Default-deny posture; per obs-plan §10 Build / deploy failure conditions). §6 states the enum at 13 values; whether the schema still reads 13 is research's question.
- Obs-plan fixes the levels: `process-exit` with exit 1 at `error`, with exit 21 at `warn` (per obs-plan §6 Log levels mapping).
- Obs-plan fixes the exit-21 log shape: `detail:"instance-dead"` with `during` of `connect` or `call`, the endpoint lost mid-wait being `during:"call"` and logged once by the client; `detail:"server-verify-failed"` with `during:"connect"` on Linux and Windows, none on macOS, the recorded gap (per obs-plan §4 Scenario `wait` / `last` event-driven readback).
- Obs-plan requires an MCP `instance-unreachable` reject, on a refused instance check, the liveness pre-check or a server-verification mismatch, to be logged by no line, the `isError` code its only record, and every logged reject to be codes-only (per obs-plan §7 Error classes captured). A distinct MCP code stays inside that rule unless P4 names otherwise.
- Obs-plan requires a code word crossing a client to pass `viola_channel::sanitised_fault` and `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first), and states that a code word outside the closed list stays visible and takes `unknown`'s exit 14 through `reason_of` (per obs-plan §7 Error Capture detail, `.andromeda/obs-plan-detail/error-capture-reporting.md`). If the choice carries a new code in `error`, what an older client reads and exits with is research's question and P4's choice.

## Patterns to follow
- Each logged cause is one `process-exit{subject:"self", exit_code, detail}` at its role file, with `corr` absent (per obs-plan §3 Log format JSON schema; per obs-plan §4 Scenario `wait` / `last` event-driven readback).
- `viola revive`'s preflight is the model for a cause in two forms: one `process-exit` line with no child started, and under `--json` the same-shaped document with the code, the logged line unchanged (per obs-plan §6 `detail` code catalog).
- A refusal with no line is named as such in the plan's prose, with its stderr pair and its document, and is left out of the `detail` counts (per obs-plan §5 Metric Coverage detail, the `viola.wait.outcome` notes; per obs-plan §2 Trigger coverage map, Founder Direction 4).
- Lines reach the sink through `obs_event!` with explicit typed fields only (per obs-plan §8 Integration points).
- Derived counts group by the closed `detail` value; `instance`, `pid`, `conn` and `corr` stay join keys (per obs-plan §5 Metric label cardinality discipline).

## Anti-patterns to avoid
- NEVER put paths or pids inside `detail` codes, and never log a field outside the §6 catalog (per obs-plan §11 PII Scrubbing).
- NEVER add an `event` value or rename a tests field without a Decisions Log entry; causes are told apart in `detail`, not in new `event` values (per obs-plan §11 Logs).
- NEVER print an anyhow chain that still holds a serde source to stderr, `--json`, MCP `isError` or channel `error.data` (per obs-plan §11 Error Reporting).

## Contract bindings
- obs ↔ tests §3 Log format: `event`, `corr`, `process` and `instance` stay as bound, and obs only adds (per obs-plan §3 Log format JSON schema). G4's check body is tests-owned, both schemas obs-owned (per obs-plan §9 G4, schema conformance).
- obs ↔ tests §6 Exit-cause matrix, four plan readings the `tests/cross_exit_causes.rs` table meets:
  - `internal-error` on the short-lived `cli` role is exactly `error: internal error` with no hint, which sits against the table's last-line `hint:` assertion (per obs-plan §7 Panic hooks).
  - `already-live` is the one `detail` for a `live` or a `stale` holder, which sits against the scope listing already live and stale heartbeat as two causes (per obs-plan §6 `detail` code catalog).
  - `interrupted` (`viola verify` stopped, exit 1) is a catalogued cause the scope's list does not name (per obs-plan §6 `detail` code catalog).
  - `instance-dead` is the only catalogued exit-21 word for a name that does not answer, so whether an unwrapped name gets its own code, and in which of the three surfaces, is research's question (per obs-plan §6 `detail` code catalog).
- obs ↔ security error rules: the plan's `session-live` entry reads as landed, against the scope's CARRY; the code decides, and that is research's question (per obs-plan §6 `detail` code catalog).
- obs ↔ arch / design-system / security-plan / test-plan document text: §4, §5, §6 and §7 name `{"v":1,"error":"instance-unreachable","detail":null}` and `detail: null` as the refusal output, so a moved document amends those obs-plan sections too (per obs-plan §6 Filesystem refusals; per obs-plan §7 Error classes captured).
- obs ↔ harness: `agent-run status` exposes a closed `last_error`; whether a new code reaches that enum is research's question (per obs-plan §3 Snapshot / paste-to-AI integration).
- obs ↔ exit-1 no-record paths: `viola mcp` at its catch site writes no self `process-exit`, and a refused check of its home or `diagnostics/` runs before any role file opens, so both are named outside the table or carried by a reason (per obs-plan §7 Panic hooks; per obs-plan §7 Error Capture detail, the pre-init window).

## Acceptance criteria contributions
- (obs) Each table cause that writes a line writes one `process-exit` whose `detail` is its own closed code, at `ERROR` for exit 1 and `WARN` for exit 21, and no two causes share a `detail` unless the table names why (per obs-plan §6 `detail` code catalog).
- (obs) Each strict-modes cause refused before a log opens is asserted to leave no `process-start`, no `process-exit` and no role-file line, with an exit code and a stderr pair or `--json` document as its record (per obs-plan §6 Filesystem refusals).
- (obs) Every new or moved code is in `schemas/diag-line.v1.json` where it is logged, and G4 reads zero conformance failures over the table's homes under `target/e2e-home/` (per obs-plan §9 G4, schema conformance).
- (obs) No code on `--json`, MCP or a process-log line holds a path, a pid or upstream text, and the secret scan reads zero hits over the `diagnostics/` files of those homes (per obs-plan §8 Verification).
