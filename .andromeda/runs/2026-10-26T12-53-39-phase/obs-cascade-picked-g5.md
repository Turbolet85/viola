# obs · picked accounting · group 5 of 5

## Picked
log-coverage#1.1 · not in scope · standing S3 · every field a new line carries is named in the §6 field catalog; the verbs' `process-start{subject:"self"}` adds no subject or field to this row
log-coverage#1.2 · carried · the verbs' `process-exit` lines carry `exit_code`, `detail` and `during` (`connect` before the frame, `call` after it)
log-coverage#1.3 · carried · the two served methods' `channel-request` lines carry only this row's fields, never `driver` / `driven`
log-coverage#1.4 · carried · `link` / `unlink` move from the `-32601` class to served, and their two refusals are `-32602` responses
log-coverage#1.10 · not in scope · the `events.ndjson` append and the replay the `unlink` line goes through are used unchanged, and the scope names no torn-line or snapshot recovery
log-coverage#1.11 · carried · a logged link name failing its `ViolaName::try_new` re-parse, if logged, is a `parse-rejected` from this closed `parser` list or a new value joining it
log-coverage#2.1 · carried · the verbs' exit 1 causes (`internal-error`, and under P4's second form revive's no-line instance-check refusal) and the 13-value `process-exit` `detail` enum
log-coverage#2.2 · carried · the verbs' exit 21 `server-verify-failed` line, written today by "the six CLI frame verbs", a count the two verbs change
log-coverage#2.5 · carried · the closed `parse-rejected` `detail` list a failed link-name re-parse line would take
log-coverage#2.7 · carried · the Vector 7 rule P4's second form (revive's `check_instance` ahead of the snapshot read) takes: a stderr pair, exit 1 and no log line
error-capture-reporting#1.1 · not in scope · the panic hook's detail line and raw backtrace are a component the verbs only run under, unchanged, and the scope names no panic path
error-capture-reporting#2.1 · carried · `role_of` makes the verbs `cli`, so an internal fault reaches the `cli` catch site, and a panic in a `link` / `unlink` connection worker is contained by the `channel.dispatch` `Drop` guard
error-capture-reporting#3.1 · carried · under P4's second form the verbs' instance check runs in the pre-init window this list records
pii-scrubbing-compliance#1.8 · carried · the snapshot `endpoint` the verbs read is path-valued, so it follows the Medium rule and stays off home-level lines
pii-scrubbing-compliance#1.9 · carried · the verbs read `snapshot.json`'s `endpoint` (crossing (c)), which is never logged
pii-scrubbing-compliance#2.1 · carried · the verbs' server-verification refusal joins the `digit_runs` scan of `tests/security_negatives_server_identity.rs`
ci-integration#3.3 · not in scope · standing S3 · every field and `detail` code a new line carries passes G4 against `schemas/diag-line.v1.json`
slo-invariants-telemetry-budgets#1.1 · carried · under P4's second form the verbs' refused instance check joins bounded exemptions 1 and 7
obs-anti-patterns#1.1 · carried · the verbs' human refusals go through `human::refuse`, whose per-verb caller count the chunk changes
