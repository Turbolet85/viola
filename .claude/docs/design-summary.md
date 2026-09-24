# Design Summary — viola

_Distilled from `.andromeda/design-system.md` + `.andromeda/layout-templates.md` by `/andromeda-setup-project`. wrap-session does not modify._

## Brand identity

**"Strip-bay handoff, read back."** The page is an air-traffic-control tower strip bay after dark. Buff paper strips sit in anthracite holder boots on ruled aluminium racks, and every session keeps its slot for life. The only warm light is the one amber arrival holder cocked out of line when a session needs you. The voice is standard phraseology: short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`), no filler, no guessing. Nothing announces success before it is confirmed: a send stays `open` until the matching `prompt-submitted` reads it back. Missing readings print `unknown`, and old information carries its age (`read 4m ago`, `expired`). It is explicitly NOT a monitoring dashboard: no status dots, toasts, graphs, spinners, glow or chat bubbles.

**Expression:** base 0.2 · web-spa 0.3 (exactly two motions) · CLI TTY 0.2 · `--json`/non-TTY/`viola run` 0.0. Dark only.

**Signature:** the readback box — a 16px ruled square at the far right of every tape send line and every outbound transfer marker: `open` (outline) · `RB` / `read back` (solid buff fill, graphite `RB`) · `unable` (1px `/` strike + reason) · `unconfirmable` (open drawing, never filled). CLI mirror `[RB]` / `[  ]` / `[/ ]`. Supporting element: the **cocked strip** (`dialog_pending`: 12px translate over 160ms, once, amber 4px band, `DIALOG <kind>`).

## Key design tokens

### Color palette (the only eight hex values)

| Token | Value | Use |
|---|---|---|
| `--c-anthracite` → `--surface-bay` / `--surface-inset` | `#1E2124` | page, rack gap, tape, stale + unwrapped strips, inset cells |
| `--c-holder` → `--surface-strip` | `#2A2E33` | a lit wrapped strip (buff ink only) |
| `--c-buff` → `--ink`, `--rule-info`, `--rb-fill`, `--focus-ring` | `#E6D8AE` | primary ink, state borders, read-back fill, focus |
| `--c-lampoff` → `--ink-dim`, `--rule-field` | `#9C9278` | secondary ink on anthracite, field grid, stale ink |
| `--c-graphite` → `--ink-on-paper`, `--rb-ink` | `#3B3A36` | text on a buff fill only (`RB`) |
| `--c-rail` → `--rule-deco` | `#4A5057` | decorative rules only (2.0:1) |
| `--c-amber` → `--attention` | `#D97706` | `dialog_pending` ONLY |
| `--c-departure` → `--handoff` | `#5B8DB8` | transfer markers ONLY |

Contrast rule: lamp-off, amber and blue fail AA as text on holder → they live only on anthracite (captions in the rack header, cocked DIALOG cell inset, markers in the rack gap, stale strips drop to anthracite).

### Typography

- **Labels / headings / callsign:** `--font-label: "Bahnschrift", "DIN Alternate", "Avenir Next Condensed", "DejaVu Sans Condensed", sans-serif` (DIN; semi-condensed via `font-stretch: 87.5%`)
- **Fields / tape / data:** `--font-field: "Cascadia Mono", Consolas, "SF Mono", Menlo, "DejaVu Sans Mono", monospace` (no ligatures, tabular figures)
- Sizes: display 16 · callsign 15 · field 13 · code 12 · label 11 (px); weights 400 / 600. No web fonts (`font-src 'none'`). **The Linux DejaVu render is what CI asserts.**

### Spacing

- **Base unit:** 4px — `space-micro` 4 · `xs` 8 · `sm` 12 · `md` 16 · `lg` 24 · `xl` 32; geometry `--strip-h` 32 · `--line-h` 20 · `--band-w` 4 · `--rb-size` 16; `ch`-sized tracks `--strip-cols`, `--strip-cols-wrapped`, `--tape-cols`.

### Motion

- **Two motions only:** the cock (`--cock-offset` 12px, `--cock-dur` 160ms, `cubic-bezier(0.2,0,0,1)`, into pending only) and the `@starting-style` fade on `data-live` tape lines / markers (`--fade-dur` 120ms linear). Everything else is instant.
- **Reduce-motion override:** `prefers-reduced-motion: reduce` sets `--cock-dur` / `--fade-dur` to 0ms; the offset and band stay (they are state).

### Depth & shape
- Borders-only (`--shadow: none`); radius 0 everywhere; one gradient (the refused strike).

## Primary surfaces

- **Bay, steady state (≥1024px)** — sticky `<viola-atis>` header · WRAPPED rack with transfer markers · UNWRAPPED · READ-ONLY rack · the TAPE (the only bounded scroll region)
- **Bay, narrow (760–1023px)** — strips wrap to two lines; the ATIS takes 3+ lines
- **Bay, first reading / empty** — `sessions: no reading yet`, `unknown` in every ATIS box, the empty-rack command in Code type
- **Bay, degraded** — `TAPE stopped · viola ui not answering`, 503 `unable · state-unreadable`, the 401 access strip; last values kept
- **Tape line, expanded** — native `<details>` body, full text as pre-wrapped plain text
- **CLI** — `viola list` (the k9s-style board, same six columns and words), the `viola send` readback mirror, phraseology verb output, `viola verify` step counter

## Component patterns

- `<viola-session-row>` — the flight strip: six fixed fields NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, stable slot by name, never blank (`unknown` / `n/a`), native `<tr>` under a `display: contents` host
- `<viola-readback>` — the signature box, identical drawing in tape and marker, flips in the same frame
- `<viola-transfer>` — `→ builder  since …` (blue, rack gap) under the driver, `← overseer` twin under the driven strip; no node graph
- `<viola-event-feed>` — the tower tape: native `<details>` lines, oldest on top, 2000-line cap, follows only at the bottom, never announced
- `<viola-atis>` — sticky header of `<dt>/<dd>` cells: `BAY`, `5H`, `7D`, `read … ago`, gate, `TAPE`, `skipped`
- Access/error strips and reserved v1.x brake buttons (`I HAVE CONTROL`, `UNLINK` — not built in v1)

## Universal bans

- NEVER use generic families (Inter, Roboto, Arial, Helvetica, `system-ui`), Tailwind/Bootstrap palette values, or a ninth hex value.
- NEVER use colour for decoration: amber = dialog pending, blue = handoff, nothing else.
- NEVER use card grids, status dots, toasts, spinners, skeletons, "Loading…", chat bubbles, node graphs or budget rings.
- NEVER auto-sort strips by state — slots are stable.
- NEVER use shadows, glow, gradients (except the strike) or a radius other than 0.
- NEVER render event text as HTML or Markdown; NEVER show the token, URL, `?t=` or `viola_home`.
- CLI: colour only on `DIALOG` (amber), `stale` rows (dim) and NAME (bold), always beside the word; no ✓/✗, no emoji, no spinner, ASCII only.

---

**Full plans:** `.andromeda/design-system.md` + `.andromeda/layout-templates.md`.
