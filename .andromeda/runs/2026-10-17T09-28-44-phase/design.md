# design extract

## Relevance
partial — `viola mcp` has no human design surface (per design-system §Surface: cli · Platform-Specific Notes); what applies is the machine-view discipline, the refusal phraseology and the readback semantics its five tools mirror from the CLI's `--json` forms.

## Constraints
- design-system §Surface: cli · Colour decision order puts `viola mcp` at step 3: stdout carries MCP frames only, so the server requires no colour, no glyph and no SGR on any path, decided before `NO_COLOR` / `TERM` / TTY are ever consulted (whether the `role_of` / styling code already routes `mcp` past the style layer is research's question).
- design-system §Surface: cli · Platform-Specific Notes requires `mcp` to write only MCP frames on stdout and gives it no human design surface: no context line (`waiting:`), no `hint:` line, no human result line reaches its stdout.
- design-system §Surface: cli · Streams and §Component Patterns (2. `viola send`) require hints to be human-mode only: the machine view carries the cause as a typed code (`refusal` / `error` + `detail`), never prose; an MCP refusal mirroring the `--json` refusal object therefore carries no hint text as its cause.
- design-system §Component Patterns (2. `viola send`) requires that no driver-facing text names `viola release` (T3): the `budget-paused` and `human-typing` refusals reach drivers, and `release` stays a human verb — this binds the `send` tool's `budget-paused` form (CARRY 4) and any fixed message on an `isError` result.
- design-system §Brand Identity (Domain anchors: Standard phraseology; Readback) requires the refusal vocabulary to be the short fixed words (`unable`, `human-typing`, `budget-paused`, `not-delivered`, `unverified-cli`, `instance-unreachable`, `unconfirmable`) and a send to count only once read back by the matching `prompt-submitted` or, for `/clear`, its measured post-condition — so a `send` tool result never presumes delivery.
- design-system §Brand Identity (Signature element) requires `unconfirmable` as a distinct outcome that is never filled/read back: a `send` of a local command with no measured post-condition (`/compact`) returns `unconfirmable`, not a success.
- design-system §Component Patterns (3. `viola wait` / `viola last`) requires `--json` message text to stay serde-escaped (the `\xHH` message-mode escaper is the human mode's), and `wait` to have no spinner and no elapsed-time counter.

## Patterns to follow
- The `--json` result and refusal shapes per design-system §Surface: cli · Exit-code phraseology (`{"v":1,"ok":{…}}`, the refusal object for exits 10–14, `{"v":1,"error":"wrapper-fault",…}` for 20, `{"v":1,"error":"instance-unreachable","detail":null}` for 21) are the shapes each tool's result and refusal mirror (arch §Standard Contracts is their authority).
- The reason · detail pairs per design-system §Component Patterns (2. `viola send`): `not-delivered` · `turn-running` / `control-character` / `empty-text` / `input-not-ready` / `no-prompt-submitted` / `unknown-dialog`, and `budget-paused` · `five-hour` / `seven-day` — the closed codes the tools return.
- The machine view keeps names whole per design-system §Surface: cli · Width (`--json` and a piped board never cut a NAME), so the `list` tool returns every name unabridged.
- Unwrapped sessions are bare strips, visible but never a `target`, per design-system §Brand Identity (Domain anchors: Strip holder): the `list` tool shows them read-only and a `send` / `answer` naming one is refused `instance-unreachable` with no detail (design-system §Surface: cli · Exit-code phraseology, row 21).

## Anti-patterns to avoid
- Optimistic success before confirmation (a "sent" result, a toast-like notice) per design-system §Anti-Patterns · Rejected Defaults (Optimistic "Sending…" spinner): the result is read back, unable plus a typed reason, or unconfirmable.
- Progress presumed before it is confirmed per design-system §Anti-Patterns · Per-Surface Bans (cli: no spinner, progress bar or live-redrawing output) — the plan does not name MCP progress notifications; whether `wait` emits any is a planning question, and the ban's reason is the reading to apply.
- Upstream text, paths, pids or anyhow chains with serde sources in an error or refusal per design-system §Anti-Patterns · Per-Surface Bans (cli), and no stack trace (§Surface: cli · Platform-Specific Notes).

## Contract bindings
- design ↔ arch: the `--json` document shapes the tools mirror and the per-cause `detail` codes of exit 21 / exit 1 that design-system §Component Patterns (2. `viola send`) marks "Pending an arch amendment" — the tools keep arch §Standard Contracts' shape until arch names them.
- design ↔ security: the code-only refusal rule (security-plan §Error Handling, the NEVER-log floor) is the same floor as design-system §Anti-Patterns · Per-Surface Bans (cli).
- design ↔ obs: obs-plan D-20's log detail codes are the codes design-system §Component Patterns (2. `viola send`) names for the machine view of `instance-unreachable` causes.

## Acceptance criteria contributions
- The `viola mcp` process writes no raw ESC (0x1B) byte and no non-frame line on stdout under any of `NO_COLOR`, `TERM`, `COLORTERM` or a TTY stdout (per design-system §Surface: cli · Colour decision order)
- No tool result, `isError` content or advertised tool description contains `release` as a suggested action; the `budget-paused` refusal carries `five-hour` / `seven-day` and no cursor (per design-system §Component Patterns (2. `viola send`))
- Each tool's refusal carries the closed code and detail of its CLI verb's `--json` refusal and no hint prose (per design-system §Surface: cli · Streams)
- A `send` tool call of `/compact` returns `unconfirmable`, never a read-back result (per design-system §Brand Identity (Signature element))
