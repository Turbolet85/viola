## 6. Creator Brief Excerpt

_Source: `.andromeda/input.md` (which folds in `refs/viola-brief.md` and `refs/viola-prior-art.md` at full fidelity; refs/ read directly as well, no additional files). Quotes are verbatim._

### Must-Work Scenarios

- First live test (brief §6): "the overseer sends `/andromeda-new-session` to the builder, waits for `Stop`, and reads the dashboard. When that passes, the foundation stands."
- Wheel take-over (brief §6, PASSED on Pulse): "the founder typing into the window took the wheel, `send` was refused, `release` returned it."
- R2 One wheel (brief §3.3): "A submitted prompt the bridge did not send means the human took the wheel, and automation pauses by itself — no button needed."
- Dialog answering (brief §3.3 R7): "`PreToolUse` answers `AskUserQuestion` and approves a plan, `PermissionRequest` answers a permission prompt (a suggestion included) or sends a plan back with feedback, and the answered dialog never renders".
- Waking without polling (brief §4.1 S5): "A background `wait` that exits on the next `Stop` completed when the driven turn ended, and the driving session's harness woke on its exit."
- Identity stripping (brief §3.3 R8 / §4.1 S6): "Started from inside a Claude session — the normal case once an overseer launches a builder — the child would inherit that session's id, its Remote Control bridge and its messaging socket. The wrapper removes them".
- Shell path rewrite (brief §6): "a leading-slash argument is taken for a path: `/andromeda-new-session` arrived as `C:/Program Files/Git/andromeda-new-session` (42 bytes sent for 22) … The product must not depend on that: take the text from stdin or a file, and warn when an argument carries a rewritten-path prefix."
- Windows exit detection (brief §4.1): "ConPTY did not close the output stream when the child exited, so exit is detected on the process, never on EOF."
- Budget governor (brief O8): "The statusline's input JSON carries the plan limits as official fields, `rate_limits.five_hour.used_percentage` and `rate_limits.seven_day.used_percentage` … record the figures, then hand over to the user's script unchanged."
- GUI (brief D5): "a minimal GUI: the active sessions and the links between them."

### Rigor Hints

- R4 (brief §3.3): "The bridge carries, logs (ndjson on disk, so it survives restarts on either side) and holds the wheel. It never decides what to answer."
- Brief §7: "Security is a first-class concern: the bridge types into sessions that can run commands. Localhost only by default, authentication for anything remote, and the event log as the audit trail."
- Brief §3.4: "CI on all three OSes against a FAKE AGENT — a small program that behaves like Claude Code (prints, waits for input, calls the hook commands). Tests spend no tokens and do not flake; the real CLI runs only in local live tests."
- Brief §3.4: "an agent can verify it through a headless browser, which is how the founder's projects verify UI by default."
- Brief §4: "The docs lag the installed build. … Design against the installed CLI, measured."
- Brief §4.1: "Timing: the screen lags the hooks. … Hooks are the source of truth; the screen is an eventually consistent view."
- Brief O6: "Limits — the largest paste the input box takes whole; hook latency per tool call (the spike's hooks start a Python interpreter each time; the product's hooks call the native binary)."
- Brief §7 (policy): "Advertised usage limits for Pro and Max plans assume ordinary, individual usage of Claude Code and the Agent SDK — so a budget governor belongs in the first version, not a later one."
- Brief Appendix A: "A step that moves the rate of caught errors is reverted."

### Obs Anti-Patterns (creator's explicit asks)

- Brief §4 (transcripts): "the entry format *\"is internal to Claude Code and changes between versions, so scripts that parse these files directly can break on any release\"* — usable for reading on demand, never as the bridge's contract."
- Brief §3.2: "Events, not screen-scraping. … Nobody parses the rendered screen for content."
- Brief §7 (policy): "developers may not collect, store, or intermediate Claude.ai credentials or session tokens"; input.md Extracted: "never touches credentials".
- Brief §2 D2: "It drives the official `claude` CLI under the user's own login. No API key."
- Prior art §4: "Overclaiming. The GUI shows only what the hooks and the wrapper observed."
- Prior art §4: "Fragile shared state. … viola's on-disk state parses defensively and heals itself from day one."
- Brief §3.4: "No central daemon: each wrapper owns its channel, state lives on disk".

### Founder Directions (overseer, founder-delegated — received during this obs run, 2026-09-24; BINDING for Phase 1 onward)

_Delivered by the overseer session on the founder's behalf mid-run. Recorded verbatim; they rank above derived defaults._

1. "Align to tests: adopt test-plan §3 log fields AND its fixed event-name list verbatim; tests owns them."
2. "Structured JSON logs from every process (run/hook/mcp/ui) under diagnostics/; hook never writes stderr; the harness `logs` command merges them with events.ndjson."
3. "Normal mode, not inverted: viola is not a telemetry product."
4. "From the cross-plan audit: release-from-driver must log as its own event, not a generic wrapper fault -32602; each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, .cmd/.bat child) logs a distinct detail code, never a path or pid."
5. "send-issued / send-refused (CL-1) are logged events."
