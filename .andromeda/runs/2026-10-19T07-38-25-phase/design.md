# design extract

## Relevance
partial — the chunk's only design surface is the human CLI line a driver or person reads when a held dialog expires (the `[inferred]` new event kind's `viola wait` result line and a late `answer`'s refusal); the hold length, the hook `timeout` and the reply at expiry carry no design content, and the web surface is deferred to 0.2.0.

## Constraints
- A `viola wait` result for the new expiry kind takes one of the two forms design-system §Surface: cli §Component Patterns (3. `viola wait` / `viola last`) fixes: the dialog form `<kind>  <name>  dialog <id>  cursor <n>` (with `dialog unknown` when the event carries no `dialog_id`), or the non-dialog form `<kind>  <name>  <time>  cursor <n>`. That section names `question` as a dialog kind and "every other non-dialog kind" as the time form, so which form an expiry line takes, since it carries a `dialog_id` but is no pending dialog, is the plan's choice to state, and it must be one of the two (per design-system §Surface: cli §Component Patterns).
- The kind word itself is short, fixed, lower-case phraseology, matching the existing vocabulary (`turn-ended`, `session-end`, `not-delivered`): no sentence, no filler, no guessing (per design-system §Brand Identity, "Standard phraseology and 'unable' plus a reason").
- The `wait` result line is not coloured, `DIALOG` included ("this is a result, not the board"). Colour is allowed only on the `viola list` `DIALOG` word and on `stale` rows (per design-system §Anti-Patterns Per-Surface Bans, cli).
- A longer hold adds no progress display: `wait` prints one static `waiting: <name>` line on stderr (TTY only) and then one result line, with no spinner, no elapsed-time counter and no countdown to the hold's end (per design-system §Surface: cli §Component Patterns, item 3). The same ban: no spinner, progress bar or live redraw (per design-system §Anti-Patterns Per-Surface Bans, cli).
- The result line goes to stdout and the `waiting:` context line and any refusal or `hint:` go to stderr. Under `--json` the whole result is one JSON document on stdout with no hint text (per design-system §Surface: cli §Streams).
- Human output is ASCII only, with two spaces between fields and ` - ` inside a field in place of `·`. It has no glyphs and is never wrapped (per design-system §Iconography, Separator; per design-system §Surface: cli §Width).
- No upstream text (question text, plan text, tool `input`), path or pid reaches the expiry line or a refusal or hint about it. Only the kind word, the name, the dialog id or time, and the cursor are printed (per design-system §Anti-Patterns Per-Surface Bans, cli).

## Patterns to follow
- The `wait` line anatomy in design-system §Surface: cli §Component Patterns (3): fields separated by two spaces, `cursor <n>` last, and `dialog unknown` as the printed fallback for a missing id. An expiry line reuses this anatomy and invents no new layout.
- The late-`answer` refusal in design-system §Surface: cli §Component Patterns (2. `viola send`, the hint table, and 4. `viola answer`): `unable  builder  not-delivered  unknown-dialog`, followed by the hint `that dialog is not pending; viola list shows the current DIALOG`. If the chunk keeps `unknown-dialog` for an answer that arrives after expiry, the line and hint stay as they are. A new detail word would need its own hint row keyed by `reason · detail`, a matching exit-13 row in the Exit-code phraseology table, and no hint naming `viola release`.
- The board's DIALOG column in design-system §Surface: cli §Component Patterns (1. `viola list`): a wrapped row with no pending dialog prints `none`. After expiry, the strip returns to `none` in its own slot and is not re-sorted (per design-system §Anti-Patterns Rejected Defaults, "Auto-sorting 'needs input'…").

## Anti-patterns to avoid
- No toast-like or "expired!" success/failure decoration: no green `✓` or red `✗` prefixes, no emoji, and no bare "done" without naming the name, the dialog and the cursor (per design-system §Anti-Patterns Per-Surface Bans, cli).
- No colour on the expiry line and no amber outside the board's `DIALOG` word: amber means `dialog_pending` and nothing else, and an expired hold is history, not attention (per design-system §Anti-Patterns Universal Bans; per design-system §Color Palette Domain status colors, EventKind::question row, "No amber in the tape: history is not attention").

## Contract bindings
- design ↔ architecture §Standard Contracts (event line) and obs-plan's event catalog: the new kind word printed by `wait` is the event kind those contracts name, so the three spellings must be identical.
- design ↔ the 0.2.0 frontend input: design-system §Color Palette Domain status colors requires one row per `EventKind` variant that gets a visual. The new kind has no row there. Whether this chunk adds the row (a 0.2.0 design input only, with no amber per the EventKind::question row) or records that it is owed is the plan's to state.
- design ↔ security-plan §Error Handling / NEVER-log floor: no upstream text in human lines (see the Constraints).

## Acceptance criteria contributions
- (design) A `viola wait` that wakes on an expired hold prints exactly one stdout line in a design-system component-3 form (two-space-separated fields, kind word first, the name, `dialog <id>` or `dialog unknown`, or `<time>`, then `cursor <n>`). It is ASCII only with no SGR bytes, on a TTY too, and has no question, plan or tool text in it (per design-system §Surface: cli §Component Patterns).
- (design) During a held dialog of any length, `viola wait` writes nothing except the one static `waiting: <name>` stderr line (TTY only) before its result: no spinner, counter or redraw bytes (per design-system §Anti-Patterns Per-Surface Bans, cli).
- (design) An `answer` sent after the hold expired prints `unable  <name>  not-delivered  <detail>` on stderr with exit 13, then one `hint:` line keyed by that detail, and no `hint:` line under `--json` (per design-system §Surface: cli §Component Patterns, item 2 and the Exit-code phraseology table).
