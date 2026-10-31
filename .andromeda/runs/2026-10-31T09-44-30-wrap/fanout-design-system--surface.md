proposals:
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 147
    section: '## Surface: cli › ### Component Patterns'
    change: 'Row 14 reads: | 14 | `unable  unknown  <detail>` (the detail a code word or nothing, never free text; a reader keeps it opaque) | refusal object, `detail` a code word or `null` |'
    sidecar: 'Surface cli exit-code row 14: the detail of `unknown` is a code word or nothing, no longer opaque text (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report Changes, Symbols / APIs: in `reply_of` and `of_reply` a string `detail` that is no code word reads `None` / `null`; Expected amendments name exit-code row 14 and pattern `code-word` at surface.md:147 as changed, so `<detail as opaque text>` is a standing old wording.'
    basis: .andromeda/design-system-detail/surface.md:147
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 81
    section: '## Surface: cli › ### Component Patterns'
    change: 'The first sentence reads: No hint line for `unknown` (exit 14), because its detail is a code word or nothing, opaque to a reader, and a hint would have to guess. The sentence on `error: wrapper fault` and `error: internal error` stays as it stands.'
    sidecar: 'Surface cli send hints: the no-hint reason for `unknown` states its detail as a code word or nothing (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report Changes, Expected amendments: pattern `code-word` lists surface.md:81 as changed, the same claim as row 14; Outcome keeps `unknown` without a hint, and the plan keeps the opaque rule for a reader.'
    basis: .andromeda/design-system-detail/surface.md:81
    dependent-of: D-design-cli-wording
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 155
    section: '## Surface: cli › ### Component Patterns'
    change: 'New list item: - A reply from a peer is screened by both its readers, `reply_of` (CLI) and `of_reply` (MCP) (chunk 2026-10-31-sanitised-error-surfaces). A code word is 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first. A `refusal` that is no code word reads `unknown`; a string `detail` that is no code word reads nothing (`null` in a document); a non-string `detail` is still no known reply. A code word outside the closed list stays visible in the document and in `refused: <word>`, and exits 14 as `unknown` does.'
    sidecar: 'Surface cli exit codes: new item for the screen on a peer reply, its code word form and the exit of a word outside the closed list (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report Changes, Symbols / APIs (`viola_core::is_code_word`; `reply_of` and `of_reply`) and Coverage of new surfaces (the screen on a peer reply): this output state stands nowhere in the file.'
    basis: crates/viola-core/src/lib.rs:347-357
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 156
    section: '## Surface: cli › ### Component Patterns'
    change: 'New list item: - The `detail` of the exit-20 `wrapper-fault` document is the sanitised fault, closed to `code`, `message` and `data`, nothing of the peer but closed values (chunk 2026-10-31-sanitised-error-surfaces). For the five known codes `message` is the fixed text of the fault and `data` is `null`, except `-32602`: with an integer `data.supported` and a version-text `data.wrapper` (1 to 64 ASCII letters, digits, `.`, `-` or `+`) it reads the unsupported-version body with those two values, with `data.reason` exactly `release-from-driver` it reads that body, and any other `-32602` reads `invalid params` with `data` `null`. A code outside the five reads `{"code":<code>,"message":"wrapper fault","data":null}`. The human line `error: wrapper fault  <code>` is unchanged.'
    sidecar: 'Surface cli exit codes: new item for the closed `detail` of `wrapper-fault` and the new fixed `message` literal `wrapper fault` (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report Changes, Symbols / APIs (`viola_channel::sanitised_fault`) and Coverage of new surfaces: the `message` literal `wrapper fault` is a new fixed output word on `--json`, and the file states the document only as `"detail":{…}`.'
    basis: crates/viola-channel/src/lib.rs:113-142
note: 'The report names `refused: <word>` for `reply_of` and `of_reply` without saying which surface prints it; the proposal at near 155 keeps the wording of the report and assigns it to neither reader.'
