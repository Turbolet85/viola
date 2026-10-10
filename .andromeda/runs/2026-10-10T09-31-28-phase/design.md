# design extract

## Relevance
partial — the chunk renders nothing on the web page and draws no token; design-system binds it only through the
CLI surface's output rules (any human or `--json` line the chunk adds or changes) and through the `skipped`
vocabulary that later surfaces print from the counts this chunk's reader yields.

## Constraints
- design-system §Color Palette (Domain status colors, the `Skipped::zero` and `Skipped::nonzero` rows) requires the
  skip count to be three separate figures in a fixed order: unknown kinds, unknown fields, torn lines; each prints
  at zero too. Whether the reader's count type at HEAD can supply these three (scope item 1 names two other
  counts, over-long and non-object lines) is research's question; design-system names no figure for those two.
- design-system §Color Palette (Domain status colors, the `Skipped::nonzero` row) requires a nonzero count to stay
  visible and never clear by itself: a heal or a replay in this chunk must not reset or drop a count that a
  surface would still owe the reader.
- design-system §Brand Identity (domain anchor "Handoff / transfer of control, and the tower voice recorder") and
  §Color Palette (Domain status colors, the `EventKind::(unknown to this page)` row) require a skipped line to be
  shown at the point in the tape where it occurred: the reader's result must not foreclose a position per skip
  (the web tape itself is Epoch 8's, not built here).
- design-system §Brand Identity (domain anchors "Flight-progress strip" and "Standard phraseology and 'unable'
  plus a reason") requires a field with no reading to print `unknown`, with no filler and no guessing: a field the
  replay does not recover from the log must stay expressible as absent, never filled with a plausible default.
- design-system §Surface: cli → Tokens (the colour decision order and Streams) requires `--json` output to be one
  JSON document on stdout with the typed exit code, no colour, no glyph and no `hint:` line; human context lines,
  refusals, hints and errors go to stderr, results to stdout. Any count, refusal or notice this chunk surfaces on
  the CLI follows that split.
- design-system §Surface: cli → Component Patterns (Exit-code phraseology) fixes the human first word and the
  `--json` shape per exit code and holds no row for a version refusal: which existing row the newer-peer refusal
  prints through is architecture's to name, and whether a refusal already exists in the code is research's
  question. A new human word or row is a design-system amendment, not an invention at implement.
- design-system §Surface: cli → Component Patterns (5, `verify`) requires `viola verify`'s stdout to be static
  appended step lines, one per ledger row, ending in the summary as the last stdout line; it names no line for a
  leftover probe dir or its removal. Scope item 6 may not add a stdout line after the summary or change a step
  line's form.

## Patterns to follow
- The `skipped` figure and its separators: middle dot on the web, ` - ` inside one field on the CLI, as in the
  `viola list` sample's BAY line (design-system §Iconography, Separator; §Surface: cli → Component Patterns 1).
  The BAY line belongs to the later entry "The board: viola list"; this chunk supplies the counts, not the line.
- Refusal and fault phraseology: `unable` plus typed reason and detail for a refusal, `error:` plus a fixed
  message for a fault, and no hint line for a fault or for an opaque `unknown` detail (design-system §Surface:
  cli → Component Patterns 2).
- `wait` and `last` result lines have fixed forms, one static context line on stderr and one result on stdout
  (design-system §Surface: cli → Component Patterns 3). The plan gives neither verb a skip or recovery word; if
  research finds a surface is owed there, it comes to the review card as a named addition.
- Colour is never the carrier of a recovery or skip state: on the CLI only `DIALOG` and `stale` rows are styled,
  and on the web a nonzero skip is a buff rule and a word, never amber or a red (design-system §Surface: cli →
  Tokens; §Color Palette, Semantic Colors, the Info row).

## Anti-patterns to avoid
- No upstream text, path, pid or error chain in any error, hint or notice: a healed file, a replayed snapshot, a
  refused peer and a removed probe dir are named without a path or a pid on human output (design-system
  §Anti-Patterns → Per-Surface Bans, cli).
- No bare "done", "recovered" or "success" line without its context, and no spinner or progress redraw while a
  log replays (design-system §Anti-Patterns → Per-Surface Bans, cli).
- No colour, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`, and nothing
  printed by `viola run` while the child runs: a heal or replay at wrapper start is silent on the terminal
  (design-system §Surface: cli → Tokens; §Anti-Patterns → Per-Surface Bans, cli).

## Contract bindings
- design ↔ architecture: the three count names design-system prints (§Color Palette, Domain status colors,
  `Skipped`) must match the field names of the contract that carries them (the reader's result, later `/api` and
  SSE). Where over-long and non-object lines are counted among the three, or beside them, is architecture's
  contract to say; design-system is silent.
- design ↔ obs: `state-recovered` and its details (`torn-line-healed`, `snapshot-replayed`,
  `snapshot-unsupported-v`) have no row in design-system's domain status table: they are obs records with no
  visual. Showing one on a human surface would need a new row first.
- design ↔ security: the no-path, no-pid, no-upstream-text rule of §Anti-Patterns → Per-Surface Bans (cli) is the
  human-output face of the security plan's error sanitisation; the remover of scope item 6 is bound by both.
- design ↔ architecture: `Problem::state-unreadable` (§Color Palette, Domain status colors; §Surface: web-spa →
  Component Patterns 6) is the page's word for an unreadable home. Whether a snapshot healed by replay still
  yields that problem is the State Store contract's to say; the strip is Epoch 8's.
- design ↔ a11y (later surface, flagged only): the nonzero `skipped` box is state carried by rule and word, which
  is the not-colour-alone binding; nothing in this chunk renders it.

## Acceptance criteria contributions
- Every human line this chunk adds or changes is ASCII, carries no SGR outside the two allowed uses, and names no
  path, pid or upstream text (per design-system §Anti-Patterns → Per-Surface Bans, cli)
- Under `--json`, anything this chunk surfaces (a count, a refusal) sits inside the one stdout document with the
  typed exit code, and no `hint:` line, colour or glyph is written (per design-system §Surface: cli → Tokens)
- The count the reader yields keeps unknown kinds, unknown fields and torn lines as three separate figures, each
  readable at zero, and a heal or replay does not clear a nonzero one (per design-system §Color Palette, Domain
  status colors, `Skipped::zero` / `Skipped::nonzero`)
- `viola verify`'s stdout still ends in its summary line, with every line before it a step line, when a leftover
  probe dir is found or removed (per design-system §Surface: cli → Component Patterns 5)
