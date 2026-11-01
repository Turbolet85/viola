# design extract

## Relevance
partial: no rendered surface and no tokens beyond the CLI's, but every refusal this chunk adds or moves prints through the CLI phraseology the plan owns.

## Constraints
- A refused socket directory and a peer euid mismatch each require a typed exit, a closed code and exactly one fixed hint per cause, in the existing human and `--json` forms, per design-system §Surface: cli › Exit-code phraseology. The plan names which of the existing exit-21 and `run` start-refusal forms exist; which one each new refusal takes is research's and P4's question.
- The plan closes exit 21's `--json` `detail` at five cause words, and `.andromeda/design-system-detail/surface.md` §cli-send names the socket-directory cause and macOS's server check as without codes, owned by this route entry, per design-system §Surface: cli › Exit-code phraseology. A new cause word therefore requires amending that closed list and its detail row in the same chunk; whether a peer euid mismatch takes a new word or the existing `server-verify-failed` is P4's question.
- The board's two-rule home check is a named pending item owned by this entry: which of the home-state and instance-state pairs `viola list` prints must stop depending on whether an instance exists, per design-system §Surface: cli › Component Patterns (detail `cli-list`, the pending paragraph). The plan's text must be restated when the rule becomes one.
- Refusal text requires fixed messages that hold no path, pid, start time, endpoint or upstream text, per design-system §Anti-Patterns › Per-Surface Bans (cli). This applies to the socket directory's location and to both euids.
- Refusals and their `hint:` lines go to stderr, plain ASCII with no SGR, and under `--json` the whole answer is one stdout document with no hint and stderr empty, per design-system §Surface: cli › Streams.
- `run` prints nothing once the child starts, and `hook` and `mcp` hold their silent streams at the catch site, per design-system §Surface: cli › Colour decision order and §Surface: cli › Platform-Specific Notes. A refusal reaching `hook` is a silent fail-open with no human line.
- macOS's server check needs a printed form that does not claim a comparison it never made: the server-verification hint speaks of a process that is not the recorded wrapper, per design-system §Surface: cli › Exit-code phraseology (detail `cli-exit-codes`, the server-verification item). Whether any macOS output reads that way today is research's question.

## Patterns to follow
- The fixed-message refusal pairs (home-state and instance-state `unable:` / `hint:`), reused rather than reworded where the cause is the same, per design-system §Surface: cli › Component Patterns (detail `cli-list`, Refusal).
- One hint per cause on a shared code, keyed by the cause, per design-system §Surface: cli › Component Patterns (detail `cli-send`, the `instance-unreachable` list).
- The `run` start-refusal list's form, one fixed line and one hint per cause, where a socket-directory refusal lands at a wrapper's start, per design-system §Surface: cli › Component Patterns (detail `cli-send`, the exit-1 start refusals).
- `revive --json`'s closed start-refusal codes, which share `run`'s start: a new start cause touches that list too, per design-system §Surface: cli › Component Patterns (5. `viola verify` / `viola run`).
- Standard phraseology for any new word or hint: short, fixed, lower-case, no filler and no guessing, per design-system §Brand Identity (Standard phraseology and "unable" plus a reason).

## Anti-patterns to avoid
- Printing a path, pid, anyhow chain or stack trace in a refusal or hint; full detail goes to `diagnostics/` only, per design-system §Anti-Patterns › Per-Surface Bans (cli).
- Colouring a refusal: amber is `DIALOG` only and dim is `stale` rows only, so a security refusal stays uncoloured, per design-system §Anti-Patterns › Universal Bans.
- A hint naming `viola release`, which a driver cannot use, per design-system §Surface: cli › Component Patterns (2. `viola send`).

## Contract bindings
- design ↔ tests: each new refusal's human pair and `--json` document binds to its row in `tests/cross_exit_causes.rs` (scope's freight) and to the no-path, no-upstream, stderr-empty assertion of `tests/cli_error_bodies.rs`, per design-system §Surface: cli › Exit-code phraseology (detail `cli-exit-codes`).
- design ↔ obs: the plan's list of log detail codes refers to obs-plan D-20, and a new cause's code must agree in both plans, per design-system §Surface: cli › Component Patterns (detail `cli-send`).
- design ↔ security: the no-path, no-pid refusal text is the security plan's NEVER-log floor, per design-system §Anti-Patterns › Per-Surface Bans (cli).
- design ↔ architecture: the typed exit set and `--json` document shapes come from architecture.md, per design-system §Surface: cli › Exit-code phraseology.

## Acceptance criteria contributions
- Each new refusal (socket directory at a wrapper's start and at a client's connect, peer euid mismatch on either side) prints one fixed `unable` line and one `hint:` line on stderr, plain ASCII with no SGR, naming no path, pid or euid value (per design-system §Anti-Patterns › Per-Surface Bans (cli)).
- Under `--json` each such refusal prints one stdout document with its closed code, no hint and empty stderr, and the plan's closed code lists name that code (per design-system §Surface: cli › Exit-code phraseology).
- A `viola list` on a group-readable home prints the same home-state refusal whether or not an instance exists, and the plan's pending paragraph is replaced by the one rule (per design-system §Surface: cli › Component Patterns).
- `viola run` still prints nothing once the child starts and `viola hook` still exits 0 with empty stderr when the new checks refuse (per design-system §Surface: cli › Platform-Specific Notes).
