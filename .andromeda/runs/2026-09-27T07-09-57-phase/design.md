# design extract

## Relevance
partial. The chunk is mostly transport, with no web-spa rendering and no tokens, colour, motion or typography in play. Design applies only where the channel shows up on the cli surface: the silence of `viola run`, the exit-1 start refusals when the bind is lost or squatted, and the fixed-message rule for errors such as `ChannelError`.

## Constraints
- design-system §Surface: cli → Component Patterns 5 (`run`) requires that `viola run` prints nothing once the child starts. Binding the endpoint, serving it on std threads and accepting or refusing frames must never write to the wrapped child's terminal (stdout or stderr), and must never emit colour, glyphs or cursor control. The same rule appears in Colour decision order item 2 and in the expression-0.0 row of §Brand Identity.
- design-system §Surface: cli → Component Patterns 2 (exit-1 start refusals) requires the squatted-endpoint cause to print exactly `unable: the endpoint for <name> is held by another process`, followed by `hint: another process holds this name's endpoint; stop it or pick another name`, then exit 1. The already-live cause is `unable: <name> is already live` → `hint: viola list`. This chunk makes the exclusive bind the arbiter, so the loser of two concurrent starts of one name must land on one of these fixed lines. Which one the bind-race loser reports (already-live or squatted-name) is not settled by the plan and is a P4 decision.
- design-system §Surface: cli → Exit-code phraseology (row 1) requires exit-1 start refusals to have no `--json` document until arch amends it. Human output goes to stderr, and each cause has its own hint.
- design-system §Surface: cli → Platform-Specific Notes ("Stack traces never print") requires errors to be fixed messages, with full detail only in `instances/<name>/diagnostics/`. The fixed-`Display` `ChannelError` (freight 3) must therefore never reach a human line with a source chain. Whether the code already routes channel faults this way is research's question.
- design-system §Surface: cli → Streams requires refusals, their `hint:` lines and `error: …` lines on stderr, and results on stdout, never mixed.
- design-system §Brand Identity (Standard phraseology) requires every refusal to be `unable` plus a typed reason, with no filler and no guessing. This covers human-facing text only. The JSON-RPC `-32602` "unsupported protocol version" and `-32601` wire messages belong to architecture, not design.

## Patterns to follow
- Per-cause fixed-message line plus one hint for `run` start refusals (design-system §Surface: cli → Component Patterns 2, the T4 list). Reuse the existing already-live / stale-heartbeat refusal emitters from the instance-state chunk rather than writing a new format. Whether such emitters exist at HEAD is research's question.
- An `error: internal error` fault has no hint and keeps its detail only in `diagnostics/` (design-system §Surface: cli → Component Patterns 2, last bullets; §Design Decisions Log T4 "Not changed"). Channel/bind faults that are not refusals follow this path.
- ASCII-only human output, two-space field separation, and ` - ` inside a field (design-system §Iconography → Separator; §Surface: cli → Width).

## Anti-patterns to avoid
- NEVER print paths, pids, the pipe/socket endpoint name, upstream text or anyhow chains with serde sources in errors or hints (design-system §Anti-Patterns → Per-Surface Bans → cli). A bind/squat refusal must not name the pipe path or the holding pid.
- NEVER emit anything (colour, glyphs, cursor control, or any line at all) while `viola run`'s child owns the terminal (design-system §Anti-Patterns → Per-Surface Bans → cli, "NEVER emit colour … under … `viola run`").
- NEVER print stack traces (design-system §Anti-Patterns → Per-Surface Bans → cli).

## Contract bindings
- design cli exit-1 cause lines ↔ obs-plan D-20 log detail codes (`already-live`, `squatted-name`). The human line and the diagnostic code for the same cause must agree (design-system §Surface: cli → Component Patterns 2, the "Pending an arch amendment" bullet).
- design "no paths/pids in errors" ↔ security-plan NEVER-log floor, and the `veil` `#[derive(Redact)]` / fixed-`Display` `ChannelError` in freight 3.
- design exit-1 refusal set ↔ the tests exit-cause matrix (design-system §Design Decisions Log T4) and ↔ architecture's exit-1 "no `--json` document".

## Acceptance criteria contributions
- (design) Losing the exclusive endpoint bind makes `viola run` exit 1. Its stderr is exactly one fixed-message `unable: …` line for the chosen cause plus that cause's `hint:` line, with no path, pid or pipe name in either (per design-system §Surface: cli → Component Patterns 2).
- (design) While the wrapped child runs, the channel server writes zero bytes to the parent terminal's stdout/stderr, including on malformed, oversize (`> MAX_FRAME`) or newer-`v` frames (per design-system §Surface: cli → Component Patterns 5).
- (design) No human-facing output from a `ChannelError` includes a source chain or a stack trace; detail goes only to `diagnostics/` (per design-system §Surface: cli → Platform-Specific Notes).

## Relevant amendment history
(none). `D:/dev/projects/viola/.andromeda/design-system-amendments.md` does not exist yet, which is normal on a fresh project. For context only, design-system §Design Decisions Log T4 already sets the per-cause exit-1 lines, including the squatted-endpoint cause.
