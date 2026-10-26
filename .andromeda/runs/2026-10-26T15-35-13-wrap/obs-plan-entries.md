## 2026-10-26-explicit-session-links — the link verbs' instance check before their log: a refused check writes no line
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: the pre-step-4 window) · §6 Log Coverage (log-coverage.md: Filesystem refusals, Vector 7) · §1 Obs Scope Summary (obs-scope-summary.md: the Vector 7 trigger's no-line strict-modes refusals; the Founder Direction 4 trigger's strict-mode item) · §10 SLO Invariants & Telemetry Budgets (slo-invariants-telemetry-budgets.md: bounded exemptions 1 and 7) · §2 Telemetry Strategy (telemetry-strategy.md: the panic exemptions) · §3 → OTel SDK init (registries/contracts/obs-plan/otel-sdk-init.md: per-role anchors, a new `cli` anchor)
**Change:**
- §7's pre-step-4 window and exemption 1 add the instance check `viola link` / `viola unlink` run on the driven instance before their `cli` role file opens.
- Its refusal — exit 21 `instance-unreachable`, the fixed instance-state pair on stderr or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` — opens no role file, writes no line in any log (no `process-start`, no `process-exit`) and leaves the directory's mode as found: §6 Vector 7, exemption 7 (link and unlink: exit 21, the pair or the document) and §2's exemption list.
- §1 Vector 7: the strict-modes refusals that leave no line, was four (`revive`'s check, `hook statusline`'s check, the `budget.json` read, `list`'s scan); now five, the link verbs' check of the driven instance added. Founder Direction 4: their strict-mode fail carries no logged `detail`.
- OTel SDK init, a new `cli` anchor: the verbs' instance is the driven name from their own argument, log `cli-<driven>.ndjson`; `check_instance` on the home and `instances/<driven>` runs before the log init (`client::start`), since the role-file open narrows an existing home to 0700 and a check made after it passes a home at 0770; only after a passed check does the log open with `process-start`, the liveness pre-check and the server check following.
**Why:** the chunk built the verbs' shared call with `check_instance` before `client::start`, in the form the founder gave as his own answer to this chunk's P4 fork (live by dialog, relayed verbatim by the operator): the check before any log or snapshot read, a refusal exit 21 (as `list` / `plugin install`, not `revive`'s exit 1) with no log and no line. The order trap is held by `link_verbs_refuse_a_widened_instance_before_any_log`, which reads the mode after the refusal and reads red with the check moved.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — `channel-response` logs `method` only for a closed-list name
**Section:** §6 Log Coverage (log-coverage.md: the field catalog's `channel-response` row)
**Change:** was `method` listed bare; now `method` is logged only for a name on the channel's closed method list (`link` and `unlink` among them), and any other name logs no `method` key.
**Why:** measured, disproving the plan's assumption that an unserved method's conn-less response line logs its `method`: the channel's `method_label` names only the closed list, so `channel_wrapper_logs_each_call_with_corr_and_conn` now asserts the key's absence for its unserved example (`anything`). A test asserting a `method` value on a line for a name outside the list is wrong by construction.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — `server-verify-failed`: eight CLI frame verbs
**Section:** §6 Log Coverage (log-coverage.md: the `process-exit` exit-21 line) · §1 Obs Scope Summary (obs-scope-summary.md: the Founder Direction 4 trigger's server-verify item) · §4 Span / Trace Coverage (span-trace-coverage.md: the cli server-check block)
**Change:**
- Was the six CLI frame verbs, per «2026-10-26-server-verification-before-any-frame — `server-verify-failed` written: the CLI exit-21 cause and the hook fail-open code»; now eight (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`), each writing `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN — `link` / `unlink` into the driven instance's `cli-<driven>.ndjson` — with no pid, start time, path or endpoint.
- §4: was the six, per «2026-10-26-server-verification-before-any-frame — the pre-frame server check's place in the span tree»; now the eight verify the wrapper through `cmd::client::connect` inside their dispatch span after their own client-side refusals — for `link` / `unlink`, their instance check of the driven instance, then the liveness pre-check — and before any frame.
**Why:** the report's counts: CLI frame verbs six to eight, `cmd::client::connect` callers six to eight; each link verb's one request goes through it, its refusal inside its span asserted by its `request_refuses_an_unverified_server_inside_its_dispatch_span` case on Linux and Windows (compiled out on macOS, the recorded gap).
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — `v_rejected`'s over-count: the link methods' `-32602` causes
**Section:** §5 Metric Coverage (metric-coverage.md: the `unknown` fallback / higher-`v` rejections row; obs-plan.md: Per-surface / per-path metrics) · §1 Obs Scope Summary (obs-scope-summary.md: the compat trigger's higher-`v` item)
**Change:** the `-32602` causes behind `viola.compat.v_rejected`'s over-count, was `send`'s invalid params and `release` carrying `from`; now also a `link` / `unlink` whose `driver` or `driven` is missing, not a string or no valid name, whose `driven` is not the serving wrapper's own name, or whose `driver` equals `driven` (the body's broad line: a bad `link` / `unlink` name or pair). The over-count stands until the line tells the causes apart.
**Why:** the served methods read both names through `ViolaName::try_new` and answer each of these with `ProtocolError::InvalidParams` (`-32602`, `data: null`), no line written.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the `human::refuse` census, 14 to 15
**Section:** §11 Obs Anti-Patterns → Logs (obs-anti-patterns.md: the print-macro ban's `src/human.rs` exception) · §3 Observability Harness Contract (obs-plan.md: the `run` diagnostic-output bullet)
**Change:**
- Was 14 (`run` 5 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1), per «2026-10-17-optional-plugin-install — plugin install's span, log shape and no-line refusal»; now 15 with `link` 1, the one `link_call` site `viola link` / `viola unlink` share, re-derived by `grep -rcE 'human::refuse\(' src` at chunk 2026-10-26-explicit-session-links.
- §11 adds the link verbs' instance-check refusal to the refusals written through `refuse` / `write_refusal`, and states the rule beside the count: the `human::refuse(` call sites under `src/`.
- §3: the `list` colour-layer clause no longer names a chunk ("`list`'s colour layer", was "that chunk's").
**Why:** the report's count by the rule its master states (verify 5 · run 5 · list 2 · revive 1 · plugin 1 · link 1); the verbs' refused instance check writes the fixed instance-state pair through `human::refuse`.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link verbs log into the driven instance's `cli` file, inside `cli.link` / `cli.unlink`
**Section:** §1 Obs Scope Summary (obs-scope-summary.md: the cli Exporter) · §4 Span / Trace Coverage (span-trace-coverage.md: the span kinds' `CLIENT` list; the surface table's cli row)
**Change:**
- Exporter: `viola link` / `viola unlink` log as `cli` into the driven instance's `cli-<driven>.ndjson` (the instance their `<driven>` argument names, never the caller's `VIOLA_NAME`) once their instance check of it has passed; a refused check opens none.
- `CLIENT` gains `cli.link` and `cli.unlink`.
- cli row: the spans `cli.link` / `cli.unlink` hold the verbs' one request; the verbs run their instance check of the driven instance before any role file opens, then open `cli-<driven>.ndjson` and write `process-start` / `process-exit`; a refused check (exit 21) opens no process log and writes no line.
**Why:** the verb's instance is the driven one: it is dispatched with the driven name's `cli` sink, `client::start` opens the `cli-<driven>` role file after the check, and each request runs under `#[instrument(skip_all, name = "cli.link")]` / `name = "cli.unlink"`.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link pair names in the PII table
**Section:** §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: the classification table, a new row)
**Change:** a new row: `link` / `unlink` pair names (`driver`, `driven`, each a `ViolaName`), Low — OK in the contract payload, the wrapper's `link` / `unlink` event lines in `events.ndjson` (`data: {driver, driven}`, `source: wrapper`), never in a process-log line. The served methods open no span and write no process-log line of their own, and the wrapper's `channel-request` / `channel-response` pair logs `method`, never the `driver` / `driven` params; the verbs' result lines print their own parsed `ViolaName` arguments, never a name read back from the reply or the log; asserted by `link_verbs_log_the_channel_pair_and_no_names` (`tests/cli_links.rs`).
**Why:** the served methods and the first product `unlink` line bring a user-supplied name class onto the logged surface, which the table did not classify; the report's coverage reads no driver name in the wrapper's process log, asserted.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — E3: the explicit methods served, the hold retired
**Section:** §4 Span / Trace Coverage (span-trace-coverage.md: Edge flows E3, the paths block and a new block for the explicit methods; obs-plan.md: Edge flows E3)
**Change:**
- Was «the explicit methods, which are not built: neither is served (both answer `-32601`), and no product path appends an `unlink` line», per «2026-10-10-session-links — a link has no line and no span»; now the driven wrapper serves both, and the `method:"link"|"unlink"` pair is theirs. The "as landed, only the implicit link exists" qualifier is gone; the implicit link's text stands.
- New block: the wrapper reads `driver` and `driven` as strings through `ViolaName::try_new`; a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null` and no line, a `record` / `remove` error is `-32603`, and neither method reads `from`. Neither opens a span or writes a process-log line of its own: the trace is the channel pair (holding neither name), the product line and the snapshot's `links`. `link` appends one `link` line `{driver, driven}` (`source: wrapper`) for a new pair, nothing for a held one; `unlink`, `LinkSet::remove`'s one product caller, appends one `unlink` line only when it removes a pair; each answers after its line and the snapshot write.
- obs-plan.md E3: was "only the implicit link exists … the explicit methods are not built"; now the implicit link and the served methods have no process-log line, no `event` value and no span of their own, traced by the call's channel pair, the product line in `events.ndjson` and the snapshot's `links`; the verbs log into `cli-<driven>.ndjson`, their request inside `cli.link` / `cli.unlink`.
**Why:** the report retires the hold: the wrapper serves both (was `-32601`) and a product path writes an `unlink` line, the first; built in the founder's form, his own answer to this chunk's P4 fork (live by dialog, relayed verbatim by the operator).
**Kept:** the implicit link's shapes stay the operator's answers, not ruled by the founder (architecture [Links]).
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/
