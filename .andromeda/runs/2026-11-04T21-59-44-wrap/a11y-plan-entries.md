@amendment marker=2026-11-04-linux-and-macos-parity
title: P6's stale-row dim witness under the outer PTY, on the Unix legs
section: §4 ARIA Patterns & Roles (a11y-plan.md: the stale-row line; aria-patterns-roles.md: P6's outer-PTY SGR bullet, the new P6 stale-row path); the subjects index (a11y-plan.md, Universal (agent-driven specific))
change:
  - a11y-plan.md §4: the line reads **Unix legs only:** dim only on stale rows is proven at the unit tier and, under the outer PTY, on ubuntu-latest and macos-latest; the outer-PTY witness does not run on windows-2025 (was **Owed:** proven at the unit tier only, its outer-PTY witness owed to "Linux and macOS parity").
  - aria-patterns-roles.md, P6's outer-PTY SGR bullet: dim only on stale rows is proven at the unit tier (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`) and under the outer PTY on ubuntu-latest and macos-latest only, not on windows-2025 (was proven at the unit tier only, its outer-PTY witness a CARRY on "Linux and macOS parity").
  - aria-patterns-roles.md, a new path **P6 — stale-row dim under the outer PTY** (cli, Unix legs): `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`tests/cross_list_rows.rs`, `cfg(unix)`, on a booted wrapper) freezes the wrapper under the `Frozen` guard (`tests/support/frozen.rs`, test-plan's), back-dates its `heartbeat` 60 s so its row reads `stale`, and runs `viola list` under the outer PTY with `TERM=xterm-256color`; the row is dim from its first byte to its last and `stale` is printed. It runs on ubuntu-latest and macos-latest and not on windows-2025.
  - The subjects index gains `path-p6-stale-dim`.
why: the outer-PTY witness owed to this chunk landed: it passes on ubuntu-latest and macos-latest, and, gated `cfg(unix)`, it does not run on windows-2025.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
@amendment marker=2026-11-04-linux-and-macos-parity
title: the stale row's SGR sequence on a terminal, and its Unix-only check
section: §6 Visual Design Verification (visual-design-verification.md: the Checked-by line, the new stale-row values); the subjects index (a11y-plan.md, Universal (agent-driven specific))
change:
  - visual-design-verification.md, the Checked-by line: it names `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal`, on ubuntu-latest and macos-latest only (`cfg(unix)`, not run on windows-2025), the outer-PTY witness of a stale row dim from its first byte to its last (was dim only on stale rows proven at the unit tier only, its outer-PTY witness a CARRY on "Linux and macOS parity").
  - Its closing qualifier: checked with assert_cmd + portable-pty on `[windows-2025, macos-latest, ubuntu-latest]`, the stale-row dim witness on the two Unix legs only (was the three legs, no exception named).
  - A new value, **A stale row on a terminal:** `viola list`'s stream for a board whose wrapped row is `stale` carries exactly the escape sequences `[DIM, BOLD, RESET, DIM, RESET]`, the word `stale` printed beside the dim, as the outer-PTY stale-row witness asserts it on the two Unix legs.
  - The subjects index gains `visual-sgr-stale-row`.
why: the stale-row witness asserts exactly that sequence with `stale` shown and passes on ubuntu-latest and macos-latest; gated `cfg(unix)`, it does not run on windows-2025, so the three-leg qualifier names the exception.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
