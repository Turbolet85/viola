## Personality Directions

### Direction 1: "Signal-box interlocking: one token, one line"

**Physical-world metaphor:** A railway signal box on single-track line. The signaller works a lever frame. A block shelf of instruments shows each section as "line clear", "train on line" or "line blocked". Only the driver holding the single-line token may enter the section. Every bell code and movement goes into the train register book.

**Domain anchor:** viola's core function is also a physical interlock. One holder of the "wheel" (`human` · `driver`), and automation gets a typed refusal (`human-typing`, `budget-paused`, `unverified-cli`, `not-delivered`), the way a signal stays at danger and the reason is logged. The block shelf maps directly to the view-only `web-spa` session rows (liveness `live`/`stale`, status `idle`/`busy`/`unknown`, `dialog_pending`, `driver → driven` links). The train register maps to the ndjson audit trail and the live SSE event feed. "Mechanism, not policy" is how a signaller works: he does not write the timetable. He only lets trains through safely and records every one.

**Voice:** Terse and procedural, like block-bell codes. Each state is stated once in a fixed word ("Line clear. Wheel: human. Held: budget-paused."), and nothing is filled in beyond what the instruments report.

### Direction 2: "Conn handover on the watch"

**Physical-world metaphor:** A ship's bridge during a watch change. The officer of the deck says "I have the conn" and the other replies "You have the conn." The helm has exactly one hand on it, and the deck log records every order and bearing.

**Domain anchor:** The creator brief already uses "the wheel" and "the bridge", and viola's rule "the human always wins the wheel" works like naval conn handover. Control moves by explicit announcement and is never taken quietly. The founder is the `audience`: today he is the manual "transport and operator" between an overseer session and a builder session. He becomes the captain watching two helmsmen, and the deck log is the ndjson trail. The v1.x brake (pause, unlink) and the later phone view fit the idea of a captain keeping watch from somewhere else.

**Voice:** Clipped call-and-response. Every change of hands is announced and acknowledged, and it stays calm under load.

### Direction 3: "Calibration bench chart recorder"

**Physical-world metaphor:** A metrology lab bench. A pen-on-paper strip-chart recorder draws readings as they come in. Each instrument has a calibration tag with a date, and there is a bound calibration ledger. A reading with no current calibration is marked as unverified and not trusted.

**Domain anchor:** viola's rule "Measured, never assumed" is a lab rule. Its version-stamped capability ledger (unverified CLI builds degrade to transport-only) works like calibration tags. The budget panel shows each reading's age (`five_hour` / `seven_day` used %, `resets_at`, or `unknown`), the way a gauge shows its last-read time. The `skipped` counts are like the chart recorder marking unreadable samples instead of smoothing over them. This speaks to the creator's stated negative anchor, "Overclaiming": the GUI shows only what the hooks and the wrapper observed.

**Voice:** Quiet and exact, like annotations in a lab ledger. It uses units and timestamps, writes "unknown" instead of guessing, and never uses exclamation marks.

## Recommended

**Recommended direction:** 1 — "Signal-box interlocking: one token, one line"

**Reasoning:** This metaphor matches all three things the `core_functionality` does:
- The single-line token is the one-driver wheel.
- The block instruments are the session rows, and a signal held at danger with a logged reason is the typed refusal.
- The train register is the ndjson audit trail and event feed.

It also covers the creator's MulmoTerminal "working / done / needs you" reference and the "Overclaiming" rule, because a block instrument only shows what the bell told it. For the `audience` (the founder as operator, and later individual Claude Code subscribers), it matches the stated "no dancing with a tambourine" posture: no ritual, only interlocks. It also fits `scale_intent: personal` and a view-only v1. A signal-box shelf is a small, fixed panel of labelled states and needs no widgets, which suits the `family_chosen` Web Components / Lit with no component library and hand-written CSS. It also works within the CSP's system-fonts-only limit, since stencilled enamel signage relies on weight, case and spacing rather than a custom typeface. Direction 2 is a strong second, but the brief uses "bridge" to mean a connector between agents, not a ship's bridge, so the nautical reading risks confusing the two.

**Research basis:** Web searches run 2026-09-23:
- 2026 dev-tool dashboards (Vercel, Linear, Supabase) are dark-first and nearly monochrome, with status colour used only for meaning. Sources: [Fuselab Creative, "Dashboard Design Trends 2026"](https://fuselabcreative.com/top-dashboard-design-trends-2025/) and [AdminLTE, "35 Best Dashboard Templates 2026"](https://adminlte.io/blog/dashboard-templates/).
- "Tactile brutalism" and industrial/telemetry interfaces (1px explicit containers, exposed grid, components that feel engineered) are a named 2026 trend. Sources: [Fireart Studio, "Web Design Trends 2026"](https://fireart.studio/blog/the-best-web-design-trends/) and [Setproduct, "Retro and brutalist UI design: a 2026 field guide"](https://www.setproduct.com/blog/retro-brutalist-ui-design-2026).
- 2026 agent-observability UIs show handoffs, delegation and trace stories between orchestrator and sub-agent. Sources: [Augment Code, "AI Agent Monitoring: 2026 Observability Guide"](https://www.augmentcode.com/guides/ai-agent-monitoring) and [MLflow, "LLM Observability with the Best UI: A 2026 Engineer's Guide"](https://mlflow.org/articles/llm-observability-with-the-best-ui-a-2026-engineers-guide/).
- The signal-box, conn-handover and calibration-bench metaphors themselves come from training data (2026).
