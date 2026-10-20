group: design-system
read: .claude/docs/design-summary.md · 73 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:7   class: stale
  leaf-text: «The page is an air-traffic-control tower strip bay after dark.»
  source: .andromeda/design-system.md:49 «The direction a 0.2.0 frontend inherits (0.1.0 renders no page) is a control-tower strip bay after dark.»
  replace-with: «The direction a 0.2.0 frontend inherits (0.1.0 renders no page) is an air-traffic-control tower strip bay after dark.»

- leaf: .claude/docs/design-summary.md:9   class: retired
  leaf-text: «base 0.2 · web-spa 0.3 (exactly two motions) · CLI TTY 0.2»
  source: .andromeda/design-system.md:44 «N/A — the frontend's expression level: the frontend is deferred to 0.2.0, and its technology is not chosen.»
  replace-with: «base 0.2 · frontend N/A (deferred to 0.2.0, its technology not chosen) · CLI TTY 0.2»

- leaf: .claude/docs/design-summary.md:11   class: stale
  leaf-text: «a 16px ruled square at the far right of every tape send line and every outbound transfer marker once a send on its link has been seen since page open (before that the marker prints only the word `unknown`, no box)»
  source: .andromeda/design-system.md:21 «It is a fixed-width, 1px-ruled square cell; in 0.1.0 its one place is the CLI mirror below, and where a 0.2.0 frontend draws it is that version's to name (its technology is not chosen).»
  replace-with: «a 16px 1px-ruled square cell whose one 0.1.0 place is the CLI mirror (where a 0.2.0 frontend draws it is that version's to name, its technology not chosen)»

- leaf: .claude/docs/design-summary.md:11   class: stale
  leaf-text: «Supporting element: the **cocked strip** (`dialog_pending`: 12px translate over 160ms, once, amber 4px band, `DIALOG <kind>`).»
  source: .andromeda/design-system.md:28 «On `dialog_pending` the row keeps its slot and shows the word `DIALOG` plus the dialog kind, the word amber on the CLI. The strip's 12px / 160ms translate and its 4px amber holder band are a 0.2.0 frontend design input.»
  replace-with: «Supporting element: the **cocked strip** (`dialog_pending`: the row keeps its slot and shows `DIALOG <kind>`, the word amber on the CLI; its 12px / 160ms translate and 4px amber holder band are a 0.2.0 frontend design input).»

- leaf: .claude/docs/design-summary.md:42   class: retired
  leaf-text: «- **Two motions only:** the cock (`--cock-offset` 12px over `--cock-dur` 160ms, ease-out, once) and the opacity fade of a new tape line or transfer marker (`--fade-dur`, ≤120ms). The readback fill, the strike, `stale` dimming and a strip's return into line are instant.»
  source: .andromeda/design-system.md:46 «At 0.2 the duration scale's 0.0–0.2 row applies (0–100ms, no entrance), and viola's cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.»
  replace-with: «- **Calibrated to the base expression 0.2:** the 0.0–0.2 duration row applies (0–100ms, no entrance), and the cli (0.2 on a TTY, 0.0 otherwise) has no motion at all; the cocked strip's 12px / 160ms translate is a 0.2.0 frontend design input.»

- leaf: .claude/docs/design-summary.md:55   class: retired
  leaf-text: «`<viola-readback>` — the signature box, the same element and drawing on a tape send line and an outbound transfer marker»
  source: .andromeda/design-system.md:21 «It is a fixed-width, 1px-ruled square cell; in 0.1.0 its one place is the CLI mirror below, and where a 0.2.0 frontend draws it is that version's to name (its technology is not chosen).»
  replace-with: «The readback box — the signature; in 0.1.0 only the CLI `viola send` mirror, its 0.2.0 places that version's to name»

- leaf: .claude/docs/design-summary.md:56   class: stale
  leaf-text: «`<viola-transfer>` — outbound `→ builder  since …` and inbound `← overseer` in departure blue, in the rack gap on anthracite»
  source: .andromeda/design-system.md:230 «The 2px (`--rule-strong`) left tick of the outbound and inbound transfer markers only, in the rack gap on anthracite (4.6:1).»
  replace-with: «Transfer markers — outbound `→ builder  since …` and inbound `← overseer` in departure blue, in the rack gap on anthracite»

- leaf: .claude/docs/design-summary.md:57   class: stale
  leaf-text: «`<viola-event-feed>` — the tower tape: the ndjson tape played in order, one line per event, `skipped` counts kept where they happened»
  source: .andromeda/design-system.md:89 «| Base | #1E2124 | Page background (the bay), the rack gap, the tape, and bare (unwrapped) and `stale` strips.»
  replace-with: «The tower tape — the ndjson tape played in order, one line per event, `skipped` counts kept where they happened»

- leaf: .claude/docs/design-summary.md:58   class: stale
  leaf-text: «`<viola-atis>` — sticky header: `BAY`, `5H`, `7D`, `read … ago`, gate, `TAPE`, `skipped`»
  source: .andromeda/design-system.md:287 «| space-xl | 32px | Top page margin under the sticky ATIS header, and the strip height (`--strip-h`) |»
  replace-with: «The ATIS header — sticky: `BAY`, `5H`, `7D`, `read … ago`, gate, `TAPE`, `skipped`»

- leaf: .claude/docs/design-summary.md:59   class: stale
  leaf-text: «- Access/error strips and the reserved v1.x brake buttons»
  source: .andromeda/design-system.md:209 «| Problem::unauthorized (401) | #1E2124 | #E6D8AE | #E6D8AE | An access refusal. Never shows the token, the URL or the file path. |»
  replace-with: «- The access refusal, error strips and the reserved v1.x brake buttons»
