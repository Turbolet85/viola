# design · Brand Identity — category read

## Carried
- (.andromeda/design-system-detail/brand-identity.md:20) When the server-identity check fails, a CLI verb on a human TTY must print its exit-21 refusal as static, uncoloured lines. Colour belongs only to `DIALOG` and `stale`, so a refusal gets none, and nothing redraws while the check runs.
  > | cli, human TTY | 0.2 | Static k9s-style fixed-column rows and one static context header. Colour is only a second cue: amber `DIALOG` and dim `stale`, always beside the word. Nothing redraws in place and there are no spinners. |
- (.andromeda/design-system-detail/brand-identity.md:21) Under `--json`, on a non-TTY, under `NO_COLOR` or with `TERM=dumb`, the mismatch refusal must carry no colour, glyph or cursor control. If P3 finds a client in `run`'s start or in a `revive` that passed its preflight, that client must print nothing while the wrapped TUI runs.
  > | cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight. |
- (.andromeda/design-system.md:21) A refused or unverifiable server must be reported as `unable` plus a typed reason, using the existing fixed vocabulary (`instance-unreachable`). It must never print a guessed or filler state.
  > **Standard phraseology and "unable" plus a reason.** Short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`) are the vocabulary of every label, CLI column and refusal. There is no filler and no guessing.
- (.andromeda/design-system.md:38) The new check before the first frame must not add a spinner, progress or any other motion to any CLI verb.
  > At 0.2 the duration scale's 0.0–0.2 row applies (0–100ms, no entrance), and viola's cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.

## Bindings
- **Phraseology "unable" plus a reason ↔ body Surface: cli exit-code phraseology**: a mismatch takes the existing exit-21 form, `unable  instance-unreachable` followed by its cause's hint, or the `--json` document with stderr empty (.andromeda/design-system.md:338). The phraseology rule it serves is at (.andromeda/design-system.md:21).
- **Expression 0.0 rows ↔ body Surface: cli colour decision order**: `viola hook` writes nothing to stderr and `viola mcp` writes only MCP frames on stdout, so the hook's fail-open and the MCP `instance-unreachable` result add no human output (.andromeda/design-system.md:247) (.andromeda/design-system.md:353).
- **Phraseology ↔ body Anti-Patterns cli ban / security-plan NEVER-log floor**: the mismatch refusal and its hint must never print the recorded or observed pid, the start time or a path (.andromeda/design-system.md:380).
