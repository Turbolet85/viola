# Brand Quiz — viola

## Context (from architecture.md + security-plan.md + tooling-decisions.md)

- Product type: Hybrid local developer tool — a native cross-platform CLI binary (`viola`, Rust), a Claude Code plugin (hooks + stdio MCP server) that calls it, and a minimal view-only local web GUI (`viola ui`) served by the same binary. Core function: one interactive Claude Code session drives another on the user's own subscription — viola types at turn boundaries, answers dialogs through hooks, holds a one-driver wheel the human can take at any moment, and logs everything as an ndjson audit trail.
- Audience: The founder — a single developer who today is the manual "transport and operator" between an overseer session and a builder session; later (v1.x) individual Claude Code subscribers. Main automated callers: LLM driver sessions (MCP + CLI `--json`). UI verified through a headless browser by default.
- Platforms: `web-spa` (the `viola ui` page on 127.0.0.1 — session rows, links, budget, live SSE event feed; view-only in v1) and `cli` (clap subcommands, human text by default, `--json` + typed exit codes for agents; `viola run` passes the wrapped `claude` TUI through unchanged). Windows first; macOS + Linux CI-tested.
- Scale intent: personal (v1 loopback-only, no accounts; v1.x GUI brake + public distribution; later a phone view — the same page behind authentication).
- Security tier: Minimal (0), with targeted elevations for the local privilege boundary (GUI output encoding, cross-user/cross-origin readability, v1.x per-launch brake auth). CSP: no web fonts (system font stacks only), no inline styles / `style="…"`, no external assets, all event text rendered as plain text.
- Family chosen: Web Components / Lit (build-optional)
- Frontend framework: Lit 3.3.3 (vendored ESM, no JS build step)
- CSS tool: Hand-written modern CSS, no tool (native nesting, `@layer`, custom properties; Lit `static styles` available)
- Component library: none — build-from-scratch plain custom `viola-*` Lit elements over semantic HTML
- Mobile framework: N/A

## Q1: Brand Personality

- **Research recommended:** Round 1 (`research-q1.md`): Direction 1 "Signal-box interlocking: one token, one line" (railway single-line token, block instruments, train register). The user asked to dig deeper; round 2 (`research-q1-dig.md`, 7 new directions) recommended Direction 1 "Strip-bay handoff, read back" (ATC tower strip bay).
- **User response:** dig deeper → picked round-2 Direction 1 (accepted the dig-deeper recommendation, chosen from the full list of 10)
- **Final answer:** "Strip-bay handoff, read back"
- **Physical-world metaphor:** An air-traffic-control tower strip bay. Each aircraft is a paper flight-progress strip in a holder on a rack. The controller slides a strip to the next sector at handoff and "cocks" it out of line when it needs attention. Every clearance is read back by the pilot before it counts.
- **Voice:** Standard phraseology — short, fixed and read back ("Session builder, wheel human, dialog pending"), with no filler and no guessing.
- **Reasoning:** The only direction that maps all four core functions one-to-one: strip = session row (name, liveness, status, wheel, CLI version + verified); handoff = `driver → driven` link; cocked strip = `dialog_pending` ("needs you"); readback = the confirmed-after-the-fact send; pilot-in-command = "the human always wins the wheel". A fixed-field, text-only strip suits Lit with no component library, CSS grid and system fonts under the CSP, and does not clash with the brief's use of "bridge" (a connector between agents).

## Q2: Reference Products

- **Research recommended:** vStrips (vNAS) and Temporal Web UI Event History (top references), with Frequentis smartStrips / Saab EFS, Claude Code Agent View (contrast) and k9s (cli) as supporting references
- **User response:** accepted
- **Final answer:**
  - vStrips (vNAS) — web-based flight-progress strip bay — the layout grammar: bay / rack / labelled locked separators; a strip that needs attention is **offset** out of line, never re-coloured or re-sorted; stable row order so each session is always in the same slot; links rendered as transfer markers between strips, not a node graph; a push counts only when the receiver has the bay open.
  - Temporal Web UI — Event History — the readback feed: an issue → acknowledge pair shown as one unit that stays visibly **open** until acknowledged; a compact order-only reading beside the timestamped one; gaps (`skipped` counts) kept visible where they happened. _Orchestrator correction per architecture.md: a send is confirmed by its matching `prompt-submitted` (else a typed refusal, e.g. `not-delivered`); `turn-ended` closes the turn, not the send._
  - Supporting (not anchors): Frequentis smartStrips / Saab EFS — strip anatomy (ruled box grid, one field per fixed column, heavier callsign-weight name cell, `tabular-nums`, one categorical holder colour, `unknown` printed in its box); Claude Code Agent View — contrast (same state vocabulary, but viola keeps one attention colour and deliberately does NOT auto-reorder rows); k9s — cli (fixed-column table with the same columns and words as the web strip: NAME · LIVE · STATUS · WHEEL · DIALOG · CLI; one-line context header; dim `stale`; never colour without the status word).
- **Reasoning:** vStrips is the Q1 metaphor already built as a working web page — a concrete layout grammar for session rows, links and `dialog_pending` in hand-written CSS grid; Temporal supplies the readback half, so the founder sees "every send is confirmed after the fact, never presumed" in the live feed with nothing overclaimed.

## Q3: Color Mood

- **Research recommended:** Mood 1 "Tower cab strip bay after dark"
- **User response:** accepted
- **Final answer:**
  - **Mood:** "Tower cab strip bay after dark" — the strip bay of a control tower at night: dimmed cab lights, anthracite console laminate and anodised strip racks fading into the dark, buff paper strips in plastic holders catching a gooseneck task lamp, one orange arrival holder cocked a finger-width out of line.
  - **Example colors:**
    - Console anthracite #1E2124 — the matte console laminate and the dark space between racks; the page background (a lit near-black, not a void)
    - Strip buff #E6D8AE — the paper flight-progress strip under the task lamp; strip surface and primary text on dark (11.4:1 on #1E2124, computed)
    - Arrival-holder amber #D97706 — the orange arrival strip holder; the single attention colour, reserved for the cocked strip / `dialog_pending` (5.1:1 on #1E2124, computed)
  - **Sensory anchor:** Low light that falls only where the work is; matte paper in slightly glossy plastic boots on ruled aluminium rails; nothing glows unless a lamp is on it.
- **Reasoning:** It is the Q1 strip bay at the hour the page actually runs — left open for hours beside two terminals. Strips are the only lit objects on a dark rack (vStrips); amber marks only the cocked strip and is deliberately distinct from Agent View's yellow; buff outline vs filled buff on anthracite carries the Temporal open/closed readback without spending colour. Mood 2 ("Flight-data desk beside the strip printer, day shift": #F4EBD0 / #1C1C1C / #C0282D) was noted by research as a possible light variant; not adopted (see Q6).

## Q4: Expression Level

- **Research recommended:** base 0.2; web-spa 0.3; cli human TTY 0.2; cli `--json` / non-TTY / `NO_COLOR` / `viola run` passthrough 0.0
- **User response:** accepted
- **Final answer:**
  - **Base:** 0.2
  - **Per-surface:**

    | Surface | Expression | Reasoning |
    |---|---|---|
    | web-spa (`viola ui`) | 0.3 | The only surface where the metaphor's physical motions show; motion only on a state change, played once. (1) `dialog_pending` cocks the whole `<viola-session-row>`: one `translate` of ~12px over ~160ms ease-out, with the amber holder mark and a text label; it keeps its rack slot; no loop or pulse. (2) A new link marker or feed line appears with at most a 120ms opacity fade. Readback close is instant (outline → filled, no transition) when the matching `prompt-submitted` lands (or a typed refusal strikes it) — trigger corrected per arch, research had named `turn-ended`. `stale` dims instantly. No skeletons (`unknown` printed in its box), no spinner or pulse for `busy`, no staggered entrances, no re-sort. Hover limited to focus rings and link underline (no controls in v1). `prefers-reduced-motion: reduce` → every duration 0; the offset position and amber mark remain. |
    | cli — human TTY | 0.2 | k9s-style fixed-column rows, below the cargo/gh 0.3 tier. Colour only as a second cue (amber `DIALOG`, dimmed `stale` rows), always beside the text word. One static context header (budget `five_hour`/`seven_day` %, reading age). No live-redrawing dashboard; `viola wait` prints one static "waiting: <session>" line then a result line, no spinner. |
    | cli — `--json` / non-TTY / `NO_COLOR` / `viola run` passthrough | 0.0 | LLM driver sessions are the main callers and `viola run` must print nothing while the wrapped `claude` TUI runs: no colour, no spinner, no cursor control, typed exit codes only. |
- **Reasoning:** A personal, Minimal-tier local developer tool watched for hours by one developer and parsed by LLM callers; in a strip bay motion is physical and always means something, so nothing may animate progress or success before it is confirmed. No motion dependency (no framer-motion / GSAP / Lottie / Motion One): two or three CSS transitions of ≤200ms in hand-written CSS, CSP-clean.

## Q5: Signature Element

- **Research recommended:** Candidate 1 "The readback box" (Candidate 2 "The cocked strip" to ship as the supporting attention element)
- **User response:** accepted
- **Final answer:**
  - **Element:** "The readback box"
  - **What it is:** Every send line in `<viola-event-feed>`, and the last-send cell of every `driver → driven` transfer marker, ends in one fixed-width, 1px-ruled square cell labelled `RB` with exactly three states, each printed as a word as well as drawn: **open** — buff #E6D8AE 1px outline on anthracite #1E2124 with the word `open`; **read back** — the cell turns solid buff with `RB` in anthracite (11.4:1), instantly, when the matching `prompt-submitted` lands; **refused** — the outline stays with a single 1px diagonal strike, and the typed reason (e.g. `not-delivered`) prints in the next cell. Issue on the left, readback at the far right, shown as one unit; `turn-ended` never fills the box.
  - **Implementation notes:** A Lit 3.3.3 `viola-*` element sets `data-rb="open|read|refused"`; hand-written CSS in `/assets/app.css` styles each state by attribute selector (`border` / `background` / a stylesheet `linear-gradient` strike) — no inline style, no `transition`; the state word is a text binding. The human-TTY CLI mirrors it as a fixed column `[RB]` / `[  ]` / `[/ ] not-delivered`; `--json` / `NO_COLOR` output carries no glyph.
  - **Core-action tie:** The core action is a send (typed at a turn boundary, confirmed after the fact). Every send creates a box; the box closes only when "confirmed after the fact, never presumed" is satisfied — the founder's recurring glance "did my relay land?".
  - **Data dependency (decided 2026-09-24, after Phase 3):** architecture.md today logs no send-issued or send-refused event, so a v1 page could render only `RB`. Decision: design specifies ALL THREE states; the missing events are an arch gap recorded as cross-lane follow-up CL-1 (`cross-lane-followups.md`: `send-issued` carrying the send's `cursor`, `send-refused` carrying `refusal` + `detail`), folded into architecture.md by wrap's reconcile. The box maps `open` ← `send-issued` with no matching `prompt-submitted` yet; `RB` ← matching `prompt-submitted`; `refused` ← `send-refused`.
- **Reasoning:** The only candidate that belongs to viola rather than its category (Agent View, MulmoTerminal and vStrips all have a "needs you" state; none shows a send staying visibly open until the far end reads it back). It carries every prior pick: Q1 readback phraseology (`open` / `RB` / `not-delivered`), Q2 Temporal issue→acknowledge pair in strip anatomy, Q3 buff-on-anthracite (leaving amber free for the cocked strip), Q4 zero motion because acknowledgement is discrete. The cocked strip (Q5 Candidate 2 — `dialog_pending` row offset ~12px / 160ms, 4px amber holder band, dialog kind printed, stable slot) ships as the supporting attention element within the Q4 budget, not as the signature.

## Q6: Dark/Light/Auto (orchestrator-only)

- **Final answer:** dark (dark only — no light variant in v1)
- **Reasoning:** Developer tool left open for hours beside two terminals; the chosen Q3 mood is the night strip bay. User picked "Dark only" over Auto (which would have paired the Mood 2 day-desk palette) and over a planned v1.x light variant.

## Surface-Specific Confirms (orchestrator-only, conditional)

- **cli:** Minimal — colour only as a second cue on a human TTY (amber `DIALOG`, dimmed `stale`), always beside its status word; no colour for `--json`, non-TTY, `NO_COLOR` or `viola run` — consistent with Q4 cli 0.2 / 0.0.
- **web-spa:** N/A — no per-surface confirm defined for web-spa; style set by Q1–Q6 and Phase 1 tooling.

## Decisions Log

`2026-09-23` — Initial brand quiz by `/andromeda-design` Phase 2

- Personality: "Strip-bay handoff, read back" (ATC tower strip bay; standard phraseology voice)
- References: vStrips (vNAS) + Temporal Web UI Event History (supporting: Frequentis/Saab EFS, Claude Code Agent View as contrast, k9s for cli)
- Mood: "Tower cab strip bay after dark" — #1E2124 / #E6D8AE / #D97706
- Expression: base 0.2, surfaces (web-spa 0.3; cli human TTY 0.2; cli machine output / passthrough 0.0)
- Signature: "The readback box" (supporting attention element: the cocked strip)
- Dark/Light: dark only
- Per-surface confirms: cli minimal — colour as a second cue only
- Notes: Q1 — user asked to dig deeper after round 1 (signal-box / conn handover / calibration bench); picked the round-2 recommendation from the full list of 10. Q2–Q6 — all recommendations accepted. Orchestrator corrections carried from architecture.md into Q2 / Q4 / Q5: a send is confirmed by its matching `prompt-submitted` (else a typed refusal); `turn-ended` closes the turn, not the send. Contrast ratios for Q3 computed by the orchestrator (WCAG relative luminance): #E6D8AE on #1E2124 = 11.4:1; #D97706 on #1E2124 = 5.08:1.

`2026-09-24` — Signature data dependency (after Phase 3 exploration)

- Readback box: keep all three states (`open` / `RB` / `refused`). The absent send-issued / send-refused events are an arch gap, not a design stretch (overseer, founder-delegated: brief §7 makes the event log the audit trail). Recorded as cross-lane follow-up CL-1 for wrap's reconcile into architecture.md.
