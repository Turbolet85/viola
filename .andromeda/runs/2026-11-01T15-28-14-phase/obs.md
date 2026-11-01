# obs extract

## Relevance
partial: obs does not govern the directory, socket and euid checks themselves; it binds what each new refusal writes on every role, and the codes-only floor on those writes (tier Standard (1) with Minimal exporter carve-outs, per obs-plan §1 Obs Scope Summary).

## Constraints
- Each cause behind exit 1 and exit 21 requires its own closed `detail` code holding no path or pid; the one exception is a check refused before any log opens, which writes no line and leaves its cause to the stderr pair or the `--json` document (per obs-plan §1 Telemetry triggers, Founder Direction 4). Which side of the log open the socket-directory refusal falls on is research's question: the plan places `channel.bind` after `run.version_gate` in the start chain, while the home-tree check precedes the log (per obs-plan §4 Scenario: `viola run` start sequence to child spawn).
- `schemas/diag-line.v1.json` closes `process-exit` `detail` at a counted set of values, and `hook-decision` `detail` at seven fail-open codes. A new word requires the catalog row and the schema enum together, and the count stated in the plan moves with it (per obs-plan §6 Log Coverage, `log-details`).
- `server-verify-failed` is defined for Linux and Windows only, on a server that is not the recorded wrapper or a check that cannot be made. A client euid mismatch reusing that word on macOS widens its definition, and a new word adds a catalog row; the plan names neither, so P4 chooses (per obs-plan §6 Log Coverage, `log-details`). The same row's macOS gap stays worded as a recorded gap, never as a comparison (per obs-plan §4 Span / Trace Coverage, the cli server check).
- A client-side refusal that lands before the first frame requires no `channel.request` span and no `channel-request` line (per obs-plan §4 Span / Trace Coverage, the cli server check). In `viola mcp`, an `instance-unreachable` from a frame tool is codes-only with no log line, its outcome kept inside `mcp.tool_call` (per obs-plan §7 Error Capture & Reporting, Error classes captured).
- `hook` stays exit 0 with empty stdout and stderr on any refusal of this chunk; a line goes only to `hook-<name>.ndjson`, and a refused instance check made before obs init writes none (per obs-plan §6 Log Coverage, `log-details`; the stream rule per obs-plan §3 Observability Harness Contract, preamble).
- The socket directory path, the endpoint, the peer's and the owner's euid or uid, and any pid appear in no refusal pair, `--json` document, MCP result or home-level line. `/tmp/viola-<uid>/` embeds a uid, so printing that path leaks it as well (per obs-plan §8 PII Scrubbing, data classification rules). `channel.bind` keeps `endpoint_kind` as its only attribute and never records the endpoint name (per obs-plan §4 Scenario: `viola run` start sequence to child spawn).
- `viola list`'s refused scan of the home, `instances/` or an instance exits 21 and writes no line on either form; making the board's home check one rule must hold that, and whether the code does at the home rule's new reading is research's question (per obs-plan §6 Log Coverage, `log-details`, Filesystem refusals).

## Patterns to follow
- The server check's line as the shape for a logged client refusal at connect: one `process-exit{subject:"self", exit_code:21, detail, during:"connect"}` at WARN into `cli-<name>.ndjson`, inside the dispatch span, with `link` / `unlink` writing to the driven instance's file (per obs-plan §6 Log Coverage, `log-details`).
- Start and client refusals as two fixed stderr lines through `src/human.rs` `refuse` → `write_refusal`, one `write_all` on the locked stderr with no print macro. The per-caller site counts are re-derived by grep, so a new call site moves them (per obs-plan §3 Observability Harness Contract, preamble; per obs-plan §11 Logs).
- All emission through `obs_event!` with explicit typed fields; a new span is static-named under `#[instrument(skip_all, name = "...", fields(...))]`, with `corr` and `instance` passed as values (per obs-plan §4 Span / Trace Coverage, naming convention; per obs-plan §11 Spans / Traces).
- The no-digits witness: `digit_runs` over every output and the role-file line, as `tests/security_negatives_strict_modes.rs` runs it on Linux, macOS and Windows. `tests/security_negatives_server_identity.rs` compiles out on macOS, so it cannot carry a macOS euid or directory case (per obs-plan §8 PII Scrubbing, Verification).

## Anti-patterns to avoid
- Paths or pids inside a `detail` code, or any field outside the §6 catalog (default-deny), a uid or euid field included (per obs-plan §11 PII Scrubbing).
- A new `event` value without a Decisions Log entry. The wrapper's refusal of a connecting peer comes before any frame, so it holds no `corr` and no `channel-request`; the plan names no line for it, and a line, if P4 wants one, must fit the closed enum or take that entry (per obs-plan §11 Logs).
- Any print macro, a raw `tracing` level macro, or stderr output on the `run` and `hook` paths while the child runs or a hook is served (per obs-plan §11 Logs).

## Contract bindings
- obs ↔ tests §3 Log format: a new `detail` or `event` word is additive under the binding schema, nothing renamed, and is checked by G4 against `schemas/diag-line.v1.json` (per obs-plan §3 Log format JSON schema; per obs-plan §9 CI Integration, G4).
- obs ↔ tests §6 and `tests/cross_exit_causes.rs` (scope's rows, and the macOS case of the directory loosened to 0755 → exit 21): the word on the logged line, where one is written, must equal the code the catalogue row and the `--json` document carry (per obs-plan §6 Log Coverage, `log-details`).
- obs ↔ security Vector 1 and Vector 7: refused filesystem and channel checks stay codes-only; a refused `diagnostics/` folder or file remains a failed open with its line dropped (per obs-plan §1 Telemetry triggers).
- obs ↔ tests perf gate: any euid or directory check on a hook's connect sits on the timed path judged by `gate --require perf`, its hyperfine `max` under the spine deadline (per obs-plan §10 SLO Invariants & Telemetry Budgets, performance budgets).

## Acceptance criteria contributions
- Every line a new refusal writes passes G4 against `schemas/diag-line.v1.json`, its `detail` a member of the closed enum and of the §6 catalog (per obs-plan §9 CI Integration, G4).
- A refused socket directory and a peer euid mismatch, read on every surface they reach, show no whole number, absolute path or endpoint in any output or role-file line, on Linux and on macOS (per obs-plan §8 PII Scrubbing, Verification).
- With a refused directory or a mismatched peer, `viola hook` exits 0 with empty stdout and stderr and sends no frame (per obs-plan §3 Observability Harness Contract, preamble).
- A client euid mismatch leaves no `channel-request` line in the client's role file, and no test reads the macOS server check's no-comparison match as a comparison (per obs-plan §4 Span / Trace Coverage, the cli server check).
