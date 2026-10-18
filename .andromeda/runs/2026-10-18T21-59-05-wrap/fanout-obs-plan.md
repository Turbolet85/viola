proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage → Scenario: Confirmed `send` (CL-1) from driver to readback → Cleanup'
    change: 'Add the third close condition: for `/compact` on a verified CLI (post-condition `CompactedSession`) `run.confirm_window` closes on the `session-start` whose `cause` is `compact` and whose `agent_session_id` is a string EQUAL to the id the send slot remembered before the line (nothing remembered claims nothing; no `prompt-submitted` claims it); its window is `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL) while every other confirmed send keeps `CONFIRM_WINDOW_FALLBACK` (10 s); expiry is `send-refused{detail:"no-prompt-submitted"}` for all three (no new detail); `/compact` on an unverified CLI stays unconfirmable and opens no window.'
    sidecar: 'obs-plan §4 CL-1 Cleanup: the `/compact` same-session `session-start{cause:"compact"}` close condition and its own 40 s PROVISIONAL window (chunk 2026-10-18-compact-post-condition-ledger-row).'
    rationale: 'Report Changes → Symbols / APIs: `PostCondition::CompactedSession` and `SendSlot::session_started` claim a compact send on cause `compact` with the id equal to the prior one; `SendSlot::window()` returns `COMPACT_CONFIRM_WINDOW` for it; Counts `cl1` / `readback` mark obs-plan 566-585 and 585 changed; Expected amendment 5 names the third close condition.'
    basis: 'src/run/send/slot.rs:102-121; src/run/send/slot.rs:173-180; crates/viola-agent-claude/src/screen.rs:22-27; .andromeda/obs-plan.md:585'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage → Scenario: Confirmed `send` (CL-1) from driver to readback → Required span attributes'
    change: '`run.confirm_window`: `window_ms`, read from `SendSlot::window()` (the same bound the confirm deadline reads): 40 000 for a `/compact` send waiting on its compacted-session post-condition (`COMPACT_CONFIRM_WINDOW`, PROVISIONAL), else 10 000 (`CONFIRM_WINDOW_FALLBACK`); the span field itself is proved at the unit tier through `window_ms(slot.window())`, not captured by a test.'
    sidecar: 'obs-plan §4 CL-1 span attributes: `run.confirm_window.window_ms` reads `SendSlot::window()`, 40 000 for a `/compact` send, 10 000 otherwise.'
    rationale: 'Report Symbols / APIs: `confirm`s deadline and the `run.confirm_window` span`s `window_ms` both read `window()`, so a compact send`s span reads 40 000; Deviation 6 and the Outcome (obs) row: proved at the unit tier, the span field not captured by a test; Counts `cl1` names the CL-1 scenario`s `window_ms` as changed.'
    basis: 'src/run/send/slot.rs:173-180; .andromeda/obs-plan.md:566-585'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage → Scenario: Confirmed `send` (CL-1) from driver to readback → Surfaces involved'
    change: 'The hook leg reads: `hook` user-prompt-submit; `hook` session-start for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI only); no hook for an unconfirmable send.'
    sidecar: 'obs-plan §4 CL-1 Surfaces: `/compact` confirmed by its same-session session-start beside `/clear`.'
    rationale: 'Same claim as the primary (only `/clear` is confirmed through a session-start hook), restated in the scenario`s Surfaces line; report Symbols / APIs (`CompactedSession`, the slot) and Counts `cl1` (obs-plan 566-585 changed).'
    basis: '.andromeda/obs-plan.md:566-585'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage → Scenario: Confirmed `send` (CL-1) from driver to readback → Must-trace spans'
    change: 'In the hook process: the user-prompt-submit hook for an ordinary send and the session-start hook for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session on a verified CLI.'
    sidecar: 'obs-plan §4 CL-1 Must-trace spans: the session-start hook also confirms a `/compact` send.'
    rationale: 'Same claim as the primary restated in the Must-trace spans bullet; report Symbols / APIs (`LOCAL_COMMANDS`: `/compact` now `Some(PostCondition::CompactedSession)`, `classify("/compact", true)` = `Confirms::Post(CompactedSession)`).'
    basis: '.andromeda/obs-plan.md:566-585; crates/viola-agent-claude/src/ledger.rs:201-202'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage → Scenario: Confirmed `send` (CL-1) from driver to readback → Required log fields'
    change: 'hook: `hook-invoked{hook_event:"session-start", corr:null}` for a `/clear` confirmed by its new session and for a `/compact` confirmed by its same session (verified CLI); no new event, detail or field (a compact send reuses `send-issued` / `send-confirmed` / `send-refused`).'
    sidecar: 'obs-plan §4 CL-1 log fields: the session-start `hook-invoked` line also on a `/compact` confirmation; no new event or detail.'
    rationale: 'Same claim as the primary restated in the hook log-field bullet; report Schema / config: no event kind, `detail` or `subject` added; Coverage: the existing `send-issued` / `send-confirmed` / `send-refused` lines, no new event.'
    basis: '.andromeda/obs-plan.md:566-585'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§1 Obs Scope Summary → Critical paths (must-trace) → Confirmed `send` (CL-1) from driver to readback'
    change: 'Surfaces and Must-trace spans / Log events read: `hook` SessionStart for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI); `hook-invoked` (UserPromptSubmit, or SessionStart for either of those); `run.confirm_window` not opened for an unconfirmable send.'
    sidecar: 'obs-plan §1 CL-1 critical path copy: `/compact` confirmed by its same-session SessionStart beside `/clear`.'
    rationale: 'The §1 copy of the CL-1 scenario restates the only-`/clear` session-start confirmation; report Counts `cl1`: obs-plan 276-278 (the §1 copy) changed; Expected amendment 5 names the §1 copy.'
    basis: '.andromeda/obs-plan.md:276-278'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§3 → Snapshot / paste-to-AI integration'
    change: 'The hook side reads: `hook-invoked{hook_event:"user-prompt-submit"}` for an ordinary send, `hook-invoked{hook_event:"session-start"}` for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI); `corr` null; joined inside `run.confirm_window` as before.'
    sidecar: 'obs-plan §3 Snapshot / paste-to-AI integration key: the session-start hook side also joins a `/compact` confirmation.'
    rationale: 'The key restates the same enumeration of which hook a confirmation window joins, which names `/clear` alone; a `/compact` send on a verified CLI now has a session-start hook inside its window (report Symbols / APIs, the slot). The report`s pre-pass read this key as naming CL-1 events only; its `/clear`-only clause is the primary`s claim, so it is offered for the orchestrator`s P2 read.'
    basis: '.andromeda/registries/contracts/obs-plan/snapshot-paste-to-ai-integration.md:13'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage → Edge flows → `verify` / `plugin install`'
    change: '`MM` the relied count (`LedgerRow::relied()`): 24 on Unix, 21 on Windows, the 22nd row `resume-session`, the 23rd `agents-join` and the 24th `local-command-compact` relied on every OS; the last step line `[24/24] local-command-compact …` / `[21/21] local-command-compact …`; the eight spawn pairs stay eight (Run B now pastes six compiled texts, `/compact` before `/clear`, with no new spawn pair or subject).'
    sidecar: 'obs-plan §4 Edge flows → verify: relied count 23/20 → 24/21 with `local-command-compact` the 24th row; eight spawn pairs unchanged.'
    rationale: 'Report Symbols / APIs: `LedgerRow::LocalCommandCompact` the twenty-fourth member of `LedgerRow::ALL`, relied on every OS, `relied()` 24 Unix / 21 Windows; Counts `rows-count` lists obs-plan 664 as a standing row stating the current count → changed; Coverage: no new spawn pair or subject, eight pairs.'
    basis: '.andromeda/obs-plan.md:664'
note: 'D-obs-stack: no drift (Dependencies none; no logger or OTel change). D-obs-pii: no drift (no new log line; `window_ms` is a number; no session id, text or path in any line, `compact-1` scrubbed by `--record`). D-retired: no obs-plan passage names `/compact` as unconfirmable on a verified CLI or names the retired tests; the generic unconfirmable wording (obs-plan 585, 694) stays true for `/remote-control` and an unverified CLI. obs-plan 694 (`viola.send.confirm_ms`) needs no change. The §3 key proposal departs from the report`s pre-pass "no change" read of that key and is flagged for P2.'
