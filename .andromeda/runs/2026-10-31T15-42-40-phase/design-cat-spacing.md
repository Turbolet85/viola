# design · Spacing — category read

## Carried
- (.andromeda/design-system.md:153) Any human CLI line this chunk adds or changes for an exit-21 or exit-1 cause must keep the platform's fixed-width grid and separate its columns by two spaces, with no spacing token of its own.
  > CLI spacing is platform-controlled: a fixed-width terminal grid, two spaces between columns.
- (.andromeda/design-system.md:261) A refusal line in the column form must stay two-space separated, and the fixed `unable: <text>` form keeps its own shape for the causes it covers, so telling causes apart must not reflow either form.
  > stderr: context lines, refusals (two-space `unable  <name>  <reason>  <detail>`, or the fixed `unable: <text>` form for the exit-1 refusals of `run`, `revive` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release`), each refusal's `hint:` line directly after it, and errors.
- (.andromeda/design-system.md:338) The exit-21 human form must stay the two-space `unable  instance-unreachable` followed by the cause's own hint, and a per-cause code must not change that spacing.
  > 21 `unable  instance-unreachable` then the cause's hint

## Bindings
- **CLI two-space columns ↔ Surface: cli refusal forms**: the spacing rule (.andromeda/design-system.md:153) governs the column separators of the refusal forms the Surface section names (.andromeda/design-system.md:261), and the exit-21 phraseology holds the same separator (.andromeda/design-system.md:338).
