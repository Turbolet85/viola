proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 31
    section: ## 7. Error Capture & Reporting
    change: "New group after line 31, in its own `<!-- detail: values · errors -->` … `<!-- /detail -->` block, holding one item: - MCP rejects on a refused instance check: the `viola mcp` frame tools (`send`, `wait`, `last`, `answer`) run `check_instance(&ctx.home, &instance_dir)` in `frame` (`crates/viola-mcp/src/tools.rs`) before `live_endpoint`; a refused check returns `instance-unreachable` with `detail: null`, the same `isError` code as the liveness pre-check's, and writes no frame and no line (the founder's own answer, `inputs#I3` Q2: a refused check writes no line, as `link` / `unlink`) (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "2026-10-26-home-and-code-bearing-file-integrity: new MCP-rejects detail group — the frame tools' check_instance before live_endpoint, a refusal instance-unreachable with detail null, no frame, no line (F7)."
    rationale: "The report's Changes, Symbols, states tools.rs frame (337-339) runs check_instance before live_endpoint and a refusal returns result::unreachable() (instance-unreachable, detail null), no frame, no line; Retired (8) names the send/wait/last/answer tools; Decisions & corrections records the founder's no-line answer (inputs#I3 Q2). No group of this file details the body's §7 MCP-rejects line, so the fact opens one after the vt100 group, in the body's order."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14
    fact: F7
note: the body's §7 MCP-rejects broad line (obs-plan.md:537) naming the refused check_instance beside the liveness pre-check is the cascade task's Touch, the strong readers' edit, not this proposal.
