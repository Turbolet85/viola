proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 111
    section: '## Standard Contracts'
    change: 'Add an item to the contract-cli-json-output group: - The `wrapper-fault` document holds closed values only (chunk 2026-10-31-sanitised-error-surfaces): its `detail` is `viola_channel::sanitised_fault` over the peer `error`, called by `reply_of` (`src/cmd/client.rs`) and by `of_reply` (`crates/viola-mcp/src/result.rs`) for the MCP result of the same shape. It gives `None` without an integer `code`. For the five known codes `message` is the fixed text of the fault; `-32602` with an integer `data.supported` and a `data.wrapper` of 1 to 64 ASCII letters, digits, `.`, `-` or `+` reads the unsupported-version body with those two values, `data.reason` exactly `release-from-driver` reads that body, and any other `-32602` reads `invalid params` with `data: null`; every other known code reads `data: null`. A code outside the five reads `{"code": <code>, "message": "wrapper fault", "data": null}`. So the `message` and `data` of a peer reach no document beyond those closed values, and the seven wire bodies are unchanged.'
    sidecar: 'standard-contracts: the closed fields of the `wrapper-fault` document added, as `viola_channel::sanitised_fault` reads them (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'The report, Changes > Symbols / APIs (`sanitised_fault`) and Expected amendments (§Standard Contracts, CLI `--json` output, `wrapper-fault` fields closed), brings this detail; the group holds nothing on it yet.'
    basis: crates/viola-channel/src/lib.rs:113
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 111
    section: '## Standard Contracts'
    change: 'Add an item to the contract-cli-json-output group: - A refusal from a peer is screened before the document is written, in `reply_of` and, for the MCP result, `of_reply`, through `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first): a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads `null`, and a non-string `detail` is still no known reply. A code word outside the closed list stays visible in the document and in `refused: <word>`, and its exit stays the 14 of `unknown` (chunk 2026-10-31-sanitised-error-surfaces).'
    sidecar: 'standard-contracts: the code-word screen on a refusal in the CLI `--json` and MCP documents added (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'The report, Changes > Symbols / APIs (`is_code_word`, `reply_of` and `of_reply`): the documents no longer mirror every peer `refusal` and `detail`; the reader rule itself is the Conventions amendment the report names, this item the document contract.'
    basis: src/cmd/client.rs:77
note: 'No retired item stands in this file (`viola_agent_claude::Refusal`, `fault_detail`, `CoreError` absent; `run::resolve_program` at lines 93 and 133 is the report no-change), and D-arch-resources and D-arch-decisions read clean (no new resource, dependency or crate); the body edit at .andromeda/architecture.md:227 is outside this file.'
