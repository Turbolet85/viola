# design extract

## Relevance
partial — the chunk adds no surface and no token; it binds only where the switched overseer reads the product's cli output (`send` · `wait` · `last` · `answer` · `list`, `pause` / `release`) in place of the prototype's, and where a defect the switch exposes is fixed in that output.

## Constraints
- Driver-facing output read by the overseer (a non-TTY pipe or `--json`) carries no colour, no glyph, no cursor control and no spinner; `viola run` and a passed `viola revive` print nothing while the wrapped `claude` runs (per design-system §Brand Identity, the Expression level table's cli 0.0 row; per design-system §Surface: cli, Colour decision order).
- `wait` prints its result as one stdout line in the fixed forms `<kind>  <name>  <time>  cursor <n>` and, for a dialog kind, `<kind>  <name>  dialog <id>  cursor <n>`; `timed out  <name>  <ms> ms` on a timeout; `waiting:` only on stderr and only on a TTY. The overseer's adaptation of its prototype first-line words (`turn-end`, `question`, `plan`, `permission`, `exited`, `held`) is to these forms, an overseer step outside this repository (per design-system §Surface: cli, Component Patterns 3).
- `dialog-expired  <name>  dialog <id>  cursor <n>` is a planning form whose kind word and line are the founder's to word; a live long hold the switch reads must not be taken as settling that wording (per design-system §Surface: cli, Component Patterns 3).
- The readback mirror is the send result: `[RB] read back` (stdout, exit 0), `[/ ] unable  <reason>  <detail>` plus one `hint:` (stderr), `[  ] unconfirmable` for a local command without a measured post-condition; a `/clear` counts as read back only on its new-session post-condition (per design-system §Brand Identity, Signature element; per design-system §Surface: cli, Component Patterns 2).
- The wheel phraseology is fixed: `viola pause` → `<name>  wheel human  manual-pause  I have control`, `viola release` → `<name>  wheel driver  you have control`; the WHEEL field prints `human` / `driver` (per design-system §Surface: cli, Component Patterns 4; per design-system §Brand Identity, Domain anchors "Pilot-in-command").
- No hint, driver-facing doc or MCP text names `viola release`; `release` stays the human's verb, so the takeover's release in the live proof is the human's, never the overseer's (per design-system §Surface: cli, Component Patterns 2, T3 note).
- `last` prints the message text on stdout through the message-mode escaper (C0/DEL/C1 as `\xHH`, `\n` and `\t` kept), `no message` on stderr for `null`; `list` rows escape `\n` and `\t` too (per design-system §Surface: cli, Component Patterns 3; per design-system §Surface: cli, Component Patterns 1).

## Patterns to follow
- `viola list` board: fixed columns NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, rows by name, `DIALOG <kind>` in the DIALOG column, `unverified-cli` / `verified` in CLI; the overseer reads the builder's state from these columns (and from `--json`) in place of the prototype's column 3 (per design-system §Surface: cli, Component Patterns 1).
- Streams: results and data on stdout; `waiting:`, refusals with their `hint:`, and errors on stderr; under `--json` one document on stdout with the typed exit and no hint (per design-system §Surface: cli, Streams).
- Exit-code phraseology: 10 `human-typing`, 11 `budget-paused`, 12 `unverified-cli`, 13 `not-delivered · <detail>`, 21 `instance-unreachable` — the typed codes the overseer branches on (per design-system §Surface: cli, Exit-code phraseology).
- Start and revive refusals in the fixed `unable: …` / `hint: …` form with no path or pid, e.g. the home strict-modes pair for a home not private to the user (per design-system §Surface: cli, Component Patterns 2 and 5).

## Anti-patterns to avoid
- Any colour, glyph or cursor control in output an LLM driver parses (`--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`) (per design-system §Anti-Patterns, Per-Surface Bans cli).
- Printing upstream text, paths, pids or anyhow chains in an error or hint, or "done" / "success" without saying what was read back, by whom and at which cursor (per design-system §Anti-Patterns, Per-Surface Bans cli).
- An optimistic send presumed before readback (per design-system §Anti-Patterns, Rejected Defaults).

## Contract bindings
- design §Surface: cli Component Patterns 1–4 ↔ architecture §Standard Contracts (the `list` payload, `--json` documents, refusal codes) — the overseer's re-pointed parsing reads one or the other; which surface (human line or `--json`) its scripts read is the overseer's step, and whether the product's printed forms match these patterns as built is research's question.
- design §Surface: cli Component Patterns 3 (`dialog-expired` planning form) ↔ the PROVISIONAL `dialog_hold_s` ceiling carried from 2026-10-19-dialog-hold-for-the-driver and its owed founder wording.
- design §Surface: cli Component Patterns 4 (wheel lines) ↔ the takeover proof (a key in the session terminal → wheel `human`, then the human's `viola release`) and obs-plan's wheel line.

## Acceptance criteria contributions
- (design) Every product output the switched overseer captures in the live record (`send`, `wait`, `last`, `answer`, `list`, `release`) carries no SGR byte, glyph or cursor-control sequence when read through a pipe or `--json` (per design-system §Anti-Patterns, Per-Surface Bans cli).
- (design) The live record's `wait` result lines match `<kind>  <name>  <time>  cursor <n>` / `<kind>  <name>  dialog <id>  cursor <n>`, and the `/clear` send reads `[RB] read back` only once its new-session post-condition lands (per design-system §Surface: cli, Component Patterns 3; per design-system §Surface: cli, Component Patterns 2).
- (design) The takeover is recorded with WHEEL `human` and the human's release printing `<name>  wheel driver  you have control`; no driver-facing text the chunk adds names `viola release` (per design-system §Surface: cli, Component Patterns 4).
- (design) A product defect fixed in this chunk that touches a refusal or error prints no path, pid or upstream text and keeps the fixed `unable` / `hint:` forms (per design-system §Surface: cli, Component Patterns 2).
