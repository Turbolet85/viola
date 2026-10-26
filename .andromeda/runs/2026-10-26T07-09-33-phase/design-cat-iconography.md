# design · Iconography — category read

## Carried
- (.andromeda/design-system-detail/iconography.md:10) A server-verification mismatch, or a check that cannot be made, refused by CLI `send` / `wait` / `last` / `answer` / `pause` / `release` must print its human refusal in ASCII only, its reason and its detail separate fields set two spaces apart, the same form the existing liveness pre-check's exit-21 refusal takes.
  > The CLI is ASCII only:
  >   - Two spaces separate columns, and a refusal's reason from its detail (they are separate fields, as in the exit-code table).
- (.andromeda/design-system-detail/iconography.md:12) Any label or field the chunk adds to a refusal or hint line that would need a separator inside one field must use ` - `, never `·`.
  > ` - ` (space, hyphen, space) replaces `·` inside one field or label
- (.andromeda/design-system.md:215) The unreachable state a failed verification reports must be carried by its printed word (`unable  instance-unreachable`), with no warning glyph or status mark added.
  > **Rule:** icons clarify, not decorate. If removing an icon loses no meaning, remove it. In viola there are no robot or person icons, no status dots, no checkmarks and no warning triangles. Every one of those states is already a printed word.

## Bindings
- **CLI separator rule ↔ the body's Surface: cli exit-code phraseology**: the two-space reason/detail separation (.andromeda/design-system-detail/iconography.md:11) is the form of the exit-21 line the body names, "21 `unable  instance-unreachable` then the cause's hint" (.andromeda/design-system.md:338); a verification refusal reuses that line and its `--json` document, stderr empty.
- **No-glyph rule ↔ the body's cli Per-Surface Bans**: the printed-word-only rule (.andromeda/design-system.md:215) meets the ban on green `✓` / red `✗` prefixes and emoji (.andromeda/design-system.md:375) and the ban on printing paths or pids in errors or hints (.andromeda/design-system.md:380), which binds a mismatch refusal that must not name the recorded or the connected server pid.
- **`viola hook` / `viola mcp` ↔ the body's Platform-Specific Notes**: neither has a human design surface (.andromeda/design-system.md:353), so the fail-open hook mismatch and the MCP `instance-unreachable` result carry no glyph or separator of this category.
