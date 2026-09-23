# Design System — viola

## Brand Identity

**Personality:** Tower strip-bay handoff, read back: standard-phraseology precision under low cab light. Every word is short and fixed, every send stays open until the far end reads it back, and nothing glows unless the lamp is on it.

**Domain anchors:**
- **Flight-progress strip.** One strip per `viola run` instance. The fields are fixed, in the order NAME · LIVE · STATUS · WHEEL · DIALOG · CLI. The name cell is set at callsign weight. A field with no reading prints `unknown`.
- **Strip bay and racks with labelled separators.** Every session keeps a stable slot and nothing is re-sorted.
- **Cocked strip.** `dialog_pending` is the strip pushed a finger-width out of line, with an arrival-holder amber band.
- **Readback.** A send counts only once the matching `prompt-submitted` reads it back. Otherwise it is "unable" plus a typed reason.
- **Handoff / transfer of control, and the tower voice recorder.** `driver → driven` transfer markers, and the ndjson tape played in order with `skipped` counts kept where they happened.
- **Strip holder (plastic boot).** Wrapped instances sit in a holder. Unwrapped sessions are bare strips: another facility's traffic, visible but never a `target`.
- **Coasting track.** `stale` liveness. The strip is dimmed to lamp-off, never deleted, and keeps its slot.
- **Fuel state / minimum fuel.** The budget governor: `5H` / `7D` figures, with `budget-paused` as a declared minimum-fuel state that stops new sends while the running turn finishes.
- **ATIS information with its age.** Every budget reading carries its age (`read 4m ago`). This is the `<viola-atis>` header. Old information is shown as old (`expired`), never as current.
- **Verified aircraft type.** A CLI version stamped by `viola verify` prints `verified`. An unstamped build flies transport only as `unverified-cli`.
- **Pilot-in-command, "I have control".** `human` in WHEEL at weight 600. `viola pause` / `viola release` are the spoken "I have control" / "you have control".
- **Standard phraseology and "unable" plus a reason.** Short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`) are the vocabulary of every label, CLI column and refusal. There is no filler and no guessing.

**Signature element:** **The readback box.** It is a fixed-width, 1px-ruled square cell at the far right of every send line in `<viola-event-feed>` and of every outbound transfer marker. It has three drawn states, and each state is printed as a word too:
- **`open`:** a buff outline on anthracite.
- **`RB` / `read back`:** filled solid buff, with `RB` in graphite. It fills instantly when the matching `prompt-submitted` lands.
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
| cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs. |

This number is the SINGLE SOURCE OF TRUTH for how much motion and visual dynamism downstream phases implement. At 0.2 the duration scale's 0.0–0.2 row applies (0–100ms, no entrance), and viola's cli (0.2 on a TTY, 0.0 otherwise) has no motion at all. viola's web page is 0.3, but its only two motions are the ones in the table above, and Motion's "This project's values" override the 0.3–0.4 row (see the Design Decisions Log).

**Design direction:**
The page is a control-tower strip bay after dark. Buff paper strips sit in anthracite holder boots on ruled aluminium racks. Every session holds the same slot for its whole life, so the eye goes to where `builder` always is. The only warm light in the bay is the one arrival holder cocked out of line when a session needs you.

Nothing announces success before it is confirmed. Sends stay visibly open until they are read back. Missing readings print `unknown` in their box instead of shimmering. Old information always carries its age, like an ATIS letter. The density is the calm density of a strip board, not a monitoring dashboard: fixed fields, tabular numbers, one attention colour and one handoff colour. Everything else is paper and ink.

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
| Primary | #E6D8AE | Primary ink, focus rings, information-carrying borders (readback box outline, `budget-paused` box), the read-back fill | Strip buff: the paper flight-progress strip under the gooseneck task lamp. It is the thing the controller reads, and the readback box is drawn on it. |
| Secondary | #9C9278 | Secondary ink on anthracite (timestamps, reading ages, column captions, `n/a`), the printed field-grid rule inside strips, and all ink of a `stale` strip | Lamp-off buff: the same paper after the lamp moves off. A coasting track is dimmed, not deleted. An ATIS age is printed small. |
| Accent | #D97706 | The only attention colour: the 4px holder band and the `DIALOG` word of a cocked strip, and the CLI `DIALOG` column. The `DIALOG builder · viola` document title is plain text with no colour. Never used for errors, warnings, budget or hover. | Arrival-holder amber: the orange holder cocked out of line. It is the physical "needs you". |
| Role (handoff) | #5B8DB8 | Transfer-marker arrows and names (outbound `→ builder`, inbound `← overseer`), the marker's 2px left tick (`--rule-strong`, the only blue rule), and the names in `link` / `unlink` tape lines only. Never a fill, never text on holder and never a status. | Departure-holder blue: outbound traffic being handed to the next sector (`driver → driven`). |

### Surface Scale (elevation hierarchy)
| Level | Value | Usage |
|-------|-------|-------|
| Base | #1E2124 | Page background (the bay), the rack gap, the tape, and bare (unwrapped) and `stale` strips. Console anthracite: a lit near-black, not a void. |
| Raised-1 | #2A2E33 | A lit wrapped session strip: the holder boot. It carries buff ink only. |
| Raised-2 | #2A2E33 + 1px #E6D8AE rule | Reserved: there are no popovers or tooltips in v1. If one is ever needed it is holder plastic with a buff rule. Lift comes from the rule, never from a third tint or a shadow. |
| Raised-3 | #2A2E33 + 2px #E6D8AE rule | Reserved for the v1.x brake confirmation, the only modal-like surface ever planned. v1 has none. |
| Inset | #1E2124 | Cells set into a holder: the DIALOG cell of a cocked strip (so the amber word keeps 5.1:1), the interior of an `open` readback box, and the expanded tape text well |

### Text Hierarchy
| Level | Value | Usage |
|-------|-------|-------|
| Primary | #E6D8AE | Session names (callsign cell), every field value on a lit strip, tape kind and text, and phraseology words (`open`, `unable`, `DIALOG` kind, `budget-paused`) |
| Secondary | #9C9278 | Column captions (LIVE, STATUS…) in the rack header on anthracite, `since` times, ATIS reading age, `n/a` on bare strips |
| Tertiary | #9C9278 | Tape timestamps (`19:43:58.123`), `activity` lines, `harness`-origin lines, the expanded-line `source`. Separated from Secondary by size (11–12px mono) and position, never by a dimmer hex. |
| Muted | #9C9278 | `stale` strips (every field) and zero-count `skipped` figures. No ink is dimmer than lamp-off buff, because anything darker fails AA on anthracite. There are no disabled controls or placeholders in v1. |
| On-paper | #3B3A36 | Graphite pencil: text on a buff fill only (the `RB` in a read-back box) |

### Semantic Colors
Generic UI feedback states (baseline, all four kept). viola deliberately carries them with **ink, fill, rule, strike and a printed word, never with a green/yellow/red hue**. Hue is spent only on the cocked strip (amber) and the handoff (blue).

| State | Background | Border | Text |
|-------|------------|--------|------|
| Success, confirmed after the fact (read back) | #E6D8AE | #E6D8AE | #3B3A36 |
| Warning, degraded but not refused (`unverified-cli`, `unconfirmable`, expired budget window) | #2A2E33 for `unverified-cli` (a word on a lit strip); #1E2124 for `unconfirmable` and `expired` | #9C9278 for `unverified-cli` (field grid) and `expired`; #E6D8AE for `unconfirmable` (the readback-box outline is a state border) | #E6D8AE |
| Error, "unable" plus a reason (refusal, HTTP problem) | #1E2124 | #E6D8AE (plus a 1px `/` strike where a readback box exists) | #E6D8AE |
| Info (activity, harness turn, skipped counts, tape status) | #1E2124 | none for activity, harness and zero `skipped`; #4A5057 for `TAPE connecting` / `TAPE live`; #E6D8AE where the border is the state (nonzero `skipped` box, `TAPE stopped`) | #9C9278 for activity, harness, zero `skipped` and `TAPE connecting`; #E6D8AE for `TAPE live`, `TAPE stopped` and nonzero `skipped` |

The domain status rows below are authoritative. This table only summarises them, and where a row names several cases it gives each case's value from those rows.

**Domain status colors.** These cover every enum in architecture.md's data model and contracts that gets a visual, one row per variant. The fifth column holds the printed word and the treatment. Colour never stands alone.

| Enum::Variant | Background | Border | Text | Printed as · treatment |
|---|---|---|---|---|
| Liveness::live | #2A2E33 | #9C9278 (field grid) | #E6D8AE | `live`. Holder strip lit. |
| Liveness::stale | #1E2124 | #4A5057 (solid 1px) | #9C9278 | `stale`. The whole strip goes lamp-off, instantly: the holder fill drops to anthracite, all ink becomes lamp-off, the band slot stays rail. Slot kept. (Gone instances are omitted by `/api/sessions`: no row, no visual.) |
| SessionStatus::idle | #2A2E33 | #9C9278 | #E6D8AE | `idle` |
| SessionStatus::busy | #2A2E33 | #9C9278 | #E6D8AE | `busy`. No spinner, pulse or animation. |
| SessionStatus::unknown | #2A2E33 | #9C9278 | #E6D8AE | `unknown` printed in the box (`claude agents --json` missing, failed or not joined) |
| WheelHolder::driver | #2A2E33 | #9C9278 | #E6D8AE | `driver`, weight 400 |
| WheelHolder::human | #2A2E33 | #9C9278 | #E6D8AE | `human`, weight 600. Pilot in command is the one heavier value after the callsign. |
| WheelHolder::(absent, unwrapped) | #1E2124 | #4A5057 (dashed) | #9C9278 | `n/a`: absent by contract, not a failed reading |
| Wrapped::true | #2A2E33 | #9C9278 grid; 4px band slot #4A5057 | #E6D8AE | Strip in a holder boot, in the WRAPPED rack |
| Wrapped::false | #1E2124 | #4A5057 (1px dashed outline, no band slot) | #E6D8AE (name, live, status) / #9C9278 (`n/a`) | Bare strip, another facility's traffic, in the UNWRAPPED · READ-ONLY rack. The name is shown as text, never as a target. |
| DialogPending::false | #2A2E33 | #4A5057 band | #E6D8AE | DIALOG prints `none` |
| DialogPending::true | #1E2124 (DIALOG cell inset) | #D97706 (4px holder band) | #D97706 (`DIALOG`) + #E6D8AE (kind) | Cocked: `translateX(12px)` once over 160ms, amber band, `DIALOG <kind>`. Slot kept. Announced politely. |
| PendingDialogKind::question | #1E2124 | #D97706 | #D97706 / #E6D8AE | `DIALOG question` |
| PendingDialogKind::permission | #1E2124 | #D97706 | #D97706 / #E6D8AE | `DIALOG permission` |
| PendingDialogKind::plan | #1E2124 | #D97706 | #D97706 / #E6D8AE | `DIALOG plan` |
| CliVerified::true | #2A2E33 | #9C9278 | #E6D8AE | `2.1.280 verified` (known aircraft type) |
| CliVerified::false | #2A2E33 | #9C9278 | #E6D8AE | `2.1.281 unverified-cli`: transport only, dialog answers held back. No colour. |
| CliVersion::(absent) | #2A2E33 | #9C9278 | #E6D8AE | `unknown` |
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
| Intensity | Value | Usage |
|-----------|-------|-------|
| Subtle | #4A5057 | Rack-rail aluminium: rack separators, the strip outer edge, the band slot of a non-cocked strip, the dashed outline of bare strips, the tape minute rule. Decorative, 2.0:1. |
| Standard | #9C9278 | The printed field-grid rule between cells inside a lit strip: the rule printed on the paper, quieter than the ink |
| Emphasis | #E6D8AE | Borders whose presence is the state: the readback box outline and strike, the `budget-paused` box, nonzero `skipped` boxes, error strips |
| Focus | #E6D8AE | 2px solid focus ring, offset 2px, radius 0 (11.4:1 on anthracite, 9.6:1 on holder) |
| Handoff | #5B8DB8 | The 2px (`--rule-strong`) left tick of the outbound and inbound transfer markers only, in the rack gap on anthracite (4.6:1). The only blue rule. Never on holder, never a strip or cell border. |

Deviation note: the Color World names strip buff for "the ruled field grid". This system uses lamp-off buff (the same paper) for the in-strip grid. That way the squint test sees the values and the readback outline first, and buff borders stay reserved for borders that are a state.

---

## Typography

**Rationale:** The foundation is library pairing #1 (Fira Code + Fira Sans, "Dashboard Data"), translated to installed OS faces. The CSP resolves `font-src` to `'none'`, so there is no `@font-face`, no `local()` and no web fonts.
- **Bahnschrift** is Microsoft's DIN 1451. It ships with Windows 10 1709+, and Windows is the first target. DIN is the lettering of transport signage and of printed flight-progress strip forms (Frequentis/Saab EFS anatomy). Its semi-condensed width fits fixed strip fields, and its uppercase reads as standard phraseology.
- **Cascadia Mono** is the strip-printer and tape face. It is fixed pitch, has no ligatures (so `->`, `[RB]` and `[/ ]` render literally) and uses tabular figures. This keeps the web strip column-identical to the k9s-style CLI table.
- **Both faces are Windows-only.** D3 makes macOS and Linux supported, and the headless GUI checks in CI run on ubuntu. So each role carries an explicit per-OS fallback stack (below), and the Linux fallback is a first-class rendering, not a degraded one: it is what CI renders. The stacks end at the generic `sans-serif` / `monospace`, never `system-ui`.

| Role | Font | Weight | Size | Tracking | Usage |
|------|------|--------|------|----------|-------|
| Display | Bahnschrift stack, semi-condensed | 600 | 16px / 24px line, uppercase | 0.08em | The bay header: `VIOLA  BAY 127.0.0.1:47319` (there is no hero) |
| Heading | Bahnschrift stack, semi-condensed | 600 | 11px / 16px, uppercase | 0.12em | Rack separator labels: `WRAPPED`, `UNWRAPPED · READ-ONLY`, `TAPE` |
| Callsign | Bahnschrift stack, normal width | 600 | 15px / 32px (strip height) | 0.02em | The NAME cell. Never text-transformed: a `ViolaName` is lower-case `[a-z0-9-]` and must read exactly as it is typed as a `target`. |
| Body | Cascadia Mono stack | 400 | 13px / 20px | 0 | Field values, tape text, transfer markers, phraseology words |
| Label | Bahnschrift stack, semi-condensed | 400 | 11px / 16px, uppercase | 0.06em | Column captions (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI), `5H` / `7D`, and `TAPE` in the ATIS cell (the tape panel's `TAPE` separator is Heading). The strip's `DIALOG <kind>` value is not Label: it is Body (Cascadia 13px) with `DIALOG` at weight 600, so the 20ch DIALOG track measures it in `ch` like every other field. |
| Code | Cascadia Mono stack | 400 | 12px / 20px | 0 | Commands shown in empty states (`viola run <name> -- claude`), the expanded tape body (`white-space: pre-wrap`) |
| Data | Cascadia Mono stack | 400 (600 for `RB`) | 13px / 20px. The one exception is `RB` inside the readback box: `--fs-label` 11px, line-height 1, `--fw-strong` 600, identical in every place the box appears | `font-variant-numeric: tabular-nums` | Percentages, timestamps `HH:MM:SS.mmm`, `since`, cursors, dialog ids, ages, skipped counts |

**Loading:** System font stack only, with named installed faces referenced by `font-family`. Nothing is fetched: no web fonts, no `@font-face`, no `local()`. That keeps the page CSP-clean (`font-src` resolves to `'none'`).

**Per-OS fallback stacks (tokens):**
- `--font-label: "Bahnschrift", "DIN Alternate", "Avenir Next Condensed", "DejaVu Sans Condensed", sans-serif;`
- `--font-field: "Cascadia Mono", Consolas, "SF Mono", Menlo, "DejaVu Sans Mono", monospace;`

| Role token | Windows (live target) | macOS (CI-tested) | Linux (CI-tested; **the CI render**) | Last resort |
|---|---|---|---|---|
| `--font-label` | Bahnschrift (DIN 1451) | "DIN Alternate" (DIN, ships with macOS), then "Avenir Next Condensed" | "DejaVu Sans Condensed" (the condensed system sans) | `sans-serif` |
| `--font-field` | "Cascadia Mono", then Consolas (on every Windows) | "SF Mono", then Menlo | "DejaVu Sans Mono" | `monospace` |

- Semi-condensed labels use `font-stretch: 87.5%`, which Bahnschrift's variable width axis honours. On the fallbacks it is a no-op: DIN Alternate, Avenir Next Condensed and DejaVu Sans Condensed are already narrow cuts.
- Strip and tape columns are sized in `ch` of `--font-field` (`--strip-cols`, `--tape-cols`), so they re-measure to whichever monospace resolves. DejaVu Sans Mono is wider than Cascadia Mono, so the Linux render is the widest case the layout must hold without horizontal scroll.

**Assertions hold on the Linux fallback.** The headless GUI checks run on ubuntu in CI, so the Linux stack is what CI renders. Every contrast assertion (the Color Palette contrast rules) and every render assertion (no horizontal scroll at ≥1024px, callsign cell not clipped, `[RB]` / `->` rendered literally without ligatures, `unknown` fitting its box, tabular figures aligned) must pass with "DejaVu Sans Condensed" + "DejaVu Sans Mono" resolved. The check asserts the resolved family, not only the declared stack: a resolved face outside the stacks above (for example a banned face picked by fontconfig substitution) fails the check. The CI image must therefore provide the DejaVu families (on Debian/Ubuntu, the Sans Mono cut is in `fonts-dejavu-core` and the Condensed cut in `fonts-dejavu-extra`); a missing family is a CI setup failure, not a design fallback.

**cli:** Terminal monospace (the user's configured font). The brand shows through the fixed-column structure and the phraseology words (`open`, `RB`, `unable`, `DIALOG`, `stale`). The readback mirror is `[RB]` / `[  ]` / `[/ ]`. Bold is used only for the NAME column. SGR dim marks `stale` rows, and amber the `DIALOG` word. Output is ASCII only, and density is high.

---

## Spacing

| Token | Value | Usage |
|-------|-------|-------|
| space-micro | 4px | Gap between the readback box and its word cell, and between `→` and a name. Never cell padding: field-cell inline padding is `space-xs` (8px), which `--strip-cols` is sized for |
| space-xs | 8px | Field-cell inline padding in strips, rack gap between strips, the transfer-marker indent step |
| space-sm | 12px | Gap between ATIS header cells (the cells have no inline padding, so neighbours sit exactly 12px apart). Equals `--cock-offset`: the rack reserves this much end padding so a cocked strip never causes horizontal scroll. |
| space-md | 16px | Page inline margin, gap between rack separator label and first strip |
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

**Chosen approach:** borders-only.

**Rationale:** A strip bay is flat. Paper sits in plastic boots on ruled aluminium rails, "lit only by the lamp", and elevation has no physical meaning there. Separation comes from three things, none of them shadows:
1. **Material colour.** Holder #2A2E33 on anthracite #1E2124. This is the boot, not a lift.
2. **1px rules.** Buff where the border is the state, lamp-off for the in-strip field grid, rail for decorative rack lines.
3. **The cocked strip's physical offset.** A position, never elevation.

Shadows would read as "glow" on a dark ground, which the "nothing glows unless a lamp is on it" anchor forbids.

Values:
- `--rule: 1px` (every structural and field rule)
- `--rule-strong: 2px` (focus ring; the reserved v1.x Raised-3)
- `--band-w: 4px` (holder band slot: rail normally, amber when cocked)
- `--shadow: none`. `box-shadow`, `filter: drop-shadow` and `text-shadow` never appear.
- There are no gradients, with one exception: the stylesheet `linear-gradient` that draws the refused readback strike.

---

## Border Radius

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

**Easing:**
- The cock uses `cubic-bezier(0.2, 0, 0, 1)` (ease-out): a hand pushes the strip out, and it stops against the finger.
- The fade uses `linear`.

**Duration scale (adjusted for expression level):**

| Expression | Hover/Focus | Page transition | Entrance | Scroll effects |
|------------|-------------|-----------------|----------|----------------|
| 0.0-0.2 | 0-100ms | instant/none | none | none |
| 0.3-0.4 | 150ms ease-out | 200ms fade | none | none |
| 0.5-0.6 | 200ms ease-out | 300ms slide | stagger 50ms | subtle reveal |
| 0.7-0.8 | 250ms spring | 400ms spring | stagger 80ms | parallax |
| 0.9-1.0 | 300ms spring | 500ms+ custom | stagger 100ms+ | full scroll-driven |

**This project's values:**
- **Micro-interactions (hover, focus):** 0ms. The focus ring appears instantly. Hover on links is an underline only, with no colour change and no transition.
- **Transitions (panel open, page change):** 0ms. It is a single page with no panels. A native `<details>` tape line expands instantly.
- **Entrance animations:** only newly arrived tape lines and newly created transfer markers. They fade in with opacity 0 → 1 over 120ms linear, via `@starting-style`, and only on elements the page marks `data-live` (SSE arrivals after first paint). The initial load renders instantly.
- **Scroll effects:** none. The tape follows the bottom only while the reader is already at the bottom. Otherwise a static line `N new lines below` is printed and nothing jumps.

**High-impact moments** (max 1, on web-spa too: the cock. The readback close is listed only because it is deliberately not animated. The web surface's two motions from the Brand Identity table are the cock and the ambient tape-line / marker fade under Entrance animations above, and nothing more. The 0.3–0.4 row of the duration scale is overridden for viola, as logged in the Design Decisions Log: its 150ms hover and 200ms page fade are banned (0ms hover, no page transition), and its Entrance `none` is replaced by the one 120ms fade. Where that row and this section disagree, this section wins):
- **The cock.** When a strip's `dialog_pending` turns true, `<viola-session-row data-dialog="pending">` moves `translateX(12px)` over 160ms ease-out, once. The amber band and `DIALOG <kind>` appear with it. The transition is declared only on the `pending` selector, so the return into line is instant: the controller squares the strip once it is dealt with.
- **The readback close.** This moment is deliberately not animated. Outline to filled `RB`, or outline to strike, happens in 0ms, because acknowledgement is discrete. There is no transition, flash or glow.

**Hard limits for this expression level:**
- **Banned:** spinners, pulses, blinking, skeleton shimmer, looped animation, staggered entrances, hover colour transitions, parallax, spring physics, 3D transforms, scroll-driven animation, canvas/WebGL, and any animation library (framer-motion / GSAP / Lottie / Motion One).
- **Banned triggers:** motion on `busy`, on `stale`, on `turn-ended`, or on any success before it is confirmed.
- **Reduced motion.** `@media (prefers-reduced-motion: reduce)` sets `--cock-dur` and `--fade-dur` to `0ms`. The 12px offset position and the amber band remain, because they are state, not decoration.

---

## Iconography

**Style:** Custom text glyphs only. Phraseology words carry the meaning.
- **Transfer arrows.** On the web, `→` (U+2192) marks the outbound marker (blue, with the name), `link` / `unlink` tape lines, and the target of a `send` tape line (`send  → builder`). In tape lines the arrow is buff and only link names are blue. It also marks `←` (U+2190) marks only the inbound twin (`← overseer`). The CLI uses ASCII `->` and has no inbound form. `viola list` carries no link column in v1, so the six columns shared with the web strip stay fixed. Links print only as `link` / `unlink` verb output (`overseer -> builder`).
- **Readback strike.** A CSS `linear-gradient` inside the box.
- **Separator.** Middle dot `·` on the web. The CLI is ASCII only:
  - Two spaces separate columns, and a refusal's reason from its detail (they are separate fields, as in the exit-code table).
  - ` - ` (space, hyphen, space) replaces `·` inside one field or label: `driver - budget-paused`, `skipped 0 - 0 - 0`, `-- UNWRAPPED - READ-ONLY --`.

**Library:** None. `img-src 'self'` rules out CDN icon sets. `unsafeSVG` and inline SVG markup are banned, and no icon font can load (`font-src 'none'`).

**Size grid:** Glyphs are 1em of the text that carries them (13px), with a 4px (`--space-micro`) gap. The readback box is a 16px square, drawn with borders, not a glyph.

**Rule:** icons clarify, not decorate. If removing an icon loses no meaning, remove it. In viola there are no robot or person icons, no status dots, no checkmarks and no warning triangles. Every one of those states is already a printed word.

---

## Surface: web-spa

**Platform:**
- The `viola ui` page on `http://127.0.0.1:47319` (loopback only).
- Windows 11 / 10 first (Edge / Chrome), plus macOS and Linux (Chromium, Firefox, Safari 17.5+).
- The primary viewport is ≥1024px wide, beside two terminals.
- The page is verified in a headless browser by default.

**Toolkit / Framework (from tooling-decisions.md):**
- Lit 3.3.3, vendored ESM, embedded via `include_bytes!`, with no JS build step.
- Hand-written modern CSS (native nesting, `@layer`, custom properties) in `/assets/app.css`.
- No component library: plain custom `viola-*` elements over semantic HTML.
- Every `viola-*` element renders into light DOM (`createRenderRoot(){ return this; }`) so `/assets/app.css` applies. Lit `static styles` are not used in v1 until the headless check proves that constructable stylesheets pass `style-src 'self'` plus Trusted Types.

### Tokens (platform-specific)

```css
/* /assets/app.css — served from include_str!, style-src 'self' */
@layer tokens, base, components, states, motion;

@layer tokens {
  :root {
    color-scheme: dark;                      /* dark only (Q6): no prefers-color-scheme switch */

    /* Color World — the only eight hex values in the codebase */
    --c-anthracite: #1E2124;                 /* console laminate: bay, gap, tape, inset */
    --c-holder:     #2A2E33;                 /* holder boot: lit wrapped strip */
    --c-rail:       #4A5057;                 /* rack rail: decorative rules only */
    --c-buff:       #E6D8AE;                 /* strip paper: ink, info borders, RB fill, focus */
    --c-graphite:   #3B3A36;                 /* pencil: text on buff only */
    --c-lampoff:    #9C9278;                 /* lamp moved off: secondary ink on anthracite, field grid, stale */
    --c-amber:      #D97706;                 /* arrival holder: dialog_pending ONLY */
    --c-departure:  #5B8DB8;                 /* departure holder: transfer markers ONLY */

    /* role aliases (components use these, never --c-*) */
    --surface-bay:    var(--c-anthracite);
    --surface-strip:  var(--c-holder);
    --surface-inset:  var(--c-anthracite);
    --ink:            var(--c-buff);
    --ink-dim:        var(--c-lampoff);     /* only on anthracite */
    --ink-on-paper:   var(--c-graphite);
    --rule-deco:      var(--c-rail);
    --rule-field:     var(--c-lampoff);
    --rule-info:      var(--c-buff);
    --attention:      var(--c-amber);        /* text only on anthracite */
    --handoff:        var(--c-departure);    /* text only on anthracite */
    --focus-ring:     var(--c-buff);

    /* typography — installed faces only, no web fonts (font-src 'none').
       per-OS order: Windows → macOS → Linux (the CI render) → generic */
    --font-label: "Bahnschrift", "DIN Alternate", "Avenir Next Condensed", "DejaVu Sans Condensed", sans-serif;
    --font-field: "Cascadia Mono", Consolas, "SF Mono", Menlo, "DejaVu Sans Mono", monospace;
    --fs-display: 16px;  --fs-callsign: 15px;  --fs-field: 13px;  --fs-code: 12px;  --fs-label: 11px;
    --fw-regular: 400;   --fw-strong: 600;
    --track-display: 0.08em; --track-rack: 0.12em; --track-label: 0.06em; --track-callsign: 0.02em;
    --stretch-label: 87.5%;

    /* spacing (base 4px) + strip geometry */
    --space-micro: 4px;  --space-xs: 8px;  --space-sm: 12px;
    --space-md: 16px;    --space-lg: 24px; --space-xl: 32px;
    --strip-h: 32px;  --line-h: 20px;  --band-w: 4px;
    /* each fixed track = longest printed word + 2 × --space-xs (≈2ch at 13px) + 1px --rule-field, rounded up:
       LIVE `stale` 5→8ch · STATUS `unknown` 7→10ch · DIALOG `DIALOG permission` 17→20ch ·
       WHEEL `driver · budget-paused` / CLI `2.1.281 unverified-cli` 22→25ch.
       Sum of maxima 120ch ≈ 940px in DejaVu Sans Mono (the widest case) + 4px band
       + 2 × 16px page margin + 12px --cock-offset ≈ 988px < 1024px: no horizontal scroll. */
    --strip-cols: var(--band-w) minmax(16ch, 32ch) 8ch 10ch minmax(10ch, 25ch) 20ch minmax(10ch, 25ch);
    /*            band           NAME            LIVE STATUS WHEEL             DIALOG CLI */
    /* 760–1023px (Navigation → Width): the strip wraps to two lines and uses these instead of --strip-cols / --strip-h.
       Row 1 = band NAME LIVE STATUS DIALOG at --strip-h (the callsign line); row 2 = WHEEL under NAME + LIVE and
       CLI under STATUS + DIALOG at --line-h; the band spans both rows. Sum of maxima 70ch ≈ 548px in DejaVu Sans Mono
       + 4px band + 2 × 16px page margin + 12px --cock-offset ≈ 596px < 760px, so NAME always resolves to 32ch,
       WHEEL gets 32 + 8 = 40ch and CLI gets 10 + 20 = 30ch: both hold their 22ch words plus padding. */
    --strip-cols-wrapped:  var(--band-w) minmax(16ch, 32ch) 8ch 10ch 20ch;
    --strip-rows-wrapped:  var(--strip-h) var(--line-h);     /* 52px wrapped strip block-size */
    --strip-areas-wrapped: "band name  live  status dialog" "band wheel wheel cli cli";
    /* tape cells have no inline padding: tracks are separated by column-gap var(--space-xs). TIME 12ch holds
       `HH:MM:SS.mmm` exactly; KIND 17ch holds `prompt · harness` (16ch); word / reason 46ch holds the longest typed
       refusal `unable · not-delivered · no-prompt-submitted` (44ch). At 1024px: 91ch ≈ 713px in DejaVu Sans Mono
       + 16px RB box + 5 × 8px gaps + 2 × 16px page margin ≈ 801px, so TEXT gets ≈ 28ch. TEXT never drops below 12ch;
       when the word / reason track resolves narrower than its word (760–1023px, or an opaque `unknown` detail), the
       cell ends in an ellipsis and the full `unable · <reason> · <detail>` is printed in the line's expanded body. */
    --tape-cols: 12ch minmax(8ch, 16ch) 17ch minmax(12ch, 1fr) var(--rb-size) minmax(8ch, 46ch);
    /*           TIME INSTANCE          KIND TEXT            RB box         word / reason */

    /* depth + radius */
    --rule: 1px;  --rule-strong: 2px;  --shadow: none;  --radius: 0px;

    /* motion (Q4 budget) */
    --cock-offset: 12px;  --cock-dur: 160ms;  --cock-ease: cubic-bezier(0.2, 0, 0, 1);
    --fade-dur: 120ms;    --fade-ease: linear;

    /* the readback box (signature) */
    --rb-size: 16px;
    --rb-rule: var(--c-buff);
    --rb-fill: var(--c-buff);
    --rb-ink:  var(--c-graphite);
    --rb-strike: var(--c-buff);

    /* focus */
    --focus-w: 2px;  --focus-offset: 2px;
  }
  @media (prefers-reduced-motion: reduce) {
    :root { --cock-dur: 0ms; --fade-dur: 0ms; }
  }
}

@layer base {
  html { background: var(--surface-bay); color: var(--ink);
         font: var(--fw-regular) var(--fs-field)/var(--line-h) var(--font-field); }
  :focus-visible { outline: var(--focus-w) solid var(--focus-ring);
                   outline-offset: var(--focus-offset); border-radius: var(--radius); }
  a { color: inherit; text-decoration: none; }
  a:hover, a:focus-visible { text-decoration: underline; }
}

@layer components {
  viola-readback { display: inline-grid; grid-template-columns: var(--rb-size) auto;
                   gap: var(--space-micro); align-items: center; }
  viola-readback .rb { inline-size: var(--rb-size); block-size: var(--rb-size);
                       border: var(--rule) solid var(--rb-rule); background: var(--surface-inset);
                       font: var(--fw-strong) var(--fs-label)/1 var(--font-field);
                       display: grid; place-items: center; }
}

@layer states {
  /* instant: no transition anywhere in this layer */
  viola-readback[data-rb="read"] .rb    { background: var(--rb-fill); color: var(--rb-ink); }
  viola-readback[data-rb="refused"] .rb { background:
      linear-gradient(to bottom right,          /* isoline runs bottom-left → top-right = "/" */
        transparent calc(50% - 0.5px), var(--rb-strike) calc(50% - 0.5px),
        var(--rb-strike) calc(50% + 0.5px), transparent calc(50% + 0.5px)),
      var(--surface-inset); }
  /* data-rb="open" and data-rb="unconfirmable": base outline only */

  viola-session-row[data-liveness="stale"] { --surface-strip: var(--c-anthracite);   /* data-live is reserved for SSE arrivals (@layer motion) */
      --ink: var(--c-lampoff); --rule-field: var(--c-rail); }
  viola-session-row[data-dialog="pending"] { transform: translateX(var(--cock-offset)); }
  viola-session-row[data-dialog="pending"] .band { background: var(--attention); }
  viola-session-row[data-dialog="pending"] .f-dialog { background: var(--surface-inset); }
  viola-session-row[data-dialog="pending"] .f-dialog .word { color: var(--attention); }
}

@layer motion {
  viola-session-row[data-dialog="pending"] {             /* into pending only; return is instant */
    transition: transform var(--cock-dur) var(--cock-ease); }
  .tape-line[data-live], viola-transfer[data-live] {
    transition: opacity var(--fade-dur) var(--fade-ease);
    @starting-style { opacity: 0; } }
}

@media (forced-colors: active) {
  viola-readback[data-rb="refused"] .rb { border-style: dashed; }   /* gradient strike is dropped; the word still says "unable" */
  viola-session-row[data-dialog="pending"] .band { background: Highlight; forced-color-adjust: none; }
}
```

State lives only in `data-*` attributes set by Lit. The following are never used: `style="…"`, Lit `styleMap`, and runtime `<style>` injection (CSP `style-src 'self'`).

### Component Patterns

**1. `<viola-session-row>`: the flight-progress strip** (primitive; bootstrap first)
- **Anatomy:** `display: grid; grid-template-columns: var(--strip-cols); block-size: var(--strip-h);` on the holder surface #2A2E33, with a 1px `--rule-deco` outer edge and 1px `--rule-field` dividers between cells.
  - Cell 1 is the 4px `.band` slot, which is rail normally and amber when cocked.
  - NAME is callsign type (Bahnschrift 600 15px, buff, as typed).
  - The other cells are Cascadia 13px buff values with 8px inline padding.
- **Values printed:**
  - LIVE: `live` | `stale`
  - STATUS: `idle` | `busy` | `unknown`
  - WHEEL: `driver` | `human` (600), with ` · budget-paused` appended when the instance's `budget_paused` is true
  - DIALOG: `none` | `DIALOG question` / `DIALOG permission` / `DIALOG plan`
  - CLI: `<version> verified` | `<version> unverified-cli` | `unknown`
- **Never blank.** An absent reading prints `unknown`. Absent-by-contract fields on unwrapped rows print `n/a`.
- **Column captions** appear once per rack, in a header row on anthracite: Bahnschrift 11px uppercase, lamp-off, `role="columnheader"`. They are never repeated inside strips, because lamp-off on holder fails AA. The header row uses the same grid as the strips under it: `--strip-cols` at ≥1024px, and at 760–1023px `--strip-cols-wrapped` with `--strip-areas-wrapped` on two Label lines (11px / 16px): NAME · LIVE · STATUS · DIALOG, then WHEEL under NAME + LIVE and CLI under STATUS + DIALOG. The band track stays empty, and captions take the field cells' `space-xs` inline padding, so every caption starts where its value starts.
- **States:**
  - Default (live, lit).
  - `stale`: an instant lamp-off swap (see the `states` layer).
  - Cocked: `data-dialog="pending"`.
  - Cocked + `stale`: the cock wins for the offset, the amber band, the inset DIALOG cell and the amber `DIALOG` word. The kind word follows the `stale` `--ink` swap and prints lamp-off (5.2:1 on the inset anthracite), like every other cell. This is the one exception to the `DialogPending::true` kind colour #E6D8AE.
  - Hover: none.
  - Active: n/a.
  - Focus: the row is not focusable in v1. There are no controls.
  - Disabled: n/a.
- **Unwrapped variant:** `data-wrapped="false"`. Anthracite surface, 1px dashed rail outline, no band slot. The name renders as a text binding only, never as a link or a target.
- **Semantics:** light-DOM `role="row"` inside a `role="table"` rack, with cells as `role="cell"`. Lit `${}` text bindings only.

**2. `<viola-readback>`: the readback box** (signature; bootstrap first)
- **Anatomy:** a 16px square `.rb` (1px buff rule, `aria-hidden="true"`) plus a word cell (Cascadia 13px buff on anthracite). It always sits at the far right of its line: issue on the left, readback on the right, read as one unit.
- **`data-rb` values:**

  | `data-rb` | Box | Word cell |
  |---|---|---|
  | `open` | Empty outline on anthracite | `open` |
  | `read` | Solid buff with `RB` in graphite, 11px 600 | `read back` |
  | `refused` | Outline + 1px `/` strike | `unable · <reason> · <detail>` |
  | `unconfirmable` | The open drawing | `unconfirmable` |

- Every change is instant, with no transition.
- **Sources:**
  - `open` ← CL-1 `send-issued`.
  - `read` ← the next `prompt-submitted` with origin `driver` in that instance's log, or `session-start` cause `clear` with a new session id for a `/clear` send. At most one send is in flight per instance, because a send during a running turn is refused `turn-running`.
  - `refused` ← CL-1 `send-refused` (`refusal` + `detail`).
  - `unconfirmable` ← the send's `ok` payload with `confirmed:false`.
- **Accessible name:** carried by the word cell. The whole send line reads, for example, "send to builder, read back".
- **Data dependency:** `unconfirmable`, and the `from` needed for the outbound marker, must be carried by the CL-1 events (`send-issued` with `cursor` and `from`, `send-refused`, plus an outcome record for `ok`/`confirmed:false`). This extends cross-lane follow-up CL-1 and is recorded for wrap's reconcile.
- **Until CL-1 lands:** the page can show only `read` boxes. It prints `unknown` in the marker's last-send word cell rather than inventing an `open`.

**3. `<viola-transfer>`: the transfer marker** (handoff)
- **Anatomy:** a 20px (`--line-h`) line in the rack gap on anthracite, directly under the driver's strip. It is its own item in the rack's flow, so the rack's `space-xs` gap applies above and below it (strip · 8px · marker · 8px · next marker or strip). The inbound twin follows the same rule. When a marker appears or is removed, the strips below shift instantly by that amount and their order never changes. It is indented by `--band-w + --space-xs` and has a 2px departure-blue left tick. There is one marker per link, ordered by driven name.
- **Content:**
  - `→ builder`, with the arrow and name in #5B8DB8.
  - `since 19:43:58Z` in lamp-off tabular-nums.
  - The last send's `<viola-readback>` at the far right. Before any send is seen since page open: no box, and the word `unknown`.
- **Inbound twin:** the driven strip gets `← overseer` (blue) with no box. It is its own 20px line in the rack gap on anthracite, directly under the driven strip, with the same indent and 2px left tick as the outbound marker. It is never printed inside the strip, because blue on holder is 3.9:1 and fails AA.
- **Behaviour:** a new marker fades in at ≤120ms, and an `unlink` removes it instantly. There is no node graph and no drawn edge between strips.

**4. `<viola-event-feed>`: the tower tape**
- **Anatomy:** a `role="log"` panel on anthracite. Each line is a `<details class="tape-line">` (the `.tape-line` that `@layer motion` fades). Its `<summary>` carries `display: grid; grid-template-columns: var(--tape-cols); column-gap: var(--space-xs)` and the 20px (`--line-h`) row, so the cells below are grid items of the summary. The expanded body sits under the summary across the full line width. The summary's default disclosure marker is not drawn, and no replacement glyph is added.
  - TIME: `HH:MM:SS.mmm` UTC, lamp-off, tabular-nums. The panel header says `UTC`.
  - INSTANCE: buff, truncated with an ellipsis at 16ch.
  - KIND: the event-kind word.
  - TEXT: the first line of the content, one line with `text-overflow: ellipsis`.
  - The RB box and word or reason, on send lines only. The whole `<viola-readback>` is one grid item spanning the last two tracks (`grid-column: -3 / -1`). Its own inline-grid (`var(--rb-size) auto`, `space-micro` gap) puts the box at the start of the `--rb-size` track and the word after it in the word / reason track, so the tape box is the same element and drawing as the transfer-marker box. Other lines let TEXT span to the end.
- **Per-kind treatment:** see the domain status table.
- **Order:** oldest at the top, new lines appended at the bottom (tape order). A 1px rail rule marks each minute boundary.
- **Expanded body.** Opening a line's `<summary>` (keyboard: Enter/Space) shows the full text in Code type with `white-space: pre-wrap` on the inset well: prompt text, `last_assistant_message`, plan text, question options, permission `input` as JSON text, and `source`. It is always a `${}` text binding and never Markdown.
- **Empty:** `TAPE live since 19:40:02Z — no events yet`. The SSE stream starts at each file's current end, so the tape holds only events after page open.
- **Line cap.** The component keeps at most 2000 lines in the DOM. Beyond that it prints `older lines trimmed from view: N — the full tape is events.ndjson` at the top.
- **Follow rule.** The tape auto-follows only while scrolled to the bottom. Otherwise it prints the static line `N new lines below`, a link-styled button-less anchor to `#tape-end`.

**5. `<viola-atis>`: the ATIS / fuel-state header**
- **Anatomy:** a single sticky line at the top on anthracite, with a 1px rail rule below. Cells are separated by 12px, with labels in Label type and figures in Data type.
- **Content:**
  - `VIOLA` (Display)
  - `BAY 127.0.0.1:47319`
  - `5H 62 %` with `resets 21:00Z`, and `7D 41 %` with `resets Mon 09:00Z`, each followed by `expired` once its `resets_at` has passed
  - `read 4m ago`: always printed, computed from `read_at`, updated each minute without motion
  - `gate open` or a boxed `budget-paused`
  - `TAPE live` / `TAPE connecting` / `TAPE stopped · viola ui not answering`
  - `skipped 0 · 0 · 0`, boxed in buff when nonzero
- **Not shown:** `viola_home` from `/api/info`. It carries the OS username, which is the only PII in scope, and the page has no need for it.

**6. Access and error strips** (security UX)
- **Anatomy:** a full-width strip in the rack position. Anthracite surface, 1px buff rule, Cascadia 13px buff, with a phraseology first word and one plain instruction line.
- **401 `unauthorized`:**
  - Line 1: `UNAUTHORIZED  this page has no session for 127.0.0.1:47319`.
  - Line 2: `open the launch line printed by "viola ui" at start, or restart viola ui`.
  - It never shows or echoes the token, the launch URL, the `?t=` query, the `ui/<port>.url` path or its contents, or any cookie value. It never reads or prints `location.search`.
  - There is no "retry with token" control and no input field. The page has no credential fields at all in v1.
- **503 `state-unreadable`:** `unable · state-unreadable  viola home could not be read`.
- **Tape closed:** the ATIS `TAPE stopped` cell plus a strip at the top of the tape. The strips keep their last values, and the ATIS age keeps growing.
- **Initial read** (before `/api/sessions` answers, typically under 100ms locally): each rack prints one ruled line `sessions: no reading yet`, and the ATIS fields print `unknown`. There is no skeleton and no spinner.
- **Empty rack:** `no wrapped sessions — start one with  viola run <name> -- claude`, with the command in Code type.

**7. Reserved v1.x brake controls** (not built in v1; tokens only)
- **Controls:** two ruled rectangular `<button>`s, `I HAVE CONTROL` (POST pause) and `UNLINK`. They use Label type, uppercase, with a 1px buff outline, radius 0, the standard focus ring and a 32px minimum height.
- **Pressed:** no fill, because a buff fill means "read back". The confirmation is the resulting `wheel` / `unlink` tape line and strip value.
- **Surface:** the confirmation surface is Raised-3.
- **Security:** no token is exposed in markup. The requests use the same-origin `SameSite=Strict` cookie, and no amber is used.

### Navigation Pattern
- **One page, one bay, no routes and no nav chrome.** The vertical order is:
  1. the sticky `<viola-atis>` header
  2. the WRAPPED rack
  3. the UNWRAPPED · READ-ONLY rack
  4. the TAPE
- **Slots are stable.** Each rack orders strips by `ViolaName` ascending, the same order as CLI `list`. State never changes order. Only a session starting or ending changes slots.
- **Keyboard:**
  - A visible-on-focus skip link, `skip to tape`, is the first tab stop.
  - Tab then moves through tape `<summary>` elements and the `new lines below` anchor.
  - There are no command palette and no shortcuts. In v1, taking the wheel is spoken in the CLI (`viola pause`) or by a human keystroke in the terminal, never on this page. The reserved v1.x brake (web-spa component 7, `I HAVE CONTROL`) is the only planned page control for it (see the Design Decisions Log).
- **Attention outside the tab:**
  - `document.title` becomes `DIALOG builder · viola` (the first cocked strip by slot, plus `+N` when there are more). It returns to `viola` when none are cocked.
  - A visually hidden `role="status" aria-live="polite"` region announces only strips turning cocked ("builder: dialog pending, permission"), readback refusals ("send to builder unable, not-delivered, no-prompt-submitted") and `TAPE stopped`. The whole tape is never a live region.
- **Width:**
  - ≥1024px: primary.
  - 760–1023px: a strip wraps to two lines (band · NAME · LIVE · STATUS · DIALOG / WHEEL · CLI).
  - Below 760px: reserved for the later phone view.
  - Never horizontal scroll. The rack reserves `--cock-offset` of end padding.

### Platform-Specific Notes
- **CSP** (`default-src 'none'; script-src 'self'; style-src 'self'; … require-trusted-types-for 'script'`):
  - Lit's built-in `lit-html` Trusted Types policy satisfies `require-trusted-types-for`. No `trusted-types` allowlist directive is set.
  - Banned: `unsafeHTML`, `unsafeSVG`, `innerHTML`, `styleMap` and `style="…"`.
  - Every event field and unwrapped session name is a `${}` text binding.
  - Assistant Markdown is never rendered. Code-like content shows as plain pre-wrapped text.
- **Fonts:**
  - Per-OS stacks as in Typography: labels Bahnschrift → "DIN Alternate", "Avenir Next Condensed" → "DejaVu Sans Condensed" → `sans-serif`; fields "Cascadia Mono", Consolas → "SF Mono", Menlo → "DejaVu Sans Mono" → `monospace`. No web fonts.
  - Bahnschrift ships on Windows 10 1709+. Cascadia Mono ships with Windows Terminal on Windows 11; on Windows 10 without it, the field stack falls to Consolas, which is on every Windows.
  - **Linux is the CI render.** The headless GUI checks run on ubuntu, so contrast and render assertions must hold with DejaVu Sans Condensed + DejaVu Sans Mono resolved. The check asserts the resolved family (not a banned face, not an unlisted fontconfig substitute), and the CI image provides the DejaVu families.
- **Forced colors (Windows High Contrast):** borders survive, and the strike gradient is dropped (dashed outline fallback plus the word `unable`). The cock band maps to `Highlight`.
- **URL hygiene:** after the `/?t=` exchange the server's 303 leaves the address bar at `/`. The page never writes the token into history, title, storage or DOM.
- **Constructable stylesheets:** light DOM only in v1, so `/assets/app.css` is the single stylesheet. Revisit Lit `static styles` only after the headless CSP check passes.
- **`@starting-style`:** Chromium 117+, Firefox 129+, Safari 17.5+. Older engines show new lines without the fade, which is acceptable at 0.3.

---

## Surface: cli

**Platform:**
- Windows 10/11 (Windows Terminal, conhost), macOS and Linux terminals.
- stdout/stderr of the `viola` binary.
- Callers: the human on a TTY, and LLM driver sessions via `--json` / MCP.

**Toolkit / Framework:**
- clap 4.6.7 (derive), Rust stable.
- TTY detection uses std `IsTerminal`.
- Windows VT enabling uses windows-sys 0.61.2 `SetConsoleMode(ENABLE_VIRTUAL_TERMINAL_PROCESSING)`, already in the stack. If it fails, output has no colour.
- Styling is a small hand-written SGR module in the `viola` bin. It adds no dependency.

### Tokens (platform-specific)

| Token | Truecolor (`COLORTERM=truecolor\|24bit`) | 256-colour | 16-colour fallback | Use |
|---|---|---|---|---|
| attention (arrival amber #D97706) | `\x1b[38;2;217;119;6m` | `\x1b[38;5;172m` | `\x1b[33m` | The `DIALOG` word only, always followed by the kind word |
| stale (lamp-off) | `\x1b[2m` (SGR dim) | `\x1b[2m` | `\x1b[2m` | A whole `stale` row. The word `stale` is always in LIVE. |
| callsign | `\x1b[1m` (bold) | `\x1b[1m` | `\x1b[1m` | NAME column values |
| reset | `\x1b[0m` | `\x1b[0m` | `\x1b[0m` | After every styled span |
| handoff blue #5B8DB8 | not used | 67 reserved | not used | The quiz limits CLI colour to amber and dim, so link output is plain |
| backgrounds | never set | never set | never set | The terminal background belongs to the user |

**Colour decision order** (the first match wins, and results in no colour, no glyph and no SGR):
1. `--json`
2. `viola run` (while the child runs, nothing is printed at all)
3. `viola hook` (never writes stderr) and `viola mcp` (stdout carries MCP frames only)
4. `NO_COLOR` set
5. `TERM=dumb`
6. stdout is not a terminal
7. Windows VT enable failed

Otherwise the depth is chosen once per process, first match wins:
- truecolor if `COLORTERM` is `truecolor` or `24bit`;
- 256-colour if `TERM` contains `256color`, or on Windows once VT enabling has succeeded;
- otherwise the 16-colour fallback column.

**Streams:**
- stdout: results and data (tables, send/wait results, `last` text).
- stderr: context lines (`waiting: <name>`, the issue line of a send), refusals (`unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column, as in its sample; the exit-code table omits the `<name>` field; only the exit-1 start refusal uses the fixed message form `unable: <name> is already live`) and errors (`error: …`), and the `viola ui` launch line.
- For `--json`, the whole result, including refusals and `error` objects, is one JSON document on stdout with the typed exit code.

**Width:**
- Where the width is detectable (windows-sys `GetConsoleScreenBufferInfo`; `COLUMNS` elsewhere), human tables truncate only the NAME of unwrapped rows (`...`) and the tape/text columns.
- Otherwise nothing is truncated. Lines are never wrapped.
- Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).

### Component Patterns

**1. `viola list`: the k9s strip table** (the same columns and words as the web strip)
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
- Rows are ordered by name, the same as the web racks.
- The BAY line is the web ATIS without `resets` times (those are printed by `viola release <name> --budget`) and without a TAPE cell (the CLI has no tape). A window whose `resets_at` has passed still prints `expired` after its figure (`5H 62 % expired`), exactly as on the web, because old information is shown as old.
- Every field of `claude agents --json` origin (unwrapped names) has C0/C1 controls escaped as `\x1B`-style hex text, keeping `\n` and `\t` (security plan).
- Empty: `no wrapped sessions  start one: viola run <name> -- claude`.

**2. `viola send`: the readback mirror**
```
[  ] open           builder  issued 19:43:58.912Z                 (stderr, TTY only)
[RB] read back      builder  19:43:59.004Z  cursor 48213          (stdout, exit 0)
[/ ] unable         builder  not-delivered  no-prompt-submitted   (stderr, exit 13)
hint: builder did not submit the prompt; check it, then send again
[  ] unconfirmable  builder  local command, no measured post-condition   (stdout, exit 0)
```
- A refusal prints `unable` plus `reason  detail`, then one `hint:` line, keyed by the reason (or by reason · detail where listed):
  - `human-typing`: `the human has the wheel; viola release <name> returns it`
  - `budget-paused`: `budget gate is closed; it lifts at the window reset or with viola release <name> --budget`
  - `unverified-cli`: `run viola verify for this CLI version`
  - `not-delivered · turn-running`: `a turn is running; viola wait <name> first`
  - `not-delivered · control-character`: `the text contains a control character (only LF, CR, TAB are allowed)`
  - `not-delivered · input-not-ready`: `<name> was not ready for input; viola wait <name>, then send again`
  - `not-delivered · no-prompt-submitted`: `<name> did not submit the prompt; check it, then send again` (the sample above)
  - `not-delivered · unknown-dialog`: `that dialog is not pending; viola list shows the current DIALOG`
  - `instance-unreachable` (exit 21): `<name> is not running; viola list shows the live instances`
  - No hint line for `unknown` (exit 14), because its detail is opaque and a hint would have to guess. No hint line for `error: wrapper fault` (exit 20) or `error: internal error` (exit 1), because they are faults, not refusals, and their detail goes only to `diagnostics/`.
- A hint never quotes the sent text or any upstream text.

**3. `viola wait` / `viola last`**
- `wait` prints one static `waiting: builder` line on stderr (TTY only), then one result line on stdout:
  - `turn-ended  builder  19:44:10.221Z  cursor 49102`
  - `question  builder  dialog 7  cursor 49310` (`DIALOG` is not coloured here: this is a result, not the board)
  - `timed out  builder  30000 ms`
- There is no spinner and no elapsed-time counter.
- `last` prints `last  builder  turn-ended 19:44:10.221Z` on stderr (TTY only), then the message text on stdout with C0/C1 escaped, or `no message` on stderr for `null`.

**4. Wheel, link and gate verbs** (the phraseology is the design)
- `viola pause builder` → `builder  wheel human  manual-pause  I have control`
- `viola release builder` → `builder  wheel driver  you have control`
- `viola release builder --budget` → `builder  budget gate lifted until 21:00:00Z  wheel human`
- `viola link overseer builder` → `link    overseer -> builder  since 19:43:58Z` (an existing pair prints `link    overseer -> builder  already linked`)
- `viola unlink overseer builder` → `unlink  overseer -> builder`
- `viola answer builder 7 < response.json` → `answered  builder  dialog 7`, or `unable  builder  not-delivered  unknown-dialog`

**5. `viola verify` / `viola ui` / `viola run`**
- **`verify`:** a step counter (the project's one progress pattern) with static appended lines: `[03/14] S3 ask-user-question updated-input  pass` / `... fail`. Then a summary: `stamped 2.1.280  14 pass  0 fail`. The word `fail` is uncoloured.
- **`ui`:** the launch line goes to stderr exactly once. It is unstyled even on a TTY: no colour, no bold, no OSC 8 hyperlink. The URL sits whole on its own line with no leading indent and no trailing punctuation, so terminal link detection and copy take exactly the URL:
  ```
  viola ui  127.0.0.1:47319
  http://127.0.0.1:47319/?t=<64 hex characters>
  ```
  No other line, from any verb, ever prints the token, the URL or the `.url` path.
- **`run`:** prints nothing once the child starts. A start refusal (exit 1) prints `unable: builder is already live` (no paths, no pids).

**Exit-code phraseology** (typed codes from architecture.md):

| Exit | Human stderr first word | `--json` |
|---|---|---|
| 0 | the result line (`[RB] read back`, `answered`, `wheel human` …) | `{"v":1,"ok":{…}}` |
| 1 | `error: internal error` (fixed message; no chain with serde sources, no paths), or `unable: <name> is already live` | — |
| 2 | clap usage text (never produced by `viola hook`) | — |
| 10 | `unable  human-typing` (+ `manual-pause`) | `{"v":1,"refusal":"human-typing","detail":…}` |
| 11 | `unable  budget-paused  five-hour` / `seven-day` | refusal object |
| 12 | `unable  unverified-cli` | refusal object |
| 13 | `unable  not-delivered  input-not-ready` / `no-prompt-submitted` / `turn-running` / `unknown-dialog` / `control-character` | refusal object |
| 14 | `unable  unknown  <detail as opaque text>` | refusal object |
| 20 | `error: wrapper fault  <code>` | `{"v":1,"error":"wrapper-fault","detail":{…}}` |
| 21 | `unable  instance-unreachable` (no detail; the name is the omitted `<name>` field, as in every row) | `{"v":1,"error":"instance-unreachable","detail":null}` |

### Navigation Pattern
- Flat verbs, one lower-case word each: `run · send · wait · last · list · answer · verify · pause · release · link · unlink · ui · plugin install`.
- `viola list` is the board and the other verbs act on one strip. There is no interactive prompt, no TUI mode and no pager.
- `viola --help` groups verbs as `board: list, ui` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, verify, plugin install`.

### Platform-Specific Notes
- **Git Bash on Windows** rewrites leading-slash arguments. Prompt text therefore comes only from stdin or `--file`. The warning for a rewritten-path argument is one plain stderr line, `warning: argument looks like a Git Bash rewritten path`.
- **conhost without VT** falls back to no colour. The words already carry every state, so nothing is lost.
- **`hook`** writes nothing to stderr and always exits 0. **`mcp`** writes only MCP frames on stdout. Neither has a human design surface.
- **Stack traces** never print. Errors are fixed messages, and full detail goes only to `instances/<name>/diagnostics/`.

---

## Anti-Patterns (NEVER do these)

### Universal Bans
- **NEVER use generic font families: Inter, Roboto, Arial, Helvetica, system-ui default.** viola's face is DIN (Bahnschrift) because strip forms and transport signage are set in it. A generic UI face erases the strip-bay reading. Open Sans, Lato, Space Grotesk and `-apple-system` are banned for the same reason.
- **NEVER use purple-gradient-on-white as a color scheme.** The page is a night strip bay: dark only, flat, gradient-free except for the one readback strike.
- **NEVER use cookie-cutter card grids without adapting card internals to content.** Sessions are fixed-field strips in racks, not cards. A card grid breaks the stable-slot muscle memory.
- **NEVER use the same layout for different information types.** Strips (current state), transfer markers (handoffs) and the tape (history) each have their own geometry: `--strip-cols`, the 20px marker line and `--tape-cols`.
- **NEVER use color purely for decoration. Every color communicates meaning.** Amber means `dialog_pending` and nothing else. Blue means handoff and nothing else. Any third hue has no domain meaning.
- **NEVER converge on common "safe" choices across generations.** The readback box and the cocked strip exist because agent dashboards converge on dots, toasts and graphs.

### Rejected Defaults (from exploration)
- **Traffic-light status dots** (green `live`, yellow `busy`, red `stale`). Rejected because a strip prints its status word in a fixed column. `stale` is a coasting track (lamp-off), not red, and the bay's only colour state is the cocked strip's amber beside its `DIALOG` word.
- **Optimistic "Sending…" spinner, then a green "Sent ✓" or a toast.** Rejected because a clearance does not count until it is read back. The readback box stays `open` until the matching `prompt-submitted`, then fills instantly or is struck with `unable` plus the typed reason. There is no green, no toast and no presumption.
- **A node-graph canvas of sessions and arrows.** Rejected because handoffs are transfer markers between strips in the bay (`→ builder since …` in departure blue). The strips keep their rack slots and the page stays text.
- **Auto-sorting "needs input" or recent sessions to the top.** Rejected because a controller cocks a strip in place. Re-ranking breaks the founder's "where builder lives" memory, and the cock (12px offset plus amber band) already says "needs you".
- **A chat-transcript feed** (bubbles, avatars, Markdown-rendered replies, code blocks). Rejected because the feed is the tower tape: one fixed-column line per event, sends paired with their readback, all text as plain text. The security plan also bans Markdown-to-HTML and `innerHTML`.
- **Budget as circular rings or green/yellow/red bars.** Rejected because fuel state is figures in ruled fields (`5H 62 %  7D 41 %`), always with the ATIS age (`read 4m ago`). `budget-paused` is a phraseology word, not a colour.
- **Robot and person icons or emoji for the wheel, and a clickable-looking toggle.** Rejected because the WHEEL field prints `human` / `driver`. The v1 page is view-only, and taking control is spoken (`viola pause`, "I have control"), never implied by a GUI switch.
- **Skeleton shimmer and a terminal-green or neon-glow "hacker" theme.** Rejected because nothing glows in a tower cab unless the lamp is on it. Missing values print `unknown` in their box at once, and the ground is lit anthracite, not an OLED void.

### Per-Surface Bans

**web-spa**
- **NEVER use any Tailwind or Bootstrap palette value (blue-500, #0d6efd, gray-100…).** The only hex values are the eight Color World tokens. Blue exists solely as departure-holder #5B8DB8.
- **NEVER put lamp-off, amber or blue text on holder #2A2E33.** They measure 4.4, 4.3 and 3.9:1 there and fail AA. Those inks live on anthracite, which is why captions sit in the rack header, the cocked DIALOG cell is inset and markers sit in the rack gap.
- **NEVER use `box-shadow`, `text-shadow`, glow or `filter: drop-shadow`.** The depth strategy is borders-only, and shadows read as glow on dark.
- **NEVER use a radius other than 0.** Strips, boxes and boots are ruled rectangles, and a round element reads as a status dot.
- **NEVER ship loading as "Loading…", a spinner or a skeleton.** Initial state is phraseology with context (`sessions: no reading yet`, fields `unknown`). This overrides the generic web-guide "skeleton shimmer" advice per the exploration.
- **NEVER animate `busy`, `stale`, `turn-ended`, the readback fill or the strip's return into line.** Only the cock (160ms) and new-line/new-marker fades (≤120ms) move.
- **NEVER use hover colour changes or add hover-only information.** There are no controls in v1. Hover is a link underline only, and every piece of information is visible or reachable by keyboard (`<details>`).
- **NEVER use `innerHTML`, `unsafeHTML`, `unsafeSVG`, `styleMap`, `style="…"`, inline `<script>`, inline handlers, `eval`, `@font-face`, CDN assets or Markdown rendering.** The CSP (`style-src 'self'`, `font-src 'none'`, `require-trusted-types-for 'script'`) and the GUI output-encoding elevation forbid them, and event text is untrusted upstream content.
- **NEVER display, echo or log the GUI token, the launch URL, the `?t=` query, the `.url` path, or a cookie value on the page, including in the 401 strip.** The security plan treats all of them as secrets, and recovery is only the launch line or a restart. No credential input field exists in v1.
- **NEVER render `viola_home` from `/api/info`.** It contains the OS username, the only PII in scope, and the bay has no use for it.
- **NEVER put the whole tape in an `aria-live` region.** Announce only cocks, refusals and `TAPE stopped`. Anything else floods a screen reader for hours.
- **NEVER text-transform session names.** A `ViolaName` is the case-sensitive `target` the founder types.

**cli**
- **NEVER use green `✓` / red `✗` prefixes or emoji.** The readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
- **NEVER colour anything except `DIALOG` (amber) and `stale` rows (dim), and never either without its word.** The quiz confirms colour as a second cue only.
- **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.** LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.
- **NEVER use a spinner, progress bar or live-redrawing dashboard.** `wait` prints one static `waiting:` line and `verify` prints step-counter lines, because a spinner is progress presumed before it is confirmed.
- **NEVER mix data and messages.** Results go to stdout. `waiting:`, refusals, errors and the launch line go to stderr.
- **NEVER print the token or launch URL anywhere but the single `viola ui` launch line, and never style or OSC-8-wrap that line.** Styling or hyperlinking risks terminal-history or copy leakage and breaks exact copy.
- **NEVER print upstream text, paths, pids or anyhow chains with serde sources in errors or hints.** This is the security plan's NEVER-log floor, and human output of `last` / `list` escapes C0/C1 controls.
- **NEVER print "done" or "success" without context.** Say what was read back, by whom, and at which cursor (`[RB] read back  builder  19:43:59.004Z  cursor 48213`).
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

### 3. Signature Test
Point to the readback box in all three required places: (1) the far right of every send line in `<viola-event-feed>`, (2) the far right of every outbound `<viola-transfer>` marker, and (3) the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`. On the web, places 1 and 2 must render the same `<viola-readback>` element under the same `.rb` rule: a 16px square, a 1px buff outline, a buff fill with `RB` in graphite at 11px 600, and a 1px `/` strike. The tape box and the marker box must therefore be indistinguishable. If any place lacks the box or draws it differently, the signature does not exist. Inject it.

### 4. Token Test
Every colour traces to one of the eight `--c-*` values (#1E2124, #2A2E33, #4A5057, #E6D8AE, #9C9278, #3B3A36, #D97706, #5B8DB8). `--c-*` is read only where a role alias is assigned or reassigned (`@layer tokens`, and the `stale` swap in `@layer states`). Components use the role aliases (`--surface-*`, `--ink*`, `--rule-*`, `--attention`, `--handoff`, `--focus-ring`, `--rb-*`) and never `--c-*`. Every length traces to a token (`--space-*`, `--strip-h`, `--line-h`, `--band-w`, `--rb-size`, `--rule`, `--rule-strong`, `--cock-offset`, `--focus-w`, `--focus-offset`) or to a `ch` track in `--strip-cols` / `--tape-cols`. A ninth hex value or an untokened px value signals no system.

### 5. Sameness Test (from Interface Design)
If another AI given a similar prompt would produce substantially the same
output — you have failed. The interface must emerge from THIS product's
domain exploration, not from statistical patterns in training data.

---

## Design Decisions Log

_Orchestrator records key decisions here. Manual additions welcome._

2026-09-24: Initial design system generated by /andromeda-design
- **Brand personality:** "Strip-bay handoff, read back". An ATC tower strip bay after dark with standard-phraseology voice. Base expression is 0.2 (web-spa 0.3; cli TTY 0.2; machine output and passthrough 0.0). Dark only.
- **Surfaces:** web-spa (`viola ui`: Lit 3.3.3 light-DOM `viola-*` elements, hand-written `@layer` CSS, system-installed Bahnschrift + Cascadia Mono) and cli (clap, hand-written SGR: amber `DIALOG` + dim `stale` only).
- **Signature:** the readback box (`open` / `RB` read back / `unable` strike, plus the printed word `unconfirmable`) in tape send lines, transfer markers and the CLI `[RB]` / `[  ]` / `[/ ]` column. The cocked strip is the supporting attention element.
- **Key rejection:** the optimistic "Sending… → Sent ✓" toast. Nothing is shown as delivered until it is read back.
- **Contrast finding:**
  - Lamp-off #9C9278 (4.4:1), amber (4.3:1) and blue (3.9:1) fail AA as text on holder #2A2E33.
  - Therefore: column captions live in the rack header on anthracite, the cocked DIALOG cell is inset anthracite, transfer markers sit in the rack gap, and a `stale` strip drops its holder fill to anthracite (the lamp has moved off).
- **Customisations of the Color World:**
  - The in-strip field grid uses lamp-off buff rather than strip buff, so buff borders stay reserved for borders that are a state.
  - Instance `budget_paused` prints inside the WHEEL field (`driver · budget-paused`) to keep the six fixed columns shared with the CLI.
  - Absent-by-contract fields print `n/a`, and failed readings print `unknown`.
- **Data dependency (extends cross-lane follow-up CL-1):**
  - `send-issued` must carry `cursor` and `from`.
  - `send-refused` must carry `refusal` + `detail`.
  - A logged outcome is needed for `ok`/`confirmed:false` (`unconfirmable`) so the box can reach that word.
  - Readback pairing uses the next `prompt-submitted` with origin `driver` on that instance (at most one send in flight), or `session-start` cause `clear` for `/clear`.
  - Until CL-1 lands the page shows only `read` boxes, and `unknown` in marker last-send cells.

2026-09-24: Phase 4.5 review round 1 — fonts get explicit per-OS fallback stacks (founder: Bahnschrift and Cascadia Mono are Windows-only, D3 supports macOS + Linux, and CI's headless GUI checks run on ubuntu). `--font-label`: Bahnschrift → "DIN Alternate" → "DejaVu Sans Condensed" → sans-serif; `--font-field`: "Cascadia Mono" → "SF Mono", Menlo → "DejaVu Sans Mono" → monospace. Contrast and render assertions must hold on the Linux fallback, the resolved family is asserted, no web fonts. Colours, expression levels and signature approved as drafted.

2026-09-24: Phase 4.5 review round 2 — restored the two fallbacks the round-1 exact list dropped: Consolas right after "Cascadia Mono" in `--font-field` (on every Windows, so Windows 10 without Windows Terminal stays in a named face), and "Avenir Next Condensed" right after "DIN Alternate" in `--font-label` (macOS). Draft approved; proceed to Phase 5.

2026-09-24: Phase 5 review iteration 1. This entry records decisions the body already applied but the log did not carry. No founder-approved colour, expression level, signature drawing or font stack changed.
- **Library shortlist direction:** Precision & Density (primary) with Utility & Function (secondary). The style preset is Minimalism & Swiss Style (#1), with E-Ink / Paper (#56) as the surface reference, inverted to buff on anthracite. Rejected presets: Dark Mode (OLED) (#7), Real-Time Monitoring (#31: pulsing dots, blink, toasts) and HUD / Sci-Fi FUI (#51: glow, telemetry animation). The Swiss preset's 200–250ms hover is dropped.
- **Shortlist deviations:**
  - Palette #81's run-green accent is dropped and its slate warmed to neutral laminate.
  - #96's route blue is lightened from #2563EB (fails AA) to #5B8DB8.
  - Typography pairing #1 (Fira Code + Fira Sans) is translated to Bahnschrift + Cascadia Mono under `font-src 'none'`.
  - Fallback pairing #3 (a mono name cell) is not taken. The Callsign role sets NAME in Bahnschrift at normal width, which answers the "too condensed / too signage" concern.
  - UX guideline 8, Loading Indicators (#78, "spinner or skeleton"), is overridden per the exploration. Waiting is shown as words: `open`, `unknown`, `read 4m ago`, `sessions: no reading yet`, `waiting:`.
- **Signature states:** the exploration's `data-rb="open|read|refused"` is extended to four values with `unconfirmable`, the honest outcome of an `ok` send with `confirmed:false`. It deliberately reuses the open drawing and is never filled. In the Squint Test, `open` and `unconfirmable` therefore look identical, and only the word cell tells them apart.
- **Motion vs the 0.3–0.4 calibration row:** viola overrides the row. Hover is 0ms (link underline only), there is no page transition, and there is one entrance: the 120ms linear `@starting-style` fade on `data-live` tape lines and transfer markers. The cock (12px / 160ms) is the only high-impact moment.
- **Brake scope:** "never on this page" and "view-only" are v1 statements. Web-spa component 7 (`I HAVE CONTROL`, `UNLINK`), Raised-3, the `radius-sm` brake usage and `Problem::cross-origin-forbidden` stay reserved for v1.x and do not contradict the v1 bans.
- **Geometry:** `--strip-cols` changes LIVE from 6ch to 8ch, STATUS from 8ch to 10ch, and the WHEEL and CLI maxima from 24ch to 25ch, so `stale`, `unknown` and the 22ch WHEEL / CLI words fit with `space-xs` padding in DejaVu Sans Mono. Field-cell inline padding is `space-xs` (8px) only.

[Iteration 1] [substantive] Resized `--strip-cols` so the longest words fit (with the arithmetic shown) and made `space-xs` the only field-cell padding. Aligned the Typography `RB` weight with the CSS (600). Rewrote the Signature and Token Tests for viola. Fixed the Motion "named pair" and stated the override of the 0.3–0.4 row. Made the Semantic Warning and Info rows match the domain rows and declared the domain rows authoritative. Scoped "never on this page" to v1. Logged the shortlist direction and presets, the shortlist deviations, the four-state `data-rb`, the motion override, the brake scope and the geometry change.
[Iteration 2] [substantive] Added wrapped-strip tokens for 760–1023px with the fit arithmetic. Specified how `<viola-readback>` spans the two tape tracks. Moved the inbound `← overseer` twin off the holder into the rack gap (blue on holder is 3.9:1). Added the marker's 2px blue tick and the inbound twin to the handoff Core Colors usage. Moved the strip `DIALOG` word from Label to Body and split the duplicate `TAPE`. Resolved the cocked + `stale` kind-word colour in favour of the CSS. Removed the "150ms at most for 0.2" drift. Completed the CLI hint list and stated which outcomes print no hint.
[Iteration 3] [substantive] Removed the phantom amber "page-title marker" and added the blue transfer-marker tick to Border Progression. Widened the tape word / reason track to 46ch so the longest refusal fits, and stated tape gaps, overflow and the 1024px arithmetic. Defined the CLI ASCII separator (` - ` inside a field, two spaces between columns) and the inbound `←`, and aligned the `viola list` sample, including `expired` on the BAY line. Made the 16-colour CLI depth reachable and made the Streams refusal form match the samples. Completed the Brand Identity domain anchors from the exploration's 13 concepts.
[Iteration 4] [substantive] Renamed the session-row liveness hook to `data-liveness`, so `data-live` means only SSE arrival. Moved the tape grid onto `<summary>` and named `.tape-line`. Specified how the 20px transfer-marker rows sit in the 8px rack flow. Gave the caption header row the 760–1023px wrapped template. Made `space-sm` a gap between ATIS cells, matching the anatomy. Added the buff send-line `→` to Iconography. Aligned the CLI Streams refusal form and exit-21 row with the name-first samples.
