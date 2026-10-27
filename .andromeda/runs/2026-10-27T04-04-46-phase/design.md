# design extract

## Relevance
partial — the chunk is input bounding (no new surface, no tokens, no motion), but four of its boundaries end in CLI human output whose words, escaping and stream discipline design-system fixes: the paste-rule refusals, the `input-not-ready` degrade, the oversize `claude agents --json` read on the board, and upstream text escaped in `list` / `wait` / `last`.

## Constraints
- design-system §Surface: cli › Component Patterns (2. `viola send`; detail `.andromeda/design-system-detail/surface.md` cli-send) requires that a paste-rule refusal print as the `[/ ]` mirror line `unable  <name>  not-delivered  control-character` (or `empty-text`) on stderr, exit 13, followed by exactly one `hint:` line, keyed by reason · detail, that matches the listed `control-character` / `empty-text` wording byte for byte. Whether the CLI `send` client and the wrapper reply both reach that same line is research's question.
- design-system §Surface: cli › Component Patterns (2. `viola send`) requires that no hint quote the sent text or any upstream text, so a refused control character is never echoed back in the `unable` line or the hint. The refusal names the class, never the character or its position.
- design-system §Surface: cli › Component Patterns (1. `viola list`; detail `surface.md` cli-list) requires that a failed, oversize or malformed `claude agents --json` read print the board as it prints with no `claude`: no separator, no unwrapped rows, no error line. A `MAX_FRAME + 1` over-size verdict therefore has no human surface of its own.
- design-system §Surface: cli › Component Patterns (1. `viola list`, row mode `escape_row`; 3. `viola wait` / `viola last`, message mode) requires that every control character of an upstream-derived cell print as `\xHH` text: C0, DEL and C1 in rows (`\n` / `\t` included), and in message text everything except `\n` / `\t`. Nothing is stripped. An input bounded at entry still goes through these escapers at output.
- design-system §Surface: cli › Component Patterns (2. `viola send`, the `not-delivered · input-not-ready` hint) fixes the human wording for the terminal-parser panic degrade: a vt100 panic that degrades readiness surfaces to a driver's `send` only as `input-not-ready` and its listed hint. Per §Surface: cli › Component Patterns (5. `viola run`), `run` prints nothing once the child starts, so the degrade writes no byte of its own to the human's terminal.
- design-system §Surface: cli › Platform-Specific Notes requires that `hook` write nothing to stderr and that `mcp` write only MCP frames on stdout. A bounded-stdin refusal on either one (oversize, bad shape, unknown kind) therefore has no human design surface, and stack traces never print.
- design-system §Surface: cli › Tokens (Colour decision order) puts `--json`, `viola hook` and `viola mcp` ahead of any SGR. Under `--json` a refusal from a bounded input is the refusal object alone, with no hint (per §Surface: cli › Streams).

## Patterns to follow
- The two-space refusal line plus a reason-keyed `hint:` line (design-system §Surface: cli › Component Patterns, 2. `viola send`): reuse the existing `control-character` / `empty-text` hint keys. A new boundary refusal should reach an existing reason · detail pair, not a new prose line.
- The shared escapers (design-system §Surface: cli › Component Patterns, 1. row mode `escape_row` and 3. message mode): any newly bounded upstream field that reaches human output goes through one of them. Do not add a third escaper.
- Fail-open board rendering (design-system §Surface: cli › Component Patterns, 1. `viola list` As built): a bounded read that fails degrades to the board as read, with no added word.

## Anti-patterns to avoid
- Printing upstream text, paths, pids or anyhow chains with serde sources in an error or a hint (design-system §Anti-Patterns › Per-Surface Bans › cli). This includes the over-size input, the refused character and a parse error's source.
- Printing stack traces from a caught panic or an over-depth parse. Full detail goes only to `diagnostics/` (design-system §Anti-Patterns › Per-Surface Bans › cli).
- Mixing data and messages (design-system §Anti-Patterns › Per-Surface Bans › cli): a boundary refusal goes to stderr (or to the one `--json` document on stdout), never into the result stream.

## Contract bindings
- design ↔ security: the hint wording and the escapers are the human face of the security plan's paste rule (`validate_paste_text`, reject never strip) and its C0/C1 escaping floor. design-system §Surface: cli › Component Patterns (1. `viola list`) is stricter than that floor (`\n` / `\t` escaped in rows, T5 ratified).
- design ↔ architecture: exit 13 and the `not-delivered` refusal object come from the typed exit-code table (design-system §Surface: cli › Exit-code phraseology, detail `surface.md` cli-exit-codes), and architecture.md §Conventions owns the codes.
- design ↔ a11y: none for this chunk. It adds no colour, motion or token, so there is no contrast or reduce-motion binding.

## Acceptance criteria contributions
- A CLI `send` of text holding a refused control character prints, on stderr with exit 13, `[/ ]` `unable` `<name>  not-delivered  control-character`, then the hint `the text contains a control character (only LF, CR, TAB are allowed)`. Neither line holds the sent text (per design-system §Surface: cli › Component Patterns, 2. `viola send`).
- A CLI `send` whose typed text is empty prints `not-delivered  empty-text` with the hint `the text is empty once its trailing newlines are removed; send a text with content`, exit 13. Under `--json` it prints the refusal object and no hint (per design-system §Surface: cli › Component Patterns, 2. `viola send`).
- `viola list` with an oversize (`MAX_FRAME + 1`) or malformed `claude agents --json` read prints no `-- UNWRAPPED - READ-ONLY --` line and no error line, and exits as an unjoined board (per design-system §Surface: cli › Component Patterns, 1. `viola list`).
- A `hook` or `mcp` stdin over `MAX_FRAME` or over depth writes no stderr byte and no human text (per design-system §Surface: cli › Platform-Specific Notes).
