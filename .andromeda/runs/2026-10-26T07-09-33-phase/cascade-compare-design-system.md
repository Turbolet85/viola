# Cascade compare — design-system · phase · 2026-10-26-server-verification-before-any-frame
E items 22 · held 3 · lost at classify 0 · lost at strong 1 · beside a placed fact - · cascade only 0 · listed 2 · body 13 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### brand-identity#2.2 · lost at strong · .andromeda/design-system-detail/brand-identity.md:21
- (.andromeda/design-system-detail/brand-identity.md:21) Under `--json`, on a non-TTY, under `NO_COLOR` or with `TERM=dumb`, the mismatch refusal must carry no colour, glyph or cursor control. If P3 finds a client in `run`'s start or in a `revive` that passed its preflight, that client must print nothing while the wrapped TUI runs.
  > | cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight. |

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### brand-identity#2.1 · listed (lost at strong) · .andromeda/design-system-detail/brand-identity.md:20
- (.andromeda/design-system-detail/brand-identity.md:20) When the server-identity check fails, a CLI verb on a human TTY must print its exit-21 refusal as static, uncoloured lines. Colour belongs only to `DIALOG` and `stale`, so a refusal gets none, and nothing redraws while the check runs.
  > | cli, human TTY | 0.2 | Static k9s-style fixed-column rows and one static context header. Colour is only a second cue: amber `DIALOG` and dim `stale`, always beside the word. Nothing redraws in place and there are no spinners. |

### iconography#1.3 · listed (lost at strong) · .andromeda/design-system-detail/iconography.md:10
- (.andromeda/design-system-detail/iconography.md:10) A server-verification mismatch, or a check that cannot be made, refused by CLI `send` / `wait` / `last` / `answer` / `pause` / `release` must print its human refusal in ASCII only, its reason and its detail separate fields set two spaces apart, the same form the existing liveness pre-check's exit-21 refusal takes.
  > The CLI is ASCII only:
  >   - Two spaces separate columns, and a refusal's reason from its detail (they are separate fields, as in the exit-code table).

### iconography#1.3 · listed (lost at strong) · .andromeda/design-system-detail/iconography.md:12
- (.andromeda/design-system-detail/iconography.md:12) Any label or field the chunk adds to a refusal or hint line that would need a separator inside one field must use ` - `, never `·`.
  > ` - ` (space, hyphen, space) replaces `·` inside one field or label

## Held

- surface#2.1
- surface#5.1
- surface#9.10

## Cascade only
