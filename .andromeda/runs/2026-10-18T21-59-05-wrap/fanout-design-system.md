proposals:
  - detector: D-retired
    severity: warning
    section: Brand Identity (Domain anchors, the Readback bullet)
    change: "A send counts only once the matching `prompt-submitted` reads it back or, for `/clear`, once its measured post-condition (a new session) does, or, for `/compact` on a verified CLI, once its measured post-condition (a `session-start` of cause `compact` carrying the session's own id) does; on an unverified CLI `/compact` stays `unconfirmable`. Otherwise it is \"unable\" plus a typed reason."
    sidecar: "Brand Identity Readback anchor: `/compact` joins `/clear` as a post-condition-confirmed local command on a verified CLI (its same-session compact SessionStart); the retired `unconfirmable`-at-once reading on a verified CLI removed (chunk 2026-10-18-compact-post-condition-ledger-row)."
    rationale: "The report's Retired bullet retires \"`/compact` reads `unconfirmable` at once on a verified CLI\"; the anchor names `/clear` as the only local command a post-condition reads back, so it still describes `/compact` as never read back. Symbols / APIs: `/compact` carries `Some(PostCondition::CompactedSession)` and `classify(\"/compact\", true)` is `Confirms::Post(CompactedSession)`; the slot claims it on cause `compact` with the id EQUAL to the prior one. Expected amendment 6 names the Brand Identity readback sentences gaining `/compact` beside `/clear`."
    basis: viola-0.1.0/chunks/2026-10-18-compact-post-condition-ledger-row/report.md:28
  - detector: D-retired
    severity: warning
    section: Brand Identity (Signature element, the `RB` / `read back` state)
    change: "It fills instantly when the matching `prompt-submitted` lands, or for `/clear` when its `session-start` with cause `clear` does, or for `/compact` on a verified CLI when its `session-start` with cause `compact` and the session's own id does."
    sidecar: "Signature element RB state: the `/compact` fill (a same-session `session-start` of cause `compact`, verified CLI) added beside `/clear`'s (chunk 2026-10-18-compact-post-condition-ledger-row)."
    rationale: "Same retired claim restated in the signature element: only `/clear`'s session-start fills the box, so a verified-CLI `/compact` would read as never filled. Report Retired bullet and Symbols / APIs (`SendSlot::session_started` claims a `Post(CompactedSession)` send on cause `compact` with the same id); Outcome: a human-mode `viola send` of `/compact` on a stamped home prints `[RB] read back`."
    basis: viola-0.1.0/chunks/2026-10-18-compact-post-condition-ledger-row/report.md:19
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: Color Palette → Semantic Colors (the `SessionStartCause::compact` row)
    change: "`compact`. Also the post-condition that fills the readback box of a `/compact` send on a verified CLI, when it carries the session's own id."
    sidecar: "Semantic Colors SessionStartCause::compact row: names its role as the readback post-condition of a verified-CLI `/compact` send, as the `clear` row does for `/clear` (chunk 2026-10-18-compact-post-condition-ledger-row)."
    rationale: "The `SessionStartCause::clear` row names its readback role and the `compact` row names none, which states as standing that a compact session-start fills no readback box (the retired behaviour). Report Retired bullet; Symbols / APIs `PostCondition::CompactedSession` — \"a SessionStart with source `compact` and the session's own `session_id`\"."
    basis: viola-0.1.0/chunks/2026-10-18-compact-post-condition-ledger-row/report.md:11
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: Anti-Patterns (NEVER do these) → Rejected Defaults (from exploration) (the optimistic "Sending…" spinner bullet)
    change: "The readback box stays `open` until the matching `prompt-submitted` (for `/clear`, its new-session post-condition; for `/compact` on a verified CLI, its same-session compact post-condition), then fills instantly or is struck with `unable` plus the typed reason."
    sidecar: "Rejected Defaults spinner bullet: `/compact`'s same-session post-condition named beside `/clear`'s (chunk 2026-10-18-compact-post-condition-ledger-row)."
    rationale: "Third restatement of the `/clear`-only readback claim, which leaves `/compact` described as never read back on any CLI. Report Retired bullet: on a verified CLI `/compact` is now confirmed by its same-session SessionStart or expires `not-delivered` / `no-prompt-submitted` at 40 s."
    basis: viola-0.1.0/chunks/2026-10-18-compact-post-condition-ledger-row/report.md:28
    dependent-of: D-retired
note: "D-design-tokens: no drift — both Coverage rows read `tokens n/a` (report.md:77, :78), no `hardcoded✗`. Outside my detectors and not a Retired item, but owed by Expected amendment 6 (report.md:72) and the `rows-count` sweep (report.md:30): design-system 505 (§Surface: cli → Component Patterns, `verify`) still reads `[01/23]`, `23 on Unix, 20 on Windows`, a row list ending at `agents-join`, and `stamped 2.1.287  23 pass  0 fail` — it should read 24 / 21, append `local-command-compact` /compact keeps the session id and submits no prompt (relied on every OS), and `24 pass  0 fail`. design-system 450 (pattern 2's `unconfirmable` line) stays, per report.md:31 (still printed for `/remote-control` and for `/compact` on an unverified CLI)."
