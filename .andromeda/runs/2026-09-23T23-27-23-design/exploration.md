## 1. Domain Concepts

- Flight-progress strip: one strip per `viola run` instance, with fixed printed fields in a fixed order (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI). The name cell is set heavier, like a callsign. A field that has no reading prints `unknown` in its box and is never left blank.
- Strip holder (plastic boot): the coloured plastic sleeve a strip sits in. Its colour says what kind of traffic the strip is, never what state it is in. Wrapped instances sit in holders. Unwrapped `claude agents --json` sessions are shown as bare strips with no holder, like another facility's traffic that you can see but not control. Their names are never a valid `target`.
- Strip bay / rack with labelled separators: the `viola ui` page is one bay. Each rack is ruled with aluminium rails and has fixed separator labels. Every session keeps the same slot for its whole life, so the founder's eye goes to the same place each time. Nothing is re-sorted.
- Cocked strip: a strip pushed a finger-width out of line in its rack. It is the physical sign for `dialog_pending` ("needs you"). The strip keeps its slot. It is offset and gets an amber band, but it is not moved or recoloured.
- Handoff / transfer of control: one sector passes an aircraft to the next. This is the `driver → driven` link, drawn as a transfer marker between two strips with its `since` time. It is not drawn as an edge in a node graph.
- Readback: a clearance counts only once the pilot has read it back. This is viola's "confirmed after the fact, never presumed". A send is confirmed by its matching `prompt-submitted`. Otherwise it gets a typed refusal such as `not-delivered` or `no-prompt-submitted`. `turn-ended` closes the turn, not the send.
- Standard phraseology: short, fixed words that are read back, such as `open`, `RB`, `unable`, `human-typing`, `budget-paused`, `unverified-cli` and `not-delivered`. This is the vocabulary for every label, CLI column and refusal line. There is no filler text and no guessing.
- Pilot-in-command, "I have control": the human wins the wheel with one keystroke, and automation is then refused with `human-typing`. `viola pause` is the spoken "I have control". `viola release` is "you have control".
- "Unable" plus a reason: the ATC way of refusing an instruction. viola answers with a typed refusal (`not-delivered` / `turn-running`, `input-not-ready`, `unknown-dialog`, `control-character`) instead of silently failing or retrying.
- Coasting track: a radar track that is still drawn after fresh returns stop. This is `stale` liveness: the heartbeat is older than 5 s but the pid and start time say the process is alive. The track is dimmed, not deleted.
- Fuel state / minimum fuel: the budget governor. The `five_hour` and `seven_day` percentages have thresholds (90 / 85), and `budget-paused` is a declared minimum-fuel state that stops new sends while the running turn finishes.
- ATIS information letter with its age: the budget reading always carries its `read_at` age, like "information Kilo, 4 min old". Old information is shown as old, never as current.
- Verified aircraft type / flight-plan check: the capability ledger. A CLI version that `viola verify` has stamped is a known type. An unstamped build flies "transport only" (`unverified-cli`) and dialog answers are held back.
- Tower voice recorder (the tape): `events.ndjson` is the append-only, never-rotated audit trail. The live SSE feed plays this tape in order and keeps visible, as `skipped` counts, any lines it could not read.

## 2. Color World

- Console anthracite #1E2124: the matte laminate of the tower console and the dark gap between racks. It is the page background and the `open` readback box interior: a lit near-black, not a void (Strip bay).
- Holder plastic #2A2E33: the slightly lighter, slightly glossy plastic boot around each strip under dimmed cab light. It is the surface of a session strip and of a feed band. Strip buff text on it is about 9.6:1 (computed). Amber text on it is only about 4.3:1, so amber stays a non-text band there (Strip holder).
- Rack-rail aluminium #4A5057: the anodised rails and labelled separators between racks, catching almost no light. It is for decorative 1px rules and separator hairlines only, at about 2.0:1 on anthracite. Borders that carry information use strip buff (Strip bay / rack).
- Strip buff #E6D8AE: the paper flight-progress strip under the gooseneck task lamp. It is primary text, the ruled field grid, the readback box outline, and the fill of a read-back box. It is 11.4:1 on anthracite (Flight-progress strip, Readback).
- Graphite pencil #3B3A36: the controller's pencil marks written onto a buff strip. It is the text colour wherever text sits on a buff fill (the `RB` in a closed readback box, the name cell if it is ever filled). It is about 8.0:1 on #E6D8AE (computed) (Readback, Standard phraseology).
- Lamp-off buff #9C9278: the same paper strip after the task lamp has moved off it. It is for `stale` rows (coasting track), secondary field labels, timestamps and reading ages, at about 5.25:1 on anthracite (computed), so it still passes AA for text (Coasting track, ATIS age).
- Arrival-holder amber #D97706: the orange arrival holder that is cocked out of line. It is the only attention colour, reserved for `dialog_pending`: the 4px holder band on the cocked strip, the amber `DIALOG` word on anthracite at 5.08:1, and the CLI's amber `DIALOG` column. It is never used for errors, warnings, budget or hover (Cocked strip).
- Departure-holder blue #5B8DB8: the blue departure holder for outbound traffic being handed on. It is used only for the `driver → driven` transfer marker and link text, at about 4.6:1 on anthracite (computed). It is not a status colour and is never a background fill (Handoff / transfer of control).

## 3. Signature Element

- **Element name:** "The readback box"
- **What it is:** Every send line in `<viola-event-feed>`, and the last-send cell of every `driver → driven` transfer marker, ends in one fixed-width square cell with a 1px rule, labelled `RB`. The issue sits on the left and the readback at the far right, read as one unit. The cell has exactly three states, each printed as a word as well as drawn:
  - **Open:** a strip-buff #E6D8AE 1px outline on console anthracite #1E2124, with the word `open`.
  - **Read back:** the cell turns solid strip buff with `RB` in graphite/anthracite, instantly, when the matching `prompt-submitted` lands.
  - **Refused:** the outline stays and a single 1px diagonal strike is drawn through it. The typed reason (`not-delivered` · `no-prompt-submitted`, `turn-running`, `control-character`, and so on) prints in the next cell.

  `turn-ended` never fills the box. A ledger-listed local command that returns `ok` with `confirmed:false` / `unconfirmable` is the only other outcome. It keeps the outline and prints `unconfirmable` in the reason cell with no fill, so it is never shown as read back.
- **Implementation notes:**
  - A plain Lit 3.3.3 `viola-*` element (for example `<viola-readback>`, used inside `<viola-event-feed>` and the transfer marker) reflects `data-rb="open|read|refused"` and binds the state word and reason as `${}` text only. There is no `unsafeHTML`, no `style="…"` and no `transition`.
  - Hand-written CSS in `/assets/app.css` (in an `@layer components`, with custom properties such as `--rb-ink`, `--rb-fill` and `--rb-rule`) styles each state by attribute selector: `border` for open, `background` for read, and a stylesheet `linear-gradient` for the strike.
  - A document stylesheet does not reach into shadow DOM. So either render these elements into light DOM (`createRenderRoot(){return this}`) so `/assets/app.css` applies, or move the rules into Lit `static styles` once the headless-browser check confirms that constructable stylesheets pass the `style-src 'self'` + Trusted Types CSP.
  - The human-TTY CLI mirrors the box as a fixed column: `[RB]` / `[  ]` / `[/ ] not-delivered`. `--json` and `NO_COLOR` output carry no glyph.
  - Data dependency to confirm in Phase 4: the `open` state needs a record of the send being issued. Architecture's normalised event kinds contain `prompt-submitted` (origin `driver`) but no send-issued or send-refused line in `events.ndjson`. As written today, the feed can therefore show read-back sends but cannot show open or refused ones.
- **Domain tie:** It is ATC Readback: a clearance is not in force until the far end reads it back. That is viola's "every send is confirmed after the fact, never presumed", seen in the founder's recurring glance at the feed: "did my relay land?"

### Bootstrap phases (derive for route / setup-project)

_[ALL tiers. This is a derivation hint for downstream consumers per the D26 chain. Adapted to Phase 1 tooling (Lit 3.3.3 vendored ESM, hand-written CSS, no component library) and to the CSP (no web fonts, no inline style, no external assets). Design's full token and surface materialisation happens through Phase 8 layout-templates.md and the downstream design-system.md.]_

The downstream skills derive the following bootstrap phases from the design exploration plus the downstream design-system.md / layout-templates.md:

- **design-tokens-bundle-init:** viola's own CSS custom properties in `/assets/app.css` under `@layer tokens`, embedded via `include_str!`. They cover:
  - colour: the eight Color World entries above, with amber limited to the dialog role;
  - typography: system stacks plus `font-variant-numeric: tabular-nums` for fixed numeric fields;
  - spacing: the strip field grid and rack gutters;
  - motion: `--cock-offset: 12px`, `--cock-dur: 160ms`, `--fade-dur: 120ms`;
  - depth: none (flat, lit only by the lamp);
  - radius: 0, since strips and boxes are ruled rectangles;
  - readback-box tokens (`--rb-size`, `--rb-rule`, `--rb-fill`, `--rb-ink`, `--rb-strike`).
- **typography-stack-install:** system font stacks only, because the CSP has no `font-src`. That means no `@font-face`, no Fontsource and no WOFF2. A system UI sans is used for labels and a system monospace for fixed-width fields and the CLI mirror.
- **iconography-registry-install:** no icon library, since `img-src 'self'` and phraseology is carried by words. Any glyph (the transfer-marker arrow, the RB strike) is drawn in CSS or with text characters, and never with `unsafeSVG`.
- **component-library-install:** N/A. Phase 1 chose build-from-scratch `viola-*` Lit elements over semantic HTML. The a11y harness binds to those elements directly.
- **motion-tokens-wire:** two presets only, both played once on a state change: the cocked-strip `translate` (12px / 160ms ease-out) and a new-line or new-marker opacity fade of at most 120ms. The readback close and `stale` dimming have no transition. `@media (prefers-reduced-motion: reduce)` sets every duration to 0 while keeping the offset position and the amber band.
- **theme-provider-wire:** dark only. One token set, with no `prefers-color-scheme` switch and no light variant in v1.
- **chart-card-primitive-bootstrap:** bootstrap the strip primitive (`<viola-session-row>`: ruled field grid, holder band slot, cocked state) and the readback box (`<viola-readback>`) first. The rack, the transfer marker and `<viola-event-feed>` are all built from these two.
- **surface-handoff-wire:** the `viola ui` page shell is a single bay with racks and labelled separators. It covers the session racks, the transfer markers, the budget / ATIS header line and the tape feed. The page loads `/api/sessions` and `/api/links` first, then tails SSE `/api/events`. The k9s-style CLI table shares the same column names and words.

Route ordering (typical): design-tokens-bundle-init → typography-stack-install → iconography-registry-install (N/A path) → component-library-install (N/A) → motion-tokens-wire → theme-provider-wire (dark only) → chart-card-primitive-bootstrap → surface-handoff-wire.

## 4. Defaults to Reject

- **Default:** Traffic-light status dots (green `live`, yellow `busy`, red `stale` / error) at the left of each session row.
  - **Why tempting:** Every agent dashboard, including Claude Code Agent View, and every k8s UI uses coloured dots. It is the statistical average for "session status".
  - **Replace with:** Flight-progress strip fields that print the status word in a fixed column (`live` / `stale`, `idle` / `busy` / `unknown`). `stale` is shown as a coasting track in lamp-off buff #9C9278. The only colour state is the cocked strip's arrival-holder amber, always next to its `DIALOG` word.

- **Default:** Optimistic "Sending…" spinner followed by a green "Sent ✓" checkmark or toast.
  - **Why tempting:** Chat and messaging UIs mark a message delivered the moment the request returns, and toasts are the reflex for feedback.
  - **Replace with:** The readback box. It stays `open` until the matching `prompt-submitted` arrives, then fills instantly as `RB`, or takes a strike plus the typed reason. There is no spinner, no green and no toast. This is Readback and "Unable" plus a reason, from Standard phraseology.

- **Default:** A node-graph canvas (React Flow–style bubbles and arrows) to show which sessions drive which.
  - **Why tempting:** "Agent orchestration" pulls the model toward graph diagrams, and links feel like edges.
  - **Replace with:** Transfer markers inside the strip bay. A departure-holder-blue #5B8DB8 `driver → driven` handoff line between two strips shows `since` and the last send's readback box. Strips keep their rack slots (Handoff / transfer of control, Strip bay).

- **Default:** Auto-sorting rows so that "needs input" or most-recent sessions jump to the top.
  - **Why tempting:** Agent View and inbox UIs re-rank by urgency, which looks helpful.
  - **Replace with:** Stable rack slots. A session needing the human is cocked out of line (a 12px offset and an amber band) in place, like a controller's cocked strip, so muscle memory for where `builder` lives never breaks (Cocked strip, Strip bay).

- **Default:** A chat-transcript feed: speech bubbles with bot and human avatars, Markdown-rendered assistant replies and code blocks.
  - **Why tempting:** The content is LLM prompts and replies, so a chat UI is the obvious frame.
  - **Replace with:** The tower tape. One fixed-column line per event (time · instance · kind · text), with each send paired with its readback, `skipped` counts kept where they occurred, and all text rendered as plain text. This follows both the voice-recorder metaphor and the security plan's ban on Markdown-to-HTML and `innerHTML` (Tower voice recorder).

- **Default:** Budget shown as circular progress rings or gradient bars that go green, yellow and red.
  - **Why tempting:** Usage meters in SaaS billing pages look like this.
  - **Replace with:** Fuel-state figures in ruled strip fields (`5H 62 %  7D 41 %`), always paired with the ATIS reading age (`read 4m ago`) and `unknown` printed in the box when absent. `budget-paused` is a phraseology word, not a colour (Fuel state, ATIS information letter).

- **Default:** Robot and person icons or emoji for the wheel holder, plus a toggle switch that looks clickable.
  - **Why tempting:** Icons read fast, and a switch mirrors "who is in control".
  - **Replace with:** The phraseology word in the WHEEL field (`human` / `driver`), with no control in v1's view-only page. Taking control is spoken ("I have control") through the keyboard or `viola pause`, never through a GUI affordance that implies the page can change state (Pilot-in-command, Standard phraseology).

- **Default:** Skeleton shimmer placeholders and a terminal-green or neon-glow "hacker" dark theme.
  - **Why tempting:** Dark developer tools default to shimmer loading and glowing accent colours.
  - **Replace with:** A tower cab after dark. Buff paper on anthracite, with nothing glowing unless the lamp is on it. A missing value prints `unknown` in its box immediately instead of shimmering (Flight-progress strip, Strip bay).
