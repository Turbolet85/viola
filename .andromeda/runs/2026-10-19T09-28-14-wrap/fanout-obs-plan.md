proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 → Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch — Cleanup'
    change: '`run.dialog_await` closes on answer, hand-back or the hold''s end. At the hold''s end (`dialog_hold_s`, 60–3600 s, default 60; `DIALOG_HOLD_DEFAULT` / `_MIN` / `_MAX`, PROVISIONAL) the wrapper''s `record_expiry` appends one `dialog-expired{dialog_id, dialog_kind}` product line (`source: wrapper`, no question, plan or tool text; it wakes a parked `wait`) and writes one `dialog-answered{corr:<dialog_id>, dialog_kind, deadline_hit:true, duration_ms}` with no `from`; the reply stays `null`, so the hook logs `hook-decision{decision_emitted:false, deadline_hit:false}`, exits 0 with empty stdout, and the dialog renders for the human. A failed append changes nothing else; a hand-back or an answer writes neither. The hook''s own `deadline_hit:true` is written only when its own read times out at its backstop bound `DIALOG_HOLD_MAX + REPLY_GRACE` (3605 s). Fail-open before `dialog_id` unchanged.'
    sidecar: 'obs-plan §4 Dialog scenario Cleanup: the expiry is the wrapper''s (one `dialog-expired` line + `dialog-answered{deadline_hit:true}` with no `from`); the hook logs `deadline_hit:false` on the `null`, its `deadline_hit:true` only at its 3605 s backstop (chunk 2026-10-19-dialog-hold-for-the-driver).'
    rationale: 'The report''s Spec claims disproved bullet names obs-plan.md:615 ("On deadline the hook emits `hook-decision{decision_emitted:false, deadline_hit:true}`") as disproved: the wrapper replies `null` at its own hold end, before the hook''s bound. Its Symbols bullet adds `record_expiry` (one `dialog-expired` line, one `dialog-answered{deadline_hit:true}` with no `from`) and the hook''s bound `DIALOG_HOLD_MAX + REPLY_GRACE` = 3605 s; Counts pattern `deadlinehit` reads :615 stale.'
    basis: 'viola-0.1.0/chunks/2026-10-19-dialog-hold-for-the-driver/report.md:51-56; src/run/dialog.rs:372-394; src/cmd/hook.rs'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 → Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch — Required log fields'
    change: 'The wrapper bullet reads `dialog-raised{corr:<dialog_id>, dialog_kind, hook_event}`, then `dialog-answered{corr:<dialog_id>, dialog_kind, from, from_trust, deadline_hit, duration_ms}`, where `from` / `from_trust` are present on an answer and absent on the expiry line (`deadline_hit:true`, written by `record_expiry` at the hold''s end).'
    sidecar: 'obs-plan §4 Dialog scenario: `dialog-answered` carries no `from` / `from_trust` at expiry (chunk 2026-10-19-dialog-hold-for-the-driver).'
    rationale: 'The report''s Symbols bullet: at expiry `record_expiry` writes one `obs_event!(INFO, DialogAnswered, corr = dialog_id, dialog_kind, deadline_hit = true, duration_ms)` with no `from`; Counts pattern `deadlinehit` reads obs-plan.md:610 stale by omission.'
    basis: 'viola-0.1.0/chunks/2026-10-19-dialog-hold-for-the-driver/report.md:52-56; src/run/dialog.rs:372-394'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 → Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch — Required span attributes'
    change: '`run.dialog_await`: `deadline_ms` (the configured hold the slot was started with: `config.json` `dialog_hold_s` × 1000, 60 000–3 600 000, default 60 000 = `DIALOG_HOLD_DEFAULT`), `deadline_hit`.'
    sidecar: 'obs-plan §4 Dialog scenario: `run.dialog_await`''s `deadline_ms` reports the configured hold (`dialog_hold_s`) (chunk 2026-10-19-dialog-hold-for-the-driver).'
    rationale: 'The report''s Symbols bullet: the await''s deadline is `clock.now() + self.hold` and the span `run.dialog_await`''s `deadline_ms` reports `self.hold`; its Expected amendments name obs-plan §4 Dialog scenario "`deadline_ms` the configured hold". The span attribute list names no value, so the configured source is unstated.'
    basis: 'viola-0.1.0/chunks/2026-10-19-dialog-hold-for-the-driver/report.md:51-52; src/run/dialog.rs:219-223'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 → Scenario: `wait` / `last` event-driven readback — Required log fields'
    change: 'The `channel-response` `outcome` for method `wait` is the waking kind (`turn-ended|question|permission|plan|session-end|dialog-expired`, the six of `EventKind::WAIT_WAKE`) or `timed-out`, derived once in `viola-channel` from the result over the shared wake set; the rest of the bullet unchanged.'
    sidecar: 'obs-plan §4 wait scenario: the wake set is six kinds, `dialog-expired` added (chunk 2026-10-19-dialog-hold-for-the-driver).'
    rationale: 'The report''s Symbols bullet: `EventKind::WAIT_WAKE: [Self; 6]` (was 5) adds `dialog-expired`, and `wait_outcome` derives from it unchanged, so the channel `outcome` takes the kind; Counts pattern `wake5` reads obs-plan.md:596 stale (the `outcome` list of five).'
    basis: 'viola-0.1.0/chunks/2026-10-19-dialog-hold-for-the-driver/report.md:40-44; crates/viola-core/src/lib.rs'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 → Scenario: `wait` / `last` event-driven readback — Required span attributes'
    change: '`run.wait_dispatch`: `woken_kind` (`turn-ended|question|permission|plan|session-end|dialog-expired`, the six of `EventKind::WAIT_WAKE`) or `timed_out`.'
    sidecar: 'obs-plan §4 wait scenario: `run.wait_dispatch`''s `woken_kind` list gains `dialog-expired` (chunk 2026-10-19-dialog-hold-for-the-driver).'
    rationale: 'Same retired claim (a wake set of five) restated in the span attribute list; `src/run/wait.rs`''s scan derives from `WAIT_WAKE` unchanged, so a `dialog-expired` line wakes the dispatch (report Symbols bullet).'
    basis: 'viola-0.1.0/chunks/2026-10-19-dialog-hold-for-the-driver/report.md:40-44'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§3 → OTel SDK init'
    change: 'Init order step 6: in `run`, after the `read_thresholds` lines, the start reads `config.json` once more for `dialog_hold_s` (`run::dialog::read_hold`, through `take(MAX_FRAME)`, `v` = 1) and writes exactly one `parse-rejected{parser:"config-json", detail:"malformed", count}` WARN line when the key holds anything but a whole number of seconds from 60 to 3600 (the hold then takes 60); an absent, unreadable or malformed document, or a `v` other than 1, gives the default with no further line.'
    sidecar: 'obs-plan §3 → OTel SDK init step 6: `run`''s start reads `dialog_hold_s` after the thresholds, one `parse-rejected{config-json, malformed}` on a bad value (chunk 2026-10-19-dialog-hold-for-the-driver).'
    rationale: 'The report''s Schema / config bullet: `dialog_hold_s` is read once at the wrapper''s start through `take(MAX_FRAME)` and `v` = 1, right after `read_thresholds`, a bad value giving 60 plus exactly one `parse-rejected{parser:"config-json", detail:"malformed"}`; Counts pattern `cfgkeys` reads `registries/contracts/obs-plan/otel-sdk-init.md:17` stale (the start''s reads after `ProcessStart`).'
    basis: 'viola-0.1.0/chunks/2026-10-19-dialog-hold-for-the-driver/report.md:73-78; src/run/dialog.rs:155-187; src/cmd/run.rs:263-266'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§3 → OTel SDK init'
    change: 'Init order step 6: `CONFIG_KEYS` holds six names (`v`, `diagnostics_level`, `claude_env_keep`, `five_hour`, `seven_day`, `dialog_hold_s`; the array in `src/obs.rs`), so neither threshold key nor the hold key counts as unknown (chunks 2026-10-10-budget-governor, 2026-10-19-dialog-hold-for-the-driver).'
    sidecar: 'obs-plan §3 → OTel SDK init: `CONFIG_KEYS` five → six names, `dialog_hold_s` added (chunk 2026-10-19-dialog-hold-for-the-driver).'
    rationale: 'The report''s Schema / config bullet: `src/obs.rs` `CONFIG_KEYS: [&str; 6]` (was 5) adds `dialog_hold_s`, so a document holding it reads no `unknown-keys`; Counts pattern `cfgkeys` reads otel-sdk-init.md:17 stale ("`CONFIG_KEYS` holds five names").'
    basis: 'viola-0.1.0/chunks/2026-10-19-dialog-hold-for-the-driver/report.md:79-80; src/obs.rs:32'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§3 → OTel SDK init'
    change: 'Init order step 5''s parenthesis: `run` at the wrapper''s start (which reads the file a second time for the two budget thresholds, `read_thresholds`, and a third time for the dialog hold, `read_hold`, step 6), `viola list` and `viola mcp`''s `list` tool (a second time for the two budget thresholds).'
    sidecar: 'obs-plan §3 → OTel SDK init step 5: `run`''s start reads `config.json` a third time, for `dialog_hold_s` (chunk 2026-10-19-dialog-hold-for-the-driver).'
    rationale: 'Same claim (the start''s `config.json` reads end at `read_thresholds`) restated in step 5; the report''s Schema / config bullet adds the `dialog_hold_s` read right after `read_thresholds`.'
    basis: 'viola-0.1.0/chunks/2026-10-19-dialog-hold-for-the-driver/report.md:73-74'
    dependent-of: D-obs-instrumentation
note: 'D-obs-stack: no drift (report Dependencies: none). D-obs-pii: no drift (the `dialog-expired` line carries `{dialog_id, dialog_kind}` only, no question, plan or tool text; no redaction shape changed). D-retired: `DIALOG_DEADLINE` stands nowhere in obs-plan or its key files. obs-plan.md:758 (the statusline `deadline`) and the §6 field-catalog rows stay as the report''s sweep reads them (no-change). The §4 Dialog-scenario proposals all touch one section; apply them together.'
