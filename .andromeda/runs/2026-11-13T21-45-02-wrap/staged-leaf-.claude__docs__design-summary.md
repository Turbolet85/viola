# Design Summary — viola

_Distilled from `.andromeda/design-system.md` + `.andromeda/layout-templates.md` by `/andromeda-setup-project`. wrap-session does not modify._

## Brand identity

**"Strip-bay handoff, read back."** The direction the CLI carries, and the terminal fleet console (matrix#v2-14) inherits, is an air-traffic-control tower strip bay after dark. Buff paper strips sit in anthracite holder boots on ruled aluminium racks, and every session keeps its slot for life. The only warm light is the one amber arrival holder cocked out of line when a session needs you. The voice is standard phraseology: short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`), no filler, no guessing. Nothing announces success before it is confirmed: a send stays `open` until the matching `prompt-submitted` reads it back (on a verified CLI, for `/clear` until its new-session post-condition does and for `/compact` until its same-session compact post-condition does; on an unverified CLI both `/clear` and `/compact` stay `unconfirmable`). Missing readings print `unknown`, and old information carries its age (`read 4m ago`, `expired`). It is explicitly NOT a monitoring dashboard: no status dots, toasts, graphs, spinners or glow.

**Expression:** base 0.2 · frontend N/A (viola has no web or graphical frontend) · CLI TTY 0.2 · `--json`/non-TTY/`NO_COLOR`/`TERM=dumb`/`viola run` and `viola revive` passthrough 0.0.

**Signature:** the readback box — a fixed-width 1px-ruled square cell whose one place is the CLI mirror: `open` (outline) · `RB` / `read back` (solid buff fill, graphite `RB`) · `unable` (1px `/` strike + reason) · `unconfirmable` (open drawing, never filled). CLI mirror `[RB]` / `[  ]` / `[/ ]`. Supporting element: the **cocked strip** (`dialog_pending`: the row keeps its slot and shows `DIALOG <kind>`, the word amber on the CLI).

## Key design tokens (viola renders no page; the cli's ink, type and spacing are the terminal's)

### Color palette (the only eight hex values)

| Role | Value | Use |
|---|---|---|
| Surface (ladder 1) | `#1E2124` | first of three surfaces (no surface scale: viola renders no page) |
| Surface (ladder 2) | `#2A2E33` | second of three surfaces (no surface scale: viola renders no page) |
| Primary | `#E6D8AE` | primary ink, state borders, read-back fill, focus |
| Secondary | `#9C9278` | secondary ink on anthracite, field grid, stale ink |
| Ink (graphite) | `#3B3A36` | the third ink (no text hierarchy: viola renders no page) |
| Border Subtle | `#4A5057` | decorative rules only (2.0:1) |
| Accent | `#D97706` | `dialog_pending` ONLY |
| Role (handoff) | `#5B8DB8` | transfer markers and `link` / `unlink` names ONLY |

CLI colour: amber on the `DIALOG` word only, SGR dim on a whole `stale` row, bold for NAME; handoff blue is never used and backgrounds are never set.

### Typography

- **No type tokens:** viola renders no page; the cli's type is the terminal's.
- **cli:** terminal monospace (the user's configured font); the brand shows through the fixed-column structure and the phraseology words.
- Bold only for the NAME column, SGR dim for `stale` rows, amber for the `DIALOG` word; ASCII only, high density. No font stacks or render assertions: viola renders no page.

### Spacing

- No spacing tokens (viola renders no page): CLI spacing is platform-controlled, a fixed-width terminal grid, two spaces between columns.

### Motion

- **Calibrated to the base expression 0.2:** the cli (0.2 on a TTY, 0.0 otherwise) has no motion at all, and viola renders no page.

### Depth & shape
- No depth or radius tokens (viola renders no page); the cli draws no depth, and square brackets `[ ]` keep the ruled-rectangle reading.

## Primary surfaces

- **web-spa** — no web surface: viola renders no page; the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14) takes its own surface section at the route entry «Console scaffold»
- **CLI** — `viola list` (the k9s-style board, six fixed columns and words the terminal fleet console (matrix#v2-14) reuses; as built: uncoloured under `--json`, a piped stdout, `NO_COLOR` (an empty value included) or `TERM=dumb`, else NAME bold, the `DIALOG` word amber with its kind word plain and a wrapped `stale` row dim end to end; on a terminal whose width is known (`viola_pty::host_size()`, never `COLUMNS`) only an unwrapped NAME too long for its column is cut with `...`; the wrapped rows, then, when one `claude agents --json` read lists a session no wrapped row joins, the `-- UNWRAPPED - READ-ONLY --` line and one row per unwrapped session (NAME its `sessionId`, `n/a` in WHEEL, DIALOG and CLI); STATUS `idle` / `busy` on a row the read joins, else `unknown`; a failed read prints the board as with no `claude`; and a failed strict-modes check refuses the whole board with exit 21, or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`), the `viola send` readback mirror (an unusable `viola send` and an oversize `viola answer` instead exit 2 with their fixed stderr line, `UNUSABLE_TEXT` / `UNUSABLE_RESPONSE`, whatever `--json` says, and no document; a failed `cli` verb's `error: internal error`, exit 1, is with `--json` among the arguments before a `--` the document `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, while `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site), phraseology verb output (`viola plugin install`: four plain-ASCII stdout lines in one write, no SGR, `viola@viola  written  <dir>` / `run these to add it to Claude Code:` / `  claude plugin marketplace add <dir>` / `  claude plugin install viola@viola`, `<dir>` quoted only where the host's shell would split it; under `--json` `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`; a refused strict-modes check is exit 21 with `viola list`'s home pair on stderr, or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty), `viola wait` / `viola last` (one static result line on stdout, no spinner; a `wait` woken by an expired dialog hold prints `dialog-expired  <name>  dialog <id>  cursor <n>`, ASCII, no SGR, written once), `viola link` / `viola unlink` (one plain-ASCII stdout line in one write, no SGR, exit 0: `link    <driver> -> <driven>  since <HH:MM:SSZ>`, `link    <driver> -> <driven>  already linked` for a held pair, `unlink  <driver> -> <driven>` whether or not a pair was removed; under `--json` `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}` / `{"v":1,"ok":{}}`; a refused strict-modes check of the driven instance is exit 21 with `revive`'s instance pair on stderr naming it, and so is the one `viola send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance before any frame (that pair, not the `[/ ]` mirror; no frame and no line written), or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty (exit 21's document is `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`)), `viola verify` step counter, the `viola run` / `viola revive` passthrough (nothing printed while the child runs; `viola revive <name> --list` prints one static line per logged session; under `--json` both arms print one stdout document: `--list`'s `{"v":1,"ok":{"sessions":[…]}}`, every refusal of the start it shares with `run`, `{"v":1,"error":"<code>","detail":null}` with exit 1, no hint and stderr empty, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`; a `pump_child` error prints none, the child having held the terminal)

## Component patterns (viola renders no page; the CLI carries them)

- The flight strip — one per `viola run` instance, fixed fields NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, stable slot, never blank (`unknown` / `n/a`)
- The readback box — the signature; its one place the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`
- Transfer markers — only `link` / `unlink` verb output (`overseer -> builder`), plain, ASCII `->`, no inbound form; `viola list` has no link column
- The tower tape — the ndjson tape in `events.ndjson`; the CLI has no tape, and terminal scrollback is the history
- The BAY line — the one static header, atop `viola list`: `BAY`, `5H`, `7D`, `read … ago`, gate, `skipped` (no TAPE cell)
- Refusal lines — `unable  <name>  <reason>  <detail>` on stderr, then the cause's `hint:` line last (`send` and `answer` print the same hint for the same reason or detail; a `not-delivered` refusal with an opaque detail or none prints no hint); the exit-1 refusals of `run`, `revive` and `verify` and every exit-21 strict-modes refusal use the fixed `unable: <text>` / `hint:` pair instead

## Universal bans

- NEVER use a ninth hex value: all eight come from the Color World, and only two are hues.
- NEVER use colour for decoration: amber = dialog pending, blue = handoff, nothing else.
- NEVER use status dots, toasts, spinners, skeleton shimmer or budget rings.
- NEVER auto-sort strips by state — slots are stable.
- NEVER use glow or a terminal-green / neon "hacker" theme; the cli draws no depth.
- A refused send folds into its send line; `turn-ended` never fills a readback box; a nonzero `skipped` count stays visible; a bare strip is never cocked nor a target.
- CLI: colour only on `DIALOG` (amber), `stale` rows (dim) and NAME (bold), always beside the word; no ✓/✗, no emoji, no spinner, ASCII only.

---

**Full plans:** `.andromeda/design-system.md` + `.andromeda/layout-templates.md`.
