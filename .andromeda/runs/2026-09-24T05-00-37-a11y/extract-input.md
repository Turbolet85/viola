## 7. Creator Brief Excerpt

_Source: `.andromeda/input.md` (User Input + Extracted + folded refs) and project-root `refs/viola-brief.md`, `refs/viola-prior-art.md`, read at full fidelity. The brief carries no explicit accessibility requirement; the quotes below are its only a11y-bearing signals._

### Must-Work Scenarios

- input.md, User Input: *"holds a one-driver wheel the human can take at any moment, and shows the active and linked sessions in a minimal local GUI."*
- refs/viola-brief.md §1 (founder, translated): *"a minimal GUI showing which sessions are active and which are linked."*
- refs/viola-brief.md §2 D4: *"both agents stay interactive terminal sessions the human can watch and take over at any moment."*
- refs/viola-brief.md §3.2 item 4: *"`bridge ui` — a local web page served by the same binary: the sessions, the links, a feed of recent events. Read-only first; take-the-wheel, pause and unlink come next."*
- refs/viola-brief.md §5 O1: pass-through to a real terminal — *"Cyrillic, emoji, CJK, box tables, a ~300-character wrapped line, code blocks, a folded long output, a diff and a question dialog all rendered as without the wrapper"* (the wrapped `claude` TUI must render unchanged through `viola run`).
- refs/viola-brief.md §6: the first live test — *"the overseer sends `/andromeda-new-session` to the builder, waits for `Stop`, and reads the dashboard."*

### Rigor Hints

- No WCAG target is stated anywhere in the brief (no "WCAG 2.1 AA", no "AAA", no cognitive-accessibility ask).
- refs/viola-brief.md §3.4: *"The GUI as a local web page rather than a native window: no GUI toolkit on any platform, and an agent can verify it through a headless browser, which is how the founder's projects verify UI by default."*
- refs/viola-brief.md §7: *"The GUI is a UI surface, so `/andromeda-design` runs and a11y's UI machinery switches on; the page itself is minimal."*
- refs/viola-brief.md §3.4: *"CI on all three OSes against a FAKE AGENT … Tests spend no tokens and do not flake."*
- refs/viola-brief.md Appendix A: *"Always the founder's … anything that needs human eyes on a GUI."*

### A11y Anti-Patterns (creator's explicit asks)

- refs/viola-prior-art.md §4: *"Overclaiming. The GUI shows only what the hooks and the wrapper observed."*
- refs/viola-prior-art.md §4: *"Screen parsing as the primary channel … Viola parses no screen for content (R7)."*
- No explicit a11y anti-pattern (captcha, ARIA-without-semantics, manual-only review) is named by the creator.

### Overseer Directions (founder-delegated, recorded 2026-09-24 at the start of this a11y run; apply from Phase 1 on)

Verbatim from the overseer:

1. *"Inject axe into the tests plan own Playwright driver (@axe-core/playwright), no second browser stack; a11y supplies the axe configuration the test plan is waiting for."*
2. *"Bind to design token NAMES only; contrast/focus/size assertions must hold on the ubuntu CI DejaVu font fallback (founder: ubuntu is primary, CI must pass)."*
3. *"Violation JSON aligned to the obs plan log schema and event list (obs aligns to tests; a11y aligns to obs)."*
4. *"Cross-plan audit item: after a `viola ui` restart the SSE stream gets 401, and the page must show the 401 access strip (announced), not \"TAPE stopped / not answering\"."*
5. *"CLI/TUI: keyboard and screen-reader discipline, no fake automation."*
6. *"Dark only."*
