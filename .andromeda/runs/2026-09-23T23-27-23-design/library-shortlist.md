# Library Shortlist — viola

## Color Palettes

(Top 3, anchored to the quiz mood "Tower cab strip bay after dark" and the exploration's Color World. Each one comes from a different library category. The security tier is Minimal, so no conservative-palette constraint applies. WCAG contrast is still required: the exploration's computed ratios are the target.)

### 1. Developer Tool / IDE (#81, SaaS & Software)

- **Primary:** #1E293B | **Secondary:** #334155 | **Accent:** #22C55E
- **Background:** #0F172A | **Card:** #1B2336 | **Border:** #475569
- **Match reason:** This is the industry match for a developer tool that runs dark only (Q6). It also has the same three-step dark tonal ladder as the Color World. Background #0F172A becomes console anthracite #1E2124. Card #1B2336 becomes holder plastic #2A2E33, the strip surface. Border #475569 becomes rack-rail aluminium #4A5057, which is close in value; both are decorative 1px rules. Take the structure and warm the hues. The slate-blue cast should move toward the neutral laminate of "matte console laminate… nothing glows" (Q3 sensory anchor).
- **Adapt for viola:** Drop the accent #22C55E ("run green"). It is the traffic-light and terminal-green default that the exploration rejects (Defaults to Reject #1 and #8). The only attention colour is arrival-holder amber #D97706, used for `dialog_pending` only. Primary text is strip buff #E6D8AE (11.4:1 on #1E2124). Secondary text is lamp-off buff #9C9278 (5.25:1).

### 2. Coding Challenge & Practice (#151, Other)

- **Primary:** #22C55E | **Secondary:** #059669 | **Accent:** #D97706
- **Background:** #0F172A | **Card:** #192134 | **Border:** rgba(255,255,255,0.08)
- **Match reason:** This is the only library palette whose accent is exactly the quiz's arrival-holder amber #D97706 on a dark ground, and it uses amber as the one sharp accent rather than a primary. That matches Q3 ("amber marks only the cocked strip"). Computed ratio: #D97706 on #0F172A is about 5.6:1, and the exploration gives 5.08:1 on #1E2124. Both pass AA for the `DIALOG` word.
- **Adapt for viola:** Keep the amber accent and its dark ground. Discard the green primary and secondary, and discard the library's "difficulty gradient" (green/amber/red). That gradient is the traffic-light status scheme the exploration rejects. Amber is never used for warnings, budget, errors or hover (Color World). Replace the alpha border with solid #4A5057, which is decorative only. Any border that carries information uses strip buff.

### 3. Ride Hailing / Transportation (#96, Travel & Transportation)

- **Primary:** #1E293B | **Secondary:** #334155 | **Accent:** #2563EB
- **Background:** #0F172A | **Card:** #192134 | **Border:** rgba(255,255,255,0.08)
- **Match reason:** This is the transport-category dark palette ("map dark + route blue"). Its single blue marks the route, the path handed from one point to the next. That is the role of departure-holder blue #5B8DB8 in the Color World: only the `driver → driven` transfer marker (Handoff / transfer of control), never a status or a fill. It also matches the vStrips reference (Q2): links are drawn as transfer markers, not as a node graph.
- **Adapt for viola:** The library's #2563EB is only about 3.5:1 on #0F172A (computed), so it fails AA for link text. Use the exploration's lighter, desaturated #5B8DB8 (about 4.6:1 on #1E2124). It also reads as "anodised holder plastic" rather than Bootstrap/Tailwind blue, which the anti-patterns ban as a brand colour. Blue appears on link markers only. No other page element gets a hue.

**CLI mapping (xterm-256, human TTY only; `--json`, non-TTY, `NO_COLOR` and `viola run` get no colour):**

| Role | Hex | ANSI 256 | Use |
|---|---|---|---|
| Arrival-holder amber | #D97706 | 172 (#D78700). 16-colour fallback: 33 yellow | `DIALOG` column only, always next to the word |
| Lamp-off buff (`stale`) | #9C9278 | Prefer SGR 2 (dim) on the default foreground. Fixed-colour fallback: 144 (#AFAF87), which keeps the hue. The nearest by distance is 138, but it skews rose. | `stale` rows, dimmed, as the quiz specifies |
| Strip buff | #E6D8AE | 187 (#D7D7AF) | Optional header emphasis. Normally the default foreground |
| Departure-holder blue | #5B8DB8 | 67 (#5F87AF) | Not used in the CLI unless link rows get a marker. The quiz limits CLI colour to amber and dim |
| Anthracite / holder / rail / graphite | #1E2124 / #2A2E33 / #4A5057 / #3B3A36 | 234 / 236 / 239 / 237 | Do not set backgrounds. The terminal background belongs to the user |

## Font Pairings

(Top 3, checked against the anti-patterns. **Binding constraint:** the CSP resolves `font-src` to `'none'`, so nothing listed below can be loaded from Google Fonts or self-hosted. Each pairing is kept for its mood and structure and translated into a stack of named, already-installed OS fonts. Installed fonts referenced by `font-family` name are not fetched, so they are CSP-clean. The anti-patterns ban `system-ui` / `-apple-system`, Arial and Helvetica as primary faces, so the stacks below name specific faces and end at the generic `sans-serif` / `monospace`. The **cli** surface uses the terminal's monospace, which the user configures. There, the brand shows through the column structure, phraseology words and amber/dim cues, not through typography.)

### 1. Fira Code + Fira Sans (#42 Dashboard Data) → Bahnschrift + Cascadia Mono

- **Mood:** dashboard, data, technical, precise. A cohesive pairing: mono for values, sans for labels.
- **Heading:** Name cell and rack separator labels. Stack: `"Bahnschrift", "DIN Alternate", "Avenir Next Condensed", "DejaVu Sans Condensed", sans-serif`. Weight 600 (semibold) for the callsign-weight name cell and 400 for labels. Use the SemiCondensed width (the named instance, or `font-stretch`) for the narrow fixed fields. Uppercase phraseology labels (`LIVE`, `WHEEL`, `DIALOG`).
- **Body:** Field values, feed lines and timestamps. Stack: `"Cascadia Mono", Consolas, Menlo, "DejaVu Sans Mono", "Noto Sans Mono", monospace`. Weight 400, with `font-variant-numeric: tabular-nums` on numeric fields.
- **Google Fonts:** Reference only; it cannot be loaded under the CSP: https://fonts.google.com/share?selection.family=Fira+Code:wght@400;500;600;700|Fira+Sans:wght@300;400;500;600;700
- **Match reason:** Bahnschrift is Microsoft's DIN 1451 face and ships with Windows 10 1709 and later. Windows is the first target. DIN is the lettering of transport and aviation signage and printed forms, so it fits the Frequentis/Saab EFS strip anatomy from Q2: "ruled box grid, one field per fixed column, heavier callsign-weight name cell". Its condensed widths fit the fixed NAME · LIVE · STATUS · WHEEL · DIALOG · CLI fields. Cascadia Mono has no ligatures, so `->`, `[RB]` and `[/ ]` render literally, and the monospace values keep the web strip column-identical to the k9s-style CLI table. "Standard phraseology — short, fixed" is carried by the uppercase DIN labels.
- **Anti-pattern check:** PASS. No banned face is in either stack. Fira itself is not banned, but it is unavailable under the CSP, so the system translation above replaces it.
- **CSS (hand-written, no Tailwind):** `--font-label: "Bahnschrift", "DIN Alternate", "Avenir Next Condensed", "DejaVu Sans Condensed", sans-serif; --font-field: "Cascadia Mono", Consolas, Menlo, "DejaVu Sans Mono", "Noto Sans Mono", monospace;`

### 2. JetBrains Mono (#61 Terminal CLI Monospace) → Cascadia Mono, single family

- **Mood:** terminal, cli, developer, precision, information density
- **Heading:** Stack: `"Cascadia Mono", Consolas, Menlo, "DejaVu Sans Mono", monospace`. Weight 400 and uppercase for labels. The name cell is the one exception at 600: the library's "bold ruins mono character" rule is overridden once, for the callsign cell.
- **Body:** Same stack, weight 400. The library's strict three-step scale (12/14/16) and line-height of about 1.2 suit strip density.
- **Google Fonts:** Reference only; it cannot be loaded under the CSP: https://fonts.google.com/share?selection.family=JetBrains+Mono:ital,wght@0,400;0,500;1,400
- **Match reason:** A strip printer prints flight-progress strips in fixed pitch, and the "tower tape" feed is one fixed-column line per event (time · instance · kind · text). An all-mono page lines up character for character with the CLI mirror (`[RB]` / `[  ]` / `[/ ] not-delivered`, k9s columns). That is the strongest case for "same columns and words" across web and CLI (Q2 k9s). Take the library's "ASCII borders and text-based" note as ruled 1px CSS borders, not box-drawing art. Its "hacker / matrix / OLED" keywords are out: the exploration rejects neon hacker themes.
- **Anti-pattern check:** PASS. No banned face. JetBrains Mono is unavailable under the CSP and is replaced by Cascadia Mono / Consolas.
- **CSS (hand-written):** `--font-field: "Cascadia Mono", Consolas, Menlo, "DejaVu Sans Mono", monospace; --font-label: var(--font-field);`

### 3. JetBrains Mono + IBM Plex Sans (#9 Developer Mono) → Cascadia Mono + Bahnschrift

- **Mood:** code, developer, technical, precise, functional
- **Heading:** The library heading is JetBrains Mono. Translated: `"Cascadia Mono", Consolas, Menlo, monospace` at 600 for the name cell.
- **Body:** The library body is IBM Plex Sans, a UI sans for chrome and labels. Translated: `"Bahnschrift", "Avenir Next Condensed", "Noto Sans", "DejaVu Sans", sans-serif` at 400.
- **Google Fonts:** Reference only; it cannot be loaded under the CSP: https://fonts.google.com/share?selection.family=IBM+Plex+Sans:wght@300;400;500;600;700|JetBrains+Mono:wght@400;500;600;700
- **Match reason:** This is the inverse split of pairing #1. The mono face carries the name cell and all values, the "callsign" as printed. The sans carries only rack separators, column headers and the budget/ATIS header line. It fits a page where almost everything is a fixed field and the label layer is thin. This option has the weakest domain tie of the three. Keep it as a fallback if Phase 4 finds Bahnschrift too condensed or too "signage" for the name cell.
- **Anti-pattern check:** Replaced IBM Plex Sans with Bahnschrift. IBM Plex Sans cannot be loaded under the CSP, and its only zero-install stand-in is the OS UI face (Segoe UI Variable / San Francisco via `system-ui` / `-apple-system`), which the anti-patterns ban. IBM Plex Sans itself is not banned.

## Design Direction

- **Primary direction:** Precision & Density. Utility & Function is the secondary influence: "neutral with semantic colour for status only; everything has its place" matches the stable rack slots.
- **Style preset:** Minimalism & Swiss Style (#1). E-Ink / Paper (#56) is a secondary reference for surface behaviour, inverted to buff-on-anthracite. Rejected presets:
  - Dark Mode (OLED) (#7): pure-black void and neon glow, against "a lit near-black, not a void" and "nothing glows unless a lamp is on it".
  - Real-Time Monitoring (#31): pulsing status dots, blink and toasts, all of which the exploration rejects.
  - HUD / Sci-Fi FUI (#51): glow and telemetry animation.
- **Key properties:**
  - **Keywords:** borders-only, tabular numbers, monochrome with one functional colour, grid-based, essential elements only, matte and paper-like, high contrast, distraction-free. From Precision & Density: "cold like a terminal, dense like a trading floor", which becomes "a tower strip bay at night" here.
  - **Effects:**
    - No shadows, no gradients except the stylesheet `linear-gradient` strike on a refused readback box, and no glow.
    - E-Ink "sharp transitions (no fade)" for state changes that are discrete. The readback close (outline → filled) and `stale` dimming are instant.
    - The only motion follows the quiz's Q4 budget: a one-time 12px/160ms ease-out `translate` for the cocked strip, and a ≤120ms opacity fade for a new feed line or link marker.
    - `prefers-reduced-motion` sets every duration to 0.
    - The Swiss preset's "subtle hover 200–250ms" is dropped. Hover is limited to focus rings and link underline, because v1 has no controls.
  - **Color focus:** Two layers:
    - Monochrome buff-on-anthracite tonal ladder: #1E2124 → #2A2E33 → #4A5057 for surfaces and rules; #E6D8AE, #9C9278 and #3B3A36 for ink.
    - Hue: one attention colour, amber #D97706 (`dialog_pending` only), and one role colour, blue #5B8DB8 (transfer markers only). This is the Swiss "single accent only" rule with a second hue limited to one role.
  - **Depth approach:** Borders only. Flat, lit only by the lamp. Radius 0 (`--border-radius: 0px` from the Swiss preset), because strips and readback boxes are ruled rectangles. Separation comes from:
    - surface tint: holder #2A2E33 on anthracite
    - 1px ruled field grids: buff where the border carries information, rail #4A5057 where it is decorative (about 2.0:1)
    - the cocked strip's physical offset, never elevation

## Industry Rules

- **Product type match:** Developer Tool / IDE (#81), merged with Productivity Tool (#16) because it is the closest "operator utility" entry. Filtered for the web-spa plus cli surfaces. Landing-page rules are dropped (there is no landing page).
- **Primary style:** Dark Mode + Minimalism (#81). The dark default and the minimalism apply. The library's "OLED" variant (pure black, glow) does not: the exploration wants a lit near-black.
- **Dashboard style:** "Real-Time Monitor + Terminal" (#81). Take the Terminal half: fixed-column rows and a tape-style feed. Reject the Real-Time Monitor effect set (pulse, blinking status dots, toasts, auto-refresh indicators) under Defaults to Reject. The live SSE feed appends lines and never animates progress.
- **Color focus:** "Dark syntax theme colors + Blue focus" (#81) and "clear hierarchy + functional colors" (#16). Apply them as a dark neutral ground with blue in one role only (transfer marker #5B8DB8). Functional colour is limited to amber for `dialog_pending`. Syntax-theme colouring does not apply: event text is plain text, and Markdown and highlighting are banned by the security plan's output-encoding rule.
- **Anti-patterns:**
  - Light mode default (#81). This is satisfied: the page is dark only (Q6).
  - Slow performance (#81, #16). Embedded assets, no web fonts, no build step and no JS library other than vendored Lit.
  - Complex onboarding (#16). The page is view-only. Its first view is the bay itself, and missing values print `unknown` in their boxes.
  - Excluded as not applicable: #1 SaaS (General)'s "dark mode by default" anti-pattern belongs to a different product type.
- **Key considerations:**
  - Fast performance and "speed and efficiency focus" (#16). Tie this to "no skeletons, no spinner".
  - Keyboard shortcuts (#81/#16 must-have). In v1 this is covered by the CLI verbs (`viola pause` / `release`) and the human's keystroke on the wheel. The view-only page needs only correct tab order and focus rings on links. Do not add a command palette: #81's "Key Effects: command palette" implies the page can change state, which the exploration rejects.
  - Documentation (#81 must-have). This is not a page feature. The phraseology vocabulary (`open`, `RB`, `not-delivered`, `human-typing`…) acts as self-describing labels.
  - Quick actions at 150ms (#16) map to the ≤160ms motion ceiling.

## UX Guidelines

(Top 10, all High severity. Filtered to web-spa, cli and "all" platforms. Mobile, VisionOS, touch, form and landing guidelines are excluded because the page is view-only with no inputs. Where the library's DO conflicts with a settled exploration or quiz decision, the viola reading is given in brackets.)

| # | Guideline | Severity | Platform | DO | DON'T |
|---|---|---|---|---|---|
| 1 | Color Only (#37): don't convey information by colour alone | high | all (web + cli) | Use icons/text in addition to colour [viola: the amber band always sits next to the `DIALOG` word and dialog kind; `stale` dimming sits next to the printed `stale`; a CLI colour is never shown without its status word] | Red/green only for error/success |
| 2 | Color Contrast (#36): text must be readable against its background | high | all | Minimum 4.5:1 for normal text [buff 11.4:1, lamp-off buff 5.25:1, amber 5.08:1, blue ≈4.6:1 on #1E2124; graphite on buff ≈8.0:1; amber on holder #2A2E33 is only ≈4.3:1, so it stays a non-text band; rail #4A5057 at 2.0:1 is decorative only] | Low-contrast text |
| 3 | Reduced Motion (#9): respect the user's motion preference | high | all | Check the `prefers-reduced-motion` media query [every duration set to 0; the cocked offset position and amber band remain] | Ignore accessibility motion settings |
| 4 | Excessive Motion (#7): too many animations distract | high | all | Animate 1–2 key elements per view at most [exactly two: the cocked-strip translate and the new-line/marker fade; each plays once, with no loop or pulse] | Animate everything that moves |
| 5 | Content Jumping (#19): layout shift when content loads is jarring | high | web | Reserve space for async content [stable rack slots, never re-sorted; `unknown` printed in its fixed box right away; the cocked strip uses `transform: translate`, so the rack does not reflow] | Let content push the layout around |
| 6 | Error Feedback (#33): users need to know when something fails | high | all (web + cli) | Show clear error messages near the problem [the typed refusal (`not-delivered` · `no-prompt-submitted`, `turn-running`, `control-character`) prints in the cell next to the struck readback box; the CLI prints `[/ ] not-delivered` plus a typed exit code] | Silent failures with no feedback |
| 7 | Error Messages announced (#44): errors must be announced | high | all | Use `aria-live` or `role=alert` for errors [a polite live region for a strip turning cocked (`dialog_pending`) and for readback refusals only; do not put the whole SSE tape in a live region] | Visual-only error indication |
| 8 | Loading Indicators (#78): show system status during waits | high | all | Show a spinner or skeleton for operations over 300ms [**overridden by the exploration:** system status is shown as state words, not motion. An `open` readback box until confirmed, `unknown` in empty fields, the ATIS age `read 4m ago`, and a static CLI line `waiting: <session>`] | No feedback during loading |
| 9 | Horizontal Scroll (#69): avoid horizontal scrolling | high | web | Make sure content fits the viewport width [fixed strip columns use CSS grid with a truncating text column in the feed; long prompt text wraps or truncates inside its cell as plain text; this also prepares for the later phone view] | Content wider than the viewport |
| 10 | Focus States (#28): keyboard users need visible focus indicators | high | all | Use visible focus rings on interactive elements [only links exist in v1; draw the ring in strip buff, 1–2px, radius 0] | Remove the focus outline without a replacement |
