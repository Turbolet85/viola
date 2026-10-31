# Design Summary — viola

_Distilled from `.andromeda/design-system.md` + `.andromeda/layout-templates.md` by `/andromeda-setup-project`. wrap-session does not modify._

## Brand identity

**"Strip-bay handoff, read back."** The direction a 0.2.0 frontend inherits (0.1.0 renders no page) is an air-traffic-control tower strip bay after dark. Buff paper strips sit in anthracite holder boots on ruled aluminium racks, and every session keeps its slot for life. The only warm light is the one amber arrival holder cocked out of line when a session needs you. The voice is standard phraseology: short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`), no filler, no guessing. Nothing announces success before it is confirmed: a send stays `open` until the matching `prompt-submitted` reads it back (for `/clear`, until its new-session post-condition does; for `/compact` on a verified CLI, until its same-session compact post-condition does, and on an unverified CLI `/compact` stays `unconfirmable`). Missing readings print `unknown`, and old information carries its age (`read 4m ago`, `expired`). It is explicitly NOT a monitoring dashboard: no status dots, toasts, graphs, spinners or glow.

**Expression:** base 0.2 · frontend N/A (deferred to 0.2.0, its technology not chosen) · CLI TTY 0.2 · `--json`/non-TTY/`NO_COLOR`/`TERM=dumb`/`viola run` and `viola revive` passthrough 0.0.

**Signature:** the readback box — a 16px 1px-ruled square cell whose one 0.1.0 place is the CLI mirror (where a 0.2.0 frontend draws it is that version's to name, its technology not chosen): `open` (outline) · `RB` / `read back` (solid buff fill, graphite `RB`) · `unable` (1px `/` strike + reason) · `unconfirmable` (open drawing, never filled). CLI mirror `[RB]` / `[  ]` / `[/ ]`. Supporting element: the **cocked strip** (`dialog_pending`: the row keeps its slot and shows `DIALOG <kind>`, the word amber on the CLI; its 12px / 160ms translate and 4px amber holder band are a 0.2.0 frontend design input).

## Key design tokens (a 0.2.0 frontend design input; 0.1.0 renders no page)

### Color palette (the only eight hex values)

| Role | Value | Use |
|---|---|---|
| Surface Base / Inset | `#1E2124` | page, rack gap, tape, stale + unwrapped strips, inset cells |
| Surface Raised-1 | `#2A2E33` | a lit wrapped strip (buff ink only) |
| Primary | `#E6D8AE` | primary ink, state borders, read-back fill, focus |
| Secondary | `#9C9278` | secondary ink on anthracite, field grid, stale ink |
| On-paper (Text Hierarchy) | `#3B3A36` | text on a buff fill only (`RB`) |
| Border Subtle | `#4A5057` | decorative rules only (2.0:1) |
| Accent | `#D97706` | `dialog_pending` ONLY |
| Role (handoff) | `#5B8DB8` | transfer markers and `link` / `unlink` names ONLY |

Contrast rule: lamp-off, amber and blue fail AA as text on holder → they live only on anthracite (captions in the rack header, cocked DIALOG cell inset, markers in the rack gap, stale strips drop to anthracite).

### Typography

- **Labels / headings / callsign:** `--font-label: "Bahnschrift", "DIN Alternate", "Avenir Next Condensed", "DejaVu Sans Condensed", sans-serif` (DIN; labels and headings semi-condensed via `font-stretch: 87.5%`, the callsign at normal width and never text-transformed)
- **Fields / tape / data:** `--font-field: "Cascadia Mono", Consolas, "SF Mono", Menlo, "DejaVu Sans Mono", monospace` (no ligatures, tabular figures)
- Sizes: display 16 · callsign 15 · field 13 · code 12 · label 11 (px); weights 400 / 600. No web fonts (`font-src 'none'`). **Every contrast and render assertion must hold on the resolved Linux DejaVu families**; 0.1.0 renders no page, so no CI step renders these stacks or provides their families (how a 0.2.0 frontend checks them is that version's to decide).

### Spacing

- **Base unit:** 4px — `space-micro` 4 · `xs` 8 · `sm` 12 · `md` 16 · `lg` 24 · `xl` 32; geometry `--strip-h` 32 · `--line-h` 20 · `--band-w` 4 · `--rb-size` 16; `ch`-sized tracks `--strip-cols`, `--strip-cols-wrapped`, `--tape-cols`.

### Motion

- **Calibrated to the base expression 0.2:** the 0.0–0.2 duration row applies (0–100ms, no entrance), and the cli (0.2 on a TTY, 0.0 otherwise) has no motion at all; the cocked strip's 12px / 160ms translate is a 0.2.0 frontend design input.

### Depth & shape
- Borders-only (`--shadow: none`); radius 0 everywhere; one gradient (the refused strike).

## Primary surfaces

- **web-spa** — deferred to 0.2.0; its technology is not chosen, and 0.1.0 renders no page (the colour, type, spacing, depth and radius sections stand as its design input)
- **CLI** — `viola list` (the k9s-style board, six fixed columns and words a future page reuses; as built: uncoloured under `--json`, a piped stdout, `NO_COLOR` (an empty value included), `TERM=dumb` or a failed Windows VT enable, else NAME bold, the `DIALOG` word amber with its kind word plain and a wrapped `stale` row dim end to end; on a terminal whose width is known (`viola_pty::host_size()`, never `COLUMNS`) only an unwrapped NAME too long for its column is cut with `...`; the wrapped rows, then, when one `claude agents --json` read lists a session no wrapped row joins, the `-- UNWRAPPED - READ-ONLY --` line and one row per unwrapped session (NAME its `sessionId`, `n/a` in WHEEL, DIALOG and CLI); STATUS `idle` / `busy` on a row the read joins, else `unknown`; a failed read prints the board as with no `claude`; and a failed strict-modes check refuses the whole board with exit 21, or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`), the `viola send` readback mirror (an unusable `viola send` and an oversize `viola answer` instead exit 2 with their fixed stderr line, `UNUSABLE_TEXT` / `UNUSABLE_RESPONSE`, whatever `--json` says, and no document; a failed `cli` verb's `error: internal error`, exit 1, is with `--json` among the arguments before a `--` the document `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, while `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site), phraseology verb output (`viola plugin install`: four plain-ASCII stdout lines in one write, no SGR, `viola@viola  written  <dir>` / `run these to add it to Claude Code:` / `  claude plugin marketplace add <dir>` / `  claude plugin install viola@viola`, `<dir>` quoted only where the host's shell would split it; under `--json` `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`; a refused strict-modes check is exit 21 with `viola list`'s home pair on stderr, or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty), `viola wait` / `viola last` (one static result line on stdout, no spinner; a `wait` woken by an expired dialog hold prints `dialog-expired  <name>  dialog <id>  cursor <n>`, ASCII, no SGR, written once), `viola link` / `viola unlink` (one plain-ASCII stdout line in one write, no SGR, exit 0: `link    <driver> -> <driven>  since <HH:MM:SSZ>`, `link    <driver> -> <driven>  already linked` for a held pair, `unlink  <driver> -> <driven>` whether or not a pair was removed; under `--json` `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}` / `{"v":1,"ok":{}}`; a refused strict-modes check of the driven instance is exit 21 with `revive`'s instance pair on stderr naming it, and so is the one `viola send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance before any frame (that pair, not the `[/ ]` mirror; no frame and no line written), or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty (exit 21's document is `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`)), `viola verify` step counter, the `viola run` / `viola revive` passthrough (nothing printed while the child runs; `viola revive <name> --list` prints one static line per logged session; under `--json` both arms print one stdout document: `--list`'s `{"v":1,"ok":{"sessions":[…]}}`, every refusal of the start it shares with `run`, `{"v":1,"error":"<code>","detail":null}` with exit 1, no hint and stderr empty, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`; a `pump_child` error prints none, the child having held the terminal)

## Component patterns (web-spa: a 0.2.0 design input; 0.1.0 renders no page)

- The flight strip — one per `viola run` instance, fixed fields NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, stable slot, never blank (`unknown` / `n/a`)
- The readback box — the signature; in 0.1.0 only the CLI `viola send` mirror, its 0.2.0 places that version's to name
- Transfer markers — outbound `→ builder  since …` and inbound `← overseer` in departure blue, in the rack gap on anthracite
- The tower tape — the ndjson tape played in order, one line per event, a refused send folded into its send line and a matched driver prompt into its send's readback, `skipped` counts kept where they happened
- The ATIS header — sticky: `BAY`, `5H`, `7D`, `read … ago`, gate, `TAPE`, `skipped`
- The access refusal, error strips and the reserved v1.x brake buttons and Raised-3 confirmation (not built in v1)

## Universal bans

- NEVER use a ninth hex value; the font stacks end at the generic `sans-serif` / `monospace`, never `system-ui`.
- NEVER use colour for decoration: amber = dialog pending, blue = handoff, nothing else.
- NEVER use status dots, toasts, spinners, skeleton shimmer or budget rings.
- NEVER auto-sort strips by state — slots are stable.
- NEVER use shadows, glow, gradients (except the strike) or a radius other than 0.
- Tape event text prints as plain text (the domain status rows).
- CLI: colour only on `DIALOG` (amber), `stale` rows (dim) and NAME (bold), always beside the word; no ✓/✗, no emoji, no spinner, ASCII only.

---

**Full plans:** `.andromeda/design-system.md` + `.andromeda/layout-templates.md`.
