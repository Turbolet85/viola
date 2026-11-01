# Apply — security-plan · part 4 of 4

## Verdicts
- fanout-security-plan--cascade-g3.md 11 · applied · .andromeda/security-plan-detail/security-anti-patterns.md:8#1
- fanout-security-plan--cascade-g3.md 12 · applied · .andromeda/security-plan-detail/security-anti-patterns.md:9#1
- cascade-compare-security-plan.md 4 · applied · .andromeda/security-plan-detail/security-anti-patterns.md:39+#1

## Edits
- line: .andromeda/security-plan-detail/security-anti-patterns.md:8
  old: «the client-side euid check owed to the route entry "Unix endpoint and home hardening"»
  new: «the client-side euid check, `peer_admitted(client.server_euid())` at the three client seams, before the server verification»
- line: .andromeda/security-plan-detail/security-anti-patterns.md:9
  old: «each verifying the server on Linux and Windows (the CLI verbs through `cmd::client::connect`); only every client's euid check and macOS's pid and start-time check stay owed to "Unix endpoint and home hardening".»
  new: «each verifying the server on Linux and Windows and, by the peer euid alone, on macOS (the CLI verbs through `cmd::client::connect`); only macOS's pid and start-time check stays absent, the named carve-out, owned by no route entry.»
- after: .andromeda/security-plan-detail/security-anti-patterns.md:39
  anchor: «- NEVER let `config.json`, a `VIOLA_*` environment variable or a CLI flag switch off any control in this plan: the IPC DACL or socket-directory check, the peer or server verification, the home strict-modes check or `validate_paste_text`. Environment variables are not a configuration channel (architecture.md Cross-cutting Patterns: Config management), and `VIOLA_DIR` is set by a child any same-session agent can influence. The two carve-outs are the test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s; Decisions Log 2026-09-27) and `FAKE_AGENT_HOOK_PANIC` (fires only on the exact value `1`, a forced fail-open `hook` panic; Decisions Log 2026-09-28): each exists only under `cfg(feature = "fake-agent")`, is absent from every release build and switches off nothing (§Input Validation). The terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM` are read in every build (the founder's own word, chunk 2026-10-17-cli-output-discipline), only by `viola list`'s human mode and only to choose whether and how deep SGR bytes are written: they switch off no control (no escaping, no `validate_paste_text`, no check), widen no redaction and are not a configuration channel (§Input Validation). NEVER add another such seam or environment read, let these reach a release build, or let a colour fact change a control or reach a log line, error or refusal, without its own Decisions Log entry.»
  add: «- NEVER add an env, flag or feature seam to force a refusing peer-euid decision: one account can make no foreign peer, so the decisions are proven over injected values (`peer_euid_admitted(observed, own, reported)`, and the wrapper's private `serve_admitting(dispatch, admitted)` over an injected `Admission`), and each client seam's `peer_admitted` wiring (`cmd::client::connect`, `cmd::hook::connect`, `viola-mcp`'s `frame`) stays the named weakest proof, read only through a run's forced refusal, which no committed test repeats.»

## Patterns
[[pattern]]
id = "sec4-01"
fixed = 'euid check owed to the route entry'

[[pattern]]
id = "sec4-02"
fixed = 'pid and start-time check stay owed'
