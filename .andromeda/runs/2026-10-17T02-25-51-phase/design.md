# design extract

## Relevance
partial — the chunk's `--json` documents, typed exits and cause-named hints are the cli surface's phraseology and stream rules (design-system §Surface: cli); tokens, colour, typography and motion are out (the colour layer is owed to "CLI output discipline", and 0.1.0 renders no page).

## Constraints
- Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit, and no `hint:` line is printed: the typed `refusal` / `error` fields are the whole answer (per design-system §Surface: cli → Streams).
- `--json` is the first match of the colour decision order: no colour, no glyph, no SGR, and the ASCII readback mirror (`[RB]` / `[  ]` / `[/ ]`) and `->` are replaced by the JSON document (per design-system §Surface: cli → Colour decision order; the 0.0 row of design-system §Brand Identity → Expression level).
- Each exit maps to one stderr first word in human mode and one `--json` shape per design-system §Surface: cli → Exit-code phraseology (0 `ok`; 10–14 refusal object; 20 `wrapper-fault`; 21 `instance-unreachable` with `detail` null pending an arch amendment, `list`'s refusal included with stderr empty); that table marks exit 1 and exit 2 as having no `--json` document "until arch amends it" — whether this chunk's added documents (revive, verify, run start refusals) amend arch first, and whether CARRY 2's "`viola revive` takes no `--json`" stands against the scope's inference, is P3/P4's reading (per design-system §Surface: cli → Exit-code phraseology).
- Hints are human-mode only, one per cause, with the exact per-cause text listed for `send` refusals, the shared exit-21 code (four causes), `run`'s six start refusals and `revive`'s refusals in a fixed order; the machine view carries the cause as a detail code, never prose (per design-system §Surface: cli → Component Patterns, pattern 2 `viola send`).
- No hint for `unknown` (exit 14), `error: wrapper fault` (exit 20) or `error: internal error` (exit 1); no hint ever names `viola release` (T3) or quotes sent or upstream text (per design-system §Surface: cli → Component Patterns, pattern 2 `viola send`).
- Streams: results on stdout; `waiting:`, refusals (`unable  <name>  <reason>  <detail>`, two-space separated), their `hint:` lines and `error: …` on stderr; only the exit-1 refusals of `run` / `revive` / `verify` and `list`'s exit-21 strict-modes refusal use the fixed `unable: <text>` form (per design-system §Surface: cli → Streams).
- `viola revive --list` prints one static plain-ASCII stdout line per logged session, `<time>  <cause>  <session id>`, no header, and the verb takes no `--json` as written (per design-system §Surface: cli → Component Patterns, pattern 5 `viola verify` / `viola run`).

## Patterns to follow
- The exit-code phraseology table as the single map of exit → human first word → `--json` shape, which one witness can read every verb against (per design-system §Surface: cli → Exit-code phraseology).
- The fixed-message `unable:` / `hint:` pair for exit-1 start refusals, shared verbatim between `run` and `revive` (`already live` and its stale sibling) and between `revive` and `list` (the instance-state pair) (per design-system §Surface: cli → Component Patterns, pattern 2 `viola send`, and pattern 1 Refusal).
- obs-plan D-20's log detail codes (`instance-dead`, `strict-modes-failed`, `server-verify-failed`, `already-live`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, `no-session`, `cwd-missing`) as the named machine causes awaiting the arch amendment; the stale-heartbeat and unwrapped-name causes have no code yet (per design-system §Surface: cli → Component Patterns, pattern 2 `viola send`).
- `verify`'s human output is static step-counter lines plus one summary line `stamped <ver>  <n> pass  <m> fail`; a stopped run prints the fixed pair and exits 1 with stdout empty (per design-system §Surface: cli → Component Patterns, pattern 5 `viola verify` / `viola run`).

## Anti-patterns to avoid
- Emitting colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run` passthrough (per design-system §Anti-Patterns → Per-Surface Bans cli).
- Mixing data and messages, or printing upstream text, paths, pids, anyhow chains with serde sources, or stack traces in an error, hint or `--json` detail (per design-system §Anti-Patterns → Per-Surface Bans cli).
- Printing "done" / "success" without context, or a green `✓` / red `✗` / emoji prefix (per design-system §Anti-Patterns → Per-Surface Bans cli).

## Contract bindings
- design ↔ architecture: the `--json` shapes and the exit set are architecture §Standard Contracts "CLI `--json` output" and §Conventions "CLI exit codes"; design-system §Surface: cli → Exit-code phraseology defers exit-1 documents and exit-21 `detail` codes to an arch amendment.
- design ↔ obs: the machine cause codes are obs-plan D-20's log detail codes (design-system §Surface: cli → Component Patterns, pattern 2 `viola send`).
- design ↔ security: no path, pid, upstream text or serde chain in a hint or error is security-plan §Error Handling's NEVER-log floor (design-system §Anti-Patterns → Per-Surface Bans cli).

## Acceptance criteria contributions
- Every verb's `--json` run writes exactly one JSON document to stdout, no `hint:` line and no SGR / non-ASCII byte on either stream, and its exit agrees with the document per the exit-code phraseology table (per design-system §Surface: cli → Streams and → Exit-code phraseology).
- Each refusal cause a human sees prints exactly one `unable` line and directly after it the one `hint:` line whose text matches the per-cause wording, with no hint for exits 14, 20 and the exit-1 internal error, and none naming `viola release` (per design-system §Surface: cli → Component Patterns, pattern 2 `viola send`).
- No hint, error line or `--json` `detail` contains an absolute path, a pid, a logged session id, the recorded directory or sent/upstream text (per design-system §Anti-Patterns → Per-Surface Bans cli).
