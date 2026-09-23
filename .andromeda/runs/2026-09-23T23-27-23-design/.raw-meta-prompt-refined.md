## Output Protocol

You are an iteration agent improving `D:/dev/projects/viola/.andromeda/runs/2026-09-23T23-27-23-design/design-system-plan-draft.md` (Design System — viola). You may cross-reference, read-only, `exploration.md`, `library-shortlist.md`, `quiz-design.md`, `tooling-decisions.md`, `review-feedback-1.md` and `review-feedback-2.md` in the same run directory.

Rules:
1. **Output patches (old → new) plus a changelog.** Do NOT reproduce the full document.
2. **Patch format:**
   ### Patch N: <one-line description>
   **Old:**
   ```
   <exact text copied from the current draft, long enough to be unique>
   ```
   **New:**
   ```
   <replacement text>
   ```
   Use **at most 8 patches per iteration.** Spend them in this priority order:
   1. Internal contradictions between sections, where a value, word or treatment disagrees across Brand Identity / Color Palette / Typography / Motion / Iconography / Surface: web-spa / Surface: cli / Anti-Patterns / Design Decisions Log.
   2. Signature (the readback box) or expression-budget drift (base `0.2`, web-spa `0.3`, cli TTY `0.2`, machine output `0.0`).
   3. Deviations from `exploration.md` or `library-shortlist.md` that are not recorded in the Design Decisions Log.
   4. Downstream-readiness gaps: tokens or states that a downstream skill would have to invent.
   5. Anti-pattern completeness and cross-reference hygiene.
3. **Changelog:** write exactly ONE line for the whole iteration, never one line per patch:
   `[Iteration N] [substantive|cosmetic] <description>`
4. **Prohibited:**
   - Reproducing the full document.
   - Restructuring sections without a stated defect.
   - Labelling cosmetic rewording as substantive.
5. **If you find no issues,** output `No patches` and a `cosmetic` changelog line.

Lane rules. Any patch that breaks one of these is invalid:
- **Stay in the design lane:** design tokens, patterns, visual and motion states, and anti-patterns. Do not change tooling settled by Phase 1 and architecture. That covers Lit 3.3.3 light-DOM `viola-*` elements, hand-written `@layer` CSS in `/assets/app.css`, clap 4.6.7, windows-sys, the hand-written SGR module and the CSP. Do not propose a component library, an animation library, web fonts, a build step, or backend or data-model changes.
- **Do not introduce a ninth hex value.** The palette is closed: #1E2124, #2A2E33, #4A5057, #E6D8AE, #9C9278, #3B3A36, #D97706, #5B8DB8.
- **Phase 4.5 settled several decisions.** The founder approved the colours, expression levels and signature "as drafted", along with the per-OS font stacks from review rounds 1 and 2. Change these only to resolve an internal contradiction, and record the reason in the Design Decisions Log.
- **Do not write content that other specialists own:**
  - no test cases (you may define visual or motion scope that tests derive from);
  - no observability instrumentation content (you may define visual error states);
  - no new accessibility-attribute rules or standards-conformance claims (you may state contrast values, the motion budget and focus-ring colour);
  - no security-plan content.

## Analysis Protocol

Work through these steps in order. Do not go straight from scanning to patching.

1. **Read the whole draft once without noting anything.** It is about 860 lines. Most real defects in this draft are disagreements between a summary table and the concrete spec it summarises, and you only see those after reading both.

2. **Cross-reference these viola-specific seams.** Each pair is a place where the draft states the same fact twice:
   - **The signature, as one chain:**
     - the Brand Identity "Signature element" bullets;
     - the Typography role that sets `RB`;
     - the `<viola-readback>` `data-rb` table;
     - the `Readback::*` domain rows;
     - the `--rb-*` tokens, the `.rb` rule and the `@layer states` selectors;
     - the CLI `[RB]` / `[  ]` / `[/ ]` mirror;
     - the Decisions Log "Signature" bullet;
     - the Self-Validation "Signature Test".
   - **Colour roles:**
     - Semantic Colors (4 summary rows) against the Domain status rows that each summary row names;
     - Border Progression against the Core Colors "Usage" column;
     - Core Colors "Usage" against every border and tick described in Component Patterns.
   - **Typography:** the Typography role table (face, weight and size per role) against the `@layer tokens` font variables and the per-cell anatomy in Component Patterns 1–5.
   - **Geometry:**
     - the Spacing table ("inline padding" appears in two rows) against `--strip-cols` / `--tape-cols`;
     - the Component Patterns padding;
     - the printed words that must fit each track;
     - the Navigation "Width" breakpoints.
   - **Expression and motion:**
     - the Brand Identity expression table;
     - the Motion duration-scale row for 0.3–0.4;
     - Motion "This project's values" and "High-impact moments";
     - `@layer motion`;
     - the web-spa "NEVER animate…" ban;
     - the Decisions Log.
   - **Web against CLI:**
     - the web strip's printed values against the `viola list` columns and words;
     - both against the Iconography separator and arrow rules.
   - **`exploration.md` against the draft:**
     - Domain Concepts (13 items) against the Brand Identity "Domain anchors" list;
     - Defaults to Reject against Rejected Defaults;
     - Signature Element against Signature;
     - Bootstrap phases against the "(bootstrap first)" labels.
   - **`library-shortlist.md` against the draft:**
     - Design Direction, the Rejected presets and Industry Rules against the Brand Identity "Design direction" paragraph, the Per-Surface Bans and the Decisions Log;
     - UX Guidelines #1–#10 against the sections that apply them.
   - **The Decisions Log against the body:** every "Deviation note", "overrides", "reserved" or "dropped" statement in the body needs a matching Log entry.

3. **Check each dimension below, using its anchor as a calibration example.** The anchors were verified byte-for-byte against the draft at refinement time. Earlier iterations may already have fixed some of them. Before you patch an anchor issue, re-find its quoted text in the current draft. If the text is gone, treat the issue as fixed, do not re-patch it, and look for the next issue of the same kind.

4. **Out-of-scope discipline.** Some findings would require writing content another specialist owns:
   - test cases (tests' domain: design defines visual, motion and surface scope, and tests writes the cases);
   - observability instrumentation content (obs' domain: design defines visual error states, and obs instruments them);
   - accessibility-attribute rules or standards-conformance claims (a11y's domain: design provides contrast values, the motion budget and focus-ring colours, and a11y derives ARIA);
   - security-plan content (security's domain);
   - naming a concrete observability or error-reporting platform.

   Do NOT patch these. Instead, check that the document states the boundary requirement: what must hold, not how it is wired. Patch only if that boundary is unstated.

   The draft already contains some specialist-adjacent text: the accessibility roles and attributes already in Component Patterns and Navigation, the CI font-package names in Typography, and cookie and CSP notes in component 7 and the Per-Surface Bans. Do not extend, reword or delete that text unless it contradicts a design value elsewhere in the draft.

5. **Resolve contradictions with a fixed tie-break.** When two statements disagree and neither is shown wrong by arithmetic or by the contrast table, keep the more concrete one and patch the summary. Concreteness, highest first:
   1. Phase 4.5 Decisions Log entries.
   2. The `@layer tokens` / `states` / `motion` CSS block and the Domain status rows.
   3. Component Patterns.
   4. Summary tables (Semantic Colors, Text Hierarchy, Typography "Usage", Border Progression).
   5. Rationale prose.

   If you change a founder-approved value, add a Decisions Log line.

6. **Make each Old block unique.** Domain status rows repeat long strings such as `| #1E2124 | #E6D8AE | #E6D8AE |`, so always include the `Enum::Variant` cell. CSS lines repeat selectors, so include the declaration. CLI samples depend on exact spacing, so copy them character for character.

7. **Prioritise by downstream impact.**
   - Band A is dimensions 1–3. A downstream skill (route, setup-project, tests, a11y, obs) would derive something wrong or have to invent something.
   - Band B is dimensions 4–6. The implementation would be wrong or would drift.
   - Band C is dimensions 7–8. Surface polish.

   Spend patches on Band A first. Use Band B only if budget remains. Use Band C only for a one-line fix that cannot wait. A typo in rationale prose never outranks a value contradiction.

## Analysis Dimensions

### 1. Downstream Readiness (geometry, bootstrap, setup-project rules) [priority: high]
- **Do the fixed `--strip-cols` tracks hold their longest printed word plus cell padding?** At 13px, every monospace in `--font-field` advances about 0.6em, so 1ch is about 7.8px and 16px of padding is about 2ch. Check these tracks:
  - LIVE `6ch` holds `stale` (5ch).
  - STATUS `8ch` holds `unknown` (7ch).
  - DIALOG `20ch` holds `DIALOG permission` (17ch).
  - WHEEL and CLI have a maximum of `24ch` and hold `driver · budget-paused` and `2.1.281 unverified-cli` (22ch each).

  Which inline padding is canonical? The Spacing table gives both `space-micro` 4px ("the inline padding inside a field cell") and `space-xs` 8px ("Field-cell inline padding in strips"). Typography lists "`unknown` fitting its box" as a render assertion, so this arithmetic has to work on paper.
- **Can the 760–1023px layout be built without invention?** Navigation says "a strip wraps to two lines (band · NAME · LIVE · STATUS · DIALOG / WHEEL · CLI)". The strip anatomy is a single-row `grid-template-columns: var(--strip-cols)` with a fixed `block-size: var(--strip-h)` (32px), and two 20px lines do not fit in 32px. Is there a second-row template or a wrapped-height token? When the refinement agent searched for "760" and "two lines", both appeared only in the Navigation Width list.
- **Does the tape layout work with the readback element?** `--tape-cols` gives "RB box" and "word / reason" separate tracks, but `<viola-readback>` is one element with its own `inline-grid` of box plus word. Does the plan say how one element fills two parent tracks, so that route and implement do not have to choose?
- **Can setup-project lift the validation rules verbatim?** The Self-Validation "Token Test" still reads "Read your color/spacing values aloud. Do they trace back to the palette and scale above?" It does not name the eight `--c-*` values or the rule "role aliases (components use these, never --c-*)".
- **Accessibility boundary:** do not add, extend or rewrite any ARIA content here. The only design-lane check is whether the announcement intent and the visual-only states are stated in words.

**Anchor example:** Surface: web-spa → Tokens (platform-specific), `--strip-cols`; Component Patterns 1, Anatomy; Spacing table

> "--strip-cols: var(--band-w) minmax(16ch, 32ch) 6ch 8ch minmax(10ch, 24ch) 20ch minmax(10ch, 24ch);"

> "The other cells are Cascadia 13px buff values with 8px inline padding."

> "and the inline padding inside a field cell"

**Issue:** With 8px padding on each side (about 2ch), LIVE needs about 7ch to hold `stale` and STATUS needs about 9ch to hold `unknown`. The tokens give 6ch and 8ch. With the `space-micro` 4px reading, both still overflow by a fraction of a ch before the 1px `--rule-field` divider is counted. The Spacing table also assigns "field-cell inline padding" to two different tokens (4px and 8px).

**Why this matters:** DejaVu Sans Mono is "the widest case", and on that Linux CI render the `unknown` and `stale` words clip or spill into the next cell. The declared render assertion fails, and implement has to either widen tracks ad hoc or silently shrink padding. The CLI column parity ("the same columns and words as the web strip") then rests on numbers nobody chose.

**Adversarial:** Suppose implement "fixes" the overflow by switching LIVE and STATUS to `minmax()` tracks. At 1024px, does the sum of the maxima plus the page margin (2 × 16px) plus the reserved `--cock-offset` end padding (12px) still avoid horizontal scroll in DejaVu Sans Mono? The plan promises "Never horizontal scroll". Is that promise now checkable only by rendering, when it should be checkable by reading?

### 2. Signature and Brand Identity Coherence [priority: high]
- **Is the `RB` glyph specified the same way everywhere it appears?** The Typography Data row gives it weight 400. The `data-rb` table says "11px 600". The `.rb` CSS uses `var(--fw-strong)` with `var(--fs-label)` (the Label size) in `var(--font-field)` (the Body face). Which Typography role owns `RB`, and does that role's row match the CSS?
- **Is the state count stated once and consistently?**
  - Brand Identity says "three drawn states", and then "A fourth printed word, `unconfirmable`, reuses the open drawing".
  - `data-rb` has four values.
  - `exploration.md` names `data-rb="open|read|refused"`.
  - The Decisions Log Signature bullet lists three states plus `unconfirmable`.

  Is the extension from the exploration's three values to four recorded as a deliberate change? And is the Squint-Test trade-off (`open` and `unconfirmable` look identical, and only the word cell differs) stated in exactly one place?
- **Does "the named pair" in Motion refer to the same two things as the Brand Identity table?** Motion "High-impact moments" says "the web surface's 0.3 allows the named pair and nothing more" and then lists "The cock" and "The readback close", which is not animated. The Brand Identity table's "Exactly two motions" are the cock and the tape-line / marker fade. Also, the Self-Validation "Signature Test" is template text ("Point to the signature element in your output"). Does it name the readback box and its three required locations: tape send lines, outbound transfer markers and the CLI `[RB]` column?

**Anchor example:** Typography table, Data row, against Component Patterns 2 (`data-rb` table) and the `.rb` CSS rule

> "| Data | Cascadia Mono stack | 400 | 13px / 20px (11px for `RB` inside the box) | `font-variant-numeric: tabular-nums` | Percentages, timestamps `HH:MM:SS.mmm`, `since`, cursors, dialog ids, ages, skipped counts |"

> "| `read` | Solid buff with `RB` in graphite, 11px 600 | `read back` |"

> "font: var(--fw-strong) var(--fs-label)/1 var(--font-field);"

**Issue:** The signature's only glyph gets weight 400 in the Typography role table, but 600 in the component spec and the CSS. The Data role also carries no Label-size token, yet the CSS borrows `--fs-label` for `RB`. Per the tie-break, the CSS and Component Patterns win, so the Typography row is the side to patch. It should either give `RB` its own weight or name it as a Data exception at 600.

**Why this matters:** The Typography table is where tests and a11y read type roles from. A filled box with a 400-weight `RB` at 11px is visibly lighter than one at 600, so the "stamp" that makes the signature recognisable in the Squint Test depends on which section the implementer reads.

**Adversarial:** Suppose the Signature Test stays generic and the `RB` weight stays split. Could a downstream implementation pass every written check while drawing the readback box differently in the tape and in the transfer marker, which are built by two different components? Which sentence in the plan forbids that today?

### 3. Library-Shortlist Faithfulness [priority: high]
- **Is the shortlist direction named?** The Brand Identity "Design direction" paragraph is pure metaphor. Does it (or the Decisions Log) name the shortlist direction, Precision & Density (primary) and Utility & Function (secondary)? Does it name the presets, Minimalism & Swiss Style (#1) and E-Ink / Paper (#56)? Does it name the rejected presets, OLED (#7), Real-Time Monitoring (#31) and HUD / Sci-Fi FUI (#51)?
- **Are the shortlist deviations logged?** The deviations appear only in rationale prose:
  - In the Color Palette rationale (line 53 at refinement time): #81's run-green was dropped and the slate was warmed; #96's blue went from #2563EB to #5B8DB8.
  - In the Typography rationale: pairing #1 was translated.

  The Decisions Log has no library-shortlist entry. The shortlist also keeps pairing #3 as a fallback "if Phase 4 finds Bahnschrift too condensed or too 'signage' for the name cell". The plan answers that question by setting Callsign at "normal width", so the rejection of that fallback should be logged.
- **Are the UX guidelines cited by number?** The plan cites none of them; searches for `#78`, `#37`, `#69`, `#28`, `#19` and "guideline" found nothing. The web-spa loading ban says it "overrides the generic web-guide 'skeleton shimmer' advice per the exploration". Shortlist guideline 8 is Loading Indicators (#78), whose DO is "spinner or skeleton". Cite the override by number so that tests and a11y can trace it.

**Anchor example:** Brand Identity → Design direction

> "The page is a control-tower strip bay after dark. Buff paper strips sit in anthracite holder boots on ruled aluminium racks."

> "The density is the calm density of a strip board, not a monitoring dashboard: fixed fields, tabular numbers, one attention colour and one handoff colour."

**Search evidence:** The whole draft was searched for `Precision`, `Swiss`, `E-Ink`, `Utility`, `preset`, `HUD` and `Real-Time`, with 0 hits each. `OLED` has 1 hit, in Rejected Defaults ("not an OLED void"), which is not a preset citation. `direction` has 1 hit, the "Design direction" heading itself.

**Issue:** "Not a monitoring dashboard" is an implicit rejection of Real-Time Monitoring (#31), but no text in the plan ties it to the shortlist. The shortlist's Primary direction, Style preset and Rejected presets therefore have no trace in the design system.

**Why this matters:** The shortlist is the pre-validated foundation that route, setup-project and later redesigns check against. Without the direction and preset names, a reviewer cannot tell a deliberate choice from an accident. The rejected #31 effect set (pulse, blinking dots, toasts) is then banned only by the reviewer's memory.

**Adversarial:** Suppose a future iteration adds "subtle 200ms hover" for polish. The Swiss preset allows 200–250ms hover, and the shortlist explicitly dropped it. Would any sentence in this plan, as opposed to the shortlist, block the change, given that Motion's own 0.3–0.4 row already lists `150ms ease-out` hover?

### 4. Colour-Role Consistency and Domain Anchoring [priority: high]
- **Does each Semantic Colors summary row agree with the Domain status rows it names?**
  - The Warning row names `unverified-cli`, `unconfirmable` and "expired budget window". Compare it with `CliVerified::false`, `Readback::unconfirmable` and `Window::expired`.
  - The Info row names "activity, harness turn, skipped counts, tape status" with text #9C9278 and border #4A5057. Compare it with `TapeConnection::open` (text #E6D8AE), `TapeConnection::closed` (border #E6D8AE), `Skipped::zero` (border `none`) and `Skipped::nonzero` (a buff box).

  Apply the Border Progression rule "Emphasis #E6D8AE = Borders whose presence is the state", and patch the summary side.
- **Is every coloured rule listed in both palette tables?** Component Patterns 3 gives `<viola-transfer>` "a 2px departure-blue left tick". Border Progression has no blue entry. The Core Colors handoff usage is "Transfer-marker arrow, link names and link tape lines only". `--rule-strong: 2px` lists only the focus ring and Raised-3. At refinement time, searches for "tick" and "departure-blue" found them only in that anatomy line.
- **Are the typography roles and domain anchors consistent?**
  - `TAPE` appears in both the Heading row (600, 0.12em) and the Label row (400, 0.06em).
  - The Label row sets `DIALOG` in the Bahnschrift stack, while strip anatomy says "The other cells are Cascadia 13px".
  - The Brand Identity "Domain anchors" list has 5 entries, and `exploration.md` Domain Concepts has 13. Coasting track, fuel state / minimum fuel, verified aircraft type and pilot-in-command appear only inline elsewhere. Searches of Brand Identity (lines 1–45) for `coasting`, `fuel`, `aircraft type`, `pilot` and `I have control` found 0 hits. ATIS appears only as the simile "like an ATIS letter", yet a whole component (`<viola-atis>`) is named after it.

**Anchor example:** Color Palette → Semantic Colors, Warning row, against the Domain status rows

> "| Warning, degraded but not refused (`unverified-cli`, `unconfirmable`, expired budget window) | #1E2124 | #9C9278 | #E6D8AE |"

> "| Readback::unconfirmable (`ok`, `confirmed:false`) | #1E2124 | #E6D8AE | #E6D8AE | The open drawing, never filled, with the word `unconfirmable` |"

> "| CliVerified::false | #2A2E33 | #9C9278 | #E6D8AE | `2.1.281 unverified-cli`: transport only, dialog answers held back. No colour. |"

**Issue:** The Warning row gives all three of its named cases an anthracite background and a lamp-off border. For `unconfirmable`, the domain row (and the `--rb-rule` token) gives a buff border, because the box outline is a state border. For `unverified-cli`, the domain row gives a holder background, because the word sits inside a lit strip. The summary row matches only `Window::expired`.

**Why this matters:** The four-row Semantic table is the first place obs and a11y look for "degraded" visuals. Read literally, `unconfirmable` would be drawn quieter than `open`, although it is the outcome where a send may have landed without confirmation. The readback box would then have two outline colours depending on which table the implementer trusted.

**Adversarial:** Should the Semantic table carry hex values at all, when every row it summarises is fully specified in the domain table? If both stay, which one does the plan declare authoritative? Would a patch that "fixes" the Warning row to #E6D8AE then break the `Window::expired` row it also summarises?

### 5. Anti-Pattern Relevance and Scope of "Never" [priority: high]
- **Do the unscoped "never" statements survive the reserved v1.x brake?** Navigation says taking the wheel is done "never on this page". Rejected Defaults scope the same idea to "The v1 page is view-only". Component 7 reserves an `I HAVE CONTROL` (POST pause) button on this page. The Raised-3 surface, `radius-sm` and `Problem::cross-origin-forbidden` also refer to the brake. At refinement time, "brake" had no Decisions Log entry. Scope the "never" (for example "in v1"), or log why the reserved control does not contradict it.
- **Does the web-spa Per-Surface list carry the shortlist's industry anti-patterns?** "Do not add a command palette" (#81 "Key Effects") appears only in Navigation prose ("There are no command palette and no shortcuts"). From the Real-Time Monitor effect set, toasts are named only inside a Rejected Default and a Universal Ban rationale, and a search for "auto-refresh" found 0 hits. Decide whether the Per-Surface list should state these, or cite Rejected Defaults as the canonical list.
- **Do inline ban lists drift from each other?** Motion "Hard limits" bans spinners, pulses, blinking, skeleton shimmer, looped animation, staggered entrances, hover colour transitions, parallax, spring physics, 3D transforms, scroll-driven animation, canvas/WebGL and animation libraries. The web-spa "NEVER animate…" ban names only triggers. Iconography repeats "no robot or person icons, no status dots, no checkmarks". Pick one canonical list per ban and have the others refer to it. Do not maintain parallel lists.

**Anchor example:** Surface: web-spa → Navigation Pattern → Keyboard, against Component Patterns 7 and Rejected Defaults

> "There are no command palette and no shortcuts. Taking the wheel is spoken in the CLI (`viola pause`) or by a human keystroke in the terminal, never on this page."

> "- **Controls:** two ruled rectangular `<button>`s, `I HAVE CONTROL` (POST pause) and `UNLINK`."

> "The v1 page is view-only, and taking control is spoken (`viola pause`, \"I have control\"), never implied by a GUI switch."

**Search evidence:** "brake" appears at lines 80, 192, 292 and 586 at refinement time, and none of those are in the Design Decisions Log (lines 839–864). "never on this page" has 1 hit. "I HAVE CONTROL" has 1 hit.

**Issue:** The same plan says the wheel is "never" taken on this page, and also specifies the tokens for a page button that does exactly that. Only the Rejected Default scopes its ban to v1.

**Why this matters:** When v1.x is planned, route and implement will find two binding statements that conflict. Either the brake is blocked by a design "never", or the "never" is quietly ignored and the ban list loses authority.

**Adversarial:** Suppose an iteration resolves this by deleting component 7. Raised-3 ("Reserved for the v1.x brake confirmation"), the `radius-sm` usage and the `cross-origin-forbidden` row would then refer to a component that no longer exists. Is scoping the "never" to v1 the only fix that leaves no orphaned references?

### 6. Expression Calibration and Motion Budget (0.2 base / 0.3 web) [trigger: expression level <= 0.3] [priority: medium]
- **Is the fade entrance recorded as a deviation from the calibration table?** The Motion duration table's 0.3–0.4 row prescribes `150ms ease-out` hover, a `200ms fade` page transition and Entrance `none`. The project uses 0ms hover, no page transition and a 120ms `@starting-style` fade as an entrance. The Brand Identity table licenses the fade. Is the difference from the row stated as a deviation in Motion, and logged? At refinement time, searches for "fade", "Entrance", "120ms" and "starting-style" found no hits in the Decisions Log.
- **Is any motion left unspecified?**
  - Tape auto-follow and the `#tape-end` anchor jump: are they instant or smooth? A search for "smooth" and "scroll-behavior" found only the generic expression-scale line.
  - The `read 4m ago` update each minute and the `document.title` change: are they stated as non-animated in one place?
- **Is the cocked + `stale` combination fully resolved in the `states` layer?** The stale selector swaps `--ink` to lamp-off. `DialogPending::true` gives the kind word #E6D8AE. The strip states say "the cock wins for offset, band and `DIALOG`". Does "`DIALOG`" include the kind word? Under the CSS as written, the kind word would go lamp-off.

**Anchor example:** Motion → Duration scale, 0.3–0.4 row, against "This project's values" → Entrance animations

> "| 0.3-0.4 | 150ms ease-out | 200ms fade | none | none |"

> "**Entrance animations:** only newly arrived tape lines and newly created transfer markers. They fade in with opacity 0 → 1 over 120ms linear, via `@starting-style`"

**Search evidence:** At refinement time, "fade" appears at lines 25, 34, 307, 314, 322, 332, 422, 435, 479, 546, 625 and 789. "Entrance" appears at lines 311 and 322. "120ms" appears at lines 34, 322, 422, 546 and 789. None of these fall in the Design Decisions Log (lines 839–864).

**Issue:** The table headed "adjusted for expression level" forbids at 0.3 exactly the entrance that the project ships, and allows the hover timing that the project forbids. Nothing labels the project values as a viola-specific override of that row.

**Why this matters:** Downstream phases are told that the expression level is "the SINGLE SOURCE OF TRUTH". A reader who looks up the 0.3 row gets the opposite of the actual budget.

**Adversarial:** Suppose a later phase follows the 0.3–0.4 row literally. It adds 150ms hover and a 200ms page fade, and removes the tape fade as "Entrance: none". Every step cites a table in this plan. Which sentence currently stops that, and is it in Motion or only in the Brand Identity table?

### 7. Multi-Surface Consistency (web-spa ↔ cli) [trigger: detected surfaces > 1] [priority: medium]
- **Do the CLI samples use the declared separator?** Iconography says the CLI separator is "two spaces". The `viola list` sample instead prints ` - ` inside WHEEL, single spaces in `skipped 0 0 0`, and `-- UNWRAPPED (read-only) --` for the rack label. The web prints `driver · budget-paused`, `skipped 0 · 0 · 0` and `UNWRAPPED · READ-ONLY`. Is the ASCII substitute for `·` defined once, and do all samples follow it?
- **Does the ATIS line match across surfaces?** The web ATIS prints `resets 21:00Z` and `expired` after each window. The CLI `BAY` line prints neither. Is that omission deliberate and stated?
- **Is the inbound arrow covered?** Iconography lists only `→` (U+2192) and `->`. The web's inbound twin `← overseer` has no Iconography entry; at refinement time, searches for `U+2190` and `<-` found 0 hits. The CLI `viola list` has no link indication at all. Is that a stated decision?

**Anchor example:** Surface: cli → Component Patterns 1 (`viola list` sample), against Iconography → Separator

> "driver - budget-paused"

> "gate open  skipped 0 0 0"

> "-- UNWRAPPED (read-only) --"

> "- **Separator.** Middle dot `·` on the web, two spaces in the CLI."

**Issue:** None of the three sample fragments uses the two-space separator that Iconography declares. The heading of this same component claims "the same columns and words as the web strip".

**Why this matters:** The CLI implementer copies samples and the web implementer copies the domain table. The same state then reads differently on the two surfaces the founder watches side by side, and the "shared six columns" customisation in the Decisions Log no longer holds.

**Adversarial:** Applied literally inside the WHEEL cell, the two-space rule makes `driver  budget-paused` look like two columns in a table whose columns are also separated by two spaces. Perhaps that is why the sample used ` - `. If so, the Iconography rule is what needs patching, not the sample. Which side is wrong?

### 8. CLI Surface Completeness [trigger: detected surfaces include cli] [priority: medium]
- **Does the hint list cover every refusal?** Compare the hint list with the exit-code table and the `viola send` sample:
  - The list has 5 entries.
  - The sample prints a hint for `no-prompt-submitted`, which is not in the list.
  - There is no hint phrasing for `input-not-ready`, `unknown-dialog`, `unknown` (exit 14), `instance-unreachable` (exit 21) or `wrapper fault` (exit 20).

  Either list which reasons deliberately get no hint, or add the phrasing without quoting upstream text.
- **Is every colour depth reachable?** The tokens table has a 16-colour column (`\x1b[33m`), but the decision order ends "Otherwise: 256-colour, or truecolor if `COLORTERM` says so." No path selects 16-colour. The table's "67 reserved" for handoff blue sits beside the ban "NEVER colour anything except `DIALOG` (amber) and `stale` rows (dim)". Is the reserved entry a latent violation?
- **Is the stderr vocabulary consistent?**
  - Streams says refusals print as `unable: …` with a colon. The samples and the exit table print `unable` plus two spaces, and only the exit-1 start refusal uses the colon.
  - The `viola verify` counter `[03/14]` is also a bracketed left-column token. Is it set apart from the `[RB]` / `[  ]` / `[/ ]` readback vocabulary?

**Anchor example:** Surface: cli → Component Patterns 2 (`viola send`)

> "- A refusal prints `unable` plus `reason  detail`, then one `hint:` line per reason:"

> "hint: builder did not submit the prompt; check it, then send again"

**Search evidence:** At refinement time, "hint" appears only at lines 696, 699, 705 and 804. `input-not-ready` appears only at lines 149 and 743, `unknown-dialog` only at 152, 721 and 743, `instance-unreachable` only at 746, and `wrapper fault` only at 745. None of those lines carries hint phrasing.

**Issue:** The rule promises one hint line per reason. The list covers 5 of the 10 typed outcomes, and the sample shows a sixth hint that is not in the list.

**Why this matters:** The human on a TTY gets recovery guidance for benign refusals (`turn-running`) but none when the instance is gone (exit 21) or the wrapper faulted (exit 20). The CLI implementer has to write hint text without a design source, which risks breaking the "never quotes upstream text" rule.

**Adversarial:** Is the `no-prompt-submitted` hint in the sample an intended list entry that went missing, or sample drift that a patch should remove? If the list is "fixed" by adding it, does the new line still pass the security floor "NEVER print upstream text, paths, pids"?

Read the document, analyse it along all dimensions, and output patches and a changelog.
