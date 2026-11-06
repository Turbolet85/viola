---
paths:
  - "src/human.rs"
  - "src/human/**"
  - "src/run/wheel.rs"
  - "tests/cli_output_plain.rs"
  - "tests/cross_list_rows.rs"
  - "tests/tui_*.rs"
---

# Accessibility Rules

Path-scoped rules for the cli output discipline, the tui boundary and the a11y harness. **Authoritative source:** `.andromeda/a11y-plan.md` §3 (Harness Contract) and §4–§8. No WCAG label in 0.1.0: the frontend is deferred to 0.2.0 and its technology is not chosen; the CLI and TUI make no conformance claim.

## The frontend (deferred to 0.2.0)
- The frontend is deferred to 0.2.0 and its technology is not chosen.

## Keyboard and focus
- cli: no interactive prompt, pager or redraw, and input is stdin / `--file` only; tui: the terminal belongs to the child (under `viola revive --json` a `pump_child` error prints no document: viola writes nothing past it, and a peer the wrapper refuses is dropped in `accept_loop`, before its worker thread and any frame: it writes nothing to the outer terminal and moves no wheel), and human keystrokes are never blocked, refused or delayed past the atomic paste, and focus / mouse / resize sequences and terminal replies (F-W2's closed list) are never counted as editing; every other byte is. The page's tab order is deferred with the frontend to 0.2.0.

## Visual
- On the CLI, state is always a printed word (`stale`, `DIALOG <kind>`, `expired`, `budget-paused`, `unverified-cli`, the STATUS words `idle` / `busy` on joined wrapped rows and the unwrapped rows' `n/a` cells; on a terminal too narrow only an unwrapped NAME is cut, to the column less 3 characters plus `...` and never inside a `\xHH` escape, a wrapped NAME never, and piped output and `--json` keep every name whole); colour is a second cue only (amber SGR only on `DIALOG`, dim on stale rows, bold only on NAME values, always beside the word), `unable` / `fail` are never coloured, and viola emits no SGR of its own under non-TTY / `NO_COLOR` (an empty value included) / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`, and `viola link` / `viola unlink` print one plain-ASCII result line in one write with no SGR in any mode. The page's contrast pairs are deferred with the frontend to 0.2.0.

## Harness
- The a11y harness is the tests' own assert_cmd / predicates / trycmd / portable-pty cases (cli output discipline, tui boundary), run by `scripts/agent-run.* run` inside the `test` job's `coverage` suite (matrix `[ubuntu-latest]`) under `gate --require coverage,doctest` (skips never allowed); 0.1.0 runs no browser a11y leg.
- cli / tui write no a11y rows: output-discipline failures surface only as nextest failures in `suites[].failures[]`; the web row shape is deferred with the frontend to 0.2.0.
- No new `suite` enum value (such as `a11y`) without a Decisions Log entry, and no separate a11y driver outside `scripts/agent-run.* run`.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
