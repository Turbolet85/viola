## Signature Candidates

**Core user action (from quiz-context):** a send. One Claude Code session types a prompt into another at a turn boundary through `viola send` or MCP. viola then confirms it after the fact: the matching `prompt-submitted` event means delivered, and otherwise a typed refusal is shown (`not-delivered`, `human-typing`, `budget-paused`, `unverified-cli`). The founder's most frequent action on the `web-spa` is a glance to answer "did my relay land, and does the builder need me?" The page is view-only in v1, so the signature has to live in what that glance sees.

### Candidate 1: "The readback box"

**What it is:** Every send line in `<viola-event-feed>`, and the last-send cell of every `driver → driven` transfer marker, ends in one fixed-width, 1px-ruled square cell labelled `RB`. It has exactly three states, and each one is printed as a word as well as drawn:
- **Open:** buff `#E6D8AE` 1px outline on anthracite `#1E2124`, with the word `open` beside it.
- **Read back:** the cell turns solid buff with `RB` in anthracite (11.4:1). This happens instantly, with no transition, when the matching `prompt-submitted` event lands.
- **Refused:** the outline stays and a single 1px diagonal rule is struck through the cell. The typed reason (for example `not-delivered`) is printed in the next cell.

The layout follows a paper strip: the issue sits on the left, the readback sits at the far right, and the pair is shown as one unit (the Temporal pattern). `turn-ended` never fills the box, because it closes the turn, not the send.

**Implementation:** A Lit 3.3.3 `viola-*` element sets a `data-rb="open|read|refused"` attribute. Hand-written CSS in `/assets/app.css` styles each state with attribute selectors: `border` for open, `background` for read, and a `linear-gradient` stroke from the stylesheet for refused, so no inline style is needed and it stays CSP-clean. The state word is a text binding, and the element sets no `transition` property. The human-TTY CLI repeats the same element as a fixed column, `[RB]` / `[  ]` / `[/ ] not-delivered`, and `--json`/NO_COLOR output carries no glyph.

**Core-action tie:** Every send creates a box. The box closes only when viola's own "confirmed after the fact, never presumed" rule is satisfied, so it shows the core action's outcome directly.

**Brand fit:** It is the "read back before it counts" half of "Strip-bay handoff, read back", drawn in the Mood 1 materials: a buff paper cell under the lamp, either filled or not, on the anthracite console. It spends no amber and no motion: the web budget of 0.3 goes elsewhere and the box changes state instantly with 0ms. That matches Q4's rule that acknowledgement is a discrete fact, and the "Overclaiming" anchor, because the box is never shown as done before the event arrives.

### Candidate 2: "The cocked strip"

**What it is:** When `dialog_pending` turns on, the whole `<viola-session-row>` strip translates about 12px to the right, once, over about 160ms ease-out. It keeps its rack slot and the row order never changes. A 4px amber `#D97706` holder band appears on its left edge, and the DIALOG box prints the dialog kind (`question` / `permission` / `plan`). When the dialog clears, the strip snaps back into line and the amber goes away. Nothing loops or pulses.

**Implementation:** CSS `translate` plus `transition: translate 160ms ease-out` on `[data-dialog="pending"]`, all in the stylesheet. `@media (prefers-reduced-motion: reduce)` sets the duration to 0, and the offset, the band and the word all remain. Rows use a CSS grid with fixed `grid-template-columns` in the Frequentis/Saab box anatomy.

**Core-action tie:** A pending dialog blocks the next send (viola types only at turn boundaries). The cocked strip therefore marks the point where the send loop needs the human.

**Brand fit:** This is the ATC "cocked strip" taken literally, together with vStrips' "offset, not re-sort". It uses the one amber holder from Mood 1 and the single 160ms motion allowed within the web-spa 0.3 budget. It differs from Claude Code Agent View, which auto-sorts rows and uses yellow for "needs input". However, the "needs you" meaning is shared across the category (MulmoTerminal, Agent View), so it is less specific to viola.

### Candidate 3: "The transfer rail"

**What it is:** A 2px buff vertical rail in a fixed gutter column to the left of the rack. It runs from the driver strip's row to the driven strip's row and ends in a small ruled arrow-cell at the driven strip. The rail carries a label cell with `since hh:mm`. It is drawn once, with a ≤120ms opacity fade when the `link` event lands, and removed instantly on `unlink`. The same labelled cell also shows the last send's readback word.

**Implementation:** The rack is a CSS grid, and the rail is a grid item placed with `grid-row: <driver> / <driven>` in a dedicated gutter column. Lit computes the row span and applies it through a small set of stylesheet classes, because inline `style="…"` is banned by the CSP. The rack holds only a few sessions, so a handful of `.span-N` classes is enough. No SVG or canvas is needed.

**Core-action tie:** A send can only travel along an existing link, so the rail is the path every send takes.

**Brand fit:** This is the "handoff" half of the metaphor (vStrips links as transfer markers, not a node graph), in buff on anthracite. It is static apart from one 120ms fade, well under the 0.3 budget. Its weak point: when two sessions are linked (the v1 norm), it reads close to a generic connector line, and the grid-span workaround adds implementation friction under the CSP.

## Recommended

**Recommended signature:** 1, "The readback box"

**Reasoning:** Of the three, it is the only element that belongs to viola and not to its category. Agent View, MulmoTerminal and vStrips all have a "needs you" state, which Candidate 2 shares. None of them shows a send staying visibly open until the far end's matching `prompt-submitted` event arrives. It carries all four prior picks:
- **Q1:** the readback rule and the fixed phraseology words `open` / `RB` / `not-delivered`.
- **Q2:** the Temporal issue-to-acknowledge pair shown as one unit, placed in the strip anatomy.
- **Q3:** buff outline versus filled buff on anthracite, which keeps amber free for the cocked strip.
- **Q4:** zero motion, because acknowledgement is discrete, and the CLI shows the same cell as `[RB]`.

The core action generates a box, and the founder checks it dozens of times a day while working as the operator between two sessions. It is the design-system form of the product's own principle ("measured, never assumed"), and it is cheap to build: one attribute, three CSS rules, and no dependency. Candidate 2 should still ship as the supporting attention element, since Q4 already budgets for it, but it is not the signature.

**Research basis:**
- **WebSearch run 2026-09-23.** Neither query surfaced a distinctive dashboard pattern for showing an acknowledgement, which supports the readback box as a point of difference rather than a copy.
  - 2026 agent-orchestration dashboards converge on per-session status and "needs input": [Marc Nuri, "AI Coding Agent Dashboard"](https://blog.marcnuri.com/ai-coding-agent-dashboard), [LowCode Agency, "AI Agent Orchestration Dashboard 2026"](https://www.lowcode.agency/blog/how-to-build-an-ai-agent-orchestration-dashboard-for-complex-workflows), [TheCrunch, "AI Agent Dashboard 2026 Comparison"](https://thecrunch.io/ai-agent-dashboard/), [AgentCenter, "AI Agent Management 2026"](https://www.agentcenter.cloud/blogs/complete-guide-ai-agent-management-2026), [awesome-agent-orchestrators](https://github.com/andyrewlee/awesome-agent-orchestrators), [UI8 AgentOS](https://ui8.net/jmj-studio/products/agentos---ai-agent-orchestration-dashboard).
  - Galleries searched without results for this pattern: [Mobbin developer tools](https://mobbin.com/explore/mobile/app-categories/developer-tools), [Dribbble developer dashboard](https://dribbble.com/search/developer-dashboard), [SaaSUI 2026](https://www.saasui.design/best-saas-dashboard-ui-inspiration).
- **Carried from this run's research files:**
  - research-q2.md: vNAS vStrips manual (offset, not re-sort; push needs a receiver), Temporal Web UI Event History (paired open/closed events), Claude Code Agent View (May 2026, auto-sort, yellow "needs input").
  - research-q1-dig.md: SKYbrary flight progress strips, readback phraseology.
  - research-q3.md: NATS Blog Jan 2026, strip holder colours.
  - research-q4.md: 150-200ms and 8-12px motion calibration, `prefers-reduced-motion`.
- **Exact cell states and CSS mechanics:** my own derivation from training data (2026).
