proposals:
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 147
    section: '## Surface: cli › ### Component Patterns'
    change: '| 14 | `unable  unknown  <detail>`, the detail a code word or nothing | refusal object |'
    sidecar: 'surface: exit 14 human form, the detail a code word or nothing, no longer opaque text (2026-10-31-sanitised-error-surfaces)'
    rationale: 'Report Changes, Symbols / APIs: a string `detail` that is no code word reads `None` / `null`; Expected amendments: row 14 reads a code word or nothing, not opaque free text, pattern `code-word`.'
    basis: .andromeda/design-system-detail/surface.md:147
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 81
    section: '## Surface: cli › ### Component Patterns'
    change: '  - No hint line for `unknown` (exit 14), because its detail is a code word or nothing and a hint would have to guess. No hint line for `error: wrapper fault` (exit 20) or `error: internal error` (exit 1), because they are faults, not refusals, and their detail goes only to `diagnostics/`.'
    sidecar: 'surface: the no-hint reason for `unknown` reads a code word or nothing, not an opaque detail (2026-10-31-sanitised-error-surfaces)'
    rationale: 'Report Expected amendments: pattern `code-word` lists `surface.md:81` as changed; Outcome keeps the rule itself, `unknown` and the faults print no hint.'
    basis: .andromeda/design-system-detail/surface.md:81
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 102
    section: '## Color Palette › ### Semantic Colors'
    change: '| RefusalReason::unknown | #1E2124 | #E6D8AE | #E6D8AE | `unable · unknown`, with the detail, a code word or nothing, printed as plain text |'
    sidecar: 'color-palette: `RefusalReason::unknown` row, the detail a code word or nothing, not opaque text (2026-10-31-sanitised-error-surfaces)'
    rationale: 'Report Expected amendments: §Semantic Colors `RefusalReason::unknown` reads a code word or nothing, not opaque free text, pattern `code-word`; no colour moved.'
    basis: .andromeda/design-system-detail/color-palette.md:102
    dependent-of: D-design-cli-wording
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 21
    section: '## Surface: cli › ### Tokens (platform-specific)'
    change: '- Code word: 1 to 64 bytes of lower-case letters, digits and hyphens, a lower-case letter first (`viola_core::is_code_word`); both clients, `reply_of` in `viola` and `of_reply` in `viola-mcp`, screen a peer''s refusal and detail through it.'
    sidecar: 'surface: the code word defined, and both clients screening a peer''s refusal and detail through it (2026-10-31-sanitised-error-surfaces)'
    rationale: 'Report Changes, Symbols / APIs: `viola_core::is_code_word` and its two callers; the refusal form at this line carries the reason and detail the screen reads.'
    basis: crates/viola-core/src/lib.rs:347-357
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 21
    section: '## Surface: cli › ### Tokens (platform-specific)'
    change: '- A peer''s `refusal` that is no code word reads `unknown`; a string `detail` that is no code word reads as no detail, null under `--json`; a non-string `detail` is still no known reply.'
    sidecar: 'surface: how a peer''s non-code refusal and detail read (2026-10-31-sanitised-error-surfaces)'
    rationale: 'Report Changes, Symbols / APIs: the `reply_of` and `of_reply` screen; Outcome: a peer''s `refusal` or `detail` reaches no document or text.'
    basis: src/cmd/client.rs:77-81
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 21
    section: '## Surface: cli › ### Tokens (platform-specific)'
    change: '- A code word outside the closed refusal list stays visible, by design, in the document and in `refused: <word>`, and exits 14 as `unknown` does.'
    sidecar: 'surface: a code word outside the closed refusal list stays visible and exits 14 (2026-10-31-sanitised-error-surfaces)'
    rationale: 'Report Changes, Symbols / APIs: the word stays visible and its exit stays `unknown`''s 14 by `reason_of`; Outcome: visible by design.'
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 142
    section: '## Surface: cli › ### Component Patterns'
    change: '| 2 | the fixed `UNUSABLE_RESPONSE` line (an oversize `viola answer`) or `UNUSABLE_TEXT` line (an unusable `viola send`) on stderr, whatever `--json` says; `tests/contract_error_bodies.rs` names them in `NOT_ERROR_BODIES` | no document |'
    sidecar: 'surface: exit 2 also carries the fixed unusable-input stderr lines, no `--json` document (2026-10-31-sanitised-error-surfaces)'
    rationale: 'Report Spec claims disproved: the exit-2 paths write the fixed line to stderr whatever `--json` says, no document, named in `NOT_ERROR_BODIES`.'
    basis: src/cmd/answer.rs:91
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 148
    section: '## Surface: cli › ### Component Patterns'
    change: '| 20 | `error: wrapper fault  <code>`, one of the five known codes | `detail` closed: each code''s fixed `message`; `-32602` carries the unsupported-version body only with an integer `supported` and a version-text `wrapper`, the release-from-driver body only on that exact reason, else `invalid params` with null `data`; every other known code has null `data` |'
    sidecar: 'surface: `wrapper-fault`''s fields closed for the five known codes (2026-10-31-sanitised-error-surfaces)'
    rationale: 'Report Changes, Symbols / APIs: `viola_channel::sanitised_fault`, which replaces both `fault_detail` copies; Expected amendments: `wrapper-fault`''s fields closed.'
    basis: crates/viola-channel/src/lib.rs:113-142
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 148
    section: '## Surface: cli › ### Component Patterns'
    change: '| 20 | `error: wrapper fault  <code>`, a code outside the five, the human line unchanged | `{"v":1,"error":"wrapper-fault","detail":{"code":<code>,"message":"wrapper fault","data":null}}`; the same fixed word on MCP |'
    sidecar: 'surface: the new fixed `message` `wrapper fault` for a code outside the five (2026-10-31-sanitised-error-surfaces)'
    rationale: 'Report Coverage of new surfaces: the `message` literal `wrapper fault`, a new fixed output word on `--json` and MCP, the human line in `src/human/mirror.rs` unchanged.'
    basis: crates/viola-channel/src/lib.rs:113-142
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system.md
    near: 263
    section: '## Surface: cli › ### Tokens (platform-specific)'
    change: '- For `--json`, the whole result, including refusals and `error` objects, is one JSON document on stdout with the typed exit code, except the exit-2 paths of an oversize `viola answer` and an unusable `viola send`, which write their fixed stderr line and no document. No `hint:` line is printed under `--json`: the typed `refusal` / `error` fields are the whole answer.'
    sidecar: 'design-system: the `--json` one-document rule carves out the two measured exit-2 stderr paths (2026-10-31-sanitised-error-surfaces)'
    rationale: 'Report Spec claims disproved: an oversize `answer` writes no `--json` document at exit 2, nor does the `send` path; the Streams broad line states every result is one document.'
    basis: src/cmd/send.rs:187
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system.md
    near: 338
    section: '## Surface: cli › ### Component Patterns'
    change: 'In the exit-code summary sentence, the exit-2 clause reads: 2 clap usage, or the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` stderr line of an oversize `viola answer` / an unusable `viola send`, no document under `--json`; the rest of the sentence stands.'
    sidecar: 'design-system: the exit-code summary''s exit 2 names the two fixed stderr paths beside clap usage (2026-10-31-sanitised-error-surfaces)'
    rationale: 'Report Spec claims disproved: the exit-2 paths; this is the broad line that surface.md''s exit-2 row details.'
    basis: src/cmd/answer.rs:91
    dependent-of: D-new-detail
note: 'F7 and F8 (the test witness and the census) fit no picked entity and are left to the new-detail pass; the `code-word` sites in layout-templates and a11y-plan are those masters'' own.'
