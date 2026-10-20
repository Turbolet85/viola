group: architecture+design-system
read: .claude/rules/frontend.md · 29 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/frontend.md:16   class: retired
  leaf-text: «## Motion (expression 0.3 on the 0.2.0 page; 0.1.0 renders no page)»
  source: .andromeda/design-system.md:44 «N/A — the frontend's expression level: the frontend is deferred to 0.2.0, and its technology is not chosen.»
  replace-with: «## Motion (base expression 0.2; the frontend's expression level N/A, deferred to 0.2.0)»

- leaf: .claude/rules/frontend.md:17   class: retired
  leaf-text: «- Exactly two motions, each played once on a state change: the cocked-strip `translate` (12px over `--cock-dur`, 160ms ease-out) and a new tape line or transfer marker fading in (opacity over `--fade-dur`, ≤120ms). The readback fill, the strike, `stale` dimming and the return of a strip into line are instant; no hover effects except link underline and focus rings.»
  source: .andromeda/design-system.md:46 «At 0.2 the duration scale's 0.0–0.2 row applies (0–100ms, no entrance), and viola's cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.»
  replace-with: «- At the base expression 0.2 the duration scale's 0.0–0.2 row applies (0–100ms, no entrance), and the cli (0.2 on a TTY, 0.0 otherwise) has no motion at all; the cocked strip's 12px / 160ms translate is a 0.2.0 frontend design input.»
