# obs extract

## Relevance
partial — obs-plan names `viola plugin install`'s dispatch span and process-log lines, and the new precedence probe extends `viola verify`'s logged spawn pairs and stdout step count; the marketplace write and the exit-1 `viola mcp` measurement add no obs surface of their own.

## Constraints
- obs-plan §4 `verify` / `plugin install` requires the dispatch span `cli.plugin_install` and, only with `VIOLA_NAME` set, `process-start` / `process-exit{exit_code, detail}` into `cli-<name>.ndjson`; run from a plain terminal with no instance it writes no process-log file (D-06). Whether the code already declares `cli.plugin_install` or any line for the verb is research's question (the verb is unserved at take-up).
- obs-plan §4 `verify` / `plugin install` enumerates `viola verify`'s spawn pairs (eight, each `process-start{subject}` / `process-exit{subject, child_exit_status, duration_ms}`) and its stdout `[NN/MM]` step line per relied row with `MM` = `LedgerRow::relied()` count (23 Unix / 20 Windows at take-up); a 24th ledger row, and any child a precedence probe spawns, moves both the pair count and `MM`, and an interrupted run still closes every pair it opened before `process-exit{detail:"interrupted"}`.
- obs-plan §6 Boundary-call wrappers (Child / shell spawns) requires every child spawn to log a pair whose `subject` is in the closed `$defs.subject.enum` of `schemas/diag-line.v1.json`; a probe that spawns `claude plugin …` (or any child not covered by `verify-probe` / `verify-pty-probe` / `agents-probe`) needs either an existing subject or a new value added to the §6 catalog and the schema (a Decisions Log entry per §11 Logs).
- obs-plan §6 `detail` code catalog closes `process-exit` exit-1 `detail` at 13 values in `schemas/diag-line.v1.json`; an exit-1 cause of `plugin install` other than `internal-error` (e.g. a refused home check or marketplace write) needs a new closed kebab-case code added to catalog and schema, never a path or pid (Founder Direction 4).
- obs-plan §8 PII Scrubbing (data classification table, Absolute paths row) keeps home-level lines path-free: the absolute exe path written into the marketplace `.mcp.json` and the `<home>/marketplace/` path never reach a span field or log line; paths appear only in `instances/<name>/diagnostics/detail-*.ndjson`.
- obs-plan §7 Error Capture & Reporting (Per-role behaviour, short-lived `cli`) requires an internal fault to write `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` under an instance, the chain only in `detail-cli.ndjson`, and exactly `error: internal error` on stderr; no anyhow chain with a serde source reaches stderr or `--json`.
- obs-plan §6 Log levels mapping puts `process-exit` with `exit_code` 1 at `error`, exit 21 at `warn`, and every other lifecycle line (including refusals) at `info`.

## Patterns to follow
- Span declaration `#[instrument(skip_all, name = "cli.plugin_install", fields(...))]` with lines only via `obs_event!` (obs-plan §2 Naming conventions; §4 Span naming convention).
- `verify`'s call-site spawn-pair logging (pairs written at the call site, `run_bounded` / `run_capped` unlogged; `run::agents::read_logged` for the agents read) for any child the precedence step spawns (obs-plan §6 Boundary-call wrappers).
- `viola list`'s ordering: a strict-modes check before the `cli` role file opens, a refused check opening no log and writing no line (obs-plan §4 Instrumentation per surface, cli row); if `plugin install` adopts it, the no-line shape joins the obs-plan §10 bounded exemption 7 list as an amendment, not by silence.
- Human refusals through `src/human.rs` `refuse` → `write_refusal` (one `write_all` on locked stderr), whose caller census is recorded in obs-plan §3 Observability Harness Contract and moves with a new caller.

## Anti-patterns to avoid
- No telemetry on stdout and no `print!`/`eprintln!`/`dbg!` in the verb's path: stdout is the `--json` result, stderr the human pair (obs-plan §11 Logs).
- No field outside the §6 catalog (no exe path, marketplace path, plugin JSON or CLI output in a line) and no path or pid inside a `detail` code (obs-plan §11 PII Scrubbing).
- No bare `#[instrument]`, no `err` / `ret` on the new span (obs-plan §11 Spans / Traces).

## Contract bindings
- obs ↔ tests: `schemas/diag-line.v1.json` closed `subject` and `process-exit.detail` enums, checked by G4 over `target/e2e-home/**`; the chunk's test homes must sit under `target/e2e-home/` so G2, G4 and the secret scan read them (obs-plan §9 Pipeline integration, Integration tests row; §9 Gate commands G4).
- obs ↔ security: the NEVER-PATH-lookup absolute `command` lives on disk in `.mcp.json`, never in a log line (obs-plan §8 PII Scrubbing).
- obs ↔ design (CLI streams): `verify`'s `[NN/MM]` step lines and `stamped <version>  N pass  N fail` summary move with the new relied row (obs-plan §4 `verify` / `plugin install`).
- obs ↔ mcp: the exit-1 `viola mcp` path being measured is obs-plan §10 bounded exemption 7 (refused home / `diagnostics/` check: exit 1, nothing on stdout or stderr, no line), so its evidence is the CLI's start behaviour, not a viola log line.

## Acceptance criteria contributions
- With `VIOLA_NAME` set, `viola plugin install` writes one `process-start{subject:"self"}` and one `process-exit{subject:"self", exit_code}` to `cli-<name>.ndjson`, both passing G4; with no `VIOLA_NAME` no `diagnostics/` file is created (per obs-plan §4 `verify` / `plugin install`).
- No home-level line from `plugin install` or the new verify step holds an absolute path, the pinned/`current_exe` path or the marketplace path; the secret scan over the chunk's homes is clean (per obs-plan §8 PII Scrubbing).
- Every child the precedence step spawns logs a pair with a closed `subject`, `child_exit_status` and `duration_ms`, and the §4 pair count and relied `MM` are amended to match (per obs-plan §6 Boundary-call wrappers).
- G2 (zero panics) and G4 (schema conformance) pass over the chunk's test homes (per obs-plan §10 Error budget).
