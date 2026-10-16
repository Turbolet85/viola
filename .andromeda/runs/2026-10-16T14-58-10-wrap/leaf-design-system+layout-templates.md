group: design-system+layout-templates
read: .claude/docs/design-summary.md · 78 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:7   class: retired
  leaf-text: «no status dots, toasts, graphs, spinners, glow or chat bubbles.»
  source: .andromeda/design-system.md:541 «**NEVER converge on common "safe" choices across generations.** The readback box and the cocked strip exist because agent dashboards converge on dots, toasts and graphs.»
  replace-with: «no status dots, toasts, graphs, spinners or glow.»

- leaf: .claude/docs/design-summary.md:9   class: retired
  leaf-text: «passthrough 0.0. Dark only.»
  source: .andromeda/design-system.md:374 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «passthrough 0.0.»

- leaf: .claude/docs/design-summary.md:13   class: omission
  leaf-text: «## Key design tokens»
  source: .andromeda/design-system.md:84 «A 0.2.0 frontend design input; 0.1.0 renders no page.»
  replace-with: «## Key design tokens (a 0.2.0 frontend design input; 0.1.0 renders no page)»

- leaf: .claude/docs/design-summary.md:17   class: retired
  leaf-text: «| Token | Value | Use |»
  source: .andromeda/design-system.md:75 «| Role | Value | Usage | Domain anchor |»
  replace-with: «| Role | Value | Use |»

- leaf: .claude/docs/design-summary.md:19   class: retired
  leaf-text: «`--c-anthracite` → `--surface-bay` / `--surface-inset`»
  source: .andromeda/design-system.md:88 «| Base | #1E2124 | Page background (the bay), the rack gap, the tape, and bare (unwrapped) and `stale` strips. Console anthracite: a lit near-black, not a void. |»
  replace-with: «Surface Base / Inset»

- leaf: .claude/docs/design-summary.md:20   class: retired
  leaf-text: «`--c-holder` → `--surface-strip`»
  source: .andromeda/design-system.md:89 «| Raised-1 | #2A2E33 | A lit wrapped session strip: the holder boot. It carries buff ink only. |»
  replace-with: «Surface Raised-1»

- leaf: .claude/docs/design-summary.md:21   class: retired
  leaf-text: «`--c-buff` → `--ink`, `--rule-info`, `--rb-fill`, `--focus-ring`»
  source: .andromeda/design-system.md:77 «| Primary | #E6D8AE | Primary ink, focus rings, information-carrying borders (every Border Progression Emphasis use: the readback box outline and strike, the `budget-paused`, nonzero `skipped`»
  replace-with: «Primary»

- leaf: .claude/docs/design-summary.md:22   class: retired
  leaf-text: «`--c-lampoff` → `--ink-dim`, `--rule-field`»
  source: .andromeda/design-system.md:78 «| Secondary | #9C9278 | Secondary ink on anthracite (timestamps, reading ages, column captions, `n/a`), the printed field-grid rule inside strips»
  replace-with: «Secondary»

- leaf: .claude/docs/design-summary.md:23   class: retired
  leaf-text: «`--c-graphite` → `--ink-on-paper`, `--rb-ink`»
  source: .andromeda/design-system.md:104 «| On-paper | #3B3A36 | Graphite pencil: text on a buff fill only (the `RB` in a read-back box) |»
  replace-with: «On-paper (Text Hierarchy)»

- leaf: .claude/docs/design-summary.md:24   class: retired
  leaf-text: «`--c-rail` → `--rule-deco`»
  source: .andromeda/design-system.md:225 «| Subtle | #4A5057 | Rack-rail aluminium: rack separators, the strip outer edge, the band slot of a non-cocked strip, the dashed outline of bare strips»
  replace-with: «Border Subtle»

- leaf: .claude/docs/design-summary.md:25   class: retired
  leaf-text: «`--c-amber` → `--attention`»
  source: .andromeda/design-system.md:79 «| Accent | #D97706 | The only attention colour: the 4px holder band and the `DIALOG` word of a cocked strip, and the CLI `DIALOG` word in `viola list`»
  replace-with: «Accent»

- leaf: .claude/docs/design-summary.md:26   class: retired
  leaf-text: «`--c-departure` → `--handoff`»
  source: .andromeda/design-system.md:80 «| Role (handoff) | #5B8DB8 | Transfer-marker arrows and names (outbound `→ builder`, inbound `← overseer`), the marker's 2px left tick (`--rule-strong`, the only blue rule)»
  replace-with: «Role (handoff)»

- leaf: .claude/docs/design-summary.md:34   class: omission
  leaf-text: «**The Linux DejaVu render is what CI asserts.**»
  source: .andromeda/design-system.md:271 «The browser test leg named here (the headless GUI render and contrast checks judged on the ubuntu CI leg, and the DejaVu families the CI image provides) stands until the route entry "Remove the browser leg"»
  replace-with: «**The Linux DejaVu render is what CI asserts** (until the route entry "Remove the browser leg" removes the browser test leg).»

- leaf: .claude/docs/design-summary.md:42   class: retired
  leaf-text: «- **Two motions only:** the cock (`--cock-offset` 12px, `--cock-dur` 160ms, `cubic-bezier(0.2,0,0,1)`, into pending only) and the `@starting-style` fade on `data-live` tape lines / markers (`--fade-dur` 120ms linear). Everything else is instant.»
  source: .andromeda/design-system.md:41 «Exactly two motions, each played once on a state change. (1) The cocked-strip `translate` (12px / 160ms ease-out). (2) A new tape line or new transfer marker fades in (opacity, ≤120ms).»
  replace-with: «- **Two motions only:** the cock (`--cock-offset` 12px over `--cock-dur` 160ms, ease-out, once) and the opacity fade of a new tape line or transfer marker (`--fade-dur`, ≤120ms). The readback fill, the strike, `stale` dimming and a strip's return into line are instant.»

- leaf: .claude/docs/design-summary.md:43   class: retired
  leaf-text: «- **Reduce-motion override:** `prefers-reduced-motion: reduce` sets `--cock-dur` / `--fade-dur` to 0ms; the offset and band stay (they are state).»
  source: .andromeda/design-system.md:339 «All motion decisions flow from the expression level set in Brand Identity.»
  replace-with: «»

- leaf: .claude/docs/design-summary.md:50   class: retired
  leaf-text: «- **Bay, steady state (≥1024px)** — sticky `<viola-atis>` header · WRAPPED rack with transfer markers · UNWRAPPED · READ-ONLY rack · the TAPE (the only bounded scroll region)»
  source: .andromeda/layout-templates.md:17 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «- **web-spa** — deferred to 0.2.0; its technology is not chosen, and 0.1.0 renders no page (the colour, type, spacing, depth and radius sections stand as its design input)»

- leaf: .claude/docs/design-summary.md:51   class: retired
  leaf-text: «- **Bay, narrow (760–1023px)** — strips wrap to two lines; the ATIS takes 3+ lines»
  source: .andromeda/layout-templates.md:17 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «»

- leaf: .claude/docs/design-summary.md:52   class: retired
  leaf-text: «- **Bay, first reading / empty** — `sessions: no reading yet`, `unknown` in every ATIS box, the empty-rack command in Code type»
  source: .andromeda/layout-templates.md:17 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «»

- leaf: .claude/docs/design-summary.md:53   class: retired
  leaf-text: «- **Bay, degraded** — `TAPE stopped · viola ui not answering`, 503 `unable · state-unreadable`, the 401 access strip; last values kept»
  source: .andromeda/layout-templates.md:17 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «»

- leaf: .claude/docs/design-summary.md:54   class: retired
  leaf-text: «- **Tape line, expanded** — native `<details>` body, full text as pre-wrapped plain text»
  source: .andromeda/layout-templates.md:17 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «»

- leaf: .claude/docs/design-summary.md:55   class: stale
  leaf-text: «the k9s-style board, same six columns and words;»
  source: .andromeda/layout-templates.md:310 «  - The six captions, words and slot order are fixed, and a future page reuses them.»
  replace-with: «the k9s-style board, six fixed columns and words a future page reuses;»

- leaf: .claude/docs/design-summary.md:57   class: omission
  leaf-text: «## Component patterns»
  source: .andromeda/design-system.md:374 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «## Component patterns (web-spa: a 0.2.0 design input; 0.1.0 renders no page)»

- leaf: .claude/docs/design-summary.md:59   class: retired
  leaf-text: «- `<viola-session-row>` — the flight strip: six fixed fields NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, stable slot by name, never blank (`unknown` / `n/a`), native `<tr>` with `<td>` cells, any element between `<tbody>` and it role-less `display: contents` (how a `viola-*` name maps onto React's output is OPEN, owned by the frontend-toolchain entry)»
  source: .andromeda/design-system.md:8 «- **Flight-progress strip.** One strip per `viola run` instance. The fields are fixed, in the order NAME · LIVE · STATUS · WHEEL · DIALOG · CLI. The name cell is set at callsign weight.»
  replace-with: «- The flight strip — one per `viola run` instance, fixed fields NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, stable slot, never blank (`unknown` / `n/a`)»

- leaf: .claude/docs/design-summary.md:60   class: retired
  leaf-text: «identical drawing in tape and marker, flips in the same frame»
  source: .andromeda/design-system.md:581 «On the web, places 1 and 2 must render the same `<viola-readback>` element under the same `.rb` rule»
  replace-with: «the same element and drawing on a tape send line and an outbound transfer marker»

- leaf: .claude/docs/design-summary.md:61   class: retired
  leaf-text: «- `<viola-transfer>` — `→ builder  since …` (blue, rack gap) under the driver, `← overseer` twin under the driven strip; no node graph»
  source: .andromeda/design-system.md:80 «| Role (handoff) | #5B8DB8 | Transfer-marker arrows and names (outbound `→ builder`, inbound `← overseer`), the marker's 2px left tick (`--rule-strong`, the only blue rule)»
  replace-with: «- `<viola-transfer>` — outbound `→ builder  since …` and inbound `← overseer` in departure blue, in the rack gap on anthracite»

- leaf: .claude/docs/design-summary.md:62   class: retired
  leaf-text: «- `<viola-event-feed>` — the tower tape: native `<details>` lines, oldest on top, 2000-line cap, follows only at the bottom, never announced»
  source: .andromeda/design-system.md:12 «- **Handoff / transfer of control, and the tower voice recorder.** `driver → driven` transfer markers, and the ndjson tape played in order with `skipped` counts kept where they happened.»
  replace-with: «- `<viola-event-feed>` — the tower tape: the ndjson tape played in order, one line per event, `skipped` counts kept where they happened»

- leaf: .claude/docs/design-summary.md:63   class: retired
  leaf-text: «sticky header of `<dt>/<dd>` cells:»
  source: .andromeda/design-system.md:288 «| space-xl | 32px | Top page margin under the sticky ATIS header, and the strip height (`--strip-h`) |»
  replace-with: «sticky header:»

- leaf: .claude/docs/design-summary.md:64   class: retired
  leaf-text: «- Access/error strips and reserved v1.x brake buttons (`I HAVE CONTROL`, `UNLINK` — not built in v1)»
  source: .andromeda/design-system.md:91 «| Raised-3 | #2A2E33 + 2px #E6D8AE rule | Reserved for the v1.x brake confirmation, the only modal-like surface ever planned. v1 has none. |»
  replace-with: «- Access/error strips and the reserved v1.x brake buttons and Raised-3 confirmation (not built in v1)»

- leaf: .claude/docs/design-summary.md:68   class: retired
  leaf-text: «- NEVER use generic families (Inter, Roboto, Arial, Helvetica, `system-ui`), Tailwind/Bootstrap palette values, or a ninth hex value.»
  source: .andromeda/design-system.md:586 «A ninth hex value or any other untokened px value signals no system.»
  replace-with: «- NEVER use a ninth hex value; the font stacks end at the generic `sans-serif` / `monospace`, never `system-ui`.»

- leaf: .claude/docs/design-summary.md:70   class: retired
  leaf-text: «card grids, status dots, toasts, spinners, skeletons, "Loading…", chat bubbles, node graphs or budget rings»
  source: .andromeda/design-system.md:549 «- **Skeleton shimmer and a terminal-green or neon-glow "hacker" theme.** Rejected because nothing glows in a tower cab unless the lamp is on it. Missing values print `unknown` in their box at once.»
  replace-with: «status dots, toasts, spinners, skeleton shimmer or budget rings»

- leaf: .claude/docs/design-summary.md:73   class: retired
  leaf-text: «- NEVER render event text as HTML or Markdown; NEVER show the token, URL, `?t=` or `viola_home`.»
  source: .andromeda/design-system.md:177 «| EventKind::turn-ended | #1E2124 | none | #E6D8AE | `turn-ended` + first line of `last_assistant_message` as plain text, or `no message` for `null`. Never fills a readback box. |»
  replace-with: «- Tape event text prints as plain text (the domain status rows).»
