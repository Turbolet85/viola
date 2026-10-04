### Log format JSON schema

**Binding. Reproduced verbatim from test-plan.md §3 Test Harness Contract → Log format (re-synced in overseer fix pass 2, 2026-09-24, after test-plan amendments D-21 / 693e083 and fix pass 2, and again in fix pass 3 for Z7). Obs aligns to tests, not vice versa.**

```markdown
- **Format:** JSON-per-line. There are two streams:
  - **Event stream:** product events in `instances/<name>/events.ndjson`, exactly the arch Event line `{"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}`.
  - **Process logs:** emitted by tracing-subscriber 0.3.23 `fmt().json().flatten_event(true).with_current_span(false)`, writing codes-only lines to `<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson`. Content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to `<home>/instances/<name>/diagnostics/detail-<process>.ndjson` (obs-plan D-08). Files are 0600, dirs 0700, with one `write` per line.
- **Required fields:**
  - For process logs: `timestamp` (RFC 3339 UTC with ms and `Z`), `level` (`DEBUG|INFO|WARN|ERROR`), `target`, and `fields.message` (or the flattened `message`).
  - `event`, a closed kebab-case enum. A new value needs a Decisions Log entry, like the §3 closed enums. Each value fixes what its `corr` holds:
    - `channel-request`, `channel-response`: `corr` = the JSON-RPC request `id`; null for an id-less `hook.event` notification and on a response to a frame that yielded no id (`error_code` -32700 or -32600)
    - `dialog-raised`, `dialog-answered`: `corr` = `dialog_id`
    - `hook-invoked`, `hook-decision`: `corr` = `dialog_id` for dialog hooks, otherwise null. A dialog `hook-invoked` may still be null (fail-open before `dialog_id` is known, D-07), and so may a `hook-decision` that carries `detail`
    - `send-issued`, `send-confirmed`, `send-refused`: `corr` = the send `cursor`. A send refused before `send-issued` (`human-typing`, `budget-paused`, `input-not-ready`, `turn-running`) has no arch cursor, so its wrapper-side `send-refused` carries `corr` = the target's `events.ndjson` end offset at the moment of refusal, a log join key only, never returned to the caller (obs-plan D-28). Wrapper-side `send-*` lines also carry `conn` and `rpc_id`, the originating `send` request's connection and JSON-RPC `id`, so `(conn, rpc_id)` joins them to that call's `channel-*` lines (obs-plan D-30). The client-side `send-refused{side:"client"}` has `corr` null.
    - `release-from-driver`: `corr` = the JSON-RPC request `id` (obs-plan D-01; the wrapper's `-32602` refusal of a `release` that carries a string `from`; a `from` of another type is a plain `-32602` with no `release-from-driver` line)
    - `process-start`, `process-exit`, `http-request`, `panic`: `corr` = null
    - `liveness-changed`, `state-recovered`, `sse-opened`, `sse-closed`, `parse-rejected`: `corr` = null (obs-plan D-02 … D-05)
    - `schemas/diag-line.v1.json` requires `corr` exactly where the rules above always define it: `dialog-raised`, `dialog-answered`, `release-from-driver`; `channel-request` unless `method` is `hook.event`; `channel-response` unless `error_code` ∈ {-32700, -32600}; `send-*` when `side` is `wrapper`; `hook-decision` when `hook_event` ∈ {`pre-tool-use`, `permission-request`} and no `detail`. `hook-invoked` never requires it.
  - `process` (`run|hook|mcp|ui|cli`; `cli` is a short-lived verb with a resolved instance, writing `cli-<name>.ndjson`, obs-plan D-06) and `instance` (a `ViolaName` or null).
  - `corr` is copied unchanged as a JSON number or string. It is never renamed (for example to `correlation_id`).
  - **Null encoding:** a null `corr` or `instance` is written as key **absence** (tracing has no null field value; obs-plan D-12). The harness and every `jq` / jaq assertion treat an absent key and `null` as the same value (`.corr == null` holds for both), and obs-plan's `schemas/diag-line.v1.json` rejects a literal `null`.

  obs-plan may add fields but must not rename or remove these. The harness greps on them.
- **Harness-side a11y rows (not part of this `event` enum):** `event:"a11y-violation"` is not a value of the product `event` enum above, has no `ObsEvent` variant, and is never emitted by a product process.
  - The a11y Playwright fixture writes one row per failing check into `e2e-web/test-results/a11y/*.ndjson`.
  - Rows follow a tests-owned schema of their own, `e2e-web/schemas/a11y-row.v1.json`. It reuses the diag-line field names without renaming any: `event` is const `"a11y-violation"`, `process` is const `"ui"`, `corr` and `instance` are absent, and the a11y plan's additive fields (a11y-plan §3 Structured violation JSON schema) are declared there.
  - The same schema-conformance check body that backs obs-plan G4 validates these files against that schema. G2, G4 and `schemas/diag-line.v1.json` never read them.
  - This revises the fix-pass-2 Y3 entry (overseer fix pass 3, Z7).
- **Constraints:**
  - The hook trace is the `hook-<name>.ndjson` file. A failed write there is swallowed, so the hook still exits 0 and never writes to stderr.
  - No line may contain the GUI token, a `Cookie` header, `?t=`, or any stripped `CLAUDE*` value. This is enforced by the secret-scan test in §6.
- **Agent parsing:** lines parseable with `jq -c 'select(.event=="dialog-raised")'` or equivalent; NEVER multi-line stack traces. A panic in a hook is caught and logged as one `level:"ERROR", event:"panic"` line.
```

**Obs extensions.** None of these renames or removes a tests field. Each enum value has its own Decisions Log entry (§12). All are ACCEPTED at review (2026-09-24, overseer, founder-delegated) and carried to test-plan.md as the tests amendment D-21, which the overseer applies right after this run.
- **Accepted `event` values (tests amendment D-21):**
  - `release-from-driver`: `corr` = JSON-RPC `id` (D-01)
  - `liveness-changed`: `corr` = null (D-02)
  - `state-recovered`: `corr` = null (D-03)
  - `sse-opened`, `sse-closed`: `corr` = null (D-04)
  - `parse-rejected`: `corr` = null (D-05)
- **Not a product `event` value: `a11y-violation`** (overseer fix pass 3, Z7, revising fix pass 2).
  - It is **not** in the product `event` enum and has **no** `ObsEvent` variant in `viola_core::obs`. No product process emits it.
  - It is a harness-only row that the a11y Playwright fixture writes into `e2e-web/test-results/a11y/*.ndjson`.
  - The row has its own tests-owned schema, `e2e-web/schemas/a11y-row.v1.json`, which reuses the diag-line field names without renaming any (tests Log format → Harness-side a11y rows).
  - `schemas/diag-line.v1.json`, G2 and G4 never contain or read it.
- **Accepted `process` value:** `cli`, for short-lived verbs with a resolved instance (D-06; tests amendment D-21 adds it to the `process` enum and to the harness `--process` filter).
- **Additive fields:** catalogued per event in Section 6.
- **Null encoding:** tracing 0.1.44 has no null field value; an `Option::None` field records nothing. A null `corr` / `instance` is therefore represented by **key absence**, and `jq '.corr'` yields `null` either way. The panic hook's hand-written line follows the same rule. It never writes `corr`, and it omits `instance` when none resolved (always for `ui`). Both emitters are therefore schema-identical: `schemas/diag-line.v1.json` declares `corr` (number | string) and `instance` (string) as optional keys — except that `corr` is required on the lines §3's per-event rules always define it for — and rejects a literal `null` for either, which catches an emitter that drifts (D-26). Key absence is ACCEPTED (D-12, review 2026-09-24); the tests amendment D-21 states "absent = null" for `corr` / `instance`, so harness readers treat a missing key and a literal `null` identically.

Illustrative line (one physical line on disk):
```json
{"timestamp":"2026-09-24T03:12:07.412Z","level":"INFO","target":"viola_channel::server","message":"channel-response","event":"channel-response","corr":7,"conn":"cli-4812-1790219525118-1","process":"run","instance":"builder","method":"send","result_class":"refusal","refusal":"human-typing","duration_ms":3}
```
