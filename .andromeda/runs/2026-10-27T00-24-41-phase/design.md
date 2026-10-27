# design extract

## Relevance
partial: the chunk adds no rendered surface. Its only contact with the design plan is the human-facing stderr of `viola run`'s start and of `viola verify` wherever the pin write's concurrency fix changes what a losing concurrent first start prints. Part B, the reproducible-build measurement, is out of this domain.

## Constraints
- Any refusal a concurrent first start makes on the pinned copy uses the existing fixed-message pair `unable: the pinned viola copy failed its integrity check` / `hint: the pinned copy was changed after it was written, so viola will not run it`. A new cause gets its own fixed `unable:` line and one hint, never a free-form message (per design-system §Surface: cli › Component Patterns, detail `.andromeda/design-system-detail/surface.md` cli-exit-codes, the exit-1 start refusals of `viola run`). Whether the fix adds any new terminal-visible cause is research's question.
- A start that waits out another start's in-flight write prints nothing while it waits: no spinner, progress bar or redrawn line (per design-system §Anti-Patterns › Per-Surface Bans, cli: "NEVER use a spinner, progress bar or live-redrawing dashboard").
- `viola run`'s terminal belongs to the child's screen, so the pin path writes no colour, glyph or cursor control there (per design-system §Anti-Patterns › Per-Surface Bans, cli: "NEVER emit colour, glyphs or cursor control under … `viola run`").
- No refusal or error line from the pin path shows a path such as `bin/<key>/`, a pid, a kernel error string such as `ETXTBSY` or an anyhow chain (per design-system §Anti-Patterns › Per-Surface Bans, cli: "NEVER print upstream text, paths, pids or anyhow chains"; and per design-system §Anti-Patterns › Per-Surface Bans, cli: "NEVER print stack traces"; full detail goes to `diagnostics/` only).
- Refusals and errors go to stderr and results to stdout. A pin fault that does not map to a named cause is `error: internal error`, a fixed message with no hint (per design-system §Surface: cli › Tokens, detail `.andromeda/design-system-detail/surface.md` cli-output; and per design-system §Surface: cli › Component Patterns, detail cli-exit-codes, exit-1 row).
- `viola verify`'s output keeps its shape when it pins first: static step-counter lines and one `stamped <ver>  <n> pass  <m> fail` summary, with no new progress pattern (per design-system §Surface: cli › Component Patterns, detail `.andromeda/design-system-detail/surface.md` cli-verify-run).

## Patterns to follow
- One fixed-message `unable:` line plus one `hint:` line per cause for exit-1 start refusals, with none of them showing a path or a pid (per design-system §Surface: cli › Component Patterns, detail cli-exit-codes).
- The obs detail code `pinned-hash-mismatch` is the logged counterpart of the integrity-check refusal. A new pin cause gets a code in that catalogue, never prose in the machine view (per design-system §Surface: cli › Component Patterns, detail cli-exit-codes, "Hints are human-mode only").
- An exit 1 from `run` has no `--json` document. The exit-cause code catalogue is owed to the route entry "Exit-cause code catalogue" (per design-system §Surface: cli › Component Patterns, detail cli-exit-codes, the exit-1 `--json` column).

## Anti-patterns to avoid
- A green "done" or "pinned" success line, or a `✓` / `✗` prefix, on a start that won or lost the race (per design-system §Anti-Patterns › Per-Surface Bans, cli: "NEVER use green `✓` / red `✗` prefixes or emoji" and "NEVER print \"done\" or \"success\" without context").
- An optimistic "waiting for pinned copy…" line that implies progress before it is confirmed (per design-system §Anti-Patterns › Rejected Defaults: the rejected optimistic "Sending…" spinner).

## Contract bindings
- design-system §Surface: cli exit-1 refusal phraseology ↔ obs-plan D-20 detail codes (`pinned-hash-mismatch`): a new terminal-visible pin cause needs both a fixed line here and a logged code there.
- design-system §Anti-Patterns › Per-Surface Bans (no paths, pids or chains) ↔ security-plan §Error Handling (fixed `Display` messages; detail only to `diagnostics/detail-*.ndjson`).

## Acceptance criteria contributions
- (design) A losing concurrent first `viola run` either starts its child and prints nothing of its own on the terminal, or exits 1 with exactly one fixed `unable:` line and its one `hint:` line on stderr. Neither line holds a path, pid, kernel error text or anyhow chain (per design-system §Surface: cli › Component Patterns, detail cli-exit-codes).
- (design) No spinner, progress or redrawn line is written while a start waits out a concurrent pin write (per design-system §Anti-Patterns › Per-Surface Bans, cli).
- (design) `viola verify` run against a fresh home still prints only its step-counter lines and its one `stamped …` summary on stdout (per design-system §Surface: cli › Component Patterns, detail cli-verify-run).
