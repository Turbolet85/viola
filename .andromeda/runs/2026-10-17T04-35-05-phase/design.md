# design extract

## Relevance
relevant — the chunk builds the cli surface's colour layer, linear static output and grouped `--help`, all of which design-system §Surface: cli governs (the web-spa surface and every §Color Palette hex table are 0.2.0 inputs and out of scope).

## Constraints
- design-system §Tokens (platform-specific) requires exactly four SGR tokens (attention, stale, callsign, reset) at three depths (truecolor / 256-colour / 16-colour fallback), the reset after every styled span, no handoff-blue SGR at any depth, and no background ever set; on a stale row that is also cocked, only the `DIALOG` word is amber and undimmed while the kind word and the rest of the row stay dim.
- design-system §Tokens (platform-specific) requires the seven-rung colour decision order (`--json` · `viola run` · `viola hook` / `viola mcp` · `NO_COLOR` set · `TERM=dumb` · stdout not a terminal · Windows VT enable failed; first match wins, yielding no colour, no glyph, no SGR) and a depth chosen once per process (truecolor on `COLORTERM` `truecolor`/`24bit`; 256 on `TERM` containing `256color` or after a successful Windows VT enable; else 16). The ASCII readback mirror and `->` are words, not glyphs, and still print in every human output.
- design-system §Surface: cli (Toolkit / Framework) requires clap built without its `color` feature (help and usage plain in every mode, no dependency reading a colour or terminal variable), TTY detection through std `IsTerminal`, Windows VT through windows-sys `SetConsoleMode(ENABLE_VIRTUAL_TERMINAL_PROCESSING)` with no colour on failure, and styling as a small hand-written SGR module in the `viola` bin adding no dependency; whether the code already holds clap's `color` off is research's question.
- design-system §Brand Identity (Expression level, per-surface table) sets cli human TTY at 0.2 (static fixed-column rows, colour only as a second cue — amber `DIALOG`, dim `stale`, beside the word — nothing redrawn in place, no spinner) and cli `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough at 0.0 (no colour, no glyphs, no cursor control; `viola run` and a passed `viola revive` print nothing while the child runs).
- design-system §Navigation Pattern requires `viola --help` to group verbs as `board` / `traffic` / `wheel` / `handoff` / `setup` with `revive` beside `run`; its `handoff: link, unlink` group names verbs CLAUDE.md records as not served, so which verbs each group lists in 0.1.0 is research's and P4's question, not a design given.
- design-system §Surface: cli (Width) requires truncation only of unwrapped-row NAMEs, with `...`, and only where the width is detectable (windows-sys `GetConsoleScreenBufferInfo`; `COLUMNS` elsewhere); otherwise nothing truncates and lines never wrap; human output is ASCII only.
- design-system §Surface: cli (Streams) requires results on stdout and context lines (`waiting: <name>`, a send's issue line), refusals, their `hint:` lines and errors on stderr; no `hint:` under `--json`.

## Patterns to follow
- design-system §Component Patterns, pattern 1 (`viola list`): the k9s strip table sample with NAME bold, `DIALOG` amber, the stale row dim; its As-built note records the board uncoloured with the colour layer owed to this entry, and the row-mode escaper `escape_row` (every control, DEL and C1, as `\xHH`) as the existing escaping pattern.
- design-system §Component Patterns, pattern 3 (`viola wait` / `viola last`): one static `waiting: <name>` stderr line (TTY only), then one stdout result line; `last`'s context line on stderr (TTY only); the message-mode escaper both verbs share — every control except `\n` / `\t` as `\xHH`, two uppercase hex digits, nothing stripped; `DIALOG` not coloured in a `wait` result.
- design-system §Component Patterns, pattern 5 (`viola verify`): the project's one progress pattern, static appended stdout lines `[NN/MM] <row> <description>  pass|fail`, `MM` the relied count, then the summary line; the word `fail` uncoloured; `run` / `revive` print nothing once the child starts.
- design-system §Component Patterns, pattern 2 (`viola send`): the readback mirror `[  ]` / `[RB]` / `[/ ]` as ASCII words and `unable` uncoloured — the colour layer adds no SGR to send output.

## Anti-patterns to avoid
- design-system §Per-Surface Bans (cli): never colour anything except `DIALOG` (amber) and `stale` rows (dim), never either without its word; never emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run` (a passed `viola revive` included).
- design-system §Per-Surface Bans (cli): never a spinner, progress bar or live-redrawing dashboard; never green `✓` / red `✗` prefixes or emoji; never mix data and messages across stdout / stderr.
- design-system §Universal Bans: never colour purely for decoration — any hue beyond amber (dialog) has no CLI meaning (blue is not used on the CLI per §Tokens (platform-specific)).

## Contract bindings
- design ↔ a11y: the colour-beside-its-word rule (§Per-Surface Bans, cli) binds to a11y-plan P6's span clause and its not-colour-alone requirement; the zero-SGR / zero-cursor-control rule under `viola run` (§Brand Identity Expression table, 0.0 row) binds to a11y-plan P4 case (1)'s zero-wrapper-bytes outer-PTY check; the `--help` group labels (§Navigation Pattern) bind to a11y-plan §ARIA equivalents.
- design ↔ architecture / security: the decision order's `NO_COLOR` / `TERM` / `COLORTERM` reads (§Tokens (platform-specific)) and the Width rule's `COLUMNS` read (§Surface: cli Width) are env reads that architecture Config management's exhaustive env sentence and security-plan Secret Management must admit — the boundary widening CARRY 2 names; `COLUMNS` is not in CARRY 2's list, so whether it joins the same fork is P4's question.
- design ↔ layouts: the grouped `--help` table and the styled board's NAME truncation are layout-templates rows; design supplies only the tokens and the group names.
- design ↔ obs: the stderr streams rule (§Surface: cli Streams) binds to obs-plan §3 / §11's human-stderr-writer sentences (CARRY 1).

## Acceptance criteria contributions
- On a TTY with colour allowed, `viola list` emits SGR only as bold on NAME values, the attention token on the `DIALOG` word (not its kind word), and dim on a whole stale row (the `DIALOG` word excepted on a cocked stale row), each span closed by `\x1b[0m`; no background and no other SGR byte appears (per design-system §Tokens (platform-specific)).
- Under each of `--json`, non-TTY stdout, `NO_COLOR` set, `TERM=dumb` and a failed Windows VT enable, every verb's output holds zero ESC bytes, and the depth picked for a coloured run matches the `COLORTERM` / `TERM` / Windows-VT rule (per design-system §Tokens (platform-specific)).
- No verb's stdout or stderr holds a `\r` redraw, cursor-movement or erase sequence, spinner or non-ASCII glyph; `wait`'s `waiting:` line prints once on stderr only on a TTY (per design-system §Per-Surface Bans).
- `viola --help` prints the grouped verb table under the §Navigation Pattern group labels, SGR-free (per design-system §Navigation Pattern).
