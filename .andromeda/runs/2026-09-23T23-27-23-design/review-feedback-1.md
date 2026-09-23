# Phase 4.5 review feedback — round 1 (2026-09-24)

**Approved as drafted:** colours, expression levels, signature (the readback box + the cocked strip).

**Change — fonts (founder):**
- Bahnschrift and Cascadia Mono are installed only on Windows, but D3 makes macOS and Linux supported, and the headless GUI checks in CI run on ubuntu.
- Add an explicit per-OS fallback stack as tokens:
  - labels: Bahnschrift → "DIN Alternate" (macOS) → a condensed system sans → sans-serif
  - fields: Cascadia Mono → "SF Mono", Menlo → "DejaVu Sans Mono" → monospace
- State that contrast and render assertions must hold on the Linux fallback, since that is what CI renders.
- No web fonts.
