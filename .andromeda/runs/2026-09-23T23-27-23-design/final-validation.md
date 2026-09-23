## Issues Found

Both passes are done. The plan is in good shape. Every item on the checklist passes:
- **Anti-Patterns:** all 3 subsections are present, all 6 universal bans are there, and all 8 of the exploration's Defaults to Reject appear with their reasons.
- **Coverage:** all 13 Domain Concepts are echoed in the Brand Identity anchors, and all 8 Color World hex values are in the palette.
- **Motion:** the expression levels (0.2 base, 0.3 web-spa, 0.0 machine output) match the Motion section, the override of the 0.3–0.4 row is logged, and the Hard limits are present.
- **Signature:** the readback box appears in Brand Identity, Color Palette, Motion, Iconography, web-spa and cli.
- **Numbers:** I recomputed every contrast ratio and every `ch`/px width sum, and they are all correct.

The remaining issues:

- **[STALE_ARTIFACT]** cli Width says human tables truncate "the tape/text columns". The CLI has no tape: the `viola list` notes say "without a TAPE cell (the CLI has no tape)". No CLI table has a text column either. The phrase has come through unchanged from `.raw-design-system-plan.md` and iteration-1, and no later iteration corrected it. It would make downstream specialists spec truncation for a column that doesn't exist.
  Section: Surface: cli → Width (versus Surface: cli → Component Patterns 1)
  Severity: medium

- **[CROSS_REF_MISMATCH]** `--font-field` names "SF Mono", which is not in `library-shortlist.md`. It also drops the shortlist's "Noto Sans Mono". Both changes come from the founder's review round 1 list, which the log records. But the log's "Shortlist deviations" bullet for typography mentions only the Fira → Bahnschrift + Cascadia translation, so the shortlist trail is incomplete.
  Section: Typography, Design Decisions Log (Phase 5 iteration 1 → Shortlist deviations)
  Severity: low

- **[DUPLICATION]** web-spa Platform-Specific Notes → Fonts writes both per-OS font stacks out again. Typography and the CSS tokens already define them, so this is a third copy that can drift out of step. The values match today.
  Section: Surface: web-spa → Platform-Specific Notes; Typography
  Severity: low

- **[DUPLICATION]** Two component patterns write hex values where the checklist asks for token names. Component 1 says "holder surface #2A2E33" and component 3 says "arrow and name in #5B8DB8". The role tokens `--surface-strip` and `--handoff` exist for exactly these uses.
  Section: Surface: web-spa → Component Patterns 1 and 3
  Severity: low

- **[CONTRADICTION]** Component 7 (brake controls) sets a literal "32px minimum height". The Token Test says every length must trace to a token, and the only literal lengths it allows are the 760px and 1024px breakpoints. 32px is already the `--strip-h` token.
  Section: Surface: web-spa → Component Patterns 7; Self-Validation → Token Test
  Severity: low

- **[INCOMPLETE]** The `space-md` token is described as the "gap between rack separator label and first strip". Component 1, however, puts a column-caption header row between the separator label and the first strip. So the gap on each side of that caption row is undefined for Phase 8 layout. No patch: an owner has to choose the value.
  Section: Spacing; Surface: web-spa → Component Patterns 1
  Severity: low

- **[INCOMPLETE]** cli Streams says every refusal gets a `hint:` line directly after it. The exit-1 start refusal (`unable: <name> is already live`) is neither in the hint list nor in the explicit no-hint list; only the `unknown`, wrapper-fault and internal-error cases are covered. No patch: an owner has to choose whether it gets a hint.
  Section: Surface: cli → Streams; Component Patterns 2 and 5; Exit-code table
  Severity: low

- **[INCOMPLETE]** Anthracite, holder, rail and graphite have no row in the Core Colors "Domain anchor" column. Their anchors are written into the Usage cells instead: Surface Scale Base / Raised-1, Border Progression Subtle, and Text Hierarchy On-paper. No patch: adding rows would repeat the Surface Scale table.
  Section: Color Palette → Core Colors
  Severity: low

- **[INCOMPLETE]** The Rejected Defaults bullets give their reasons as "Rejected because …" rather than the template's "— rejected because:". The reasons themselves are complete. No patch: the difference is only formatting.
  Section: Anti-Patterns → Rejected Defaults
  Severity: low

## Patches

### Patch 1: Remove the phantom CLI tape/text truncation
**Reason:** [STALE_ARTIFACT]. The CLI has no tape and no text column, as the `viola list` notes state.
**Old:**
```
- Where the width is detectable (windows-sys `GetConsoleScreenBufferInfo`; `COLUMNS` elsewhere), human tables truncate only the NAME of unwrapped rows (`...`) and the tape/text columns.
```
**New:**
```
- Where the width is detectable (windows-sys `GetConsoleScreenBufferInfo`; `COLUMNS` elsewhere), human tables truncate only the NAME of unwrapped rows (`...`). The CLI has no tape.
```

### Patch 2: Log the field-stack deviation from the shortlist
**Reason:** [CROSS_REF_MISMATCH]. "SF Mono" is not in `library-shortlist.md`, and "Noto Sans Mono" was dropped without a note in the shortlist deviations.
**Old:**
```
  - Typography pairing #1 (Fira Code + Fira Sans) is translated to Bahnschrift + Cascadia Mono under `font-src 'none'`.
```
**New:**
```
  - Typography pairing #1 (Fira Code + Fira Sans) is translated to Bahnschrift + Cascadia Mono under `font-src 'none'`. The `--font-field` stack follows the founder's Phase 4.5 review rounds 1–2 rather than the shortlist's: "SF Mono" is added for macOS and "Noto Sans Mono" is not carried.
```

### Patch 3: Point the Platform Notes at the font tokens instead of repeating them
**Reason:** [DUPLICATION]. This is a third copy of the per-OS font stacks.
**Old:**
```
  - Per-OS stacks as in Typography: labels Bahnschrift → "DIN Alternate", "Avenir Next Condensed" → "DejaVu Sans Condensed" → `sans-serif`; fields "Cascadia Mono", Consolas → "SF Mono", Menlo → "DejaVu Sans Mono" → `monospace`. No web fonts.
```
**New:**
```
  - Per-OS stacks exactly as the `--font-label` / `--font-field` tokens in Typography → Per-OS fallback stacks. No web fonts.
```

### Patch 4: Use a token name for the strip surface in component 1
**Reason:** [DUPLICATION]. The hex value is written inline where the `--surface-strip` role token exists.
**Old:**
```
on the holder surface #2A2E33, with a 1px `--rule-deco` outer edge
```
**New:**
```
on the holder surface (`--surface-strip`), with a 1px `--rule-deco` outer edge
```

### Patch 5: Use a token name for the handoff colour in component 3
**Reason:** [DUPLICATION]. The hex value is written inline where the `--handoff` role token exists.
**Old:**
```
  - `→ builder`, with the arrow and name in #5B8DB8.
```
**New:**
```
  - `→ builder`, with the arrow and name in `--handoff` (departure blue).
```

### Patch 6: Tokenise the brake-control minimum height
**Reason:** [CONTRADICTION]. The literal 32px breaks the Token Test's length rule.
**Old:**
```
radius 0, the standard focus ring and a 32px minimum height.
```
**New:**
```
radius 0, the standard focus ring and a `--strip-h` (32px) minimum height.
```

