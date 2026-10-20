# design extract

## Relevance
partial — the chunk renders no new surface; it touches the cli design surface only where the four rows' `viola verify` step lines and summary, and `viola send`'s readback outcomes for `/clear` / `/compact`, are printed.

## Constraints
- `viola verify` keeps its one progress pattern: a step counter with static appended stdout lines, one per relied ledger row, `[NN/MM] <row-id> <row description>  pass|fail`, with `MM` the relied count (`LedgerRow::relied()`), per design-system §Surface: cli · Component Patterns 5 (`verify`). A row whose probe or oracle changes keeps its row id and its place in `LedgerRow::ALL`'s order; if its description phrase is reworded (e.g. `tag-escaping` "tag-like text un-escapes to the text as pasted", `local-command-clear`, `paste-newline`, `local-command-compact`), the new phrase is a design-surface change the plan names, per design-system §Surface: cli · Component Patterns 5.
- The last stdout line stays the summary `stamped <ver>  <n> pass  <m> fail`, n + m equal to `MM`, with `fail` uncoloured, and a failing row prints no stderr word, per design-system §Surface: cli · Component Patterns 5 and §Surface: cli · Exit-code phraseology (exit 1 row).
- A `viola send` of `/clear` or `/compact` stays `[  ] open` until its measured post-condition (new session for `/clear`; same-session compact for `/compact` on a verified CLI), then fills or is struck with `unable` plus the typed reason; `unconfirmable` (`local command, no measured post-condition`) only on an unverified CLI, per design-system §Anti-Patterns · Rejected Defaults (optimistic "Sending…" spinner) and §Surface: cli · Component Patterns 2.
- An `unverified-cli` refusal keeps its hint `run viola verify for this CLI version` and a `no-prompt-submitted` refusal keeps its fixed hint; no hint quotes sent or upstream text, per design-system §Surface: cli · Component Patterns 2.
- verify's human output stays plain ASCII with no SGR (only `DIALOG` and `stale` rows are ever coloured), per design-system §Anti-Patterns · Per-Surface Bans (cli).
- verify's step lines and summary go to stdout; its `unable:` / `hint:` refusal pairs go to stderr, per design-system §Surface: cli · Streams.

## Patterns to follow
- The verify step-line and summary forms as worded in design-system §Surface: cli · Component Patterns 5 — the row list there is the description source; whether the code's row descriptions match it verbatim today is research's question.
- The readback mirror `[RB]` / `[  ]` / `[/ ]` with words `read back` / `unable` / `unconfirmable` for the local-command confirm path, per design-system §Surface: cli · Component Patterns 2.
- The fixed-message `unable:` / `hint:` pair form for verify's exit-1 refusals (CLI not found, version unreadable, an unclean `--record` fixture naming `<file> <code>` only), per design-system §Surface: cli · Exit-code phraseology.

## Anti-patterns to avoid
- No spinner, progress bar or live-redrawing line while a verify run or a re-measurement probe waits on the CLI, per design-system §Anti-Patterns · Per-Surface Bans (cli).
- No green `✓` / red `✗` or emoji on pass/fail rows, and no colour on `fail`, per design-system §Anti-Patterns · Per-Surface Bans (cli).
- No path, pid, upstream text or captured probe content in a verify line, refusal or hint (a re-measured row's reason, if surfaced, is a closed word, never the capture), per design-system §Anti-Patterns · Per-Surface Bans (cli).

## Contract bindings
- design-system §Surface: cli · Component Patterns 5's row list and count (`MM` = `LedgerRow::relied()`, 24 on Unix) ↔ the capability ledger in `crates/viola-agent-claude/src/ledger.rs` and `CLAUDE.md` Critical Warnings: a row description or count change here cascades to both (the wrap's cascade, per the scope).
- design-system §Anti-Patterns · Rejected Defaults (readback for `/clear` / `/compact`) ↔ the ledger rows `local-command-clear` and `local-command-compact` post-conditions and `COMPACT_CONFIRM_WINDOW`: a changed post-condition changes when the box fills.
- No colour is printed by verify, so no a11y contrast binding is opened.

## Acceptance criteria contributions
- (design) `viola verify` on 2.1.302 prints 24 static `[NN/24] <row-id> <description>  pass` lines then exactly `stamped 2.1.302  24 pass  0 fail` on stdout, no SGR byte, no spinner/redraw byte, stderr empty (per design-system §Surface: cli · Component Patterns 5)
- (design) Any reworded row description for the four rows is listed in the plan and lands in design-system §Surface: cli · Component Patterns 5 at the wrap, the row ids and their order unchanged (per design-system §Surface: cli · Component Patterns 5)
- (design) On a verified 2.1.302 home a `send` of `/clear` or `/compact` prints `[RB] read back` after its post-condition, never `unconfirmable`; no line quotes the sent text (per design-system §Surface: cli · Component Patterns 2)
