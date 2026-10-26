proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 24
    section: "## 1. A11y Scope Summary"
    change: "New group `<!-- detail: values · scope-entities -->` … `<!-- /detail -->` after the TUI passthrough group: - **Entity:** Claude Code plugin: hooks (`viola hook` event and dialog calls) and the stdio MCP server's frame tools (`send`, `wait`, `last`, `answer`) against an endpoint whose serving process is not the recorded wrapper, as built (chunk 2026-10-26-server-verification-before-any-frame; off macOS, where there is no peer pid and the check reads as passed) / - **Source:** chunk report 2026-10-26-server-verification-before-any-frame, Changes → Symbols (`src/cmd/hook.rs`, `crates/viola-mcp/src/tools.rs`) / - **Assertability:** not-assertable / - **Reason:** a hook event or dialog call that meets an impostor wrapper fails open: exit 0, empty stdout and stderr, under 1.0 s, no frame; an MCP frame tool that meets one returns `isError` with code `instance-unreachable` only and sends no frame; neither writes any human-facing byte, so the entity stays not-assertable and the human-wins guarantee holds."
    sidecar: "a11y-scope-summary: new scope-entities group — the plugin hooks fail open and the MCP frame tools return `instance-unreachable` only against an impostor wrapper, writing no human-facing byte (chunk 2026-10-26-server-verification-before-any-frame)"
    rationale: "The report's Changes → Symbols adds the hook's shared `connect` with `NotSent` detail `server-verify-failed` (no frame) and the MCP `frame`'s `server_verified` returning `result::unreachable()` with no frame; Retired names the hook and MCP gaps' server-identity half retired on Linux and Windows. The not-assertable plugin entity has no detail group yet, so the as-built fail-open outcome is new detail under section 1."
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:24
    fact: F4
note: the plugin hooks / MCP not-assertable entity itself stands in the body (a11y-plan.md section 1, Not-assertable); this group is its first detail, and the body's pointer already covers the whole section.
