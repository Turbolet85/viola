# design extract

## Relevance
partial — the chunk renders nothing new (no token, colour, type, spacing or motion is touched); it only moves a text with an inner CR or CRLF from the refused form to the confirmed form of the existing `viola send` readback phraseology, which design-system §Surface: cli fixes.

## Constraints
- design-system §Brand Identity (Domain anchors → Readback) requires that a send counts only once the matching `prompt-submitted` reads it back, and is otherwise "unable" plus a typed reason. The chunk's confirmed outcome must come from that readback on the normalised typed text, never from a presumption that an inner-CR text was delivered.
- design-system §Surface: cli → Component Patterns → 2. `viola send`: the readback mirror requires the confirmed send to print the one `[RB] read back` result line in its sample's column form. The chunk adds no line, no word and no column for a text that held a CR.
- design-system §Surface: cli → Exit-code phraseology requires exit 13 to carry a closed list of `not-delivered` details. The chunk adds none and moves none; a new detail would need a row there and is a P4 card by the scope's own boundary.
- design-system §Color Palette (Domain status colors) requires one row per `RefusalDetail` variant with its printed word. With no new variant the chunk owes no row; a variant added at P4 would owe one.
- design-system §Surface: cli → Component Patterns → 2. `viola send`: the readback mirror fixes the `not-delivered · empty-text` and `not-delivered · control-character` hint strings. The chunk keeps both: a text of only CR and LF characters stays `empty-text`, and CR stays an allowed character at validation, so the hint naming the allowed set stays true. Whether the code's hint strings match the plan's is research's question.
- design-system §Surface: cli → Tokens (Streams) requires the result line on stdout and the refusal plus its `hint:` line on stderr, and under `--json` one JSON document with no hint. Any test that reads the driver-visible outcome reads those streams as specified.

## Patterns to follow
- The readback mirror words print in every human output, piped and non-TTY included, with no colour and no SGR (design-system §Surface: cli → Tokens, Colour decision order). A test reading the CLI's output under a pipe asserts the ASCII words, not a styled form.
- A matched driver prompt is folded into its send's readback; an unmatched one stands as its own `prompt · human` line (design-system §Color Palette, Domain status colors, `PromptOrigin::driver` / `PromptOrigin::human`). The after-change outcome for an inner-CR text is the folded driver form.
- A confirmed send leaves the WHEEL word at `driver` and produces no `human · human-input` wheel line (design-system §Color Palette, Domain status colors, `WheelHolder::driver` / `WheelCause::human-input`).
- Refusal phraseology is `unable` plus reason plus detail, two-space separated on the CLI, then one `hint:` line keyed by reason or reason · detail (design-system §Surface: cli → Component Patterns → 2. `viola send`: the readback mirror). The chunk reuses it unchanged for `empty-text`.

## Anti-patterns to avoid
- Never announce a send as confirmed before it is read back (design-system §Anti-Patterns → Rejected Defaults, the optimistic "Sending… / Sent" default).
- Never quote the sent text or any upstream text in a hint or an error (design-system §Surface: cli → Component Patterns → 2. `viola send`: the readback mirror; design-system §Anti-Patterns → Per-Surface Bans → cli).
- Never print "done" or "success" without context: the result names what was read back, by whom and at which cursor (design-system §Anti-Patterns → Per-Surface Bans → cli).

## Contract bindings
- design ↔ architecture: the exit codes and refusal detail words in design-system §Surface: cli → Exit-code phraseology are architecture's typed codes; the chunk's "no refusal detail added" boundary keeps both in step.
- design ↔ security: the `control-character` hint's allowed set in design-system §Surface: cli → Component Patterns → 2. `viola send`: the readback mirror restates security-plan §Input Validation; the chunk changes what is typed, not what is allowed, so the two stay bound.
- design ↔ tests: the driver-visible outcome test, where test-plan §4 gives a tier above the unit tier, reads the result line or the `--json` document that design-system §Surface: cli → Tokens (Streams) specifies.
- design-system.md is not among the masters the scope lists for the wrap's amendments, and no sentence of it states that an inner CR is typed as received; no design amendment is expected from this chunk.

## Acceptance criteria contributions
- A sent text with an inner CR or CRLF that is confirmed prints the `[RB] read back` result line on stdout with exit 0, and no `[/ ]` refusal line or `hint:` line on stderr (per design-system §Surface: cli → Component Patterns → 2. `viola send`: the readback mirror).
- The human-mode refusal line and hint string for `not-delivered · empty-text` and for `not-delivered · control-character` are unchanged by the chunk's diff (per design-system §Surface: cli → Component Patterns → 2. `viola send`: the readback mirror).
- No output of the changed build carries a refusal detail word outside the exit-13 list (per design-system §Surface: cli → Exit-code phraseology).
- No hint, refusal or error line added or changed by the chunk quotes the sent text (per design-system §Anti-Patterns → Per-Surface Bans → cli).
