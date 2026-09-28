# design extract

## Relevance
partial — the chunk adds no rendered surface (no web-spa, no human CLI output); design-system reaches it only through the cli surface's `viola hook` stream/exit rules, which the forced-panic case must hold, and the harness perf arm sits outside the design system (test-plan §3 owns harness output).

## Constraints
- design-system §Surface: cli → Platform-Specific Notes requires `viola hook` to write nothing to stderr and always exit 0, and declares it has no human design surface — the forced-panic path (`FAKE_AGENT_HOOK_PANIC`) must keep that shape: no stderr byte, no body, exit 0.
- design-system §Surface: cli → Platform-Specific Notes requires that stack traces never print; errors are fixed messages and full detail goes only to `instances/<name>/diagnostics/` — the panic payload + backtrace (the over-4 KiB line) belongs in `detail-hook.ndjson`, never on either stream.
- design-system §Surface: cli → Exit-code phraseology requires exit 2 (clap usage text) is never produced by `viola hook`; whether the panic hook already guarantees exit 0 on the real binary is research's question.
- design-system §Surface: cli → Colour decision order places `viola hook` at rule 3 (no colour, no glyph, no SGR); any output path the seam touches stays under that rule — in practice `viola hook` emits only the hook decision body on stdout, and none on the fail-open path.
- design-system §Surface: cli → Streams requires results on stdout and messages on stderr; the hook verb has neither messages nor human results, so the perf arm must not introduce a human-facing line from `viola hook` to make timing observable.

## Patterns to follow
- The fail-open "silent" pattern of design-system §Surface: cli → Platform-Specific Notes (`hook` and `mcp` have no human design surface): the panic case asserts absence (empty stderr, empty stdout, exit 0), not a styled message.
- The detail-only diagnostics pattern of design-system §Surface: cli → Platform-Specific Notes (stack traces never print): the backtrace is data for `detail-*.ndjson`, not output.

## Anti-patterns to avoid
- design-system §Anti-Patterns → Per-Surface Bans (cli): NEVER print upstream text, paths, pids or anyhow chains with serde sources in errors — a panic message or backtrace reaching stderr/stdout would breach this and the NEVER-log floor.
- design-system §Anti-Patterns → Per-Surface Bans (cli): NEVER mix data and messages — no diagnostic or timing line from `viola hook` on stdout, which carries only the hook decision body.

## Contract bindings
- design ↔ security: the "stack traces never print" and "no paths/pids/upstream text in errors" rules (design-system §Surface: cli → Platform-Specific Notes; §Anti-Patterns → Per-Surface Bans cli) bind to the security-plan NEVER-log floor / §Error Handling and the CLAUDE.md invariant that `viola hook` exits 0, never writes stderr and fails open with no body.
- design ↔ tests: the forced-panic fail-open case (test-plan §6 Security sweep, `tests/hook_fail_open.rs`) is the executable check of design-system §Surface: cli → Platform-Specific Notes for `hook`.

## Acceptance criteria contributions
- (design) With `FAKE_AGENT_HOOK_PANIC` set on a `fake-agent` build, the real `viola hook` binary exits 0 with zero bytes on stderr and zero bytes on stdout (per design-system §Surface: cli → Platform-Specific Notes).
- (design) The panic's backtrace and payload appear in no stream output — only in `detail-hook.ndjson` (per design-system §Surface: cli → Platform-Specific Notes, "Stack traces never print").
- (design) No code path of `viola hook`, the panic seam included, yields exit 2 (per design-system §Surface: cli → Exit-code phraseology).
