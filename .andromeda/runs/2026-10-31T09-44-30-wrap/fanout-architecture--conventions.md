proposals:
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 15
    section: "## Conventions"
    change: "The line's first sentence reads: Readers treat an unrecognised `detail` that is a code word as opaque text, never as an error; a string `detail` that is no code word reads `null`. The rest of the line is unchanged."
    sidecar: "Conventions, RefusalReason: the reader's opaque rule now covers a code word only, a string detail that is no code word reads null (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Symbols / APIs (`reply_of` and `of_reply`) and Expected amendments (§Conventions, Error handling schema: `unknown`'s detail a code word or null, a reader's opaque rule kept); pattern `code-word` lists this line as changed."
    basis: src/cmd/client.rs:77
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 13
    section: "## Conventions"
    change: "The row reads: `unknown`: a code word (`viola_core::is_code_word`) or `null`"
    sidecar: "Conventions, RefusalReason: `unknown`'s detail is a code word or null, no longer any value (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Symbols / APIs: a string `detail` that is no code word reads `None` / `null`, a non-string `detail` is no known reply, so `any value` no longer stands; a further occurrence of the claim the code-word change retires, swept in this file."
    basis: crates/viola-core/src/lib.rs:347
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 64
    section: "## Conventions"
    change: "The sentence reads: A refusal is a tool result with `isError: true` and the same `refusal` + `detail` in `structuredContent`, screened as CLI `--json` is (the code-word rule of `RefusalReason`). The rest of the line is unchanged."
    sidecar: "Conventions, MCP: a refusal's structuredContent is the channel's refusal and detail after the code-word screen (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Symbols / APIs: `of_reply` reads a `refusal` that is no code word as `unknown` and a string `detail` that is no code word as `null`, so `the same` holds only for code words; a further occurrence of the claim the code-word change retires, swept in this file."
    basis: crates/viola-mcp/src/result.rs:71
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 64
    section: "## Conventions"
    change: "The phrase `wrapper-fault` with the JSON-RPC fault's `{code,message,data}` reads: `wrapper-fault` with the closed `{code,message,data}` that `viola_channel::sanitised_fault` reads from the fault, a known code's fixed message and closed data, a code outside the five the message `wrapper fault` and `data: null`. The rest of the line is unchanged."
    sidecar: "Conventions, MCP: wrapper-fault's detail is the closed object of `viola_channel::sanitised_fault`, not the peer's fault as sent (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Symbols / APIs (`viola_channel::sanitised_fault`) and Expected amendments (pattern `fault-reader` lists this line as changed; `wrapper-fault`'s fields closed); Coverage of new surfaces names the `wrapper fault` literal."
    basis: crates/viola-channel/src/lib.rs:113
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 15
    section: "## Conventions"
    change: "A new line follows this one, indented as its continuation: The two clients screen a peer's reply (`reply_of` in `src/cmd/client.rs`, `of_reply` in `crates/viola-mcp/src/result.rs`) by `viola_core::is_code_word`, 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first: a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads `null`, a non-string `detail` stays no known reply; a code word outside the closed list stays visible in the document and in `refused: <word>`, its exit `unknown`'s 14 (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "Conventions, RefusalReason: added the clients' screen of a peer's reply by `viola_core::is_code_word` (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Symbols / APIs (`viola_core::is_code_word`; `reply_of` and `of_reply`) and Coverage of new surfaces (the screen of a peer's reply); new detail this category holds, its own row so the line above stays under 2 000 chars."
    basis: crates/viola-core/src/lib.rs:347
note: The report's `CoreError` site `conventions.md:15` is a leaf among `stack.md` and `services/viola-core.md`, not this file, whose line 15 names no `CoreError`; this file also names no `Refusal` or `fault_detail`, so it takes no D-retired proposal.
