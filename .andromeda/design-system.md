# Design System — viola

## Brand Identity

**Personality:** Tower strip-bay handoff, read back: standard-phraseology precision under low cab light. Every word is short and fixed, every send stays open until the far end reads it back, and nothing glows unless the lamp is on it.

**Domain anchors:**
- **Flight-progress strip.** One strip per `viola run` instance. The fields are fixed, in the order NAME · LIVE · STATUS · WHEEL · DIALOG · CLI. The name cell is set at callsign weight. A field with no reading prints `unknown`.
- **Strip bay and racks with labelled separators.** Every session keeps a stable slot and nothing is re-sorted.
- **Cocked strip.** `dialog_pending` is the strip pushed a finger-width out of line, with an arrival-holder amber band.
- **Readback.** A send counts only once the matching `prompt-submitted` reads it back or, for `/clear`, once its measured post-condition (a new session) does. Otherwise it is "unable" plus a typed reason.
- **Handoff / transfer of control, and the tower voice recorder.** `driver → driven` transfer markers, and the ndjson tape played in order with `skipped` counts kept where they happened.
- **Strip holder (plastic boot).** Wrapped instances sit in a holder. Unwrapped sessions are bare strips: another facility's traffic, visible but never a `target`.
- **Coasting track.** `stale` liveness. The strip is dimmed to lamp-off, never deleted, and keeps its slot.
- **Fuel state / minimum fuel.** The budget governor: `5H` / `7D` figures, with `budget-paused` as a declared minimum-fuel state that stops new sends while the running turn finishes.
- **ATIS information with its age.** Every budget reading carries its age (`read 4m ago`). This is the `<viola-atis>` header. Old information is shown as old (`expired`), never as current.
- **Verified aircraft type.** A CLI version stamped by `viola verify` prints `verified`. An unstamped build flies transport only as `unverified-cli`.
- **Pilot-in-command, "I have control".** `human` in WHEEL at weight 600. `viola pause` / `viola release` are the spoken "I have control" / "you have control".
- **Standard phraseology and "unable" plus a reason.** Short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`) are the vocabulary of every label, CLI column and refusal. There is no filler and no guessing.

**Signature element:** **The readback box.** It is a fixed-width, 1px-ruled square cell at the far right of every send line in `<viola-event-feed>` and of every outbound transfer marker once a send on that link has been seen. It has three drawn states, and each state is printed as a word too:
- **`open`:** a buff outline on anthracite.
- **`RB` / `read back`:** filled solid buff, with `RB` in graphite. It fills instantly when the matching `prompt-submitted` lands, or for `/clear` when its `session-start` with cause `clear` does.
- **`unable`:** the outline plus one 1px `/` strike, with the typed reason in the next cell.

A fourth printed word, `unconfirmable`, reuses the open drawing and is never filled. `turn-ended` never fills the box. The CLI mirrors the box as `[RB]` / `[  ]` / `[/ ]`.

**Supporting attention element:** the cocked strip. On `dialog_pending` the strip translates 12px over 160ms ease-out, once, and keeps its slot. It gets a 4px amber holder band and the word `DIALOG` plus the dialog kind.

**Expression level:** `0.2` (base). This value governs animation intensity, layout freedom and interaction complexity across the whole project.
- `0.0-0.2`: static, grid-locked, zero animation, dense
- `0.3-0.4`: subtle hover states, fade transitions, quiet focus rings
- `0.5-0.6`: smooth transitions, micro-interactions, loading skeletons
- `0.7-0.8`: staggered entrances, parallax, spring physics
- `0.9-1.0`: scroll-driven, 3D, particles, canvas/WebGL

Per-surface values (quiz Q4):

| Surface | Expression | What it permits |
|---|---|---|
| web-spa (`viola ui`) | 0.3 | Exactly two motions, each played once on a state change. (1) The cocked-strip `translate` (12px / 160ms ease-out). (2) A new tape line or new transfer marker fades in (opacity, ≤120ms). The readback fill, the strike, `stale` dimming and the return of a strip into line are all instant. No hover effects except link underline and focus rings. |
| cli, human TTY | 0.2 | Static k9s-style fixed-column rows and one static context header. Colour is only a second cue: amber `DIALOG` and dim `stale`, always beside the word. Nothing redraws in place and there are no spinners. |
| cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight. |

This number is the SINGLE SOURCE OF TRUTH for how much motion and visual dynamism downstream phases implement. At 0.2 the duration scale's 0.0–0.2 row applies (0–100ms, no entrance), and viola's cli (0.2 on a TTY, 0.0 otherwise) has no motion at all. viola's web page is 0.3, but its only two motions are the ones in the table above, and Motion's "This project's values" override the 0.3–0.4 row (see the Design Decisions Log).

**Design direction:**
The page is a control-tower strip bay after dark. Buff paper strips sit in anthracite holder boots on ruled aluminium racks. Every session holds the same slot for its whole life, so the eye goes to where `builder` always is. The only warm light in the bay is the one arrival holder cocked out of line when a session needs you.

Nothing announces success before it is confirmed. Sends stay visibly open until they are read back. Missing readings print `unknown` in their box instead of shimmering. Old information always carries its age, like an ATIS letter. The density is the calm density of a strip board, not a monitoring dashboard: fixed fields, tabular numbers, one attention colour and one handoff colour. Everything else is paper and ink.
**Library shortlist direction:** Precision & Density (primary) with Utility & Function (secondary). The style preset is Minimalism & Swiss Style (#1) without its 200–250ms hover, with E-Ink / Paper (#56) as the surface reference, inverted to buff on anthracite. The rejected presets are Dark Mode (OLED) (#7), Real-Time Monitoring (#31: pulsing dots, blink, toasts) and HUD / Sci-Fi FUI (#51: glow, telemetry animation).

---

## Color Palette

**Rationale:** All eight values come from the exploration's Color World: the tower cab strip bay at night. The palette is a monochrome ladder of three surfaces (#1E2124 → #2A2E33 → #4A5057) and three inks (#E6D8AE, #9C9278, #3B3A36). Two hues are allowed, each locked to one role:
- **Arrival-holder amber #D97706:** `dialog_pending` only.
- **Departure-holder blue #5B8DB8:** transfer markers only.

The library shortlist's Developer Tool palette (#81) supplied the dark tonal-ladder structure, warmed from slate to neutral console laminate. Its run-green accent was dropped. Coding-Challenge (#151) confirmed amber as the single sharp accent. Transportation (#96) supplied the "one route blue" idea, lightened from #2563EB (3.5:1, fails) to #5B8DB8.

**Contrast rules (WCAG relative luminance, computed).** These restrict where each ink may sit:

| Ink | on Anthracite #1E2124 | on Holder #2A2E33 | on Buff #E6D8AE | Rule |
|---|---|---|---|---|
| Strip buff #E6D8AE | 11.4:1 | 9.6:1 | — | Allowed everywhere |
| Lamp-off buff #9C9278 | 5.2:1 | **4.4:1 (fails AA)** | — | Text only on anthracite. Never text on holder. |
| Arrival amber #D97706 | 5.1:1 | **4.3:1 (fails AA)** | — | Text only on anthracite. On holder it is a non-text band only (≥3:1 passes). |
| Departure blue #5B8DB8 | 4.6:1 | **3.9:1 (fails AA)** | — | Text only on anthracite, which is why transfer markers live in the rack gap, not on strips |
| Graphite #3B3A36 | — | — | 8.0:1 | Only on a buff fill (the `RB` in a read-back box) |
| Rack rail #4A5057 | 2.0:1 | — | — | Never text. Decorative rules only. |

### Core Colors
| Role | Value | Usage | Domain anchor |
|------|-------|-------|---------------|
| Primary | #E6D8AE | Primary ink, focus rings, information-carrying borders (every Border Progression Emphasis use: the readback box outline and strike, the `budget-paused`, nonzero `skipped` and `TAPE stopped` boxes, error strips), the read-back fill | Strip buff: the paper flight-progress strip under the gooseneck task lamp. It is the thing the controller reads, and the readback box is drawn on it. |
| Secondary | #9C9278 | Secondary ink on anthracite (timestamps, reading ages, column captions, `n/a`), the printed field-grid rule inside strips, the 1px box around an ATIS `expired` word, and all ink of a `stale` strip | Lamp-off buff: the same paper after the lamp moves off. A coasting track is dimmed, not deleted. An ATIS age is printed small. |
| Accent | #D97706 | The only attention colour: the 4px holder band and the `DIALOG` word of a cocked strip, and the CLI `DIALOG` word in `viola list` (the kind word after it stays uncoloured). The `DIALOG builder · viola` document title is plain text with no colour. Never used for errors, warnings, budget or hover. | Arrival-holder amber: the orange holder cocked out of line. It is the physical "needs you". |
| Role (handoff) | #5B8DB8 | Transfer-marker arrows and names (outbound `→ builder`, inbound `← overseer`), the marker's 2px left tick (`--rule-strong`, the only blue rule), and the names in `link` / `unlink` tape lines only. Never a fill, never text on holder and never a status. | Departure-holder blue: outbound traffic being handed to the next sector (`driver → driven`). |

### Surface Scale (elevation hierarchy)

A 0.2.0 frontend design input; 0.1.0 renders no page.

| Level | Value | Usage |
|-------|-------|-------|
| Base | #1E2124 | Page background (the bay), the rack gap, the tape, and bare (unwrapped) and `stale` strips. Console anthracite: a lit near-black, not a void. |
| Raised-1 | #2A2E33 | A lit wrapped session strip: the holder boot. It carries buff ink only. |
| Raised-2 | #2A2E33 + 1px #E6D8AE rule | Reserved: there are no popovers or tooltips in v1. If one is ever needed it is holder plastic with a buff rule. Lift comes from the rule, never from a third tint or a shadow. |
| Raised-3 | #2A2E33 + 2px #E6D8AE rule | Reserved for the v1.x brake confirmation, the only modal-like surface ever planned. v1 has none. |
| Inset | #1E2124 | Cells set into a holder: the DIALOG cell of a cocked strip (so the amber word keeps 5.1:1), the interior of an `open` readback box, and the expanded tape text well |

### Text Hierarchy

A 0.2.0 frontend design input; 0.1.0 renders no page.

| Level | Value | Usage |
|-------|-------|-------|
| Primary | #E6D8AE | Session names (callsign cell), every field value on a lit strip, tape kind and text, and phraseology words (`open`, `unable`, `DIALOG` kind, `budget-paused`) |
| Secondary | #9C9278 | Column captions (LIVE, STATUS…) in the rack header on anthracite, transfer-marker `since` times (the ATIS `TAPE live since` time stays buff, per `TapeConnection::open`), ATIS reading age, `n/a` on bare strips |
| Tertiary | #9C9278 | Tape timestamps (`19:43:58.123`), `activity` lines, `harness`-origin lines, the expanded-line `source`. Separated from Secondary by position (the tape, not the rack header or the ATIS line), never by a dimmer hex or a smaller size: timestamps are Data and `activity` / `harness` lines are Body, both 13px, and only the expanded body's `source` is Code (12px), as in the Typography table. |
| Muted | #9C9278 | `stale` strips (every field) and zero-count `skipped` figures. No ink is dimmer than lamp-off buff, because anything darker fails AA on anthracite. There are no disabled controls or placeholders in v1. |
| On-paper | #3B3A36 | Graphite pencil: text on a buff fill only (the `RB` in a read-back box) |

### Semantic Colors
A 0.2.0 frontend design input; 0.1.0 renders no page.

Generic UI feedback states (baseline, all four kept). viola deliberately carries them with **ink, fill, rule, strike and a printed word, never with a green/yellow/red hue**. Hue is spent only on the cocked strip (amber) and the handoff (blue).

| State | Background | Border | Text |
|-------|------------|--------|------|
| Success, confirmed after the fact (read back) | #E6D8AE for the box fill; #1E2124 behind the word `read back` | #E6D8AE | #3B3A36 for `RB` in the box; #E6D8AE for the word `read back` (graphite is never text on anthracite) |
| Warning, degraded but not refused (`unverified-cli`, `unconfirmable`, expired budget window) | #2A2E33 for `unverified-cli` (a word on a lit strip); #1E2124 for `unconfirmable` and `expired` | #9C9278 for `unverified-cli` (field grid) and `expired`; #E6D8AE for `unconfirmable` (the readback-box outline is a state border) | #E6D8AE |
| Error, "unable" plus a reason (refusal, HTTP problem) | #1E2124 | #E6D8AE (plus a 1px `/` strike where a readback box exists) | #E6D8AE |
| Info (activity, harness turn, skipped counts, tape status) | #1E2124 | none for activity, harness and zero `skipped`; #4A5057 for `TAPE connecting` / `TAPE live`; #E6D8AE where the border is the state (nonzero `skipped` box, `TAPE stopped`) | #9C9278 for activity, harness, zero `skipped` and `TAPE connecting`; #E6D8AE for `TAPE live`, `TAPE stopped` and nonzero `skipped` |

The domain status rows below are authoritative. This table only summarises them, and where a row names several cases it gives each case's value from those rows.

**Domain status colors.** A 0.2.0 frontend design input; 0.1.0 renders no page. These cover every enum in architecture.md's data model and contracts that gets a visual, one row per variant. The fifth column holds the printed word and the treatment. Colour never stands alone.

| Enum::Variant | Background | Border | Text | Printed as · treatment |
|---|---|---|---|---|
| Liveness::live | #2A2E33 | #9C9278 (field grid) | #E6D8AE | `live`. Holder strip lit. |
| Liveness::stale | #1E2124 | #4A5057 (solid 1px) | #9C9278 | `stale`. The whole strip goes lamp-off, instantly: the holder fill drops to anthracite, all ink becomes lamp-off, the band slot stays rail. Slot kept. (Gone instances are omitted by `/api/sessions`: no row, no visual.) |
| SessionStatus::idle | #2A2E33 | #9C9278 | #E6D8AE | `idle` |
| SessionStatus::busy | #2A2E33 | #9C9278 | #E6D8AE | `busy`. No spinner, pulse or animation. |
| SessionStatus::unknown | #2A2E33 | #9C9278 | #E6D8AE | `unknown` printed in the box (`claude agents --json` missing, failed or not joined) |
| WheelHolder::driver | #2A2E33 | #9C9278 | #E6D8AE | `driver`, weight 400 |
| WheelHolder::human | #2A2E33 | #9C9278 | #E6D8AE | `human`, weight 600. Pilot in command is one of the two heavier words after the callsign; the other is the cocked strip's amber `DIALOG` word (Typography, Body row). |
| WheelHolder::(absent, unwrapped) | #1E2124 | #4A5057 (dashed) | #9C9278 | `n/a`: absent by contract, not a failed reading |
| Wrapped::true | #2A2E33 | #9C9278 grid; 4px band slot #4A5057 | #E6D8AE | Strip in a holder boot, in the WRAPPED rack |
| Wrapped::false | #1E2124 | #4A5057 (1px dashed outline, no band slot) | #E6D8AE (name, live, status) / #9C9278 (`n/a`) | Bare strip, another facility's traffic, in the UNWRAPPED · READ-ONLY rack. The name is shown as text, never as a target. |
| DialogPending::false | #2A2E33 | #4A5057 band | #E6D8AE | DIALOG prints `none` |
| DialogPending::(absent, unwrapped) | #1E2124 | #4A5057 (dashed) | #9C9278 | `n/a`: absent by contract on a bare strip, which is never cocked |
| DialogPending::true | #1E2124 (DIALOG cell inset) | #D97706 (4px holder band) | #D97706 (`DIALOG`) + #E6D8AE (kind; #9C9278 when the strip is also `stale`, per the `stale` `--ink` swap, see web-spa component 1) | Cocked: `translateX(12px)` once over 160ms, amber band, `DIALOG <kind>`. Slot kept. Announced politely. |
| PendingDialogKind::question | #1E2124 | #D97706 | #D97706 / #E6D8AE | `DIALOG question` |
| PendingDialogKind::permission | #1E2124 | #D97706 | #D97706 / #E6D8AE | `DIALOG permission` |
| PendingDialogKind::plan | #1E2124 | #D97706 | #D97706 / #E6D8AE | `DIALOG plan` |
| CliVerified::true | #2A2E33 | #9C9278 | #E6D8AE | `2.1.280 verified` (known aircraft type) |
| CliVerified::false | #2A2E33 | #9C9278 | #E6D8AE | `2.1.281 unverified-cli`: transport only, dialog answers held back. No colour. |
| CliVersion::(absent) | #2A2E33 | #9C9278 | #E6D8AE | `unknown`: a failed reading on a wrapped strip |
| CliVersion::(absent, unwrapped) | #1E2124 | #4A5057 (dashed) | #9C9278 | `n/a`: absent by contract, as in the `viola list` sample |
| InstanceBudgetPaused::false | #2A2E33 | #9C9278 | #E6D8AE | The WHEEL cell prints the holder only |
| InstanceBudgetPaused::true | #2A2E33 | #9C9278 | #E6D8AE | The WHEEL cell prints `driver · budget-paused`: minimum fuel declared, new sends stop, the running turn finishes |
| InstanceBudgetPaused::(absent, unwrapped) | #1E2124 | #4A5057 | #9C9278 | Covered by `n/a` in WHEEL |
| BudgetEnvelope::reading | #1E2124 | #4A5057 | #E6D8AE figures / #9C9278 age | ATIS line `5H 62 %  7D 41 %  read 4m ago` |
| BudgetEnvelope::unknown | #1E2124 | #4A5057 | #E6D8AE | `5H unknown  7D unknown  read unknown`. Nothing is blocked. |
| BudgetEnvelope.paused::false | #1E2124 | #4A5057 | #9C9278 | `gate open` |
| BudgetEnvelope.paused::true | #1E2124 | #E6D8AE (1px box around the word) | #E6D8AE | `budget-paused`, boxed like a declared state. No amber, no red. |
| BudgetWindow::five-hour | #1E2124 | #4A5057 | #E6D8AE | Label `5H`. Detail word `five-hour` in refusals. |
| BudgetWindow::seven-day | #1E2124 | #4A5057 | #E6D8AE | Label `7D`. Detail word `seven-day` in refusals. |
| Window.used_percentage::unknown | #1E2124 | #4A5057 | #E6D8AE | `5H unknown` |
| Window.resets_at::unknown | #1E2124 | #4A5057 | #9C9278 | `resets unknown` |
| Window::expired (derived: `resets_at` has passed) | #1E2124 | #9C9278 | #E6D8AE | `5H 62 % expired`: old information shown as old |
| Readback::open (from CL-1 `send-issued`) | #1E2124 | #E6D8AE (1px) | #E6D8AE | Empty square, then the word `open` |
| Readback::read (matching `prompt-submitted`, or ledger post-condition met) | #E6D8AE | #E6D8AE | #3B3A36 in the box / #E6D8AE word | Box filled, `RB` inside, word `read back`. Instant, no transition. |
| Readback::refused (from CL-1 `send-refused`) | #1E2124 + 1px #E6D8AE `/` strike | #E6D8AE | #E6D8AE | Strike, then `unable`, then `reason · detail` |
| Readback::unconfirmable (`ok`, `confirmed:false`) | #1E2124 | #E6D8AE | #E6D8AE | The open drawing, never filled, with the word `unconfirmable` |
| RefusalReason::human-typing | #1E2124 | #E6D8AE | #E6D8AE | `unable · human-typing` |
| RefusalReason::budget-paused | #1E2124 | #E6D8AE | #E6D8AE | `unable · budget-paused` |
| RefusalReason::unverified-cli | #1E2124 | #E6D8AE | #E6D8AE | `unable · unverified-cli` |
| RefusalReason::not-delivered | #1E2124 | #E6D8AE | #E6D8AE | `unable · not-delivered` |
| RefusalReason::unknown | #1E2124 | #E6D8AE | #E6D8AE | `unable · unknown`, with the detail printed as opaque plain text |
| RefusalDetail::null | #1E2124 | #E6D8AE | #E6D8AE | Nothing after the reason (no dangling `·`) |
| RefusalDetail::manual-pause | #1E2124 | #E6D8AE | #E6D8AE | `human-typing · manual-pause` ("I have control" spoken via `viola pause`) |
| RefusalDetail::five-hour | #1E2124 | #E6D8AE | #E6D8AE | `budget-paused · five-hour` |
| RefusalDetail::seven-day | #1E2124 | #E6D8AE | #E6D8AE | `budget-paused · seven-day` |
| RefusalDetail::input-not-ready | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · input-not-ready` |
| RefusalDetail::no-prompt-submitted | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · no-prompt-submitted` |
| RefusalDetail::turn-running | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · turn-running` |
| RefusalDetail::unknown-dialog | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · unknown-dialog` |
| RefusalDetail::control-character (security plan) | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · control-character` |
| RefusalDetail::empty-text | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · empty-text` |
| EventKind::session-start | #1E2124 | none | #E6D8AE | Tape line `session-start` + cause word |
| EventKind::prompt-submitted | #1E2124 | none | #E6D8AE (#9C9278 if harness) | Folded into the open send line when it is the matching driver prompt. Otherwise its own line `prompt · <origin>` + text. |
| EventKind::turn-ended | #1E2124 | none | #E6D8AE | `turn-ended` + first line of `last_assistant_message` as plain text, or `no message` for `null`. Never fills a readback box. |
| EventKind::question | #1E2124 | none | #E6D8AE | `question  dialog 7` + first question text. No amber in the tape: history is not attention. |
| EventKind::permission | #1E2124 | none | #E6D8AE | `permission  dialog 8` + tool name. `input` only in the expanded body, as text. |
| EventKind::plan | #1E2124 | none | #E6D8AE | `plan  dialog 9` + first plan line |
| EventKind::session-end | #1E2124 | #4A5057 (1px rule under the line) | #E6D8AE | `session-end`. The rail rule closes that instance's segment. |
| EventKind::activity | #1E2124 | none | #9C9278 | Whole line lamp-off: `activity  <tool>` (log-only) |
| EventKind::link | #1E2124 | none | #5B8DB8 names / #E6D8AE word | `link  overseer → builder` |
| EventKind::unlink | #1E2124 | none | #5B8DB8 names / #E6D8AE word | `unlink  overseer → builder` |
| EventKind::wheel | #1E2124 | none | #E6D8AE | `wheel  <holder> · <cause>` |
| EventKind::budget-gate | #1E2124 | none | #E6D8AE | `budget-gate  paused · five-hour` / `budget-gate  open` |
| EventKind::send-issued (CL-1) | #1E2124 | none | #E6D8AE | `send  → builder` + text + readback box (`open`) |
| EventKind::send-refused (CL-1) | #1E2124 | none | #E6D8AE | Folded into its send line: strike + reason. Never a separate line. |
| EventKind::(unknown to this page) | #1E2124 | none | #9C9278 | `skipped  1 unknown kind`, printed at the point in the tape where it occurred |
| PromptOrigin::driver | #1E2124 | none | #E6D8AE | Matched: folded into the send's readback. Unmatched: `prompt · driver`. |
| PromptOrigin::human | #1E2124 | none | #E6D8AE | `prompt · human`, with `human` at weight 600 (the human took the wheel) |
| PromptOrigin::harness | #1E2124 | none | #9C9278 | Whole line lamp-off: `prompt · harness`. Neither party's traffic. |
| SessionStartCause::startup | #1E2124 | none | #E6D8AE | `startup` |
| SessionStartCause::clear | #1E2124 | none | #E6D8AE | `clear`. Also the post-condition that fills the readback box of a `/clear` send. |
| SessionStartCause::resume | #1E2124 | none | #E6D8AE | `resume` |
| SessionStartCause::compact | #1E2124 | none | #E6D8AE | `compact` |
| SessionStartCause::unknown | #1E2124 | none | #E6D8AE | `unknown` |
| WheelCause::start | #1E2124 | none | #E6D8AE | `driver · start` |
| WheelCause::human-input | #1E2124 | none | #E6D8AE | `human · human-input` |
| WheelCause::manual-pause | #1E2124 | none | #E6D8AE | `human · manual-pause`, i.e. "I have control" |
| WheelCause::release | #1E2124 | none | #E6D8AE | `driver · release`, i.e. "you have control" |
| EventSource::hook / wrapper / cli | #1E2124 | none | #9C9278 | `source hook` / `source wrapper` / `source cli`, only inside an expanded tape line |
| Skipped::zero (unknown_kinds / unknown_fields / torn_lines) | #1E2124 | none | #9C9278 | `skipped 0 · 0 · 0` in the ATIS line |
| Skipped::nonzero | #1E2124 | #E6D8AE (1px box around the count) | #E6D8AE | `skipped 2 unknown kinds · 0 · 1 torn line`, kept visible, never auto-cleared |
| TapeConnection::connecting (derived, EventSource) | #1E2124 | #4A5057 | #9C9278 | `TAPE connecting` |
| TapeConnection::open | #1E2124 | #4A5057 | #E6D8AE | `TAPE live since 19:40:02Z` |
| TapeConnection::closed | #1E2124 | #E6D8AE | #E6D8AE | `TAPE stopped · viola ui not answering`. Strips keep the last values, and the ATIS age keeps counting. |
| Problem::unauthorized (401) | #1E2124 | #E6D8AE | #E6D8AE | Access strip (see web-spa component 6). Never shows the token, the URL or the file path. |
| Problem::state-unreadable (503) | #1E2124 | #E6D8AE | #E6D8AE | Rack strip `unable · state-unreadable` |
| Problem::not-found (404) | #1E2124 | #E6D8AE | #E6D8AE | Rack strip `unable · not-found`. The page only requests its own routes, so this is defensive. |
| Problem::method-not-allowed (405) | #1E2124 | #E6D8AE | #E6D8AE | Never produced by the page (GET only). If seen: `unable · method-not-allowed`. |
| Problem::host-not-allowed (403) | #1E2124 | #E6D8AE | #E6D8AE | The page cannot load at all, so there is no in-page visual. The browser shows the Problem JSON. |
| Problem::cross-origin-forbidden (403, v1.x) | #1E2124 | #E6D8AE | #E6D8AE | Reserved for the brake: `unable · cross-origin-forbidden` |

Not visualised in v1:
- `/ready` checks (`ok` / `unavailable` / `error`) and `/health` status. They are probe routes for tests and obs. A `claude_agents` failure surfaces on the page as STATUS `unknown`.
- `answer` behaviours (`allow` / `deny`, `approve` / `revise`). They are not logged as events, so the tape has nothing to draw.

### Border Progression

A 0.2.0 frontend design input; 0.1.0 renders no page.

| Intensity | Value | Usage |
|-----------|-------|-------|
| Subtle | #4A5057 | Rack-rail aluminium: rack separators, the strip outer edge, the band slot of a non-cocked strip, the dashed outline of bare strips, the in-strip field grid of a `stale` strip (the `--rule-field` swap), the tape minute rule, the `session-end` rule and the rule under the ATIS header. Decorative, 2.0:1. In the domain status rows, #4A5057 on an ATIS-line value (`BudgetEnvelope`, `BudgetWindow`, `Window` other than `expired`, `gate open`, `TapeConnection::connecting` / `open`) names that header rule: those cells are never boxed. |
| Standard | #9C9278 | The printed field-grid rule between cells inside a lit strip: the rule printed on the paper, quieter than the ink. Also the 1px box around an ATIS `expired` word (`Window::expired`, on anthracite): old information is marked as old, quieter than the buff boxes of declared states. |
| Emphasis | #E6D8AE | Borders whose presence is the state: the readback box outline and strike, the `budget-paused` box, nonzero `skipped` boxes, the ATIS `TAPE stopped` box, error strips |
| Focus | #E6D8AE | 2px solid focus ring, offset 2px, radius 0 (11.4:1 on anthracite, 9.6:1 on holder) |
| Handoff | #5B8DB8 | The 2px (`--rule-strong`) left tick of the outbound and inbound transfer markers only, in the rack gap on anthracite (4.6:1). The only blue rule. Never on holder, never a strip or cell border. |

Deviation note: the Color World names strip buff for "the ruled field grid". This system uses lamp-off buff (the same paper) for the in-strip grid. That way the squint test sees the values and the readback outline first, and buff borders stay reserved for borders that are a state.
Deviation note (holder colour vs state): the exploration's Strip holder concept has a holder's colour mark the kind of traffic, never its state. Two state changes touch the holder anyway, both forced by the contrast table: `stale` drops the fill to anthracite (lamp-off ink fails AA on holder), and a cocked strip insets its DIALOG cell (amber text fails AA on holder). Kind stays readable without the fill: a `stale` wrapped strip keeps its solid rail edge and 4px band slot, and an unwrapped strip has a dashed rail outline and no band slot.

---

## Typography

A 0.2.0 frontend design input; 0.1.0 renders no page.

**Rationale:** The foundation is library pairing #1 (Fira Code + Fira Sans, "Dashboard Data"), translated to installed OS faces. The CSP resolves `font-src` to `'none'`, so there is no `@font-face`, no `local()` and no web fonts.
- **Bahnschrift** is Microsoft's DIN 1451. It ships with Windows 10 1709+, and Windows is the first target. DIN is the lettering of transport signage and of printed flight-progress strip forms (Frequentis/Saab EFS anatomy). Its semi-condensed width fits fixed strip fields, and its uppercase reads as standard phraseology.
- **Cascadia Mono** is the strip-printer and tape face. It is fixed pitch, has no ligatures (so `->`, `[RB]` and `[/ ]` render literally) and uses tabular figures. This keeps the web strip column-identical to the k9s-style CLI table.
- **Both faces are Windows-only.** D3 makes macOS and Linux supported, so each role carries an explicit per-OS fallback stack (below), and the Linux fallback is a first-class rendering, not a degraded one. The stacks end at the generic `sans-serif` / `monospace`, never `system-ui`.

| Role | Font | Weight | Size | Tracking | Usage |
|------|------|--------|------|----------|-------|
| Display | Bahnschrift stack, semi-condensed | 600 | 16px / 24px line, uppercase | 0.08em | `VIOLA` at the start of the `<viola-atis>` header (there is no hero), rendered as the page's one `<h1>` (a11y-plan D-A11Y-02). The `BAY` after it is Label and `127.0.0.1:47319` is Data, as in the ATIS anatomy |
| Heading | Bahnschrift stack, semi-condensed | 600 | 11px / 16px, uppercase | 0.12em | Rack separator labels: `WRAPPED`, `UNWRAPPED · READ-ONLY`, `TAPE`, each rendered as an `<h2>` (a11y-plan D-A11Y-02); the two rack labels sit in their rack table's `<caption>` |
| Callsign | Bahnschrift stack, normal width | 600 | 15px / 32px (strip height) | 0.02em | The NAME cell. Never text-transformed: a `ViolaName` is lower-case `[a-z0-9-]` and must read exactly as it is typed as a `target`. |
| Body | Cascadia Mono stack | 400 (600 for `human` in WHEEL and in `prompt · human`, and for the strip's `DIALOG` word) | 13px / 20px | 0 | Field values, tape text, transfer markers, phraseology words |
| Label | Bahnschrift stack, semi-condensed | 400 | 11px / 16px, uppercase | 0.06em | Column captions (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI), and `BAY`, `5H` / `7D` and `TAPE` in the ATIS header (the tape panel's `TAPE` separator is Heading). The strip's `DIALOG <kind>` value is not Label: it is Body (Cascadia 13px) with `DIALOG` at weight 600, so the 20ch DIALOG track measures it in `ch` like every other field. |
| Code | Cascadia Mono stack | 400 | 12px / 20px | 0 | Commands shown in empty states (`viola run <name> -- claude`), the expanded tape body (`white-space: pre-wrap`) |
| Data | Cascadia Mono stack | 400 (600 for `RB`) | 13px / 20px. The one exception is `RB` inside the readback box: `--fs-label` 11px, line-height 1, `--fw-strong` 600, identical in every place the box appears | `font-variant-numeric: tabular-nums` | Percentages, timestamps `HH:MM:SS.mmm`, `since`, cursors, dialog ids, ages, skipped counts |

**Loading:** System font stack only, with named installed faces referenced by `font-family`. Nothing is fetched: no web fonts, no `@font-face`, no `local()`. That keeps the page CSP-clean (`font-src` resolves to `'none'`).

**Per-OS fallback stacks (tokens):**
- `--font-label: "Bahnschrift", "DIN Alternate", "Avenir Next Condensed", "DejaVu Sans Condensed", sans-serif;`
- `--font-field: "Cascadia Mono", Consolas, "SF Mono", Menlo, "DejaVu Sans Mono", monospace;`

| Role token | Windows (live target) | macOS (CI-tested) | Linux (CI-tested) | Last resort |
|---|---|---|---|---|
| `--font-label` | Bahnschrift (DIN 1451) | "DIN Alternate" (DIN, ships with macOS), then "Avenir Next Condensed" | "DejaVu Sans Condensed" (the condensed system sans) | `sans-serif` |
| `--font-field` | "Cascadia Mono", then Consolas (on every Windows) | "SF Mono", then Menlo | "DejaVu Sans Mono" | `monospace` |

- Semi-condensed labels use `font-stretch: 87.5%`, which Bahnschrift's variable width axis honours. On the fallbacks it is a no-op: DIN Alternate, Avenir Next Condensed and DejaVu Sans Condensed are already narrow cuts.
- Strip and tape columns are sized in `ch` of `--font-field` (`--strip-cols`, `--tape-cols`), so they re-measure to whichever monospace resolves. DejaVu Sans Mono is wider than Cascadia Mono, so the Linux render is the widest case the layout must hold without horizontal scroll.

**Assertions hold on the Linux fallback.** Every contrast assertion (the Color Palette contrast rules) and every render assertion (no horizontal scroll at ≥1024px, callsign cell not clipped, `[RB]` / `->` rendered literally without ligatures, `unknown` fitting its box, tabular figures aligned) must hold with "DejaVu Sans Condensed" + "DejaVu Sans Mono" resolved, and they hold on the resolved family, not only the declared stack: a resolved face outside the stacks above (for example a banned face picked by fontconfig substitution) breaks them. 0.1.0 renders no page, so no CI step renders these stacks or provides their families; how a 0.2.0 frontend checks these assertions is that version's to decide, its technology not chosen.

**cli:** Terminal monospace (the user's configured font). The brand shows through the fixed-column structure and the phraseology words (`open`, `RB`, `unable`, `DIALOG`, `stale`). The readback mirror is `[RB]` / `[  ]` / `[/ ]`. Bold is used only for the NAME column. SGR dim marks `stale` rows, and amber the `DIALOG` word. Output is ASCII only, and density is high.

---

## Spacing

A 0.2.0 frontend design input; 0.1.0 renders no page.

| Token | Value | Usage |
|-------|-------|-------|
| space-micro | 4px | Gap between the readback box and its word cell, and between `→` and a name. Never cell padding: field-cell inline padding is `space-xs` (8px), which `--strip-cols` is sized for |
| space-xs | 8px | Field-cell inline padding in strips, the rack row gap (between strips, around transfer markers, and between the caption row and the first strip), the transfer-marker indent step |
| space-sm | 12px | Gap between ATIS header cells (the cells have no inline padding, so neighbours sit exactly 12px apart). Equals `--cock-offset`: the rack reserves this much end padding so a cocked strip never causes horizontal scroll. |
| space-md | 16px | Page inline margin, gap between rack separator label and the caption row |
| space-lg | 24px | Separation between racks, and between the bay and the tape |
| space-xl | 32px | Top page margin under the sticky ATIS header, and the strip height (`--strip-h`) |

Base unit: 4px. All values are multiples of it. This is a developer tool watched for hours beside two terminals, so strip density is tight. Geometry tokens:
- `--strip-h: 32px`
- `--line-h: 20px` (tape line and transfer marker)
- `--band-w: 4px`
- `--rb-size: 16px` (the square sits centred in a 20px line)

CLI spacing is platform-controlled: a fixed-width terminal grid, two spaces between columns.

---

## Depth Strategy

A 0.2.0 frontend design input; 0.1.0 renders no page.

**Chosen approach:** borders-only.

**Rationale:** A strip bay is flat. Paper sits in plastic boots on ruled aluminium rails, "lit only by the lamp", and elevation has no physical meaning there. Separation comes from three things, none of them shadows:
1. **Material colour.** Holder #2A2E33 on anthracite #1E2124. This is the boot, not a lift.
2. **1px rules.** Buff where the border is the state, lamp-off for the in-strip field grid, rail for decorative rack lines.
3. **The cocked strip's physical offset.** A position, never elevation.

Shadows would read as "glow" on a dark ground, which the "nothing glows unless a lamp is on it" anchor forbids.

Values:
- `--rule: 1px` (every structural and field rule)
- `--rule-strong: 2px` (the departure-blue left tick of the outbound and inbound transfer markers; the reserved v1.x Raised-3). The focus ring uses its own `--focus-w: 2px` and `--focus-offset: 2px` (Border Progression → Focus, `@layer base`), never `--rule-strong`.
- `--band-w: 4px` (holder band slot: rail normally, amber when cocked)
- `--shadow: none`. `box-shadow`, `filter: drop-shadow` and `text-shadow` never appear.
- There are no gradients, with one exception: the stylesheet `linear-gradient` that draws the refused readback strike.

---

## Border Radius

A 0.2.0 frontend design input; 0.1.0 renders no page.

| Token | Value | Usage |
|-------|-------|-------|
| radius-sm | 0px | Readback box, field cells, focus ring, the reserved v1.x brake buttons |
| radius-md | 0px | Session strips, transfer markers, access and error strips |
| radius-lg | 0px | Racks, the ATIS header, the tape panel |
| radius-full | 0px (banned value 9999px) | Not used. There are no avatars, pills or toggles. A round element would be a status dot, a rejected default. |

**Personality:** Every shape is a ruled rectangle: flight strips, holder boots and printed boxes on a strip form. Radius 0 everywhere is the intent, not a missing decision. Hierarchy comes from rule weight and material, not corner rounding.

---

## Motion (calibrated to expression level `0.3` on web-spa, base `0.2`)

All motion decisions flow from the expression level set in Brand Identity.

**Duration scale (adjusted for expression level):**

| Expression | Hover/Focus | Page transition | Entrance | Scroll effects |
|------------|-------------|-----------------|----------|----------------|
| 0.0-0.2 | 0-100ms | instant/none | none | none |
| 0.3-0.4 | 150ms ease-out | 200ms fade | none | none |
| 0.5-0.6 | 200ms ease-out | 300ms slide | stagger 50ms | subtle reveal |
| 0.7-0.8 | 250ms spring | 400ms spring | stagger 80ms | parallax |
| 0.9-1.0 | 300ms spring | 500ms+ custom | stagger 100ms+ | full scroll-driven |

---

## Iconography

A 0.2.0 frontend design input; 0.1.0 renders no page.

**Style:** Custom text glyphs only. Phraseology words carry the meaning.
- **Transfer arrows.** On the web, `→` (U+2192) marks the outbound marker (blue, with the name), `link` / `unlink` tape lines, and the target of a `send` tape line (`send  → builder`). In tape lines the arrow is buff and only link names are blue. `←` (U+2190) marks only the inbound twin (`← overseer`, arrow and name in blue). The CLI uses ASCII `->` and has no inbound form. `viola list` carries no link column in v1, so the six columns shared with the web strip stay fixed. Links print only as `link` / `unlink` verb output (`overseer -> builder`).
- **Readback strike.** A CSS `linear-gradient` inside the box.
- **Separator.** Middle dot `·` on the web. The CLI is ASCII only:
  - Two spaces separate columns, and a refusal's reason from its detail (they are separate fields, as in the exit-code table).
  - ` - ` (space, hyphen, space) replaces `·` inside one field or label: `driver - budget-paused`, `skipped 0 - 0 - 0`, `-- UNWRAPPED - READ-ONLY --`.

**Library:** None. `img-src 'self'` rules out CDN icon sets. SVG through `dangerouslySetInnerHTML` and inline SVG markup are banned, and no icon font can load (`font-src 'none'`).

**Size grid:** Glyphs are 1em of the text that carries them (13px), with a 4px (`--space-micro`) gap. The readback box is a 16px square, drawn with borders, not a glyph.

**Rule:** icons clarify, not decorate. If removing an icon loses no meaning, remove it. In viola there are no robot or person icons, no status dots, no checkmarks and no warning triangles. Every one of those states is already a printed word.

---

## Surface: web-spa

The frontend is deferred to 0.2.0; its technology is not chosen.

---

## Surface: cli

**Platform:**
- Windows 10/11 (Windows Terminal, conhost), macOS and Linux terminals.
- stdout/stderr of the `viola` binary.
- Callers: the human on a TTY, and LLM driver sessions via `--json` / MCP.

**Toolkit / Framework:**
- clap 4.6.7 (derive) built without its `color` feature, Rust stable: clap's help and usage output is plain in every mode, and no dependency reads a colour or terminal variable, so every styled byte of human output is viola's own SGR module's.
- TTY detection uses std `IsTerminal`.
- Windows VT enabling uses windows-sys 0.61.2 `SetConsoleMode(ENABLE_VIRTUAL_TERMINAL_PROCESSING)`, already in the stack. If it fails, output has no colour.
- Styling is a small hand-written SGR module in the `viola` bin. It adds no dependency.

### Tokens (platform-specific)

| Token | Truecolor (`COLORTERM=truecolor\|24bit`) | 256-colour | 16-colour fallback | Use |
|---|---|---|---|---|
| attention (arrival amber #D97706) | `\x1b[38;2;217;119;6m` | `\x1b[38;5;172m` | `\x1b[33m` | The `DIALOG` word only, always followed by the kind word |
| stale (lamp-off) | `\x1b[2m` (SGR dim) | `\x1b[2m` | `\x1b[2m` | A whole `stale` row. The word `stale` is always in LIVE. On a row that is also cocked, only the `DIALOG` word is amber and not dim (the cock wins for `DIALOG`). The kind word and the rest of the row stay dim. NAME on a stale row is dim and bold, and every span inside the row opens dim again after its reset. |
| callsign | `\x1b[1m` (bold) | `\x1b[1m` | `\x1b[1m` | NAME column values |
| reset | `\x1b[0m` | `\x1b[0m` | `\x1b[0m` | After every styled span |
| handoff blue #5B8DB8 | not used | not used | not used | The quiz limits CLI colour to amber and dim, so link output (`overseer -> builder`) is plain at every depth. No index is reserved: any blue SGR would break the ban on colouring anything but `DIALOG` and `stale`. |
| backgrounds | never set | never set | never set | The terminal background belongs to the user |

**Colour decision order** (the first match wins, and results in no colour, no glyph and no SGR; here, as in the 0.0 expression row and the cli bans, a glyph is a non-ASCII symbol, so the ASCII readback mirror `[RB]` / `[  ]` / `[/ ]` and `->` are words that still print in every human output, piped, `NO_COLOR` and `TERM=dumb` included, and only `--json` replaces them with its JSON document):
1. `--json`
2. `viola run` (while the child runs, nothing is printed at all)
3. `viola hook` (never writes stderr) and `viola mcp` (stdout carries MCP frames only)
4. `NO_COLOR` set (present, an empty value included)
5. `TERM=dumb`
6. stdout is not a terminal
7. Windows VT enable failed

Otherwise the depth is chosen once per process, first match wins:
- truecolor if `COLORTERM` is `truecolor` or `24bit`;
- 256-colour if `TERM` contains `256color`, or on Windows once VT enabling has succeeded;
- otherwise the 16-colour fallback column.

**Streams:**
- stdout: results and data (tables, send/wait results, `last` text).
- stderr: context lines (`waiting: <name>`, the issue line of a send), refusals (`unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column, as in its sample; the exit-code table omits the `<name>` field; only the exit-1 refusals of `viola run`, `viola revive` and `viola verify`, and `viola list`'s exit-21 strict-modes refusal (chunk 2026-10-11-the-board-viola-list, the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer; the operator's answer superseded 2026-10-13), use the fixed message form, `unable: <text>`, as in `unable: <name> is already live`), each refusal's `hint:` line directly after it, and errors (`error: …`).
- For `--json`, the whole result, including refusals and `error` objects, is one JSON document on stdout with the typed exit code. No `hint:` line is printed under `--json`: the typed `refusal` / `error` fields are the whole answer.

**Width:**
- Where the width is detectable, human tables truncate only the NAME of unwrapped rows (`...`). The width is stdout's own terminal size, read by `viola_pty::host_size()` (Windows `GetConsoleScreenBufferInfo`, a `TIOCGWINSZ` ioctl elsewhere); `COLUMNS` is not read, and a piped stdout has no width (chunk 2026-10-17-cli-output-discipline; the width source is the Viola overseer's technical answer, `inputs#I1` Q2 of that chunk, a delegate's — PROVISIONAL, the founder's own word owed). The CLI has no tape.
- Otherwise nothing is truncated. Lines are never wrapped.
- Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).

### Component Patterns

**1. `viola list`: the k9s strip table**
```
BAY  5H 62 %  7D 41 %  read 4m ago  gate open  skipped 0 - 0 - 0
NAME          LIVE   STATUS   WHEEL                    DIALOG             CLI
builder       live   busy     driver                   none               2.1.280 verified
overseer      live   idle     human                    DIALOG permission  2.1.280 verified
scratch       stale  unknown  driver - budget-paused   none               2.1.281 unverified-cli
-- UNWRAPPED - READ-ONLY --
c1f9e2a4      live   idle     n/a                      n/a                n/a
```
- NAME is bold, `DIALOG` is amber, and the `scratch` row is dim.
- Rows are ordered by name.
- The BAY line is the ATIS without `resets` times (those are printed by `viola release <name> --budget`) and without a TAPE cell (the CLI has no tape). A window whose `resets_at` has passed still prints `expired` after its figure (`5H 62 % expired`), because old information is shown as old.
- Every field of `claude agents --json` origin (unwrapped names) has C0/C1 controls escaped as `\x1B`-style hex text (security plan). In `list` table rows, `\n` and `\t` are escaped too (`\x0A`, `\x09`), so every row stays one fixed-width line. The security plan's "keeping `\n` and `\t`" is a floor, and this is stricter (T5, ratified). `wait` / `last` message text keeps `\n` and `\t`.
- Empty: `no wrapped sessions  start one: viola run <name> -- claude`.
- **As built** (chunk 2026-10-11-the-board-viola-list; the agents read, chunk 2026-10-16-unwrapped-sessions-on-the-board; the colour layer and the width cut, chunk 2026-10-17-cli-output-discipline). A wrapped row joined to a `claude agents --json` row (by child pid or agent session id) reads STATUS `idle` or `busy`, else `unknown`. When the read lists a session no wrapped row joins, the `-- UNWRAPPED - READ-ONLY --` line follows the wrapped rows (or the empty-board line when there are none), then one row per unwrapped session sorted by name: NAME the row's `sessionId`, LIVE `live`, its STATUS, `n/a` in WHEEL, DIALOG and CLI; with no unwrapped row no separator prints, and a failed, oversize or malformed read prints the board as with no `claude`. On a terminal whose width is known (`viola_pty::host_size()`) NAME's column is the largest of the caption's 4, the longest wrapped NAME and the longest unwrapped NAME capped at the width less the 65 columns the rest of an unwrapped row takes; an unwrapped NAME longer than the column is cut to the column less 3 characters and ends in `...` (the sample's `c1f9e2a4...`), the cut falling before an escape it would split; a wrapped NAME is never cut, and a piped board or `--json` keeps every name whole. On a terminal that takes colour (the decision order above) every NAME value is bold; the `DIALOG` word of a wrapped row with a pending dialog takes the attention token, its kind word plain; a wrapped `stale` row is dim from its first byte to its last, NAME dim and bold, and on a stale row with a dialog only `DIALOG` is amber and not dim; the BAY line, the caption, the empty-board line and the separator carry no SGR; the spans are laid over the escaped, padded cells, so the columns are the plain board's, and the board stays one write. The row mode escapes every control character of a row, DEL and C1 included, as `\xHH` (`escape_row`); as built its upstream-derived cells are the recorded CLI version and each unwrapped NAME, and NAME's width is counted in characters. WHEEL is padded to 23, the widest word `unknown - budget-paused` (a replayed row with a logged pause and no logged holder): implement's decision, confirmed by the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer (the operator's acceptance superseded 2026-10-13). Five forms are the founder's wording: planning forms on the operator's word that he confirmed as built in the same answer (superseded 2026-10-13): a window's figure rounded down to a whole number (`89.6` → `89 %`); the reading's age in whole minutes, rounded down at every age (`read 0m ago`, `read 75m ago`), and `read unknown` for no instant, one that does not parse, or one ahead of now; nonzero skip counts as bare numbers in key order (`skipped 2 - 0 - 1`); a `human` holder under a budget pause prints `human - budget-paused`; the empty board prints the BAY line and the caption row above its one line.
- **Refusal** (exit 21; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer; the operator's answer at that chunk's planning, superseded 2026-10-13): a failed strict-modes check of the home, `instances/` or any one instance refuses the whole board, nothing from state printed, in the fixed `unable:` / `hint:` form of the start refusals (pattern 2): the home → `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions` (`list` is this pair's first writer in source); an instance → `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only` (`revive`'s pair, shared).

**2. `viola send`: the readback mirror**
```
[  ] open           builder  issued 19:43:58.912Z                 (stderr, TTY only)
[RB] read back      builder  19:43:59.004Z  cursor 48213          (stdout, exit 0)
[/ ] unable         builder  not-delivered  no-prompt-submitted   (stderr, exit 13)
hint: builder did not submit the prompt; check it, then send again
[  ] unconfirmable  builder  local command, no measured post-condition   (stdout, exit 0)
```
- A refusal prints `unable` plus `reason  detail`, then one `hint:` line, keyed by the reason (or by reason · detail where listed):
  - `human-typing`: `the human has the wheel; send again after the human hands it back`
  - `budget-paused`: `budget gate is closed; it lifts at the window reset, or when the human lifts it`
  - These two refusals reach drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20). So no hint ever names `viola release` (T3). `release` stays a human verb, documented under Wheel, link and gate verbs.
  - `unverified-cli`: `run viola verify for this CLI version`
  - `not-delivered · turn-running`: `a turn is running; viola wait <name> first`
  - `not-delivered · control-character`: `the text contains a control character (only LF, CR, TAB are allowed)`
  - `not-delivered · empty-text`: `the text is empty once its trailing newlines are removed; send a text with content`
  - `not-delivered · input-not-ready`: `<name> was not ready for input; send again, and if it repeats a human must look at the session`
  - `not-delivered · no-prompt-submitted`: `<name> did not submit the prompt; check it, then send again` (the sample above)
  - `not-delivered · unknown-dialog`: `that dialog is not pending; viola list shows the current DIALOG`
  - `instance-unreachable` (exit 21) has one hint per cause, because the code is shared (T4):
    - no wrapper is running for the name: `<name> is not running; viola list shows the live instances`
    - the name is an unwrapped session: `<name> is not a wrapped session; only sessions started with viola run take commands`
    - the viola home fails strict-modes: `the viola home is not private to you; <name> is not contacted until that is fixed`
    - server verification fails: `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`
  - The exit-1 start refusals of `viola run` have one fixed-message line and one hint per cause (T4). None of them shows a path or a pid:
    - `unable: <name> is already live` → `hint: viola list`
    - `unable: <name> is still running but not answering` (stale heartbeat, live pid) → `hint: viola list shows it as stale; stop that process before starting <name> again`
    - `unable: the endpoint for <name> is held by another process` (squatted name) → `hint: another process holds this name's endpoint; stop it or pick another name`
    - `unable: the pinned viola copy failed its integrity check` (SHA-256 mismatch) → `hint: the pinned copy was changed after it was written, so viola will not run it`
    - `unable: <name>'s command is a .cmd or .bat script` (Windows) → `hint: pass the real executable, not a .cmd or .bat shim`
    - `unable: the viola home is not private to you` (strict-modes at start) → `hint: the viola home must be readable only by you; viola does not change its permissions`
  - The exit-1 refusals of `viola revive` take the same form, one fixed-message line and one hint per cause, read in a fixed order that stops at the first refusal. None of them shows the recorded directory, a pid or a logged id, and no hint names `viola release`:
    - `unable: <name>'s state files can be written by another user` (strict-modes on the instance's files) → `hint: viola will not read them; make the viola home and its files owner-only`
    - `unable: <name> is already live` and its stale sibling: `run`'s two lines and hints above, unchanged
    - `unable: <name> has no logged session to resume` → `hint: start one: viola run <name> -- claude`
    - `unable: <name> has no logged session with that id` (an `--id` the log does not hold) → `hint: viola revive <name> --list`
    - `unable: <name>'s session is open in another claude` (one `claude agents --json` read lists the logged id live in another `claude`; skipped under `--fork`; a failed read passes) → `hint: close it there first, or start a copy: viola revive <name> --fork` (the founder's wording, his own answer to P4 fork 4 of chunk 2026-10-16-unwrapped-sessions-on-the-board, `inputs#I1`, relayed verbatim by the operator)
    - `unable: <name>'s recorded directory is missing` → `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`
  - Hints are human-mode only. Under `--json`, no hint text is printed; the machine view carries the cause as a detail code, never as prose. **Pending an arch amendment:** arch currently fixes `{"v":1,"error":"instance-unreachable","detail":null}` for exit 21, and for exit 1 only `viola revive`'s `{"v":1,"error":"<code>","detail":null}` (its strict-modes and preflight refusals; chunk 2026-10-17-cli-machine-contract). Until arch names the `detail` codes, `--json` keeps arch's shape. obs-plan D-20 already names the log detail codes: `instance-dead`, `strict-modes-failed`, `server-verify-failed`, `already-live`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, and for `viola revive` `no-session` and `cwd-missing` (its strict-modes refusal writes no log line since chunk 2026-10-10-budget-governor, so it carries no logged detail code: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1). The stale-heartbeat and unwrapped-name causes have no code yet.
  - No hint line for `unknown` (exit 14), because its detail is opaque and a hint would have to guess. No hint line for `error: wrapper fault` (exit 20) or `error: internal error` (exit 1), because they are faults, not refusals, and their detail goes only to `diagnostics/`.
- A hint never quotes the sent text or any upstream text.

**3. `viola wait` / `viola last`**
- `wait` prints one static `waiting: builder` line on stderr (TTY only), then one result line on stdout:
  - `turn-ended  builder  19:44:10.221Z  cursor 49102`; `session-end` and every other non-dialog kind take the same `<kind>  <name>  <time>  cursor <n>` form
  - `question  builder  dialog 7  cursor 49310` (`DIALOG` is not coloured here: this is a result, not the board); `dialog unknown` when the event carries no `dialog_id`
  - `timed out  builder  30000 ms`
- There is no spinner and no elapsed-time counter.
- `last` prints `last  builder  turn-ended 19:44:10.221Z` on stderr (TTY only), then the message text on stdout with C0/C1 escaped, or `no message` on stderr for `null`.
- Message mode, the escaper both verbs share: over decoded characters every control character (C0, DEL, C1) except `\n` and `\t` prints as `\xHH` text, two uppercase hex digits (ESC → `\x1B`, CR → `\x0D`, DEL → `\x7F`, U+009B → `\x9B`); nothing is stripped. `--json` output stays serde-escaped.

**4. Wheel, link and gate verbs** (the phraseology is the design)
- `viola pause builder` → `builder  wheel human  manual-pause  I have control`
- `viola release builder` → `builder  wheel driver  you have control`
- `viola release builder --budget` → `builder  budget gate lifted until 21:00:00Z  wheel human` when a pause stood. The time is the override's end, taken from the reply's `override_until`: `HH:MM:SSZ` when it falls on the current UTC date, `YYYY-MM-DD HH:MM:SSZ` on any other date, and a value that is no RFC 3339 instant prints escaped. When no pause stood the line is `builder  budget gate open  wheel human`. Both are one unstyled stdout line with the two-space gutter of the other verb lines. The date form and the `budget gate open` line are the founder's wording: planning forms as built (chunk 2026-10-10-budget-governor, on the operator's word) that he confirmed as they were shown to him, his own answer, 2026-10-13, relayed verbatim by the Viola overseer (the operator's word superseded 2026-10-13).
- `viola link overseer builder` → `link    overseer -> builder  since 19:43:58Z` (an existing pair prints `link    overseer -> builder  already linked`)
- `viola unlink overseer builder` → `unlink  overseer -> builder`
- `viola answer builder 7 < response.json` → `answered  builder  dialog 7`, or `unable  builder  not-delivered  unknown-dialog`
- `viola plugin install` → four stdout lines in one write: `viola@viola  written  <dir>` / `run these to add it to Claude Code:` / `  claude plugin marketplace add <dir>` / `  claude plugin install viola@viola`, `<dir>` the marketplace dir, quoted only where the host's shell would split it; plain ASCII, no SGR. A planning form as built, the founder's wording owed.

**5. `viola verify` / `viola run`**
- **`verify`:** a step counter (the project's one progress pattern) with static appended stdout lines, one per relied ledger row: `[01/23] shim-resolution claude resolves to a real executable  pass` / `... fail` (`MM` is the relied count, `LedgerRow::relied()`: 23 on Unix, 20 on Windows, where the three statusline rows are not relied because the status line is not wrapped there; the members of `LedgerRow::ALL` in its order, after the six spine and four screen and timing rows the dialog rows, the framing rows, the newline row, on Unix the statusline rows, then the resume row and the agents row, both relied on every OS: `question-answer` a question answered through PreToolUse takes effect · `plan-approve-revise` a plan revise and approve each take effect · `question-notes` free text and notes reach the question · `dialog-concurrency` two parallel questions each raise a dialog · `long-paste-wrapper` a long paste unwraps to the text as pasted · `tag-escaping` tag-like text un-escapes to the text as pasted · `local-command-clear` /clear starts a new session and submits no prompt · `paste-newline` an LF inside a paste stays inside the one prompt submitted · `statusline-override` a --settings statusLine runs in the status line's place · `statusline-shell` a statusline command runs through /bin/sh · `statusline-rate-limits` the statusline payload carries two whole-number rate_limits windows · `resume-session` --resume <id> reopens the session under that id · `agents-join` claude agents --json names the live session by pid and session id; the count grows as owning chunks land their rows). Then the last stdout line, a summary, n + m equal to `MM`: `stamped 2.1.287  23 pass  0 fail` on Unix. The word `fail` is uncoloured. `viola verify --help` carries one static ASCII paragraph naming no path: run it from a folder you trust in Claude Code, and an unapproved external CLAUDE.md import blocks the probe. A run stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary: stderr carries exactly `unable: verify was stopped before it finished` then `hint: run viola verify again to stamp this CLI` (pattern 2's fixed form, plain ASCII, no path or pid), and it exits 1. The pair is a planning form on the operator's word (chunk 2026-10-13-interrupted-verify-cleanup); the founder's wording is owed.
- **`run`:** prints nothing once the child starts. A start refusal (exit 1) prints its cause's fixed-message line, for example `unable: builder is already live`, then that cause's hint (`hint: viola list`). The per-cause lines are listed under the send hints above. None of them shows a path or a pid.
- **`revive`:** prints nothing once the child starts, as `run`. `viola revive builder --list` prints one static stdout line per logged session, in log order, `<time>  <cause>  <session id>` with two spaces between fields: no header, no colour, plain ASCII. A refusal (exit 1) prints its cause's fixed-message line and then that cause's hint, listed under the send hints above. Under `--json` (both arms; chunk 2026-10-17-cli-machine-contract) a refused strict-modes check or a preflight refusal prints one stdout document `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`) and no hint, stderr empty, and `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}`; a refusal `run`'s start makes after a passed preflight prints no document as landed (owed to the working-route entry "Exit-cause code catalogue").

**Exit-code phraseology** (typed codes from architecture.md):

| Exit | Human stderr first word | `--json` |
|---|---|---|
| 0 | none (TTY context lines only); the result line goes to stdout (`[RB] read back`, `answered`, `wheel human` …) | `{"v":1,"ok":{…}}` |
| 1 | `error: internal error` (fixed message; no chain with serde sources, no paths, no hint), or a `viola run` start refusal `unable: <name> is already live` / `… is still running but not answering` / `the endpoint for <name> is held by another process` / `the pinned viola copy failed its integrity check` / `<name>'s command is a .cmd or .bat script` / `the viola home is not private to you`, or a `viola revive` refusal `unable: <name>'s state files can be written by another user` / `<name> has no logged session to resume` / `<name> has no logged session with that id` / `<name>'s session is open in another claude` / `<name>'s recorded directory is missing` (or `run`'s `already live` pair), or a `viola verify` refusal `unable: the claude CLI was not found` / `the claude CLI is a .cmd or .bat script` / `the CLI version could not be read` / `a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`; codes `home-path` · `absolute-path` · `username` · `email`; never the content), each followed by its own hint, or a `viola verify` stopped by Ctrl-C or TERM before its rows are checked, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` (stdout empty); a `verify` run with a failing ledger row prints no stderr word (its last stdout line reads `stamped <ver>  <n> pass  <m> fail`) | `viola revive --json`'s refusals of its strict-modes check and its preflight: `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`; arch §Standard Contracts); every other exit 1 has no `--json` document (a `run` start refusal reached through `revive` included, owed to "Exit-cause code catalogue") |
| 2 | clap usage text (never produced by `viola hook`) | — |
| 10 | `unable  human-typing` (+ `manual-pause`) | `{"v":1,"refusal":"human-typing","detail":…}` |
| 11 | `unable  budget-paused  five-hour` / `seven-day` | refusal object |
| 12 | `unable  unverified-cli` | refusal object |
| 13 | `unable  not-delivered  input-not-ready` / `no-prompt-submitted` / `turn-running` / `unknown-dialog` / `control-character` / `empty-text` | refusal object |
| 14 | `unable  unknown  <detail as opaque text>` | refusal object |
| 20 | `error: wrapper fault  <code>` | `{"v":1,"error":"wrapper-fault","detail":{…}}` |
| 21 | `unable  instance-unreachable` (no detail; the name is the omitted `<name>` field, as in every row), then the cause's hint (not running / unwrapped / strict-modes / server verification); `viola list`'s refused strict-modes check prints instead the fixed home-state or instance-state `unable:` / `hint:` pair (pattern 1, Refusal; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer) | `{"v":1,"error":"instance-unreachable","detail":null}` (a per-cause `detail` code awaits an arch amendment), `list`'s refusal included, with stderr empty |

`viola plugin install`'s refused strict-modes check (exit 21) prints the home-state `unable:` / `hint:` pair as `viola list`'s does, or under `--json` the `instance-unreachable` document, stderr empty.

### Navigation Pattern
- Flat verbs, one lower-case word each: `run · revive · send · wait · last · list · answer · verify · pause · release · link · unlink · plugin install`.
- `viola list` is the board and the other verbs act on one strip. There is no interactive prompt, no TUI mode and no pager.
- `viola --help` groups verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install` (`revive` beside `run`: the operator's answer at the chunk 2026-10-10-viola-revive wrap). As built (chunk 2026-10-17-cli-output-discipline) it lists the served verbs only: `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `setup: run, revive, verify, plugin install`, then `Options:`; the `handoff:` row joins when "Explicit session links" serves `link` and `unlink` (served verbs only is the Viola overseer's technical answer, `inputs#I1` Q3 of that chunk, a delegate's — PROVISIONAL, the founder's own word owed).

### Platform-Specific Notes
- **Git Bash on Windows** rewrites leading-slash arguments. Prompt text therefore comes only from stdin or `--file`. The warning for a rewritten-path argument is one plain stderr line, `warning: argument looks like a Git Bash rewritten path`.
- **conhost without VT** falls back to no colour. The words already carry every state, so nothing is lost.
- **`hook`** writes nothing to stderr and always exits 0. **`mcp`** writes only MCP frames on stdout. Neither has a human design surface. `mcp` is a hidden verb (chunk 2026-10-17-mcp-server-for-drivers): run by the plugin, never by a person, it appears in no `viola --help` row.
- **Stack traces** never print. Errors are fixed messages, and full detail goes only to `instances/<name>/diagnostics/`.

---

## Anti-Patterns (NEVER do these)

### Universal Bans
- **NEVER use color purely for decoration. Every color communicates meaning.** Amber means `dialog_pending` and nothing else. Blue means handoff and nothing else. Any third hue has no domain meaning.
- **NEVER converge on common "safe" choices across generations.** The readback box and the cocked strip exist because agent dashboards converge on dots, toasts and graphs.

### Rejected Defaults (from exploration)
- **Traffic-light status dots** (green `live`, yellow `busy`, red `stale`). Rejected because a strip prints its status word in a fixed column. `stale` is a coasting track (lamp-off), not red, and the bay's only colour state is the cocked strip's amber beside its `DIALOG` word.
- **Optimistic "Sending…" spinner, then a green "Sent ✓" or a toast.** Rejected because a clearance does not count until it is read back. The readback box stays `open` until the matching `prompt-submitted` (for `/clear`, its new-session post-condition), then fills instantly or is struck with `unable` plus the typed reason. There is no green, no toast and no presumption.
- **Auto-sorting "needs input" or recent sessions to the top.** Rejected because a controller cocks a strip in place. Re-ranking breaks the founder's "where builder lives" memory, and the cock (12px offset plus amber band) already says "needs you".
- **Budget as circular rings or green/yellow/red bars.** Rejected because fuel state is figures in ruled fields (`5H 62 %  7D 41 %`), always with the ATIS age (`read 4m ago`). `budget-paused` is a phraseology word, not a colour.
- **Robot and person icons or emoji for the wheel, and a clickable-looking toggle.** Rejected because the WHEEL field prints `human` / `driver`. Taking control is spoken (`viola pause`, "I have control"), never implied by a switch.
- **Skeleton shimmer and a terminal-green or neon-glow "hacker" theme.** Rejected because nothing glows in a tower cab unless the lamp is on it. Missing values print `unknown` in their box at once.

### Per-Surface Bans

**cli**
- **NEVER use green `✓` / red `✗` prefixes or emoji.** The readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
- **NEVER colour anything except `DIALOG` (amber) and `stale` rows (dim), and never either without its word.** The quiz confirms colour as a second cue only.
- **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.** LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen; a passed `viola revive` is the same passthrough.
- **NEVER use a spinner, progress bar or live-redrawing dashboard.** `wait` prints one static `waiting:` line and `verify` prints step-counter lines, because a spinner is progress presumed before it is confirmed.
- **NEVER mix data and messages.** Results go to stdout. `waiting:`, refusals and their `hint:` lines, and errors go to stderr.
- **NEVER print upstream text, paths, pids or anyhow chains with serde sources in errors or hints.** This is the security plan's NEVER-log floor, and human output of `last` / `list` escapes C0/C1 controls.
- **NEVER print "done" or "success" without context.** Say what was read back, by whom, and at which cursor (`[RB] read back      builder  19:43:59.004Z  cursor 48213`, padded as in the `viola send` sample).
- **NEVER hardcode widths or wrap at arbitrary points.** Truncate only where the terminal width is known, and never wrap.
- **NEVER print stack traces.** Full detail goes to `diagnostics/` only.

---

## Self-Validation Protocol

Before presenting ANY UI output, downstream implementation phases (per project's specialist plans) must run these checks:

### 1. Swap Test
Replace your typeface with Inter, your colors with Tailwind defaults, your
layout with a standard sidebar+cards. If the design doesn't feel
meaningfully different → you defaulted. Redo with intent.

### 2. Squint Test
Blur your eyes at the interface. Can you still perceive hierarchy? Does
anything jump out harshly? Good craft whispers — nothing should scream.
In viola, `open` and `unconfirmable` readback boxes look identical under the squint (`unconfirmable` reuses the open drawing and is never filled); only the word cell tells them apart. That is intended, not a failed squint.

### 3. Signature Test
Point to the readback box in all three required places: (1) the far right of every send line in `<viola-event-feed>`, (2) the far right of every outbound `<viola-transfer>` marker that has seen a send since page open (before its first send, a marker deliberately prints only the word `unknown` and no box, per web-spa component 3; that is not a missing signature), and (3) the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`. On the web, places 1 and 2 must render the same `<viola-readback>` element under the same `.rb` rule: a 16px square, a 1px buff outline, a buff fill with `RB` in graphite at 11px 600, and a 1px `/` strike. The tape box and the marker box must therefore be indistinguishable. If any place lacks the box or draws it differently, the signature does not exist. Inject it.

### 4. Token Test
A 0.2.0 frontend design input; 0.1.0 renders no page.

Every colour traces to one of the eight `--c-*` values (#1E2124, #2A2E33, #4A5057, #E6D8AE, #9C9278, #3B3A36, #D97706, #5B8DB8). `--c-*` is read only where a role alias is assigned or reassigned (`@layer tokens`, and the `stale` swap in `@layer states`). Components use the role aliases (`--surface-*`, `--ink*`, `--rule-*`, `--attention`, `--handoff`, `--focus-ring`, `--rb-*`) and never `--c-*`. Every length traces to a token (`--space-*`, `--fs-*`, `--track-*` (em), `--lh-display`, `--lh-label`, `--strip-h`, `--line-h`, `--band-w`, `--rb-size`, `--rule`, `--rule-strong`, `--radius`, `--cock-offset`, `--focus-w`, `--focus-offset`) or to a `ch` track in `--strip-cols` / `--strip-cols-wrapped` / `--tape-cols`, with the wrapped strip's rows in `--strip-rows-wrapped`. Every duration is `--cock-dur` or `--fade-dur`. The only literal lengths allowed are the two Navigation → Width breakpoints (`760px`, `1024px`), because `@media` conditions cannot read custom properties. A ninth hex value or any other untokened px value signals no system.

### 5. Sameness Test (from Interface Design)
If another AI given a similar prompt would produce substantially the same
output — you have failed. The interface must emerge from THIS product's
domain exploration, not from statistical patterns in training data.

---

## Design Decisions Log

History: design-system-amendments.md (live); the log moved verbatim to design-system-amendments-archive.md (U35).

