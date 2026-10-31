# Cascade compare — design-system · wrap · 2026-10-31-sanitised-error-surfaces
E items 5 · held 3 · lost at classify 0 · lost at strong 0 · beside a placed fact 2 · cascade only 3 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### surface#16.1 · lost at classify · beside a placed fact F1 (near 21, surface#2.1) · .andromeda/design-system-detail/surface.md:155
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 155
    section: '## Surface: cli › ### Component Patterns'
    change: 'New list item: - A reply from a peer is screened by both its readers, `reply_of` (CLI) and `of_reply` (MCP) (chunk 2026-10-31-sanitised-error-surfaces). A code word is 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first. A `refusal` that is no code word reads `unknown`; a string `detail` that is no code word reads nothing (`null` in a document); a non-string `detail` is still no known reply. A code word outside the closed list stays visible in the document and in `refused: <word>`, and exits 14 as `unknown` does.'
    sidecar: 'Surface cli exit codes: new item for the screen on a peer reply, its code word form and the exit of a word outside the closed list (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report Changes, Symbols / APIs (`viola_core::is_code_word`; `reply_of` and `of_reply`) and Coverage of new surfaces (the screen on a peer reply): this output state stands nowhere in the file.'
    basis: crates/viola-core/src/lib.rs:347-357

### surface#16.2 · lost at classify · beside a placed fact F2 (near 21, surface#2.1) · .andromeda/design-system-detail/surface.md:156
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 156
    section: '## Surface: cli › ### Component Patterns'
    change: 'New list item: - The `detail` of the exit-20 `wrapper-fault` document is the sanitised fault, closed to `code`, `message` and `data`, nothing of the peer but closed values (chunk 2026-10-31-sanitised-error-surfaces). For the five known codes `message` is the fixed text of the fault and `data` is `null`, except `-32602`: with an integer `data.supported` and a version-text `data.wrapper` (1 to 64 ASCII letters, digits, `.`, `-` or `+`) it reads the unsupported-version body with those two values, with `data.reason` exactly `release-from-driver` it reads that body, and any other `-32602` reads `invalid params` with `data` `null`. A code outside the five reads `{"code":<code>,"message":"wrapper fault","data":null}`. The human line `error: wrapper fault  <code>` is unchanged.'
    sidecar: 'Surface cli exit codes: new item for the closed `detail` of `wrapper-fault` and the new fixed `message` literal `wrapper fault` (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report Changes, Symbols / APIs (`viola_channel::sanitised_fault`) and Coverage of new surfaces: the `message` literal `wrapper fault` is a new fixed output word on `--json`, and the file states the document only as `"detail":{…}`.'
    basis: crates/viola-channel/src/lib.rs:113-142

## Held

- color-palette#5.41
- surface#15.8
- surface#6.1

## Cascade only

- surface#15.3
- surface#15.9
- surface#2.1
