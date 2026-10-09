# obs extract

## Relevance
partial — the chunk adds no event, field, span or refusal detail; it changes the value one existing field (`text_bytes`) carries and moves a text with an inner CR or CR LF from the refused line set to the confirmed line set of the must-trace path "Confirmed `send`" (obs tier Standard with Minimal-tier exporter carve-outs, per obs-plan §1 Obs Scope Summary).

## Constraints
- obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback requires `text_bytes` to be the length of the typed text, and the same value on the `pty.paste_write` span and on the `send-issued` line. Under this chunk's rule the typed text is the LF-normalised one, so for a text with CR LF pairs the value is smaller than the sent byte count by one per pair. Whether the code already computes both from the one typed-text value (so the rule change carries through with no edit to an obs call site) is research's question.
- obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback and obs-plan §6 Additive field catalog both gloss `text_bytes` as the sent text less its trailing CR and LF only. That gloss is narrower than the rule this chunk lands; phase amends no master, so both sites are a wrap amendment the plan should list (scope §4 already names them), and the code's doc comment on the field is the chunk's.
- obs-plan §8 PII Scrubbing (Data classification rules, the prompt-text row) requires that send text reach no home-level process-log line and that only `text_bytes` be logged. The normalised typed text is the same class as the received text: neither form, nor any fragment showing where a CR stood, may appear on a span field or a home-level line.
- obs-plan §8 PII Scrubbing (Default-deny posture) requires every logged field to be named in the §6 catalog, enforced by gate G4. A field or event recording that a CR was replaced (a count, a flag, a before/after length pair) is outside the catalog and is not admissible without an obs Decisions Log entry.
- obs-plan §6 Log Coverage (`detail` code catalog) requires refusal details to be the architecture `RefusalReason` details verbatim. The chunk adds none; a text of only CR and LF characters stays `empty-text`, and obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback requires one `send-refused` line per deciding side for it, with no `send-issued`.
- obs-plan §10 SLO Invariants & Telemetry Budgets (Correlation propagation) requires every send line to carry the send cursor as `corr`; obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback requires every wrapper `send-*` line to carry `conn` (or `srv_conn`) and `rpc_id` as well. A send with an inner CR that is now confirmed must keep that join on its `send-issued` and `send-confirmed` lines.
- obs-plan §9 CI Integration (Pipeline integration, the Mutation row) requires `obs_event!` call sites to be mutated like product code. If the `text_bytes` argument at the `send-issued` site is touched, a test must pin its value, or a mutant of it survives.

## Patterns to follow
- Take the typed text once and pass its length to the log line as an explicit typed field through `obs_event!`, never a captured value (per obs-plan §8 PII Scrubbing, Integration points, item 1).
- Any function the chunk newly instruments uses the `#[instrument(skip_all, name = "...", fields(...))]` form with a static `<area>.<operation>` name (per obs-plan §4 Span / Trace Coverage, Span naming convention). The rule function itself is a pure helper on no must-trace boundary; obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback lists no span for it and the chunk owes none.
- Test homes for the new wrapper-level and driver-level cases live under `target/e2e-home/`, so G2, G4, the secret scan and the upload cover their diagnostics (per obs-plan §9 CI Integration, Pipeline integration, the Integration tests row).
- Test texts for the inner-CR and inner-CRLF cases are synthetic and embed the tests-owned canary (per obs-plan §8 PII Scrubbing, Integration points, item 6).
- A test that reads a process-log line emitted in memory parses it against `schemas/diag-line.v1.json` (per obs-plan §9 CI Integration, Pipeline integration, the Unit tests row).

## Anti-patterns to avoid
- NEVER log send text in a home-level file, a debugging line that shows the text before or after normalisation included (per obs-plan §11 Obs Anti-Patterns, PII Scrubbing).
- NEVER log a field not named in the §6 catalog, and never add an `event` value without a Decisions Log entry (per obs-plan §11 Obs Anti-Patterns, PII Scrubbing and Logs).
- NEVER use a bare `#[instrument]`, or the `err` / `ret` options, on a function that takes or returns the text (per obs-plan §11 Obs Anti-Patterns, Spans / Traces).

## Contract bindings
- obs ↔ tests: the process-log lines the new tests read (`send-issued`, `send-confirmed`, `send-refused`) follow the log format obs aligns to the tests' own (per obs-plan §6 Log Coverage, Log format JSON schema); the check body of G4 is tests-owned and both schemas are obs-owned (per obs-plan §9 CI Integration, Gate commands, G4). The process log cannot witness the absence of a CR, since the text never appears there; it witnesses only the length. A test that must read the typed or submitted text reads it from the product record or the fake agent, which is tests' side of the binding.
- obs ↔ security: the never-log rule for `send` text and the per-deciding-side `send-refused` for `control-character` / `empty-text` bind to the security plan's paste-text vector (per obs-plan §1 Obs Scope Summary, Telemetry triggers, logging-sensitive, Vector 2).
- obs ↔ architecture: the definition of the typed text is architecture's ([Delivery Confirmation]); `text_bytes` reads it and defines nothing of its own (per obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback).
- obs ↔ design: the readback word a send line maps to is joined by the send cursor; a text with an inner CR now maps to `read back` where it mapped to `unable` (per obs-plan §4 Span / Trace Coverage, Frontend observable boundary). No boundary word or attribute changes.
- obs ↔ metrics derivation: the same text moves between two existing values of `viola.send.outcome`, with no new grouping key (per obs-plan §5 Metric Coverage, Per-surface / per-path metrics).

## Acceptance criteria contributions
- A send whose text holds an inner CR or CR LF writes one wrapper `send-issued` whose `text_bytes` equals the byte length of the typed text under the chunk's rule, then `send-confirmed{confirmed:true}` on the same `corr`, and no `send-refused{detail:"no-prompt-submitted"}` for that cursor (per obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback).
- The canary carried by the new tests' texts appears in no home-level `diagnostics/*.ndjson` line (per obs-plan §8 PII Scrubbing, Integration points, item 6).
- Every home-level line the new tests' homes hold passes `schemas/diag-line.v1.json` with no field outside the catalog, and G2 counts no panic line (per obs-plan §10 SLO Invariants & Telemetry Budgets, Error budget).
- A text of only CR and LF characters writes one `send-refused{refusal:"not-delivered", detail:"empty-text"}` per deciding side and no `send-issued` (per obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback).
