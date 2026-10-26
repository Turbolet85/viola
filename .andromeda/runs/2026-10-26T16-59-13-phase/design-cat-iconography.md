# design · Iconography — category read

## Carried
- (.andromeda/design-system-detail/iconography.md:10) Every human stderr line the new strict-modes refusals print on the CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`; exit 21) and on `run` / `verify` must be ASCII only. Columns must be separated by two spaces, and so must a refusal's reason from its detail.
  > The CLI is ASCII only:
  >   - Two spaces separate columns, and a refusal's reason from its detail (they are separate fields, as in the exit-code table).
- (.andromeda/design-system-detail/iconography.md:12) A refusal label or field this chunk adds that joins two words inside one field must use ` - `, never `·`.
  >   - ` - ` (space, hyphen, space) replaces `·` inside one field or label: `driver - budget-paused`, `skipped 0 - 0 - 0`, `-- UNWRAPPED - READ-ONLY --`.
- (.andromeda/design-system.md:215) A refused home or instance check must be shown as its printed words (`unable`, the reason, the hint), never as a warning triangle, checkmark or status-dot glyph.
  > **Rule:** icons clarify, not decorate. If removing an icon loses no meaning, remove it. In viola there are no robot or person icons, no status dots, no checkmarks and no warning triangles. Every one of those states is already a printed word.

## Bindings
- **CLI separators ↔ the body's Exit-code phraseology**: the two-space reason/detail separator (.andromeda/design-system-detail/iconography.md:11) is the separator of the exit-21 form `unable  instance-unreachable` then the cause's hint that a new strict-modes refusal of a frame verb takes (.andromeda/design-system.md:338). Under `--json` that form is replaced by the `instance-unreachable` document, with stderr empty.
- **No-glyph rule ↔ the body's cli Per-Surface Bans**: the no-checkmark / no-warning-triangle rule (.andromeda/design-system.md:215) matches the ban on green `✓` / red `✗` prefixes and emoji (.andromeda/design-system.md:377). Both bind any refusal line the chunk adds.
- **Iconography ↔ the body's Platform-Specific Notes**: `hook` (which fails open with no stderr) and `mcp` (MCP frames only) have no human design surface (.andromeda/design-system.md:355). So none of the glyph or separator rules reach the strict-modes checks this chunk adds to those two verbs.
