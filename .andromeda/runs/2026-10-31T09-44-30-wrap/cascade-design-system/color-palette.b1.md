### color-palette#1.1 · values · palette · ## Color Palette › ### Core Colors · lines 8–10
| Role | Value | Usage | Domain anchor |
|------|-------|-------|---------------|
| Primary | #E6D8AE | Primary ink, focus rings, information-carrying borders (every Border Progression Emphasis use: the readback box outline and strike, the `budget-paused`, nonzero `skipped` and `TAPE stopped` boxes, error strips), the read-back fill | Strip buff: the paper flight-progress strip under the gooseneck task lamp. It is the thing the controller reads, and the readback box is drawn on it. |
### color-palette#1.2 · values · palette · ## Color Palette › ### Core Colors · lines 11–11
| Secondary | #9C9278 | Secondary ink on anthracite (timestamps, reading ages, column captions, `n/a`), the printed field-grid rule inside strips, the 1px box around an ATIS `expired` word, and all ink of a `stale` strip | Lamp-off buff: the same paper after the lamp moves off. A coasting track is dimmed, not deleted. An ATIS age is printed small. |
### color-palette#1.3 · values · palette · ## Color Palette › ### Core Colors · lines 12–12
| Accent | #D97706 | The only attention colour: the 4px holder band and the `DIALOG` word of a cocked strip, and the CLI `DIALOG` word in `viola list` (the kind word after it stays uncoloured). The `DIALOG builder · viola` document title is plain text with no colour. Never used for errors, warnings, budget or hover. | Arrival-holder amber: the orange holder cocked out of line. It is the physical "needs you". |
### color-palette#1.4 · values · palette · ## Color Palette › ### Core Colors · lines 13–13
| Role (handoff) | #5B8DB8 | Transfer-marker arrows and names (outbound `→ builder`, inbound `← overseer`), the marker's 2px left tick (`--rule-strong`, the only blue rule), and the names in `link` / `unlink` tape lines only. Never a fill, never text on holder and never a status. | Departure-holder blue: outbound traffic being handed to the next sector (`driver → driven`). |
### color-palette#2.1 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 21–23
| Level | Value | Usage |
|-------|-------|-------|
| Base | #1E2124 | Page background (the bay), the rack gap, the tape, and bare (unwrapped) and `stale` strips. Console anthracite: a lit near-black, not a void. |
### color-palette#2.2 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 24–24
| Raised-1 | #2A2E33 | A lit wrapped session strip: the holder boot. It carries buff ink only. |
### color-palette#2.3 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 25–25
| Raised-2 | #2A2E33 + 1px #E6D8AE rule | Reserved: there are no popovers or tooltips in v1. If one is ever needed it is holder plastic with a buff rule. Lift comes from the rule, never from a third tint or a shadow. |
### color-palette#2.4 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 26–26
| Raised-3 | #2A2E33 + 2px #E6D8AE rule | Reserved for the v1.x brake confirmation, the only modal-like surface ever planned. v1 has none. |
### color-palette#2.5 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 27–27
| Inset | #1E2124 | Cells set into a holder: the DIALOG cell of a cocked strip (so the amber word keeps 5.1:1), the interior of an `open` readback box, and the expanded tape text well |
### color-palette#3.1 · values · palette · ## Color Palette › ### Text Hierarchy · lines 35–37
| Level | Value | Usage |
|-------|-------|-------|
| Primary | #E6D8AE | Session names (callsign cell), every field value on a lit strip, tape kind and text, and phraseology words (`open`, `unable`, `DIALOG` kind, `budget-paused`) |
### color-palette#3.2 · values · palette · ## Color Palette › ### Text Hierarchy · lines 38–38
| Secondary | #9C9278 | Column captions (LIVE, STATUS…) in the rack header on anthracite, transfer-marker `since` times (the ATIS `TAPE live since` time stays buff, per `TapeConnection::open`), ATIS reading age, `n/a` on bare strips |
### color-palette#3.3 · values · palette · ## Color Palette › ### Text Hierarchy · lines 39–39
| Tertiary | #9C9278 | Tape timestamps (`19:43:58.123`), `activity` lines, `harness`-origin lines, the expanded-line `source`. Separated from Secondary by position (the tape, not the rack header or the ATIS line), never by a dimmer hex or a smaller size: timestamps are Data and `activity` / `harness` lines are Body, both 13px, and only the expanded body's `source` is Code (12px), as in the Typography table. |
### color-palette#3.4 · values · palette · ## Color Palette › ### Text Hierarchy · lines 40–40
| Muted | #9C9278 | `stale` strips (every field) and zero-count `skipped` figures. No ink is dimmer than lamp-off buff, because anything darker fails AA on anthracite. There are no disabled controls or placeholders in v1. |
### color-palette#3.5 · values · palette · ## Color Palette › ### Text Hierarchy · lines 41–41
| On-paper | #3B3A36 | Graphite pencil: text on a buff fill only (the `RB` in a read-back box) |
### color-palette#4.1 · values · status-states · ## Color Palette › ### Semantic Colors · lines 49–51
| State | Background | Border | Text |
|-------|------------|--------|------|
| Success, confirmed after the fact (read back) | #E6D8AE for the box fill; #1E2124 behind the word `read back` | #E6D8AE | #3B3A36 for `RB` in the box; #E6D8AE for the word `read back` (graphite is never text on anthracite) |
### color-palette#4.2 · values · status-states · ## Color Palette › ### Semantic Colors · lines 52–52
| Warning, degraded but not refused (`unverified-cli`, `unconfirmable`, expired budget window) | #2A2E33 for `unverified-cli` (a word on a lit strip); #1E2124 for `unconfirmable` and `expired` | #9C9278 for `unverified-cli` (field grid) and `expired`; #E6D8AE for `unconfirmable` (the readback-box outline is a state border) | #E6D8AE |
### color-palette#4.3 · values · status-states · ## Color Palette › ### Semantic Colors · lines 53–53
| Error, "unable" plus a reason (refusal, HTTP problem) | #1E2124 | #E6D8AE (plus a 1px `/` strike where a readback box exists) | #E6D8AE |
### color-palette#4.4 · values · status-states · ## Color Palette › ### Semantic Colors · lines 54–54
| Info (activity, harness turn, skipped counts, tape status) | #1E2124 | none for activity, harness and zero `skipped`; #4A5057 for `TAPE connecting` / `TAPE live`; #E6D8AE where the border is the state (nonzero `skipped` box, `TAPE stopped`) | #9C9278 for activity, harness, zero `skipped` and `TAPE connecting`; #E6D8AE for `TAPE live`, `TAPE stopped` and nonzero `skipped` |
### color-palette#5.1 · values · status-states · ## Color Palette › ### Semantic Colors · lines 60–62
| Enum::Variant | Background | Border | Text | Printed as · treatment |
|---|---|---|---|---|
| Liveness::live | #2A2E33 | #9C9278 (field grid) | #E6D8AE | `live`. Holder strip lit. |
### color-palette#5.2 · values · status-states · ## Color Palette › ### Semantic Colors · lines 63–63
| Liveness::stale | #1E2124 | #4A5057 (solid 1px) | #9C9278 | `stale`. The whole strip goes lamp-off, instantly: the holder fill drops to anthracite, all ink becomes lamp-off, the band slot stays rail. Slot kept. (Gone instances are omitted by `/api/sessions`: no row, no visual.) |
### color-palette#5.3 · values · status-states · ## Color Palette › ### Semantic Colors · lines 64–64
| SessionStatus::idle | #2A2E33 | #9C9278 | #E6D8AE | `idle` |
### color-palette#5.4 · values · status-states · ## Color Palette › ### Semantic Colors · lines 65–65
| SessionStatus::busy | #2A2E33 | #9C9278 | #E6D8AE | `busy`. No spinner, pulse or animation. |
### color-palette#5.5 · values · status-states · ## Color Palette › ### Semantic Colors · lines 66–66
| SessionStatus::unknown | #2A2E33 | #9C9278 | #E6D8AE | `unknown` printed in the box (`claude agents --json` missing, failed or not joined) |
### color-palette#5.6 · values · status-states · ## Color Palette › ### Semantic Colors · lines 67–67
| WheelHolder::driver | #2A2E33 | #9C9278 | #E6D8AE | `driver`, weight 400 |
### color-palette#5.7 · values · status-states · ## Color Palette › ### Semantic Colors · lines 68–68
| WheelHolder::human | #2A2E33 | #9C9278 | #E6D8AE | `human`, weight 600. Pilot in command is one of the two heavier words after the callsign; the other is the cocked strip's amber `DIALOG` word (Typography, Body row). |
