# design · Typography — category read

## Carried
- (.andromeda/design-system.md:139) Any human CLI line this chunk adds or changes, such as the exit-21 strict-modes refusal and its `hint:` line at the newly checked entry points (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `plugin install`), must be plain ASCII in the user's terminal monospace. It may carry no styling: bold stays on the NAME column of `viola list` only, and the only other SGR is dim on `stale` rows and amber on `DIALOG`.
  > Bold is used only for the NAME column. SGR dim marks `stale` rows, and amber the `DIALOG` word. Output is ASCII only, and density is high.

## Bindings
- **cli typography (ASCII only, bold NAME only) ↔ Surface: cli**: the Typography rule that the cli is "ASCII only" and uses SGR only on NAME, `stale` and `DIALOG` (.andromeda/design-system.md:139) is the same rule the Surface section gives as "Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`)" (.andromeda/design-system.md:270). That section also fixes the exit-21 form, `unable  instance-unreachable` followed by the cause's hint, with stderr empty under `--json` (.andromeda/design-system.md:338). A strict-modes refusal this chunk adds at a frame verb must keep both.
- **typography.md (the 0.2.0 frontend's type roles and font stacks) ↔ the body**: every value in the category file is "A 0.2.0 frontend design input; 0.1.0 renders no page" (.andromeda/design-system.md:122). This chunk renders no page, so no row of .andromeda/design-system-detail/typography.md binds it (nearest: .andromeda/design-system-detail/typography.md:13, the Body role, frontend only).
