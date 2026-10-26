proposals:
  - detector: D-new-detail
    severity: warning
    fact: F7
    file: .andromeda/security-plan-detail/data-protection.md
    near: 20
    section: '## Data Protection'
    change: '- **Logs (`events.ndjson`, the link lines):** a product path now writes the `unlink` event line: the wrapper''s served `unlink` method, through `LinkSet::remove` (its first product caller), appends one `unlink` line (`source: wrapper`, `data: {driver, driven}`) to the instance''s `events.ndjson`, beside the `link` line the implicit and explicit link already write; it carries the two instance names only, no user content, and an absent pair appends nothing (as a held pair''s second `link` appends nothing). The `link` / `unlink` replies carry only `{since, already_linked}` and `{}`.'
    sidecar: 'data-protection: a Logs group records the first product-written `unlink` event line (names only) and the link replies'' shapes (chunk 2026-10-26-explicit-session-links)'
    rationale: 'report Changes: "`LinkSet::remove` has its first product caller (the `unlink` method) ... A product path now appends an `unlink` event line (`source: wrapper`, `{driver, driven}`) — the first"; `link` answers `{"ok":{"since":…,"already_linked":<bool>}}`, `unlink` answers `{"ok":{}}`, an absent pair appends nothing; Schema / config: the product `unlink` event line is now written'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:30
  - detector: D-new-detail
    severity: warning
    fact: F8
    file: .andromeda/security-plan-detail/data-protection.md
    near: 20
    section: '## Data Protection'
    change: '- **Logs (the wrapper''s process log) and the link verbs'' output:** the wrapper''s `link` / `unlink` methods open no span and write no process-log line of their own; the wrapper''s `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no names, so no driver name reaches the wrapper''s process log (asserted by test). `viola link` / `viola unlink` print only their own parsed `ViolaName` arguments, never a name read back from the reply or the log.'
    sidecar: 'data-protection: the Logs group records that no driver name reaches the wrapper''s process log and that the link verbs print only their parsed names (chunk 2026-10-26-explicit-session-links)'
    rationale: 'report Changes: "Neither opens a span nor writes a process-log line; the wrapper''s existing `channel-request` / `channel-response` pair carries `method:"link"|"unlink"`"; "Names printed are the verb''s own parsed `ViolaName` arguments, never a name read back from the reply or the log"; Coverage: "PII n/a (names only; no driver name in the wrapper''s process log — asserted)"'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:28
note: both open one new Logs group after the Files group (line 20, inside the same data-rest detail block that closes at line 22), F8's bullet placed directly after F7's; the body's Data Protection Logs bullet is the broad line they detail, and the event line's field schema stays obs-plan's (Logging format and fields belong to obs)
