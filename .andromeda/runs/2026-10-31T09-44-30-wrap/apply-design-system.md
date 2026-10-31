# Apply — design-system

## Verdicts
- fanout-design-system--cascade.md 10 · applied · .andromeda/design-system.md:263#1
- fanout-design-system--cascade.md 11 · applied · .andromeda/design-system.md:338#1
- fanout-design-system--cascade-new-surface.md 1 · applied · .andromeda/design-system-detail/surface.md:156+#3
- fanout-design-system--cascade.md 1 · applied · .andromeda/design-system-detail/surface.md:147#1
- fanout-design-system--cascade.md 2 · applied · .andromeda/design-system-detail/surface.md:81#1
- fanout-design-system--cascade.md 4 · applied · .andromeda/design-system-detail/surface.md:21+#1
- fanout-design-system--cascade.md 5 · applied · .andromeda/design-system-detail/surface.md:21+#2
- fanout-design-system--cascade.md 6 · applied · .andromeda/design-system-detail/surface.md:21+#3
- fanout-design-system--cascade.md 7 · applied · .andromeda/design-system-detail/surface.md:142#1
- fanout-design-system--cascade.md 8 · applied · .andromeda/design-system-detail/surface.md:156+#1
- fanout-design-system--cascade.md 9 · applied · .andromeda/design-system-detail/surface.md:156+#2
- fanout-design-system--cascade.md 3 · applied · .andromeda/design-system-detail/color-palette.md:102#1

## Edits
- line: .andromeda/design-system.md:263
  old: «is one JSON document on stdout with the typed exit code. No»
  new: «is one JSON document on stdout with the typed exit code, except an oversize `viola answer` and an unusable `viola send`, which exit 2 with their fixed stderr line and no document. No»
- line: .andromeda/design-system.md:338
  old: «, 2 clap usage, 10 to 14»
  new: «, 2 clap usage or the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` stderr line of an oversize `viola answer` / an unusable `viola send` (no document under `--json`), 10 to 14»
- after: .andromeda/design-system-detail/surface.md:21
  anchor: «- stderr: context lines (`waiting: <name>`, the issue line of a send), refusals (`unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column, as in its sample; the exit-code table omits the `<name>` field; only the exit-1 refusals of `viola run`, `viola revive` and `viola verify`, and `viola list`'s exit-21 strict-modes refusal (chunk 2026-10-11-the-board-viola-list, the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer; the operator's answer superseded 2026-10-13), and the exit-21 strict-modes refusal `viola link` / `viola unlink` make of the driven instance, and the one `viola send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance, use the fixed message form, `unable: <text>`, as in `unable: <name> is already live`), each refusal's `hint:` line directly after it, and errors (`error: …`).»
  add: «- Code word: 1 to 64 bytes of lower-case letters, digits and hyphens, a lower-case letter first (`viola_core::is_code_word`); both clients, `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`), screen a peer's `refusal` and `detail` through it.»
- after: .andromeda/design-system-detail/surface.md:21
  anchor: «- stderr: context lines (`waiting: <name>`, the issue line of a send), refusals (`unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column, as in its sample; the exit-code table omits the `<name>` field; only the exit-1 refusals of `viola run`, `viola revive` and `viola verify`, and `viola list`'s exit-21 strict-modes refusal (chunk 2026-10-11-the-board-viola-list, the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer; the operator's answer superseded 2026-10-13), and the exit-21 strict-modes refusal `viola link` / `viola unlink` make of the driven instance, and the one `viola send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance, use the fixed message form, `unable: <text>`, as in `unable: <name> is already live`), each refusal's `hint:` line directly after it, and errors (`error: …`).»
  add: «- A peer's `refusal` that is no code word reads `unknown`; a string `detail` that is no code word reads as no detail, null under `--json`; a non-string `detail` is no known reply.»
- after: .andromeda/design-system-detail/surface.md:21
  anchor: «- stderr: context lines (`waiting: <name>`, the issue line of a send), refusals (`unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column, as in its sample; the exit-code table omits the `<name>` field; only the exit-1 refusals of `viola run`, `viola revive` and `viola verify`, and `viola list`'s exit-21 strict-modes refusal (chunk 2026-10-11-the-board-viola-list, the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer; the operator's answer superseded 2026-10-13), and the exit-21 strict-modes refusal `viola link` / `viola unlink` make of the driven instance, and the one `viola send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance, use the fixed message form, `unable: <text>`, as in `unable: <name> is already live`), each refusal's `hint:` line directly after it, and errors (`error: …`).»
  add: «- A code word outside the closed refusal list stays visible, by design, in the document and in `refused: <word>`, and exits 14 as `unknown` does (`reason_of`).»
- line: .andromeda/design-system-detail/surface.md:81
  old: «because its detail is opaque and a hint would have to guess. No hint line for `error: wrapper fault` (exit 20) or `error: internal error` (exit 1), because they are faults, not refusals, and their detail goes only to `diagnostics/`.»
  new: «because its detail is a code word or nothing and a hint would have to guess. No hint line for `error: wrapper fault` (exit 20) or `error: internal error` (exit 1), because they are faults, not refusals: the human line holds the code only, further detail goes to `diagnostics/`, and the `--json` document's `detail` is row 20's closed object.»
- line: .andromeda/design-system-detail/surface.md:142
  old: «| 2 | clap usage text (never produced by `viola hook`) | — |»
  new: «| 2 | clap usage text (never produced by `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (an oversize `viola answer`) or `UNUSABLE_TEXT` line (an unusable `viola send`) on stderr, whatever `--json` says; `tests/contract_error_bodies.rs` names them in `NOT_ERROR_BODIES` | no document |»
- line: .andromeda/design-system-detail/surface.md:147
  old: «| 14 | `unable  unknown  <detail as opaque text>` | refusal object |»
  new: «| 14 | `unable  unknown  <detail>`, the detail a code word or nothing | refusal object |»
- after: .andromeda/design-system-detail/surface.md:156
  anchor: «- Exit 20 is also reached by `viola link` / `viola unlink` on a pair the wrapper refuses (`-32602`; a `driver` equal to `driven`, which the CLI never refuses itself): `error: wrapper fault  <code>`, or under `--json` the `wrapper-fault` document.»
  add: «- Exit 20's `--json` `detail` is closed for the five known codes (`viola_channel::sanitised_fault`): `message` is each code's fixed text; `-32602` with an integer `data.supported` and a version-text `data.wrapper` reads the unsupported-version body with those two values, `data.reason` exactly `release-from-driver` reads that body, and any other `-32602` reads `invalid params` with null `data`; every other known code has null `data`.»
- after: .andromeda/design-system-detail/surface.md:156
  anchor: «- Exit 20 is also reached by `viola link` / `viola unlink` on a pair the wrapper refuses (`-32602`; a `driver` equal to `driven`, which the CLI never refuses itself): `error: wrapper fault  <code>`, or under `--json` the `wrapper-fault` document.»
  add: «- A code outside the five prints the same human line, `error: wrapper fault  <code>`, and under `--json` the document `{"v":1,"error":"wrapper-fault","detail":{"code":<code>,"message":"wrapper fault","data":null}}`; MCP carries the same fixed `message`.»
- after: .andromeda/design-system-detail/surface.md:156
  anchor: «- Exit 20 is also reached by `viola link` / `viola unlink` on a pair the wrapper refuses (`-32602`; a `driver` equal to `driven`, which the CLI never refuses itself): `error: wrapper fault  <code>`, or under `--json` the `wrapper-fault` document.»
  add: «- Each CLI `--json` failure `tests/cli_error_bodies.rs` drives prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text.»
- line: .andromeda/design-system-detail/color-palette.md:102
  old: «`unable · unknown`, with the detail printed as opaque plain text»
  new: «`unable · unknown`, with the detail, a code word or nothing, printed as plain text»

## Patterns
[[pattern]]
id = "ds-01"
fixed = 'is one JSON document on stdout with the typed exit code'

[[pattern]]
id = "ds-02"
fixed = '2 clap usage,'

[[pattern]]
id = "ds-03"
regex = 'its detail is opaque|their detail goes only to'

[[pattern]]
id = "ds-04"
fixed = 'clap usage text (never produced by'

[[pattern]]
id = "ds-05"
fixed = 'detail as opaque text'

[[pattern]]
id = "ds-06"
fixed = 'printed as opaque plain text'
