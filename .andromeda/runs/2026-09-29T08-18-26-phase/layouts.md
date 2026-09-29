# layouts extract

## Relevance
partial — the chunk is Windows transport with no web-spa change. Its only layout touchpoint is the cli surface's `viola run` output: it must print nothing while running, and it has a fixed-message refusal form. It also must not put the new backend fact into any human output.

## Constraints
- `viola run <name> -- claude` is passthrough and prints nothing while the child runs. The terminal belongs to the wrapped TUI until it exits. This holds for both backends and for the fallback path: an absent, hash-failed or signature-failed sideload degrades with no human line (per layout-templates §Surface: cli / Primary screens (commands); §Output structure — `viola run`).
- `viola run` passthrough has expression level 0.0: no colour, no non-ASCII glyph, no cursor control. A sideload notice, warning or progress line would break this (per layout-templates §Surface: cli, Expression level).
- No banner anywhere. There is no version line on normal output and `viola run` prints nothing. This rules out a "using OpenConsole x.y.z" or "ConPTY: sideloaded/inbox" line on start (per layout-templates §Component — Header / banner, "No banner anywhere").
- A new exit-1 start refusal, if P4 makes any sideload or pin failure refusing rather than degrading, must take the fixed-message form `unable: <text>` followed by its own cause-specific `hint:` line on stderr. Neither line may carry a name, path, pid, hash or version string. The hint is the last stderr line. The existing pinned-copy refusal is the sibling it sits beside (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column, Line form and Hint line).
- Output is a contract. New fields go into `--json` first, and a new human column would break the six-column parity with the web strip, so it is a design change. The backend in use (inbox or sideloaded) must not become a `viola list` column or a web strip field in this chunk (per layout-templates §IA notes, "Output as a contract"; §Component — Primary content block 1, Columns).
- Streams stay separated. stdout is results only. Refusals, hints and errors go to stderr and are never mixed with stdout (per layout-templates §Component — Primary content block 2, Streams).

## Patterns to follow
- The `viola run` refusal pair shape is `unable: builder is already live` / `hint: viola list`: an exit-1 line with no paths or pids, and the hint as the last line (per layout-templates §Output structure — `viola run`).
- Every exit-1 cause has its own hint, so an agent can tell causes apart from the last stderr line. Any new sideload-related refusal cause, if one exists, gets a distinct hint (per layout-templates §Component — Primary content block 2, Hint line; Decisions Log `2026-09-24` fix pass T4).
- The backend fact reaches a human only through the recorded channels, never through printed output. A driver-visible surface, if any, is `--json` first (per layout-templates §IA notes, "Output as a contract").

## Anti-patterns to avoid
- Printing anything from `viola run` about the ConPTY backend, the sideload check or the fallback: a notice, warning, spinner, progress line, or "degraded to inbox ConPTY" text (per layout-templates §Component — Header / banner; §Decisions Log `2026-09-24`, "cli: no motion").
- Adding a `viola verify` step line for the sideload check. `[NN/MM]` rows are the ledger rows only, and `MM` grows as owning chunks land ledger rows. The sideload is not a claude-CLI capability row. Whether P4 routes any check through verify is a P4 question, but it must not appear as a verify step without a ledger row behind it (per layout-templates §Output structure — `viola verify`).
- Naming a path, hash or version in a refusal or hint, e.g. `<home>/bin/<version>-<hash>/conpty.dll` (per layout-templates §Component — Primary content block 2, Hint line).

## Contract bindings
- layouts ↔ obs: the backend in use is recorded, not printed. The `pty.spawn` backend field is obs-plan's shape, and the layout requirement is only that it has no human-output twin (per layout-templates §IA notes, "Output as a contract").
- layouts ↔ security: the ban on paths, hashes and pids in refusal and hint text is the cli face of security-plan §Error Handling fixed messages. A sideload refusal, if one exists, satisfies both at once.
- layouts ↔ design-system: any new `viola run` start-refusal cause joins design-system cli pattern 2's list of exit-1 causes (per layout-templates §Component — Primary content block 2, which defers the cause list there).

## Acceptance criteria contributions
- (layouts) On Windows, `viola run <name> -- <fake agent>` writes zero wrapper-originated bytes to stdout and stderr before the child's screen. This is checked in each of three cases: sideload present and valid, sideload absent, and sideload hash-mismatched. The fallback is silent on the human surface (per layout-templates §Output structure — `viola run`; §Component — Header / banner).
- (layouts) `viola list` human output keeps exactly the six captions `NAME LIVE STATUS WHEEL DIALOG CLI` after this chunk, with no backend column (per layout-templates §Component — Primary content block 1: `viola list` strip rows).
- (layouts) If the chunk adds any `viola run` exit-1 cause, its stderr is exactly one `unable: <fixed text>` line followed by one cause-specific `hint:` line as the last line. Neither line matches a path separator, a hex hash or a pid (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
