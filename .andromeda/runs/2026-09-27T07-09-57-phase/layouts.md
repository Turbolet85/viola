# layouts extract

## Relevance
Partial. The web-spa surface is out of scope because this chunk adds no bay element, route or SSE kind. The cli surface applies only where `viola run` starts: its silent passthrough and the exit-1 start refusal that the endpoint-bind arbiter's loser must print.

## Constraints
- While the wrapped child runs, `viola run` must print nothing. The terminal belongs to the claude TUI until the child exits. The channel's std-thread server, its `srv_conn` logging and the endpoint bind must not write to the passthrough terminal (per layout-templates §Surface: cli › Output structure — `viola run`; §cli › Component — Header / banner: "`viola run` prints nothing").
- `viola run` passthrough is expression level 0.0: no colour, no non-ASCII glyph and no cursor control. Any start-refusal output from the new bind path follows this rule (per layout-templates §Surface: cli › Expression level).
- The loser of two concurrent starts exits 1 with the fixed-message form `unable: <name> is already live` or a sibling cause, and `hint: <…>` must be the last line on stderr. The plan requires every exit-1 cause to have its own hint, so an agent can tell causes apart from the last stderr line alone (per layout-templates §cli › Component — Primary content block 2: refusal lines and the `unable` column; Decisions Log 2026-09-24 T4). Whether a bind-lost or squatted-endpoint start reuses the existing `already live` cause or is a new exit-1 cause that needs its own hint is a question for research and P4.
- A hint never names a path or a pid, and never quotes upstream text. The pipe or socket `endpoint` name, the `conn` id (`<process>-<pid>-<t0>-<n>`) and snapshot paths must not appear in refusal or hint lines (per layout-templates §cli › Primary content block 2; §cli › Output structure — `viola run`: "no paths, no pids").
- Streams: refusals, hints and errors go to stderr, results go to stdout, and the two are never mixed. The wrapper-fault line is `error: wrapper fault  <code>` with exit 20, and there is no hint for `wrapper fault`, `internal error` or the opaque `unknown` (per layout-templates §cli › Primary content block 2 › Exit codes / Streams). This matters only if P4 routes a newer-peer `-32602` or a `ChannelError` to a CLI surface in this chunk. The scope defers method semantics to Epochs 3–4.
- `viola hook` and `viola mcp` have no human surface (per layout-templates §cli › Component — Header / banner). Channel diagnostics belong to the diag/log plane, not to terminal output.

## Patterns to follow
- The exit-1 start-refusal pair, `unable: builder is already live` followed by `hint: viola list` on stderr as the last line, is the template for any arbiter-loser message (per layout-templates §cli › Output structure — `viola run`).
- Fixed first-word `unable` phraseology, fields separated by two spaces, and a hint keyed by reason or by reason plus detail (per layout-templates §cli › Primary content block 2). This fits the scope's fixed-`Display` `ChannelError`, so whatever `Display` text reaches a terminal stays within this vocabulary.
- Output is a contract: human words are stable because LLM drivers may read them, and new fields go into `--json` first (per layout-templates §cli › IA notes › Output as a contract).

## Anti-patterns to avoid
- No spinner, no `Sending…`, no `done` or `success` terminator, and no redraw. The exit code is the terminator (per layout-templates §cli › Component — Footer / terminator; §cli › Hero / signature output line › What never happens).
- The hand-written SGR module is the only styling, with no colour or table crate. Colour is reserved for `DIALOG` and `stale`, so a channel refusal is never red (per layout-templates §cli › Tooling context; §cli › Hero / signature output line › Colour).

## Contract bindings
- The layouts cli refusal lines tie to design-system cli pattern 2, which lists the exit-1 start-refusal sibling causes and their hints. Both tie to the tests exit-cause matrix, which requires a cause-specific hint (per layout-templates Decisions Log 2026-09-24 T4). A new arbiter-loser cause must be registered consistently in all three.
- The rule of no paths and no pids in hints ties to the security plan's squatted-name refusal (exit 1) and to obs-plan §3 D-10 `conn`. The `conn` id carries a pid, so it belongs in diag lines only, never in human output.

## Acceptance criteria contributions
- When two concurrent `viola run <name>` starts race on one name, the loser writes to stderr only, exits 1, and its last stderr line starts with `hint: `. Neither line contains a pipe or socket path, an endpoint name or a pid (per layout-templates §cli › Output structure — `viola run`; §cli › Primary content block 2).
- If the bind-lost or squatted cause is distinct from `is already live`, it has its own `unable: …` message and its own hint line, and a test asserts it can be told apart from the other exit-1 causes by the last stderr line (per layout-templates Decisions Log 2026-09-24 T4).
- While the child runs, `viola run` writes no bytes of its own to the terminal after the endpoint binds and the channel server starts. Channel activity goes only to the diag plane (per layout-templates §cli › Output structure — `viola run`).

## Relevant amendment history
(none)
