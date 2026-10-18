# design extract

## Relevance
partial — the chunk touches two human CLI outputs, `viola verify`'s step-counter lines (a new relied row moves the count) and `viola send`'s readback mirror for `/compact` (from `unconfirmable` to a read-back once its post-condition holds); tokens, colour, motion and the web surface do not apply.

## Constraints
- `viola verify` prints one static stdout step line per relied ledger row, `[NN/MM] <row-name> <one-sentence description>  pass` / `... fail`, with `MM` equal to `LedgerRow::relied()` and rows in `LedgerRow::ALL` order; a 24th relied row adds one such line with its own short description sentence in the style of the listed rows (e.g. `local-command-clear /clear starts a new session and submits no prompt`) and moves `MM` on every OS it is relied on (per design-system §Surface: cli · Component Patterns 5 `viola verify` / `viola run`).
- `viola verify`'s last stdout line is the summary `stamped <ver>  <n> pass  <m> fail` with `n + m` equal to `MM`, and the word `fail` stays uncoloured (per design-system §Surface: cli · Component Patterns 5 `viola verify` / `viola run`).
- `viola send`'s readback box stays `open` until its confirmation, then fills `[RB] read back` (stdout, exit 0, with time and cursor) or is struck `[/ ] unable` with the typed reason and one `hint:` line; the `[  ] unconfirmable  <name>  local command, no measured post-condition` line is the form for a local command with no measured post-condition, so a `/compact` confirmed by a post-condition requires the read-back form instead, not the unconfirmable one (per design-system §Surface: cli · Component Patterns 2 `viola send`: the readback mirror).
- A clearance does not count until it is read back: a `/compact` send presumes nothing while its own confirm window runs, the same as `/clear` waits on its new-session post-condition (per design-system §Anti-Patterns · Rejected Defaults, "Optimistic Sending… spinner").
- Human output stays ASCII only and is never wrapped; results go to stdout, while the `[  ] open` issue line (TTY only), refusals and `hint:` lines go to stderr (per design-system §Surface: cli · Width and Streams).
- Under `--json` the whole send or verify result is one JSON document on stdout with the typed exit code, and no `hint:` line is printed (per design-system §Surface: cli · Streams).
- A hint never quotes the sent text or any upstream text, so no `/compact` refusal hint echoes the command or compaction output (per design-system §Surface: cli · Component Patterns 2 `viola send`: the readback mirror).

## Patterns to follow
- The verify step counter (the project's one progress pattern): static appended lines, one per relied row, then the summary. The new row joins as one more numbered line in `LedgerRow::ALL` order (per design-system §Surface: cli · Component Patterns 5).
- `/clear`'s new-session post-condition as the model for a local command whose readback box fills on a measured post-condition rather than on `prompt-submitted` (per design-system §Anti-Patterns · Rejected Defaults, the readback-box sentence).
- The readback mirror's three prefixes `[  ]` / `[RB]` / `[/ ]` with the padded readback-word column, as in the `viola send` sample (per design-system §Surface: cli · Component Patterns 2).

## Anti-patterns to avoid
- No spinner, progress bar or elapsed-time counter while the longer `/compact` confirm window runs: only the one static `[  ] open` line (per design-system §Anti-Patterns · Per-Surface Bans cli).
- No green `✓` / red `✗` prefix, no emoji, and no colour on a verify step line or a send result line, since only `DIALOG` (amber) and `stale` rows (dim) are coloured (per design-system §Anti-Patterns · Per-Surface Bans cli).
- Never print "done" or "success" without context: a confirmed `/compact` says what was read back, by whom and at which cursor (per design-system §Anti-Patterns · Per-Surface Bans cli).

## Contract bindings
- design-system §Surface: cli · Component Patterns 5's `MM` binds to the capability ledger's relied set (`LedgerRow::relied()`, architecture's ledger rows). Today the master's counts (23 on Unix, 20 on Windows, and the sample `stamped 2.1.287  23 pass  0 fail`) are one row behind once this chunk lands the row. The amendment belongs in the wrap's fan-out.
- design-system §Surface: cli · Exit-code phraseology binds the send result's human words to architecture §Standard Contracts' `--json` shapes (`{"v":1,"ok":{…}}` / refusal object). Which result word and which exit a `/compact` gets when its window runs out with no `compact` SessionStart is not defined in this plan. Research should check what the send slot does today for an expired `/clear` post-condition, and the architecture extractor should check whether architecture names that shape.

## Acceptance criteria contributions
- On a host where the new row is relied, `viola verify` prints exactly one `[NN/MM] <new-row-name> <description>  pass|fail` line for it in `LedgerRow::ALL` order, and every step line's `MM` and the summary's `n + m` equal the relied count (per design-system §Surface: cli · Component Patterns 5 `viola verify` / `viola run`).
- On a CLI stamped with the new row, a human-mode `viola send` of `/compact` prints `[RB] read back` (stdout, exit 0) once the post-condition holds, and no longer prints the `[  ] unconfirmable … local command, no measured post-condition` line. Other local commands with no post-condition keep that line (per design-system §Surface: cli · Component Patterns 2 `viola send`: the readback mirror).
- The `/compact` send and the verify run write no SGR byte, no non-ASCII glyph and no live-redrawn line. With `--json` the result is one stdout document with no `hint:` line (per design-system §Anti-Patterns · Per-Surface Bans cli).
