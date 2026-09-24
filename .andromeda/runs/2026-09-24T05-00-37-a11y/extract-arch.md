## 1. Architecture Excerpt

### Stack (a11y reach)
- **axum 0.8.9 (feature `sse`) + tower-http 0.7.1 (`CompressionLayer`, `text/event-stream` excluded)**: serves the view-only local web GUI on `127.0.0.1:47319`: `/` (`text/html`), `/assets/*` (static files embedded in the binary), JSON `/api/*` routes, and RFC 9457 Problem Details errors. This is the only web surface, so axe-core, Lighthouse and pa11y can reach it at `http://127.0.0.1:<port>/`. The Host allowlist accepts only `127.0.0.1:<port>` and `localhost:<port>`, so a11y tooling must use one of those hosts.
- **SSE through axum `Sse::keep_alive`, fed by notify 8.2.0**: drives the live GUI event feed `GET /api/events`. It is one-way, sends a keep-alive comment every 15 s, and resumes on reconnect via `Last-Event-ID`. On first load the page reads state from `/api/sessions` and `/api/links`. Because the page updates dynamically, live-region (`aria-live`) patterns and announcement testing apply. The arch does not use WebSocket or polling.
- **Frontend framework: none chosen in arch.** It is "Deferred to specialists … the frontend framework (design specialist)". The UI framework, and so the ARIA pattern source, comes from design, not arch.
- **clap 4.6.7 (derive)**: the CLI parser for subcommands `run · send · wait · last · list · answer · hook · mcp · ui · verify · pause · release · link · unlink · plugin install`. CLI surface, so only manual keyboard and terminal-output discipline apply. Output is human text by default, `--json` gives machine output, and each refusal has a typed exit code.
- **portable-pty `=0.8.1` + vt100 0.16.2**: hosts the unmodified `claude` TUI in ConPTY (Windows) or openpty (Unix). vt100 is used only for the pre-send readiness gate and modal detection, "never used to read content". The terminal carries the child's own screen: `run` writes nothing to the terminal except the child's output. Terminal/TUI surface, so only manual keyboard and screen-reader checks apply. viola does not own the rendered TUI content.
- **Mobile framework: N/A.** "The phone view is a later version: the same web page behind authentication." The future mobile reach is the same web page, so it would be covered by web a11y tooling.
- **Test framework / E2E driver: not in the arch stack.** It is "Deferred … the test framework, fake-agent harness and `viola verify` probe design (tests)". The arch declares no Playwright, Cypress or similar harness for a11y to reuse.

### Surfaces
> - **Product type.** Hybrid local developer tool made of three parts:
>   - a native cross-platform CLI binary;
>   - a Claude Code plugin (hooks + stdio MCP server) that calls that binary;
>   - a minimal view-only local web GUI served by the same binary.

Deferred surfaces:
- "[Mobile] N/A in v1. Rationale: the phone view is the same web page behind authentication, in a later version."
- "A phone or remote view comes later, behind authentication."
- v1.x adds the GUI brake: "the reserved `POST /api/sessions/{name}/pause` and `POST /api/sessions/{name}/unlink` routes".

Platforms:
- "Windows is the live-supported target. The macOS and Linux paths build and pass CI on every commit against the fake agent."

### Project Intent Summary
- **Core functionality:** "Lets one interactive Claude Code session drive another on the user's own subscription."
- **Target users:** The arch has no "Growth model" user sentence; it describes architecture only: "Modular monolith: one binary with compiler-enforced internal crates … and no daemon." Users are named elsewhere:
  - "The founder runs v1 on their own subscription, with no accounts, no hosting and the GUI on 127.0.0.1."
  - "the later public version (individual Claude Code subscribers) must not need a rewrite."
  - "v1 is personal (Windows live, macOS and Linux CI-tested)."
  - The arch contains no a11y-priority user signals (no low-vision, elderly, cognitive-disability, assistive-tech or language-proficiency hints).
- **Critical paths hint:**
  1. GUI view: "The page shows each session row (wrapped, or read-only unwrapped), its liveness (`live` · `stale`), idle/busy status, wheel holder and any pending dialog."
  2. GUI view: "It also shows the budget reading and its age, the link set, the `skipped` counts, and a live event feed over SSE."
  3. GUI scope: "In v1 it is view-only, and no control accepts input."
  4. Human takeover: "Any human editing key since the last turn boundary moves the wheel to the human, and `send` is refused with `human-typing`. Focus, mouse and resize sequences do not count."
  5. Manual pause and release: "CLI `viola pause` moves the wheel to the human without a keystroke … The wheel returns only through CLI `viola release`."
  6. Dialog fallback to the human: "While the wheel is `human`, the wrapper answers `hook.dialog` with `null` at once, so the dialog renders for the human."
  7. CLI input and output: "Human text by default, `--json` for agents, a typed exit code per refusal. Prompt text comes from stdin or `--file`, never from a leading-slash argument."

### CI/CD Platform
- **Platform:** GitHub Actions
- **Pipeline note:** One workflow, `ci.yml`, runs on push and PR across a native-runner matrix `[windows-2025, macos-latest, ubuntu-latest]` (`dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2.9.2`). Jobs per OS: fmt → clippy `-D warnings` → sync-crate `cargo check` → `cargo deny` (ubuntu only) → workspace tests against the fake agent replaying `fixtures/claude/*` → `cargo build --release`. There is no deploy stage in v1, and the real `claude` CLI never runs in CI.

### A11y-Relevant Conventions
(No a11y-specific conventions in arch — Phase 3 will derive defaults from design + WCAG.)

The arch states no semantic HTML, i18n/RTL/`lang` or focus-management conventions. The following terminal and CLI output conventions affect assistive-tech reach:
- **CLI output mode:** human-readable text by default, `--json` for agents, and a typed exit code per refusal (`0`, `1`, `2`, `10`–`14`, `20`, `21`).
- **Terminal output discipline:** "While the child runs, `run` writes nothing to the terminal except the child's own output". `run`'s diagnostics go to the instance `diagnostics/` directory. `ui` and short-lived CLI verbs may use stderr.
- **Human input priority:** "Human keystrokes are never blocked, refused or delayed past the current atomic paste". Focus, mouse and resize sequences do not count as human editing input.
- **GUI error format:** RFC 9457 Problem Details (`application/problem+json`) with a `type` URN and a human `title`/`detail` (403 host-not-allowed, 404, 405, 503 state-unreadable). `/ready` returns 503 with its own readiness body.
