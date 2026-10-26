# Classified — a11y · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · yes · the `--help` group labels: the chunk lands the `handoff:` row (`link, unlink`) between `wheel:` and `setup:` and moves the pinned HELP literal with it
aria-patterns-roles#1.2 · no · the tui row is N/A for the vendor claude TUI, a surface the chunk does not add to; the wrapper's terminal-byte rule sits in #2.1
aria-patterns-roles#2.1 · yes · the new `link` / `unlink` handling in the wrapper must write zero terminal bytes and never block or delay a keystroke past the atomic paste (clauses 1 and 2); clause 3 (the wheel) and the cli `send` refusal are not reached
aria-patterns-roles#3.1 · yes · the three new result lines owe its linear static output (no cursor control, redraw or spinner) and C0/C1 escape with `\n` and `\t` raw, and the edited `viola --help` owes its no-ESC, no-lone-CR check; its SGR lines are standing S5, and the CAPTION literal and the wait / last / verify rows are not reached
