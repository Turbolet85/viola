# design · Color Palette — category read

## Carried
- (.andromeda/design-system-detail/color-palette.md:12) The socket-directory and peer-euid refusals, and their hints, must never take the attention colour: arrival amber stays off every refusal and error this chunk adds.
  > Never used for errors, warnings, budget or hover.
- (.andromeda/design-system.md:242) The CLI's colour tokens are a closed set, none of them a refusal's, so a refusal or hint this chunk adds on any verb must take no new token, no handoff blue and no background.
  > CLI colour tokens, at truecolor, 256-colour and 16-colour depth: attention (arrival amber) on the `DIALOG` word only, always followed by the kind word; SGR dim on a whole `stale` row, the cock winning for `DIALOG`; bold for NAME values; a reset after every styled span. Handoff blue is never used, and backgrounds are never set: the terminal background belongs to the user.
- (.andromeda/design-system.md:363) A refused socket directory or a peer euid mismatch must not be marked by a hue, since amber and blue each hold one meaning and a third hue has none.
  > **NEVER use color purely for decoration. Every color communicates meaning.** Amber means `dialog_pending` and nothing else. Blue means handoff and nothing else. Any third hue has no domain meaning.
- (.andromeda/design-system.md:378) Every refusal line and `hint:` line this chunk adds must print uncoloured on a human surface.
  > **NEVER colour anything except `DIALOG` (amber) and `stale` rows (dim), and never either without its word.**
- (.andromeda/design-system.md:379) A refusal this chunk adds to `viola run`'s start, or reaching a `--json` or non-TTY caller, must emit no colour or cursor control, and a passed `viola revive` holds to the same rule.
  > **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.** LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen; a passed `viola revive` is the same passthrough.

## Bindings
- **The uncoloured refusal ↔ Surface: cli, exit-code phraseology**: the no-colour rule (.andromeda/design-system.md:378) governs the printed forms the surface's closed exit-code paragraph sets for each refusal (.andromeda/design-system.md:338), so whichever exit P3 reads for a new refusal, its line stays plain.
- **Amber's one role ↔ the body's universal ban**: the Accent row's exclusion (.andromeda/design-system-detail/color-palette.md:12) and the ban on a hue without a domain meaning (.andromeda/design-system.md:363) are one rule on both sides, so a security refusal has no colour to borrow.
