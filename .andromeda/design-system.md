# Design System — viola

## Brand Identity

Detail: .andromeda/design-system-detail/brand-identity.md — read whole.

**Personality:** Tower strip-bay handoff, read back: standard-phraseology precision under low cab light. Every word is short and fixed, every send stays open until the far end reads it back, and nothing glows unless the lamp is on it.

**Domain anchors:**
- **Flight-progress strip.** One strip per `viola run` instance. The fields are fixed, in the order NAME · LIVE · STATUS · WHEEL · DIALOG · CLI. The name cell is set at callsign weight. A field with no reading prints `unknown`.
- **Strip bay and racks with labelled separators.** Every session keeps a stable slot and nothing is re-sorted.
- **Cocked strip.** `dialog_pending` is the strip pushed a finger-width out of line, with an arrival-holder amber band.
- **Readback.** A send counts only once the matching `prompt-submitted` reads it back or, for `/clear`, once its measured post-condition (a new session) does, or, for `/compact` on a verified CLI, once its measured post-condition (a `session-start` of cause `compact` carrying the session's own id) does; on an unverified CLI `/compact` stays `unconfirmable`. Otherwise it is "unable" plus a typed reason.
- **Handoff / transfer of control, and the tower voice recorder.** `driver → driven` transfer markers, and the ndjson tape played in order with `skipped` counts kept where they happened.
- **Strip holder (plastic boot).** Wrapped instances sit in a holder. Unwrapped sessions are bare strips: another facility's traffic, visible but never a `target`.
- **Coasting track.** `stale` liveness. The strip is dimmed to lamp-off, never deleted, and keeps its slot.
- **Fuel state / minimum fuel.** The budget governor: `5H` / `7D` figures, with `budget-paused` as a declared minimum-fuel state that stops new sends while the running turn finishes.
- **ATIS information with its age.** Every budget reading carries its age (`read 4m ago`). Old information is shown as old (`expired`), never as current.
- **Verified aircraft type.** A CLI version stamped by `viola verify` prints `verified`. An unstamped build flies transport only as `unverified-cli`.
- **Pilot-in-command, "I have control".** `human` in WHEEL at weight 600. `viola pause` / `viola release` are the spoken "I have control" / "you have control".
- **Standard phraseology and "unable" plus a reason.** Short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`) are the vocabulary of every label, CLI column and refusal. There is no filler and no guessing.

**Signature element:** **The readback box.** It is a fixed-width, 1px-ruled square cell; in 0.1.0 its one place is the CLI mirror below, and where a 0.2.0 frontend draws it is that version's to name (its technology is not chosen). It has three drawn states, and each state is printed as a word too:
- **`open`:** a buff outline on anthracite.
- **`RB` / `read back`:** filled solid buff, with `RB` in graphite. It fills instantly when the matching `prompt-submitted` lands, or for `/clear` when its `session-start` with cause `clear` does, or for `/compact` on a verified CLI when its `session-start` with cause `compact` and the session's own id does.
- **`unable`:** the outline plus one 1px `/` strike, with the typed reason in the next cell.

A fourth printed word, `unconfirmable`, reuses the open drawing and is never filled. `turn-ended` never fills the box. The CLI mirrors the box as `[RB]` / `[  ]` / `[/ ]`.

**Supporting attention element:** the cocked strip. On `dialog_pending` the row keeps its slot and shows the word `DIALOG` plus the dialog kind, the word amber on the CLI. The strip's 12px / 160ms translate and its 4px amber holder band are a 0.2.0 frontend design input.

**Expression level:** `0.2` (base). This value governs animation intensity, layout freedom and interaction complexity across the whole project.

Per-surface values (quiz Q4):

N/A — the frontend's expression level: the frontend is deferred to 0.2.0, and its technology is not chosen.

This number is the SINGLE SOURCE OF TRUTH for how much motion and visual dynamism downstream phases implement. At 0.2 the duration scale's 0.0–0.2 row applies (0–100ms, no entrance), and viola's cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.

**Design direction:**
The direction a 0.2.0 frontend inherits (0.1.0 renders no page) is a control-tower strip bay after dark. Buff paper strips sit in anthracite holder boots on ruled aluminium racks. Every session holds the same slot for its whole life, so the eye goes to where `builder` always is. The only warm light in the bay is the one arrival holder cocked out of line when a session needs you.

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
Detail: .andromeda/design-system-detail/color-palette.md — read whole.

Four core roles, each one ink: Primary #E6D8AE strip buff (primary ink, focus rings, information-carrying borders, the read-back fill); Secondary #9C9278 lamp-off buff (secondary ink on anthracite, the in-strip field grid, all ink of a `stale` strip); Accent #D97706 arrival amber, the only attention colour, never for errors, warnings, budget or hover; Role (handoff) #5B8DB8 departure blue, transfer markers and `link` / `unlink` names only, never a fill, never text on holder, never a status.

### Surface Scale (elevation hierarchy)

Detail: .andromeda/design-system-detail/color-palette.md — read whole.

A 0.2.0 frontend design input; 0.1.0 renders no page.

Surface scale: Base and Inset #1E2124 (the bay, the rack gap, the tape, bare and `stale` strips; cells set into a holder), Raised-1 #2A2E33 a lit wrapped strip carrying buff ink only; Raised-2 and Raised-3 are reserved, holder plastic with a 1px or 2px buff rule: lift comes from the rule, never from a third tint or a shadow.

### Text Hierarchy

Detail: .andromeda/design-system-detail/color-palette.md — read whole.

A 0.2.0 frontend design input; 0.1.0 renders no page.

Text hierarchy: Primary #E6D8AE for names, field values, tape text and phraseology words; Secondary, Tertiary and Muted all #9C9278, Tertiary told apart from Secondary by position (the tape), never by a dimmer hex or a smaller size, except the expanded body's `source` in Code (12px); Muted is every field of a `stale` strip and zero `skipped` figures; no ink is dimmer than lamp-off buff, because anything darker fails AA on anthracite; On-paper #3B3A36 is text on a buff fill only.

### Semantic Colors
Detail: .andromeda/design-system-detail/color-palette.md — read whole.

A 0.2.0 frontend design input; 0.1.0 renders no page.

Generic UI feedback states (baseline, all four kept). viola deliberately carries them with **ink, fill, rule, strike and a printed word, never with a green/yellow/red hue**. Hue is spent only on the cocked strip (amber) and the handoff (blue).

The domain status rows below are authoritative. This table only summarises them, and where a row names several cases it gives each case's value from those rows.

**Domain status colors.** A 0.2.0 frontend design input; 0.1.0 renders no page. These cover every enum in architecture.md's data model and contracts that gets a visual, one row per variant. The fifth column holds the printed word and the treatment. Colour never stands alone.

Domain status guards: the tape carries no amber (history is not attention); a refused send folds into its send line, never a separate line; `turn-ended` never fills a readback box; a nonzero `skipped` count stays visible, never auto-cleared; a bare strip is never cocked and its name never a target; an access refusal never shows the token, the URL or the file path; `busy` has no spinner, pulse or animation.

Not visualised in v1:
- `/ready` checks (`ok` / `unavailable` / `error`) and `/health` status. They are probe routes for tests and obs. A `claude_agents` failure surfaces on the page as STATUS `unknown`.
- `answer` behaviours (`allow` / `deny`, `approve` / `revise`). They are not logged as events, so the tape has nothing to draw.

### Border Progression

Detail: .andromeda/design-system-detail/color-palette.md — read whole.

A 0.2.0 frontend design input; 0.1.0 renders no page.

Border progression: Subtle #4A5057 rail, decorative only (2.0:1); Standard #9C9278 the field grid inside a lit strip and the box around an ATIS `expired` word; Emphasis #E6D8AE for borders whose presence is the state; Focus #E6D8AE, a 2px solid ring, offset 2px, radius 0; Handoff #5B8DB8 the transfer markers' 2px left tick only, the only blue rule, never on holder, never a strip or cell border.

Deviation note: the Color World names strip buff for "the ruled field grid". This system uses lamp-off buff (the same paper) for the in-strip grid. That way the squint test sees the values and the readback outline first, and buff borders stay reserved for borders that are a state.
Deviation note (holder colour vs state): the exploration's Strip holder concept has a holder's colour mark the kind of traffic, never its state. Two state changes touch the holder anyway, both forced by the contrast table: `stale` drops the fill to anthracite (lamp-off ink fails AA on holder), and a cocked strip insets its DIALOG cell (amber text fails AA on holder). Kind stays readable without the fill: a `stale` wrapped strip keeps its solid rail edge and 4px band slot, and an unwrapped strip has a dashed rail outline and no band slot.

---

## Typography

Detail: .andromeda/design-system-detail/typography.md — read whole.

A 0.2.0 frontend design input; 0.1.0 renders no page.

**Rationale:** The foundation is library pairing #1 (Fira Code + Fira Sans, "Dashboard Data"), translated to installed OS faces. The CSP resolves `font-src` to `'none'`, so there is no `@font-face`, no `local()` and no web fonts.
- **Bahnschrift** is Microsoft's DIN 1451. It ships with Windows 10 1709+, and Windows is the first target. DIN is the lettering of transport signage and of printed flight-progress strip forms (Frequentis/Saab EFS anatomy). Its semi-condensed width fits fixed strip fields, and its uppercase reads as standard phraseology.
- **Cascadia Mono** is the strip-printer and tape face. It is fixed pitch, has no ligatures (so `->`, `[RB]` and `[/ ]` render literally) and uses tabular figures. This keeps the web strip column-identical to the k9s-style CLI table.
- **Both faces are Windows-only.** D3 makes macOS and Linux supported, so each role carries an explicit per-OS fallback stack (below), and the Linux fallback is a first-class rendering, not a degraded one. The stacks end at the generic `sans-serif` / `monospace`, never `system-ui`.

Type roles: Display, Heading and Label in the Bahnschrift stack, semi-condensed and uppercase; Callsign, the NAME cell, in Bahnschrift at normal width and weight 600, never text-transformed, because a `ViolaName` must read exactly as it is typed as a `target`; Body, Code and Data in the Cascadia Mono stack, Data with tabular figures.

**Loading:** System font stack only, with named installed faces referenced by `font-family`. Nothing is fetched: no web fonts, no `@font-face`, no `local()`. That keeps the page CSP-clean (`font-src` resolves to `'none'`).

**Per-OS fallback stacks (tokens):**

Each role token's stack names a face per OS (Windows, macOS, Linux) and ends at `sans-serif` / `monospace`; strip and tape columns are sized in `ch` of `--font-field`, so the Linux render (DejaVu Sans Mono, wider than Cascadia Mono) is the widest case the layout must hold without horizontal scroll.

**Assertions hold on the Linux fallback.** Every contrast assertion (the Color Palette contrast rules) and every render assertion (no horizontal scroll at ≥1024px, callsign cell not clipped, `[RB]` / `->` rendered literally without ligatures, `unknown` fitting its box, tabular figures aligned) must hold with "DejaVu Sans Condensed" + "DejaVu Sans Mono" resolved, and they hold on the resolved family, not only the declared stack: a resolved face outside the stacks above (for example a banned face picked by fontconfig substitution) breaks them. 0.1.0 renders no page, so no CI step renders these stacks or provides their families; how a 0.2.0 frontend checks these assertions is that version's to decide, its technology not chosen.

**cli:** Terminal monospace (the user's configured font). The brand shows through the fixed-column structure and the phraseology words (`open`, `RB`, `unable`, `DIALOG`, `stale`). The readback mirror is `[RB]` / `[  ]` / `[/ ]`. Bold is used only for the NAME column. SGR dim marks `stale` rows, and amber the `DIALOG` word. Output is ASCII only, and density is high.

---

## Spacing

Detail: .andromeda/design-system-detail/spacing.md — read whole.

A 0.2.0 frontend design input; 0.1.0 renders no page.

Spacing tokens: space-micro 4px (never cell padding), space-xs 8px field-cell inline padding, space-sm 12px equal to `--cock-offset` so a cocked strip never causes horizontal scroll, space-md 16px, space-lg 24px, space-xl 32px.

Base unit: 4px. All values are multiples of it. This is a developer tool watched for hours beside two terminals, so strip density is tight. Geometry tokens:

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

Detail: .andromeda/design-system-detail/border-radius.md — read whole.

A 0.2.0 frontend design input; 0.1.0 renders no page.

Every radius token is 0px, radius-full's 9999px a banned value: there are no avatars, pills or toggles, and a round element would be a status dot, a rejected default.

**Personality:** Every shape is a ruled rectangle: flight strips, holder boots and printed boxes on a strip form. Radius 0 everywhere is the intent, not a missing decision. Hierarchy comes from rule weight and material, not corner rounding.

---

## Motion (calibrated to the base expression level `0.2`)

Detail: .andromeda/design-system-detail/motion.md — read whole.

All motion decisions flow from the expression level set in Brand Identity.

**Duration scale (adjusted for expression level):**

---

## Iconography

Detail: .andromeda/design-system-detail/iconography.md — read whole.

A 0.2.0 frontend design input; 0.1.0 renders no page.

**Style:** Custom text glyphs only. Phraseology words carry the meaning.

Glyphs: on the web `→` (U+2192) marks the outbound marker, `link` / `unlink` lines and a send's target, `←` (U+2190) only the inbound twin; the readback strike is a CSS `linear-gradient`; `·` separates on the web. The CLI is ASCII only: `->` with no inbound form, two spaces between columns, ` - ` inside one field; `viola list` carries no link column in v1, so its six columns stay fixed.

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

Detail: .andromeda/design-system-detail/surface.md — read whole.

CLI colour tokens, at truecolor, 256-colour and 16-colour depth: attention (arrival amber) on the `DIALOG` word only, always followed by the kind word; SGR dim on a whole `stale` row, the cock winning for `DIALOG`; bold for NAME values; a reset after every styled span. Handoff blue is never used, and backgrounds are never set: the terminal background belongs to the user.

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

- stderr: context lines, refusals (two-space `unable  <name>  <reason>  <detail>`, or the fixed `unable: <text>` form for the exit-1 refusals of `run`, `revive` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release`), each refusal's `hint:` line directly after it, and errors.

- For `--json`, the whole result, including refusals and `error` objects, is one JSON document on stdout with the typed exit code, except an oversize `viola answer` and an unusable `viola send`, which exit 2 with their fixed stderr line and no document. No `hint:` line is printed under `--json`: the typed `refusal` / `error` fields are the whole answer.

**Width:**

- Where stdout's own terminal width is known (`COLUMNS` is never read, a piped stdout has none), human tables truncate only the NAME of unwrapped rows (`...`); the CLI has no tape.

- Otherwise nothing is truncated. Lines are never wrapped.
- Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).

### Component Patterns

Detail: .andromeda/design-system-detail/surface.md — read whole.

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

As built: a wrapped row joined to a `claude agents --json` row reads STATUS `idle` or `busy`, else `unknown`; unwrapped sessions follow the `-- UNWRAPPED - READ-ONLY --` line; only an unwrapped NAME is cut, and only where the terminal width is known; colour follows the decision order above, laid over the plain board's columns; every control character of a row is escaped as `\xHH`. A failed strict-modes check refuses the whole board (exit 21) in the fixed `unable:` / `hint:` form.

**2. `viola send`: the readback mirror**
```
[  ] open           builder  issued 19:43:58.912Z                 (stderr, TTY only)
[RB] read back      builder  19:43:59.004Z  cursor 48213          (stdout, exit 0)
[/ ] unable         builder  not-delivered  no-prompt-submitted   (stderr, exit 13)
hint: builder did not submit the prompt; check it, then send again
[  ] unconfirmable  builder  local command, no measured post-condition   (stdout, exit 0)
```

A refusal prints `unable` plus `reason  detail`, then one `hint:` line per reason, detail or cause; `run`'s and `revive`'s start refusals take a fixed-message `unable:` line and one hint per cause, `revive`'s read in a fixed order that stops at the first; the exit-21 strict-modes refusal of `send`, `wait`, `last`, `answer`, `pause` and `release` prints the fixed instance-state `unable:` / `hint:` pair (Exit-code phraseology below), not the `[/ ]` mirror. No hint names `viola release`, since a driver's `release` is refused. Under `--json` no hint prints; a per-cause `detail` code is pending, owner: the arch amendment naming them. Faults and `unknown` get no hint.

- A hint never quotes the sent text or any upstream text.

**3. `viola wait` / `viola last`**
- `wait` prints one static `waiting: builder` line on stderr (TTY only), then one result line on stdout:
  - `turn-ended  builder  19:44:10.221Z  cursor 49102`; `session-end` and every other kind without a `dialog_id` take the same `<kind>  <name>  <time>  cursor <n>` form
  - `question  builder  dialog 7  cursor 49310` (`DIALOG` is not coloured here: this is a result, not the board); `dialog unknown` when the event carries no `dialog_id`
  - `dialog-expired  builder  dialog 7  cursor 49400`: an expired hold takes the dialog form, as `question`, `permission` and `plan` do
  - `timed out  builder  30000 ms`
- There is no spinner and no elapsed-time counter.
- `last` prints `last  builder  turn-ended 19:44:10.221Z` on stderr (TTY only), then the message text on stdout with C0/C1 escaped, or `no message` on stderr for `null`.
- Message mode, the escaper `wait` and `last` share and `viola link` uses for a `since` that is no RFC 3339 instant: over decoded characters every control character (C0, DEL, C1) except `\n` and `\t` prints as `\xHH` text, two uppercase hex digits (ESC → `\x1B`, CR → `\x0D`, DEL → `\x7F`, U+009B → `\x9B`); nothing is stripped. `--json` output stays serde-escaped.

**4. Wheel, link and gate verbs** (the phraseology is the design)
- `viola pause builder` → `builder  wheel human  manual-pause  I have control`
- `viola release builder` → `builder  wheel driver  you have control`

`viola release builder --budget` → `builder  budget gate lifted until <time>  wheel human` when a pause stood, else `builder  budget gate open  wheel human`, one unstyled stdout line.

- `viola link overseer builder` → `link    overseer -> builder  since 19:43:58Z` (an existing pair prints `link    overseer -> builder  already linked`)
- `viola unlink overseer builder` → `unlink  overseer -> builder`
- `viola answer builder 7 < response.json` → `answered  builder  dialog 7`, or `unable  builder  not-delivered  unknown-dialog`
- `viola plugin install` → four stdout lines in one write: `viola@viola  written  <dir>` / `run these to add it to Claude Code:` / `  claude plugin marketplace add <dir>` / `  claude plugin install viola@viola`, `<dir>` the marketplace dir, quoted only where the host's shell would split it; plain ASCII, no SGR.

**5. `viola verify` / `viola run`**

`verify` is a step counter, the project's one progress pattern: one static stdout line per relied ledger row (`pass` / `fail`, uncoloured), then a summary line `stamped <ver>  <n> pass  <m> fail`, n + m the relied count; a run stopped by Ctrl-C or TERM before its rows are checked, or refused by the strict-modes check of its home tree before its log, prints no step line and no summary, only its fixed `unable:` / `hint:` pair, and exits 1; a `stamps.json` update refused by its strict-modes check prints the home-state pair.

- **`run`:** prints nothing once the child starts. A start refusal (exit 1) prints its cause's fixed-message line, for example `unable: builder is already live`, then that cause's hint (`hint: viola list`). The per-cause lines are listed under the send hints above. None of them shows a path or a pid.

`revive` prints nothing once the child starts, as `run`; `--list` prints one plain stdout line per logged session; a refusal (exit 1) prints its cause's fixed-message line and hint, and under `--json` a refused strict-modes check or preflight prints one stdout document and no hint. A refusal `run`'s start makes after a passed preflight prints no `--json` document: pending, owner: the working-route entry "Exit-cause code catalogue".

**Exit-code phraseology** (typed codes from architecture.md):

Each typed exit code has its human stderr form and, where it has one, its `--json` document: 0 the result on stdout, 1 `error: internal error` or a fixed-message `unable:` refusal with its hint (a `--json` document only for `viola revive`'s strict-modes and preflight refusals), 2 clap usage or the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` stderr line of an oversize `viola answer` / an unusable `viola send` (no document under `--json`), 10 to 14 an `unable` refusal with its reason (a refusal object under `--json`), 20 `error: wrapper fault`, 21 `unable  instance-unreachable` then the cause's hint, or for a refused strict-modes check its fixed home-state or instance-state `unable:` / `hint:` pair instead (pattern 1 and the paragraphs below), its `--json` document with stderr empty.

`viola plugin install`'s refused strict-modes check (exit 21) prints the home-state `unable:` / `hint:` pair as `viola list`'s does, or under `--json` the `instance-unreachable` document, stderr empty.

`viola link` / `viola unlink`'s refused strict-modes check of the driven instance, and that of `viola send`, `wait`, `last`, `answer`, `pause` and `release` on their instance, made before any frame (exit 21), prints the instance-state `unable:` / `hint:` pair naming that instance, as `viola list`'s does, or under `--json` the `instance-unreachable` document, stderr empty; no frame and no line are written.

### Navigation Pattern
Detail: .andromeda/design-system-detail/surface.md — read whole.

- Flat verbs, one lower-case word each: `run · revive · send · wait · last · list · answer · verify · pause · release · link · unlink · plugin install`.
- `viola list` is the board and the other verbs act on one strip. There is no interactive prompt, no TUI mode and no pager.

- `viola --help` lists the served verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`.

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
- **Optimistic "Sending…" spinner, then a green "Sent ✓" or a toast.** Rejected because a clearance does not count until it is read back. The readback box stays `open` until the matching `prompt-submitted` (for `/clear`, its new-session post-condition; for `/compact` on a verified CLI, its same-session compact post-condition), then fills instantly or is struck with `unable` plus the typed reason. There is no green, no toast and no presumption.
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
Point to the readback box in its one 0.1.0 place: the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`. If that column lacks the box or draws it differently, the signature does not exist. Inject it. A 0.2.0 frontend's places for the box, and its test, are that version's to name (its technology is not chosen).

### 4. Token Test
Detail: .andromeda/design-system-detail/self-validation-protocol.md — read whole.

A 0.2.0 frontend design input; 0.1.0 renders no page.

Every colour traces to one of the eight `--c-*` values, read only where a role alias is assigned (components use the role aliases, never `--c-*`); every length traces to a token or a `ch` track, every duration to `--cock-dur` or `--fade-dur`; the only literal lengths are the two width breakpoints, because `@media` conditions cannot read custom properties. A ninth hex value or any other untokened px value signals no system.

### 5. Sameness Test (from Interface Design)
If another AI given a similar prompt would produce substantially the same
output — you have failed. The interface must emerge from THIS product's
domain exploration, not from statistical patterns in training data.

---

<!-- subjects: expression-motion · palette · status-states · typography · spacing · radius · icons · cli-tokens · cli-output · cli-list · cli-send · cli-verbs · cli-verify-run · cli-exit-codes · cli-help · validation -->

## Design Decisions Log

History: design-system-amendments.md (live); the log moved verbatim to design-system-amendments-archive.md (U35).

