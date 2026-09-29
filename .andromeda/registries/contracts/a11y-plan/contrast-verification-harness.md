### Contrast verification harness

- **Source-of-truth tokens:** design token names from upstream-context Section 3 (reproduced verbatim in Section 6):
  - colour pairs over `--surface-bay`, `--surface-strip`, `--surface-inset` and `--rb-fill`;
  - focus tokens `--focus-ring`, `--focus-w`, `--focus-offset`, `--radius`;
  - target tokens `--line-h`, `--strip-h`, `--rb-size`.
  - Dark only: one palette (Overseer Direction 6).
- **Verification tool:**
  - axe 4.13.0 `color-contrast` (SC 1.4.3 on rendered text, DejaVu fallback on ubuntu), gated on both `violations` and `incomplete`.
  - A colorjs.io 0.7.1 token checker test (`a11y-tokens` in `e2e-web/tests/bay-steady-state.spec.ts`):
    - reads each token with `getComputedStyle(document.documentElement).getPropertyValue('<name>')`;
    - composites alpha over its declared surface;
    - computes `Color.contrast(fg, bg, 'WCAG21')`;
    - attaches one JSON row per pair, `{pair, ratio, min, sc, pass}`;
    - fails on any `pass:false` (row `violation_type:"token-pair-contrast"`).
  - A DOM walk for negative placements (`token-placement`).
- **WCAG SC mapping:** SC 1.4.3 (AA 4.5:1 text; 3:1 large text as computed by axe from font size and weight) and SC 1.4.11 (AA 3:1 non-text: rules, strike, cock band, focus ring). SC 1.4.6 (AAA) is not claimed, and `color-contrast-enhanced` stays disabled.
