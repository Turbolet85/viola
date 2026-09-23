## Color Moods

### Mood 1: "Tower cab strip bay after dark"

**Physical space:** The strip bay of a control tower at night. The cab lights are dimmed, so the anthracite console laminate and the anodised strip racks fade into the dark. Buff paper strips in their plastic holders catch the light from a gooseneck task lamp. One arrival strip, in an orange holder, sits cocked a finger-width out of line in its slot.

**Color palette (derived from this space):**
- Console anthracite `#1E2124`: the matte laminate of the controller's console and the dark space between racks. This is the page background. It is a lit near-black, not a void.
- Strip buff `#E6D8AE`: the paper flight-progress strip under the task lamp (buff is a standard strip and strip-holder stock). This is the strip surface and the primary text colour on dark.
- Arrival-holder amber `#D97706`: the orange arrival strip holder (tower convention: blue for departures, orange for arrivals). This is the single attention colour, reserved for the cocked strip, i.e. `dialog_pending`.

**Sensory anchor:** The light is low and falls only where the work is. Matte paper sits in slightly glossy plastic boots on ruled aluminium rails, and nothing glows unless a lamp is on it. It is quiet enough to hear a strip slide across to the next rack at handoff.

### Mood 2: "Flight-data desk beside the strip printer, day shift"

**Physical space:** The flight-data assistant's desk in daylight. A strip printer feeds fresh card-stock strips with ruled boxes and black dot-matrix callsigns into a tray. A red grease pencil lies across the ones already marked up, and the strips wait there before being carried to the bay.

**Color palette (derived from this space):**
- Card-stock cream `#F4EBD0`: a freshly printed strip blank. This would be the page and strip surface in a light-first build.
- Printer-ink black `#1C1C1C`: the dot-matrix characters and the 1px ruled box lines. This maps to text and the Frequentis/Saab-style box grid.
- Grease-pencil red `#C0282D`: the controller's annotation pencil (a struck-through level, a circled callsign). This is the single mark for a refusal or `not-delivered`.

**Sensory anchor:** Everything is flat, ink-on-paper and ruled, in the office daylight. There is chalky matte card, the chatter of the printer, and handwriting that marks only what changed. Nothing is decorative; it is paperwork you read by the position of each box.

### Mood 3: "Approach radar room, blue-grey ambient"

**Physical space:** A windowless approach-control room lit only by a low blue-grey ceiling wash. Controllers sit at large dark scopes where cyan data tags trail each target, and a readback log scrolls on the adjacent display. The walls and carpet are finished in a dark slate so they don't reflect the screens.

**Color palette (derived from this space):**
- Scope black `#0F1418`: the dark glass of the radar display. This would be the page background.
- Room slate `#5A6B78`: the blue-grey acoustic wall panels and carpet in the ambient wash. This would be the separator, rule and dim-state colour (`stale`).
- Data-tag cyan `#7FD6E0`: the target data blocks on the scope. This would be the accent for live feed lines and link markers.

**Sensory anchor:** It is cool, sealed and hushed. The only light comes from the screens and one indirect wash, and fabric walls deaden every sound except the radio. The screen is the whole world, and the room is built to disappear.

## Recommended

**Recommended mood:** 1 — "Tower cab strip bay after dark"

**Reasoning:** "Strip-bay handoff, read back" is set literally in the tower strip bay, and Mood 1 is that bay at the hour the founder actually watches it: a page left open for hours beside two terminals. Here's how the colours map to the references:
- **vStrips:** the buff strip on anthracite gives the layout grammar its material, with strips as the only lit objects on a dark rack and labelled separators ruled in the console tone.
- **Arrival amber:** the orange holder colour works as the one mark on the offset (cocked) strip for `dialog_pending`. It is deliberately different from Claude Code Agent View's yellow "Needs input".
- **Temporal Event History:** the same buff-on-dark can show an open issue→acknowledge pair (not closed until its `prompt-submitted` arrives) as an unfilled strip outline, with a closed pair as a filled strip. Unconfirmed sends then read as visibly open rather than coloured "done". That fits "measured, never assumed" and the "Overclaiming" negative anchor.

Two notes on the other moods:
- **Mood 2** is the same bay's paperwork in daylight. It could serve later as the light-mode variant without changing metaphor.
- **Mood 3** moves away from the strip bay toward scope-and-tag. Its cyan-on-black also sits close to the neon-terminal dev-dashboard look that 2026 lists are full of.

**Research basis:** Web searches run 2026-09-23:
- **Dark-first as the 2026 dev-tool default, with near-black rather than pure-black bases and restrained status colour:** [AdminLTE, "19 Best Dark Mode Dashboard Templates (2026)"](https://adminlte.io/blog/dark-dashboard-templates/), [Recursion, "UI/UX Color Trends That Define 2026"](https://www.recursion.agency/blog/ui-color-trends-2026), [Aniq-UI, "Best Dark Mode Dashboard Designs for 2026"](https://www.aniq-ui.com/en/blog/dark-mode-dashboard-designs-2026).
- **Strip colour conventions (blue departures / orange arrivals, buff and other holder stocks):** [NATS Blog, "ATC Explained: Flight Progress Strips" (Jan 2026)](https://nats.aero/blog/2026/01/atc-explained-flight-progress-strips/), [SKYbrary, Flight Progress Strips](https://skybrary.aero/articles/flight-progress-strips), [Airport Suppliers, Flight Progress Strip Holder](https://www.airport-suppliers.com/product/flight-progress-strip-holder/).
- **Tower cab dimmable and night-blue console lighting:** [Airport Suppliers, Modular ATC Towers](https://www.airport-suppliers.com/product/modular-atc-towers/).
- **FAA-HF-STD-010A standard ATC colour palette (11 foreground colours, colour-vision-deficiency tolerant):** [FAA DOT/FAA/AM-20/08](https://www.faa.gov/sites/faa.gov/files/data_research/research/med_humanfacs/oamtechreports/202008.pdf), [Wiley Color Research & Application 2026](https://onlinelibrary.wiley.com/doi/10.1002/col.70011).
- **Where the hex values come from:** the exact values are my own approximations of these physical materials, based on training data (2026). The searches did not return published RGB values for strip stock or console laminates. `#D97706` and `#1C1C1C` are widely used standard swatches. Contrast still needs checking in the build: `#E6D8AE` on `#1E2124` should be about 12:1 and `#D97706` on `#1E2124` about 5:1.
