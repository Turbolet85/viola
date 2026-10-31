proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 21
    section: '## 4. Span / Trace Coverage'
    change: 'Row ipc-internal (viola-channel), last cell reads: `channel.request` (client) / `channel.dispatch` (server) spans. `channel-request` / `channel-response` with `method`, `corr`, `conn`, `result_class`, `error_code`, `sender`; on a `-32602` also `detail`, written on both the server''s and the client''s line by `ResponseLine`''s `Drop` from `ProtocolError::cause`, which is `None` for the other four faults, the client''s `fault_of` reading the fault''s `data` (`supported`, `reason` or neither) to choose the word. Never `params` or result bodies'
    sidecar: 'span-trace-coverage: the channel row names `channel-response`''s `detail`, its writer `ResponseLine`''s `Drop` through `ProtocolError::cause`, and the client''s `fault_of` reading `data` (2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report, Symbols / APIs: `ProtocolError::cause` gives a word for the three `-32602` faults and `None` for the other four, `ResponseLine`''s `Drop` writes it on both lines at WARN, and `fault_of(&Value)` reads `data` for `-32602`; the row''s field list did not name `detail`.'
    basis: crates/viola-channel/src/lib.rs:102-110
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 91
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'The `channel-response` item reads: `channel-response{corr:<id>, conn, result_class, refusal, duration_ms}` on both sides, plus `detail` on a `-32602` only (a word of the closed `unsupported-version|invalid-params|release-from-driver`), absent on every other `channel-response` line, a refusal''s included, whose own `detail` stays on `send-refused`;'
    sidecar: 'span-trace-coverage: send''s `channel-response` carries `detail` on a `-32602` only, absent on a refusal (2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report, Schema / config: `channel-response`''s `detail` closes to three words and is absent on every other `channel-response` line; Symbols / APIs: `ResponseLine`''s `Drop` writes `detail = fault.and_then(ProtocolError::cause)`, so a refusal, no fault, writes none. The `response-line` sweep''s no-change reads the pair named, not this field.'
    basis: schemas/diag-line.v1.json:42
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 132
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'The driver-side refusals item reads: driver-side refusals: `channel-response{result_class:"refusal", refusal:"not-delivered"}` with no `detail` on the line (exit 13; `unknown-dialog` is the refusal''s own detail, no word of the line''s closed three), or `refusal:"unverified-cli"` (exit 12, answer withheld).'
    sidecar: 'span-trace-coverage: the answer refusal''s `channel-response` no longer states `detail:"unknown-dialog"`, outside the line''s closed three words (2026-10-31-sanitised-error-surfaces).'
    rationale: 'Contradiction with the report: `schemas/diag-line.v1.json` closes `channel-response`''s `detail` to `unsupported-version`, `invalid-params` and `release-from-driver`, absent on every other such line, and `ResponseLine`''s `Drop` takes it from the fault alone; a line with `detail:"unknown-dialog"` would be outside the schema G4 reads.'
    basis: schemas/diag-line.v1.json:42
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 151
    section: '## 4. Span / Trace Coverage › #### Scenario: Human takes the wheel, automation is refused, `release` returns it'
    change: 'Add after the `-32602` `"invalid params"` reply clause: that reply''s `channel-response` carries `detail:"release-from-driver"`, and a plain `-32602` here carries `detail:"invalid-params"`, each on both sides at WARN, beside the `release-from-driver` event line, which stays.'
    sidecar: 'span-trace-coverage: `release`''s two `-32602` replies name their `channel-response` `detail` words beside the unreplaced `release-from-driver` event (2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report, Symbols / APIs: `ProtocolError::cause` reads `release-from-driver` and `invalid-params` for these faults, written by `ResponseLine`''s `Drop` on both lines, level unchanged; Outcome: `channel-response`''s three words on both sides, no name or param on the line.'
    basis: crates/viola-channel/src/lib.rs:102-110
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 194
    section: '## 4. Span / Trace Coverage › #### Scenario: Budget governor'
    change: 'Add to the sentence on the pair: on a `-32602` the driven wrapper''s `channel-response` also carries `detail:"invalid-params"`, still with neither name nor any param on the line; the no-span and no-line-of-their-own rule is unmoved.'
    sidecar: 'span-trace-coverage: a bad `link` / `unlink` name or pair reads `detail:"invalid-params"` on the `channel-response` line (2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report, Symbols / APIs: `invalid-params` is `ProtocolError::cause` for the plain `-32602`; Outcome: `channel-response`''s three words, no name or param on the line, met by `link_verbs_log_the_channel_pair_and_no_names`.'
    basis: crates/viola-channel/src/lib.rs:102-110
note: The report's response-line sweep counted span-trace-coverage's pair mentions no-change, yet its "absent on every other line" contradicts the refusal `detail` stated on send's and answer's `channel-response`, so the same reading should be checked in log-coverage and the other groups; D-obs-instrumentation, D-obs-stack and D-obs-pii fire nothing here (no line or span added, no dependency moved, closed words only); the broad line of §6 IPC invocation is log-coverage's.
