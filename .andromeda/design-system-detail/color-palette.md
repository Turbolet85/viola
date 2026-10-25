# design-system · Color Palette
Read whole. The master's broad view is .andromeda/design-system.md; this file holds its Color Palette detail.

## Color Palette › ### Core Colors

<!-- detail: values · palette -->

| Role | Value | Usage | Domain anchor |
|------|-------|-------|---------------|
| Primary | #E6D8AE | Primary ink, focus rings, information-carrying borders (every Border Progression Emphasis use: the readback box outline and strike, the `budget-paused`, nonzero `skipped` and `TAPE stopped` boxes, error strips), the read-back fill | Strip buff: the paper flight-progress strip under the gooseneck task lamp. It is the thing the controller reads, and the readback box is drawn on it. |
| Secondary | #9C9278 | Secondary ink on anthracite (timestamps, reading ages, column captions, `n/a`), the printed field-grid rule inside strips, the 1px box around an ATIS `expired` word, and all ink of a `stale` strip | Lamp-off buff: the same paper after the lamp moves off. A coasting track is dimmed, not deleted. An ATIS age is printed small. |
| Accent | #D97706 | The only attention colour: the 4px holder band and the `DIALOG` word of a cocked strip, and the CLI `DIALOG` word in `viola list` (the kind word after it stays uncoloured). The `DIALOG builder · viola` document title is plain text with no colour. Never used for errors, warnings, budget or hover. | Arrival-holder amber: the orange holder cocked out of line. It is the physical "needs you". |
| Role (handoff) | #5B8DB8 | Transfer-marker arrows and names (outbound `→ builder`, inbound `← overseer`), the marker's 2px left tick (`--rule-strong`, the only blue rule), and the names in `link` / `unlink` tape lines only. Never a fill, never text on holder and never a status. | Departure-holder blue: outbound traffic being handed to the next sector (`driver → driven`). |

<!-- /detail -->

## Color Palette › ### Surface Scale (elevation hierarchy)

<!-- detail: values · palette -->

| Level | Value | Usage |
|-------|-------|-------|
| Base | #1E2124 | Page background (the bay), the rack gap, the tape, and bare (unwrapped) and `stale` strips. Console anthracite: a lit near-black, not a void. |
| Raised-1 | #2A2E33 | A lit wrapped session strip: the holder boot. It carries buff ink only. |
| Raised-2 | #2A2E33 + 1px #E6D8AE rule | Reserved: there are no popovers or tooltips in v1. If one is ever needed it is holder plastic with a buff rule. Lift comes from the rule, never from a third tint or a shadow. |
| Raised-3 | #2A2E33 + 2px #E6D8AE rule | Reserved for the v1.x brake confirmation, the only modal-like surface ever planned. v1 has none. |
| Inset | #1E2124 | Cells set into a holder: the DIALOG cell of a cocked strip (so the amber word keeps 5.1:1), the interior of an `open` readback box, and the expanded tape text well |

<!-- /detail -->

## Color Palette › ### Text Hierarchy

<!-- detail: values · palette -->

| Level | Value | Usage |
|-------|-------|-------|
| Primary | #E6D8AE | Session names (callsign cell), every field value on a lit strip, tape kind and text, and phraseology words (`open`, `unable`, `DIALOG` kind, `budget-paused`) |
| Secondary | #9C9278 | Column captions (LIVE, STATUS…) in the rack header on anthracite, transfer-marker `since` times (the ATIS `TAPE live since` time stays buff, per `TapeConnection::open`), ATIS reading age, `n/a` on bare strips |
| Tertiary | #9C9278 | Tape timestamps (`19:43:58.123`), `activity` lines, `harness`-origin lines, the expanded-line `source`. Separated from Secondary by position (the tape, not the rack header or the ATIS line), never by a dimmer hex or a smaller size: timestamps are Data and `activity` / `harness` lines are Body, both 13px, and only the expanded body's `source` is Code (12px), as in the Typography table. |
| Muted | #9C9278 | `stale` strips (every field) and zero-count `skipped` figures. No ink is dimmer than lamp-off buff, because anything darker fails AA on anthracite. There are no disabled controls or placeholders in v1. |
| On-paper | #3B3A36 | Graphite pencil: text on a buff fill only (the `RB` in a read-back box) |

<!-- /detail -->

## Color Palette › ### Semantic Colors

<!-- detail: values · status-states -->

| State | Background | Border | Text |
|-------|------------|--------|------|
| Success, confirmed after the fact (read back) | #E6D8AE for the box fill; #1E2124 behind the word `read back` | #E6D8AE | #3B3A36 for `RB` in the box; #E6D8AE for the word `read back` (graphite is never text on anthracite) |
| Warning, degraded but not refused (`unverified-cli`, `unconfirmable`, expired budget window) | #2A2E33 for `unverified-cli` (a word on a lit strip); #1E2124 for `unconfirmable` and `expired` | #9C9278 for `unverified-cli` (field grid) and `expired`; #E6D8AE for `unconfirmable` (the readback-box outline is a state border) | #E6D8AE |
| Error, "unable" plus a reason (refusal, HTTP problem) | #1E2124 | #E6D8AE (plus a 1px `/` strike where a readback box exists) | #E6D8AE |
| Info (activity, harness turn, skipped counts, tape status) | #1E2124 | none for activity, harness and zero `skipped`; #4A5057 for `TAPE connecting` / `TAPE live`; #E6D8AE where the border is the state (nonzero `skipped` box, `TAPE stopped`) | #9C9278 for activity, harness, zero `skipped` and `TAPE connecting`; #E6D8AE for `TAPE live`, `TAPE stopped` and nonzero `skipped` |

<!-- /detail -->

<!-- detail: values · status-states -->

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
| DialogPending::true | #1E2124 (DIALOG cell inset) | #D97706 (4px holder band) | #D97706 (`DIALOG`) + #E6D8AE (kind; #9C9278 when the strip is also `stale`, per the `stale` `--ink` swap) | Cocked: `translateX(12px)` once over 160ms, amber band, `DIALOG <kind>`. Slot kept. Announced politely. |
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
| SessionStartCause::compact | #1E2124 | none | #E6D8AE | `compact`. Also the post-condition that fills the readback box of a `/compact` send on a verified CLI, when it carries the session's own id. |
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
| TapeConnection::closed | #1E2124 | #E6D8AE | #E6D8AE | `TAPE stopped · not answering`. Strips keep the last values, and the ATIS age keeps counting. |
| Problem::unauthorized (401) | #1E2124 | #E6D8AE | #E6D8AE | An access refusal. Never shows the token, the URL or the file path. |
| Problem::state-unreadable (503) | #1E2124 | #E6D8AE | #E6D8AE | Rack strip `unable · state-unreadable` |
| Problem::not-found (404) | #1E2124 | #E6D8AE | #E6D8AE | Rack strip `unable · not-found`. The page only requests its own routes, so this is defensive. |
| Problem::method-not-allowed (405) | #1E2124 | #E6D8AE | #E6D8AE | Never produced by the page (GET only). If seen: `unable · method-not-allowed`. |
| Problem::host-not-allowed (403) | #1E2124 | #E6D8AE | #E6D8AE | The page cannot load at all, so there is no in-page visual. The browser shows the Problem JSON. |
| Problem::cross-origin-forbidden (403, v1.x) | #1E2124 | #E6D8AE | #E6D8AE | Reserved for the brake: `unable · cross-origin-forbidden` |

<!-- /detail -->

## Color Palette › ### Border Progression

<!-- detail: values · palette -->

| Intensity | Value | Usage |
|-----------|-------|-------|
| Subtle | #4A5057 | Rack-rail aluminium: rack separators, the strip outer edge, the band slot of a non-cocked strip, the dashed outline of bare strips, the in-strip field grid of a `stale` strip (the `--rule-field` swap), the tape minute rule, the `session-end` rule and the rule under the ATIS header. Decorative, 2.0:1. In the domain status rows, #4A5057 on an ATIS-line value (`BudgetEnvelope`, `BudgetWindow`, `Window` other than `expired`, `gate open`, `TapeConnection::connecting` / `open`) names that header rule: those cells are never boxed. |
| Standard | #9C9278 | The printed field-grid rule between cells inside a lit strip: the rule printed on the paper, quieter than the ink. Also the 1px box around an ATIS `expired` word (`Window::expired`, on anthracite): old information is marked as old, quieter than the buff boxes of declared states. |
| Emphasis | #E6D8AE | Borders whose presence is the state: the readback box outline and strike, the `budget-paused` box, nonzero `skipped` boxes, the ATIS `TAPE stopped` box, error strips |
| Focus | #E6D8AE | 2px solid focus ring, offset 2px, radius 0 (11.4:1 on anthracite, 9.6:1 on holder) |
| Handoff | #5B8DB8 | The 2px (`--rule-strong`) left tick of the outbound and inbound transfer markers only, in the rack gap on anthracite (4.6:1). The only blue rule. Never on holder, never a strip or cell border. |

<!-- /detail -->
