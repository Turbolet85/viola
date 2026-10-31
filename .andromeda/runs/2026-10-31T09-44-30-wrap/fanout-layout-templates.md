proposals:
  - detector: D-design-cli-wording
    severity: warning
    section: 'Component — Primary content block 2: refusal lines and the `unable` column → Hint line'
    change: 'The sentence on hints no longer calls `unknown` opaque: there is no hint for the `unknown` refusal, whose detail is a code word or nothing and never opaque free text (a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads as none), for `wrapper fault` or for `internal error`.'
    sidecar: 'Primary content block 2, Hint line: the `unknown` refusal detail is a code word or nothing, the opaque wording retired (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'The report Symbols / APIs bullet on `reply_of` and `of_reply` screens a non-code-word `refusal` to `unknown` and a non-code-word string `detail` to `None` / `null`; its Expected amendments carry layout-templates §Component — Primary content block 2 with `unknown` detail a code word or nothing, not opaque free text, and its `code-word` sweep lists `layout-templates.md:331` as changed.'
    basis: 'src/cmd/client.rs:77-81'
  - detector: D-design-cli-wording
    severity: warning
    section: 'Component — Primary content block 2: refusal lines and the `unable` column → Exit codes as the typed tail'
    change: 'After `Under --json the same outcome is one document on stdout`, state the exit-20 `wrapper-fault` document on `--json` and MCP: its detail is the known fault fixed `message` for the five known codes, and for a code outside them reads `{"code": <code>, "message": "wrapper fault", "data": null}`; the human line `error: wrapper fault  <code>` is unchanged.'
    sidecar: 'Primary content block 2, Exit codes: the new fixed `message` literal `wrapper fault` in the `wrapper-fault` detail for a code outside the five is stated (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'The report Coverage of new surfaces names the `message` literal `wrapper fault` as a new fixed output word on `--json` and MCP, and its `sanitised_fault` bullet gives the body; a grep of design-system and layout-templates finds that literal in no `wrapper-fault` document, only the unchanged human line, so the new wording is unstated.'
    basis: 'crates/viola-channel/src/lib.rs:113-142'
note: 'D-layout-surface fired nothing: the report adds no user-facing surface or region. D-retired fired nothing: `viola_agent_claude::Refusal`, `fault_detail` and `CoreError` are absent from layout-templates. The opaque wording stands in design-system at surface.md:81, surface.md:147 and color-palette.md:102, which the report lists as changed and which belong to that document.'
