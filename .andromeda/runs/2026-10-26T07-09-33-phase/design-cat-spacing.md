# design · Spacing — category read

## Carried
- (.andromeda/design-system.md:153) Where a CLI client reports a failed or impossible server verification as unreachable (exit 21), its human stderr refusal must keep the fixed-width terminal grid with two spaces between columns, the same form the liveness pre-check's refusal uses; the 0.2.0 frontend spacing tokens (.andromeda/design-system.md:147, every row of .andromeda/design-system-detail/spacing.md) do not bind, since 0.1.0 renders no page.
  > CLI spacing is platform-controlled: a fixed-width terminal grid, two spaces between columns.

## Bindings
- **CLI two-space column grid ↔ the body's Surface: cli exit-code phraseology**: the mismatch outcome on a CLI verb must print the exit-21 line `unable  instance-unreachable` then the cause's hint, two spaces between its columns (.andromeda/design-system.md:153), as the body's exit-code row gives it (.andromeda/design-system.md:338); under `--json` the document goes to stdout with stderr empty, so no column spacing applies there. `viola hook` and `viola mcp` have no human design surface, so the rule does not reach them (.andromeda/design-system.md:353).
