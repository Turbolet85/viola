@amendment marker=2026-11-04-linux-and-macos-parity
title: the stale row's end-to-end witness on the Unix legs
section: Surface: cli › Component Patterns (surface: the `viola list` As built, the stale row's colour layer)
change:
  - A new fact: the stale row's colour layer is witnessed end to end on `ubuntu-latest` and `macos-latest` by `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`tests/cross_list_rows.rs`), passing on both at ci#41918694243.
  - Its reading: `viola list` under an outer PTY with `TERM=xterm-256color` prints a stale row whose escape sequences are exactly DIM, BOLD, RESET, DIM, RESET (the row dim from its first byte to its last, NAME dim and bold), `stale` shown, and no SGR on the BAY line, the caption or the separator.
  - No standing line of the body changed: no colour, word or column moved.
why: the report reads the integration-tier outer-PTY witness of the stale row's dim span landed and met on both Unix runners at the final pushed sha's green run; the chunk added no UI element or token.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
