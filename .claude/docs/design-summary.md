# Design Summary — viola

_Distilled from `.andromeda/design-system.md` + `.andromeda/layout-templates.md` by `/andromeda-setup-project`. wrap-session does not modify._

## Brand identity

**"Strip-bay handoff, read back."** The page is an air-traffic-control tower strip bay after dark. Buff paper strips sit in anthracite holder boots on ruled aluminium racks, and every session keeps its slot for life. The only warm light is the one amber arrival holder cocked out of line when a session needs you. The voice is standard phraseology: short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`), no filler, no guessing. Nothing announces success before it is confirmed: a send stays `open` until the matching `prompt-submitted` reads it back (for `/clear`, until its new-session post-condition does). Missing readings print `unknown`, and old information carries its age (`read 4m ago`, `expired`). It is explicitly NOT a monitoring dashboard: no status dots, toasts, graphs, spinners or glow.

**Expression:** base 0.2 · web-spa 0.3 (exactly two motions) · CLI TTY 0.2 · `--json`/non-TTY/`NO_COLOR`/`TERM=dumb`/`viola run` and `viola revive` passthrough 0.0.

**Signature:** the readback box — a 16px ruled square at the far right of every tape send line and every outbound transfer marker once a send on its link has been seen since page open (before that the marker prints only the word `unknown`, no box): `open` (outline) · `RB` / `read back` (solid buff fill, graphite `RB`) · `unable` (1px `/` strike + reason) · `unconfirmable` (open drawing, never filled). CLI mirror `[RB]` / `[  ]` / `[/ ]`. Supporting element: the **cocked strip** (`dialog_pending`: 12px translate over 160ms, once, amber 4px band, `DIALOG <kind>`).

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
| Role (handoff) | `#5B8DB8` | transfer markers ONLY |

Contrast rule: lamp-off, amber and blue fail AA as text on holder → they live only on anthracite (captions in the rack header, cocked DIALOG cell inset, markers in the rack gap, stale strips drop to anthracite).

### Typography

- **Labels / headings / callsign:** `--font-label: "Bahnschrift", "DIN Alternate", "Avenir Next Condensed", "DejaVu Sans Condensed", sans-serif` (DIN; semi-condensed via `font-stretch: 87.5%`)
- **Fields / tape / data:** `--font-field: "Cascadia Mono", Consolas, "SF Mono", Menlo, "DejaVu Sans Mono", monospace` (no ligatures, tabular figures)
- Sizes: display 16 · callsign 15 · field 13 · code 12 · label 11 (px); weights 400 / 600. No web fonts (`font-src 'none'`). **Every contrast and render assertion must hold on the resolved Linux DejaVu families**; 0.1.0 renders no page, so no CI step renders these stacks or provides their families (how a 0.2.0 frontend checks them is that version's to decide).

### Spacing

- **Base unit:** 4px — `space-micro` 4 · `xs` 8 · `sm` 12 · `md` 16 · `lg` 24 · `xl` 32; geometry `--strip-h` 32 · `--line-h` 20 · `--band-w` 4 · `--rb-size` 16; `ch`-sized tracks `--strip-cols`, `--strip-cols-wrapped`, `--tape-cols`.

### Motion

- **Two motions only:** the cock (`--cock-offset` 12px over `--cock-dur` 160ms, ease-out, once) and the opacity fade of a new tape line or transfer marker (`--fade-dur`, ≤120ms). The readback fill, the strike, `stale` dimming and a strip's return into line are instant.

### Depth & shape
- Borders-only (`--shadow: none`); radius 0 everywhere; one gradient (the refused strike).

## Primary surfaces

- **web-spa** — deferred to 0.2.0; its technology is not chosen, and 0.1.0 renders no page (the colour, type, spacing, depth and radius sections stand as its design input)
- **CLI** — `viola list` (the k9s-style board, six fixed columns and words a future page reuses; as built: wrapped rows only, uncoloured on every stream, STATUS always `unknown`, and a failed strict-modes check refuses the whole board with exit 21), the `viola send` readback mirror, phraseology verb output, `viola verify` step counter, the `viola run` / `viola revive` passthrough (nothing printed while the child runs; `viola revive <name> --list` prints one static line per logged session)

## Component patterns (web-spa: a 0.2.0 design input; 0.1.0 renders no page)

- The flight strip — one per `viola run` instance, fixed fields NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, stable slot, never blank (`unknown` / `n/a`)
- `<viola-readback>` — the signature box, the same element and drawing on a tape send line and an outbound transfer marker
- `<viola-transfer>` — outbound `→ builder  since …` and inbound `← overseer` in departure blue, in the rack gap on anthracite
- `<viola-event-feed>` — the tower tape: the ndjson tape played in order, one line per event, `skipped` counts kept where they happened
- `<viola-atis>` — sticky header: `BAY`, `5H`, `7D`, `read … ago`, gate, `TAPE`, `skipped`
- Access/error strips and the reserved v1.x brake buttons and Raised-3 confirmation (not built in v1)

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
