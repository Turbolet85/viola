## 2026-10-31-exit-cause-code-catalogue — the `cli` catch site's document under `--json`
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: the main-thread catch site's `revive --list` clause, the new `cli` catch site item; obs-plan.md: the short-lived `cli` per-role line) · §1 Obs Scope Summary (obs-scope-summary.md: the cli surface's stderr note)
**Change:**
- Under `--json` the `cli` catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error`, stderr then empty, exit 1 (was that stderr line alone).
- `internal_error_document` (`src/cmd/mod.rs`) prints it; `json_asked` (`src/main.rs`) decides, reading `--json` from argv before any `--`.
- `viola revive <name> --list` at that site takes the document under `--json`.
- Without `--json` the line is `error: internal error`; `run`, `revive`'s start, `mcp` and `hook` print neither there.
**Why:** the founder's own word, `inputs#I2` Q1: live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after this widening was shown to him; exit 1 and the human line stay by that answer. That these rows lie inside it is the overseer's confirm as delegate, `inputs#I6`, PROVISIONAL; the new item went in on the delegate's Q6 of that record, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — every refusal of `revive --json`'s start prints its document
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: the new `revive --json` start errors, `pump_child` and start refusals items) · §6 Log Coverage (log-coverage.md: the exit-1 list's `revive` clause)
**Change:**
- Every refusal of `revive`'s start under `--json` is the same-shaped document with its code, the logged line unchanged (was its preflight refusals alone).
- `refused_with` (`src/cmd/run.rs`) prints the exit-1 document in place of the human pair for `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`, and the `internal-error` document for a program not found, which has no human pair, all before `spawn_child`; `run` passes `Launch`'s `json` false and `revive` passes its `--json`.
- An anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints the `internal-error` document and returns the same error, so the catch site logs its chain and its line.
- A `pump_child` error prints no document: the child has held the terminal, and viola writes nothing past it.
**Why:** the chunk closed the `revive --json` post-preflight gap, the CARRY of chunk 2026-10-17-cli-machine-contract. `run`'s start reaching `--json` through `revive` is the founder's own word, `inputs#I2` Q4, live, 2026-10-31 before 16:52Z, through the overseer's dialog; that the log-coverage row lies inside it is the overseer's confirm as delegate, `inputs#I6`, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — exit 21's document names its cause
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: the pre-init window's `link` / `unlink` refusal, the new exit-21 item) · §1 Obs Scope Summary (obs-scope-summary.md: the `viola-core` entity, the Exporter, the dialog path's required fields) · §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: the frame verbs' strict-modes refusal) · §4 Span / Trace Coverage (span-trace-coverage.md: the CLI instance check, `wait` / `last`, dialog → `answer`; obs-plan.md: the `wait` / `last` line) · §3 OTel SDK init (otel-sdk-init.md: the `cli` anchor)
**Change:**
- Exit 21 prints one document per cause, `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` (was one form, `detail` null).
- `detail` is one of the five closed words of `viola_core::UnreachableCause`: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`. No serde path reaches them; the two clients write each word into their two documents, and the type writes no log line.
- `unreachable_document` (`src/cmd/client.rs`) is the document's one writer, `refuse_untrusted` and `src/cmd/plugin.rs` through it; `refused_document` in `src/cmd/list.rs` builds `strict-modes-failed` as a `Value` for `board_document`, the board's only refusal.
- A refused instance check of the six frame verbs and `link` / `unlink` reads `detail` `strict-modes-failed` under `--json` (was null), still with no line.
**Why:** the founder's own word, `inputs#I2` Q1: live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after this widening was shown to him. That these rows lie inside it is the overseer's confirm as delegate, `inputs#I6`, PROVISIONAL; the exit-21 item and the `viola-core` wording went in on the delegate's Q6 of that record, PROVISIONAL.
**Kept:** `v` stays 1, `error` stays `instance-unreachable`, and a refused strict-modes check writes no line.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — MCP's `instance-unreachable` body names its cause
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: MCP rejects, the new `UnreachableCause` map) · §6 Log Coverage (log-coverage.md: the other refused strict-modes checks) · §5 Metric Coverage (metric-coverage.md: a refused `wait`) · §1 Obs Scope Summary (obs-scope-summary.md: the MCP `isError` note, the MCP trigger) · §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: the MCP frame tools) · §4 Span / Trace Coverage (span-trace-coverage.md: the viola-mcp server check, `wait` / `last`, dialog → `answer`; obs-plan.md: the `wait` / `last` line)
**Change:**
- A frame tool's refused `check_instance` returns `instance-unreachable` with `detail` `strict-modes-failed` (was `detail: null`); a failed server check reads `server-verify-failed`; no `live_endpoint` reads `without_endpoint`'s `instance-dead` or `unwrapped-name`.
- The `isError` body is the CLI's document less `v`, its text `refused: instance-unreachable`.
- The map: in `frame` a refused `check_instance` is `StrictModes`, no `live_endpoint` is `without_endpoint`'s reading, a failed `server_verified` is `ServerVerify`; in `of_board` (`crates/viola-mcp/src/result.rs`) the board's refusal is `StrictModes`; in `of_reply` `Connect` is `InstanceDead`, `Closed` or `Io` is `EndpointLost`.
- The MCP trigger reads these rejected calls write no line (was one rejected call), each with no frame; no metric counts a refused `wait`.
**Why:** the founder's own words, `inputs#I2` Q1 and Q4: live, 2026-10-31 before 16:52Z, through the overseer's dialog, after this widening was shown to him. Q1 puts the cause in `detail` with MCP's text unmoved; Q4 has MCP's rejects write no line, `isError` their record. That these rows lie inside them is the overseer's confirm as delegate, `inputs#I6`, PROVISIONAL; the map went in on the delegate's Q6 of that record, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — a stale holder's code is `stale-heartbeat`
**Section:** §6 Log Coverage (log-coverage.md: the exit-1 list, a new continuation line) · §1 Obs Scope Summary (obs-scope-summary.md: the `viola run` start path's required log fields) · §4 Span / Trace Coverage (span-trace-coverage.md: the start's failure `detail` set, `revive`'s preflight) · §3 Logging stack (logging-stack.md: the `run-<name>.ndjson` item), OTel SDK init (otel-sdk-init.md: the `run` anchor)
**Change:**
- A `stale` holder, from `viola run` or `viola revive`, is `stale-heartbeat`, under `revive --json` the same-shaped document (was `already-live`).
- `already-live` is a live holder or a fresh replayed beat; a `gone` holder is still taken over with no exit-1 line.
- `revive`'s logged preflight refusals name `stale-heartbeat` beside `already-live`.
- `stale-heartbeat` is exit 1's cause alone, written by `collision_check` (`src/cmd/run.rs`): `live_endpoint` gives none for a stale wrapper, so a frame verb against a stale instance reads `instance-dead`.
**Why:** the chunk has `collision_check` print and log `stale-heartbeat` for a `stale` holder and retires `already-live` as that holder's refusal. The continuation line went in on the overseer's answer as delegate to Q6, `inputs#I6`, PROVISIONAL.
**Kept:** the `run.collision_check` span's `outcome` stays `already-live` for a live or a stale holder: its closed set says a holder stands and did not move.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the `process-exit` `detail` enum's count
**Section:** §6 Log Coverage (log-coverage.md: the `interrupted` item)
**Change:**
- `schemas/diag-line.v1.json` closes the `process-exit` `detail` at 19 values, as measured at this chunk (was 13): the enum's members, the words of the exit-1, exit-21 and exit-20 lists there.
- The clause's history of earlier counts (12, 11, 9, each before its chunk) is gone.
**Why:** the enum gained six: `stale-heartbeat`, `unwrapped-name`, `endpoint-lost`, `program-not-found`, `version-unreadable`, `unclean-fixture`. The count stands by its rule, the enum's members.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — `verify` writes its refusal's code
**Section:** §6 Log Coverage (log-coverage.md: the exit-1 list's strict-modes clause, a new continuation line, Filesystem refusals) · §1 Obs Scope Summary (obs-scope-summary.md: the Vector 7 signal; obs-plan.md: the Vector 7 signal) · §4 Span / Trace Coverage (span-trace-coverage.md: `verify`'s home-tree and stamps item)
**Change:**
- `viola verify` writes its refusal's code on the `process-exit` it writes under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside `interrupted`.
- A refused `update_stamps` leaves that `process-exit` with exit 1, `detail` `strict-modes-failed` (was no `detail`), so a strict-modes failure has a line in three places (was two), the broad line's "only" naming it too.
- On `run`'s and `revive`'s start `strict-modes-failed` is not one of the exit-1 codes (was no longer one of them, without the scope).
- A refused home tree writes no line, a failing row is no refusal and carries no code, the fixture's `<file>` enters no code, and `verify` takes no `--json`.
**Why:** `verify` has no `--json`, so its causes reach the log and the hint: the founder's own word, `inputs#I2` Q4, live, 2026-10-31 before 16:52Z, through the overseer's dialog. The scoped clause and the continuation line went in on the overseer's answer as delegate to Q8, `inputs#I6`, PROVISIONAL.
**Kept:** on `process-exit` exit 21 no arm of this build writes `strict-modes-failed`.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — exit 21's logged causes and their `during`
**Section:** §6 Log Coverage (log-coverage.md: the exit-21 list) · §1 Obs Scope Summary (obs-scope-summary.md: the `wait` / `last` path's required log fields, the vanished-endpoint trigger) · §4 Span / Trace Coverage (span-trace-coverage.md: the confirmed `send` path, its Cleanup and new `during` item, `wait` / `last`'s client `process-exit`)
**Change:**
- `process-exit` exit 21 lists `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed` (was `instance-dead`, `strict-modes-failed`, `server-verify-failed`).
- `during` is `call` for `endpoint-lost` alone; `instance-dead`, `unwrapped-name` and `server-verify-failed` are `connect` (was `instance-dead` with `connect` or `call`).
- A wrapper killed mid-wait and an endpoint vanished during `wait` read `detail` `endpoint-lost`, `during` `call`.
- `viola_state::liveness::without_endpoint` reads a no-endpoint instance by `symlink_metadata`: a directory is `instance-dead`, anything else (absent, a regular file, a symlink) `unwrapped-name`. It opens nothing and writes no line of its own, and the instance check stays first at every caller (CLI `Unreachable::without_endpoint`, MCP `frame`).
**Why:** the chunk retires `instance-dead` · `during` `call`: a wrapper lost with the call open is `endpoint-lost`. The `without_endpoint` and `during` items in the `send` path went in on the overseer's answer as delegate to Q6, `inputs#I6`, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — Founder Direction 4's codes and named carve-outs
**Section:** §1 Obs Scope Summary (obs-scope-summary.md: the Founder Direction 4 note and trigger, the trigger's new carve-out item; obs-plan.md: alignment gap (a)) · §2 Telemetry Strategy (telemetry-strategy.md: the Founder Direction 4 row)
**Change:**
- The trigger names the codes: exit 21 `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed`; exit 1 `already-live`, `stale-heartbeat`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, and `revive`'s `no-session`, `session-live`, `cwd-missing` (was the causes in words, no `session-live`). `link` / `unlink`'s `--json` document reads `detail` `strict-modes-failed` (was null).
- Two causes stand outside the distinct-code rule, both with the route entry "Unix endpoint and home hardening": the socket-directory cause has no code (`bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site) and macOS has no server check. OPEN — the socket-directory cause has no code, PROVISIONAL · owner: leaving its code to that entry. The note names both carve-outs.
- The telemetry row reads no logged `detail` code (was no `detail` code), the `--json` form the `instance-unreachable` document with `detail` `strict-modes-failed` (the frame verbs and `link` / `unlink`) or the `strict-modes-failed` document (`revive`).
- Gap (a): the distinct codes are an additive `detail` field on `process-exit` (`schemas/diag-line.v1.json`) (was can be).
**Why:** the carve-outs are the founder's own word, `inputs#I2` Q4, answer "Claim with named carve-outs": live, 2026-10-31 before 16:52Z, through the overseer's dialog; the cause document is his `inputs#I2` Q1. That these rows lie inside them is the overseer's confirm as delegate, `inputs#I6`, PROVISIONAL; the carve-out item went in on the delegate's Q6 of that record, PROVISIONAL. Leaving the socket-directory code out of this chunk is the delegate the overseer's word, `inputs#I3`, PROVISIONAL. The chunk measured that the code holds no socket-directory refusal.
**Kept:** a strict-modes check refused before any log opens writes no line and carries no logged code.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/
