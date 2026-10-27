# design · cascade read · group 3 of 4

## Carried
- (.andromeda/design-system-detail/color-palette.md:101) The paste rule's refusal the chunk re-asserts at CLI `send`, MCP `send` and authoritatively in the wrapper must carry the reason `not-delivered`, printed as a word with its colour never standing alone; the hex columns are a 0.2.0 frontend input, 0.1.0 renders no page.
  > | RefusalReason::not-delivered | #1E2124 | #E6D8AE | #E6D8AE | `unable · not-delivered` |
  (.andromeda/design-system.md:91)
  > A 0.2.0 frontend design input; 0.1.0 renders no page.
  (.andromeda/design-system.md:93)
  > viola deliberately carries them with **ink, fill, rule, strike and a printed word, never with a green/yellow/red hue**.
- (.andromeda/design-system-detail/color-palette.md:111) A paste text holding a refused C0, DEL or C1 character must be refused with the detail `control-character` under the reason `not-delivered`, the pair the chunk re-verifies at every client and in the wrapper.
  > | RefusalDetail::control-character (security plan) | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · control-character` |
- (.andromeda/design-system-detail/color-palette.md:112) An empty typed text must be refused with the detail `empty-text` under the reason `not-delivered`, the pair the chunk re-verifies beside `control-character`.
  > | RefusalDetail::empty-text | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · empty-text` |
- (.andromeda/design-system-detail/color-palette.md:107) A send met by a vt100 screen-model panic must read the degraded readiness as the detail `input-not-ready` under `not-delivered`, the degrade the chunk re-verifies at `src/run/gate.rs` and `crates/viola-agent-claude/src/screen.rs`.
  > | RefusalDetail::input-not-ready | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · input-not-ready` |
- (.andromeda/design-system-detail/color-palette.md:110) A dialog decision read from `viola answer`'s stdin as a closed type must, for a dialog the wrapper does not hold, be refused with the detail `unknown-dialog` under `not-delivered`, never passed through as free text.
  > | RefusalDetail::unknown-dialog | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · unknown-dialog` |
- (.andromeda/design-system-detail/color-palette.md:127) An event kind unknown to its reader must be skipped and counted as an unknown kind, never drawn or decided on as a kind, the closed-type rule the chunk re-asserts for a hook event kind; its tape position is a 0.2.0 frontend input.
  > | EventKind::(unknown to this page) | #1E2124 | none | #9C9278 | `skipped  1 unknown kind`, printed at the point in the tape where it occurred |
- (.andromeda/design-system-detail/color-palette.md:141) The BAY line must show the `events.ndjson` reader's three skip counts, unknown kinds, unknown fields and torn lines, in that order, a zero figure printed as `0`; on the CLI the separator is ` - ` (ASCII only), the `·` being the web's.
  > | Skipped::zero (unknown_kinds / unknown_fields / torn_lines) | #1E2124 | none | #9C9278 | `skipped 0 · 0 · 0` in the ATIS line |
- (.andromeda/design-system-detail/color-palette.md:142) A nonzero skip count, an unknown kind or field skipped and counted or a torn line, must stay visible where it is shown and never be auto-cleared, so a bounded reader's skip is visible, not silent.
  > | Skipped::nonzero | #1E2124 | #E6D8AE (1px box around the count) | #E6D8AE | `skipped 2 unknown kinds · 0 · 1 torn line`, kept visible, never auto-cleared |
  (.andromeda/design-system.md:99)
  > a nonzero `skipped` count stays visible, never auto-cleared
- (.andromeda/design-system-detail/color-palette.md:93) The BAY line's `5H` / `7D` figures must come only from the bounded `budget.json` read, and a window whose `resets_at` has passed must print `expired` after its figure, old information shown as old; a failed bounded read prints its no-reading word, never a guess.
  > | Window::expired (derived: `resets_at` has passed) | #1E2124 | #9C9278 | #E6D8AE | `5H 62 % expired`: old information shown as old |
  (.andromeda/design-system.md:102)
  > A `claude_agents` failure surfaces on the page as STATUS `unknown`.

## Bindings
- **Refusal detail rows ↔ Surface: cli's readback mirror**: the reason / detail pairs above are printed on the CLI as `[/ ] unable  <name>  not-delivered  <detail>` on stderr, exit 13, then its `hint:` line, with the reason and detail separated by two spaces, not `·` (.andromeda/design-system.md:298) (.andromeda/design-system.md:303).
- **Refusal detail rows ↔ Exit-code phraseology**: every `not-delivered` refusal the chunk re-asserts takes an exit in 10 to 14 with an `unable` refusal and its reason, a refusal object under `--json` (.andromeda/design-system.md:338).
- **Skipped rows ↔ Surface: cli's BAY line and Iconography**: the three counts print as `skipped 0 - 0 - 0` on the BAY line, ` - ` being the CLI's in-field separator where the web uses `·` (.andromeda/design-system.md:278) (.andromeda/design-system.md:209).
- **Window::expired ↔ Surface: cli's BAY line**: the CLI prints the same `5H 62 % expired` after a window whose `resets_at` has passed (.andromeda/design-system.md:287).
- **All rows ↔ cli bans**: on the CLI no colour of these rows is emitted, only `DIALOG` (amber) and `stale` rows (dim) are coloured, so each state is carried by its printed word alone (.andromeda/design-system.md:378).
