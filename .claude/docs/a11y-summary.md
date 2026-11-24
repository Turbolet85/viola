# Accessibility Summary — viola

_Distilled from `.andromeda/a11y-plan.md` by `/andromeda-setup-project`. wrap-session does not modify._

## A11y tier

**Tier:** Standard (1) — cli and tui get output-discipline and boundary assertions plus manual screen-reader passes, with no WCAG conformance claim; viola has no web or graphical frontend: the terminal fleet console (matrix#v2-14) takes its a11y scope at the route entry «Fleet a11y verification».

**WCAG target:** none: viola has no web or graphical frontend, and the cli and the `viola run` TUI make no conformance claim. No AAA escalation: no trigger fired.

## Harness contract (§3)

The a11y harness checks cli and tui output discipline with `std::process::Command` / literal-pin / portable-pty and emits no a11y rows (failures surface only as nextest failures in `suites[].failures[]`); viola has no web or graphical frontend, so no frontend harness. See `.claude/rules/a11y.md` for path-scoped enforcement.

- **Automated tools per surface:**
  - **web-spa:** N/A: viola has no web or graphical frontend
  - **Mobile:** N/A (phone view deferred)
  - **CLI / TUI:** `std::process::Command` + literal pins + portable-pty outer-PTY output-discipline checks on the `ubuntu-latest` CI leg, Linux only (the frame verbs' server-verify peer pid witness, the socket-directory refusal and the stale-row dim witness `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` among them); manual Orca passes (Linux) supplemental only
- **WCAG criteria mapping:**
  - **AA base SCs:** none (viola has no web or graphical frontend); cli / tui output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1 and the tui boundary for SC 2.1.1, with no conformance claim
  - **AAA escalations:** none
- **Structured violation JSON:** viola has no web or graphical frontend; cli / tui write no a11y rows, and output-discipline failures surface only as nextest failures in `suites[].failures[]`.

## Critical paths (must-be-accessible)

- **P2 — confirmed send and readback, or refusal** — cli (`viola send` readback mirror `[RB]`)
- **P3 — dialog pending, then `answer`** — cli (`viola answer` for a question, permission, plan, or a question first raised by PermissionRequest; `unverified-cli` refusal)
- **P4 — human takes the wheel** — tui boundary + cli (`pause`, `release`, `send` and `switch` refused with `human-typing`)
- **P6 — CLI board, wait/last, verify** — cli output discipline; `viola list`'s `CAPTION` literal pins the caption order, which the terminal fleet console (matrix#v2-14) reuses; the wrapped rows are drawn as a tree, their depth carried by a plain-space NAME indent; the `-- UNWRAPPED - READ-ONLY --` separator stands too, printed only when unwrapped rows exist, and the links follow behind their `-- LINKS --` separator (the `--help` group labels stand too: board / traffic / wheel / handoff / setup over the served verbs, the `handoff:` row `handoff:  link, unlink` between `wheel:` and `setup:`, `plugin install` built in `setup:` as one entry and a bare `plugin` unserved)

## Bootstrap phases (§3)

1. **a11y-tooling-install** — none: the cli cases drive the `viola` binary through `std::process::Command`, and the tui cases use the tests' own portable-pty pin; viola renders no page, so no browser a11y tooling
2. **focus-management-library-install** — none
3. **aria-component-library-install** — none: viola renders no page
4. **contrast-verification-harness-setup** — none: no page renders a token pair
5. **screen-reader-test-spec-setup** — none in 0.1.0; the manual Orca pass stays founder-owned
6. **a11y-ci-gate-wire** — none of its own: the cli / tui cases run inside the `test` job's `coverage` suite (matrix `[ubuntu-latest]`) under `gate --require coverage,doctest` (skips never allowed), and no a11y lint, aggregation or upload step exists
7. **violation-json-emission-wire** — none in 0.1.0: cli / tui emit no a11y violation rows (output-discipline failures surface only as nextest failures in `suites[].failures[]`)

## Universal anti-patterns

- NEVER use manual screen-reader / keyboard / contrast passes as the only method — they are supplemental to automated checks.
- NEVER claim WCAG conformance for the cli or the `viola run` TUI, whether `viola run`, a passed `viola revive` or a passed `viola adopt` started it.
- NEVER add a `suite` value (such as `a11y`) or a separate a11y driver outside `scripts/agent-run.* run`.
- NEVER parse the rendered `claude` TUI beyond its compiled signatures (footer literals, the input box's rule and prompt rows, modal literals, the compiled shapes over them), the prompt row only for its opening char and never the text after it, or take screenshots as a11y evidence (R7); except the test-only rig, which reads as closed words, never text: the trust selector's place, the input box as empty, holds-text or absent, modal dialogs, and the box row's cell attributes before any send or after a turn end, a send or a KR resize.
- NEVER block, refuse or delay a human keystroke in `viola run`, or in a start a passed `viola revive` or a passed `viola adopt` made, past the current paste's in-flight chunk and closing marker (the one exception: a start's automatic verify window, before any child exists, where a typed key waits in the terminal's own buffer until the child starts), and never count focus / mouse / resize sequences or terminal replies as editing (P4).

## Critical decisions

- viola has no web or graphical frontend, so no page structure, live-region, announcer or focus decision binds; the terminal fleet console (matrix#v2-14) takes its a11y scope at the route entry «Fleet a11y verification».
- D-A11Y-05 — Lighthouse / pa11y / WAVE dropped (second client, stale, or unable to reach loopback).
- Eight time bounds viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, paste write, confirmation windows, agents read, the start's Claude config lock wait) fall on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by them (provisional). The start's verify stop grace, the wait before a stopped start's verify child is killed, likewise falls on viola's own child only: no key is blocked, refused or delayed by it and the wheel is untouched. An MCP frame tool call (send, wait, last, answer) that outlives the MCP call deadline (`viola_mcp::MCP_CALL_DEADLINE`) ends `instance-unreachable` / `call-deadline`, the outcome of a send unknown; only the MCP frame tools write that detail, never the CLI, whose exit-21 cause set stays six. The one `claude agents --json` read `viola list`, `viola revive` and `viola adopt` each make (chunk 2026-10-16-unwrapped-sessions-on-the-board) is killed at 5 s, a bound on viola's own child only: a failed, oversize, malformed or timed-out read fails open, and no key is blocked, refused or delayed by it. `viola wait` returns `{timed_out:true}` only for a timeout the caller passes in. The MCP `wait` tool (chunk 2026-10-17-mcp-server-for-drivers) clamps its `timeout_ms` to `WAIT_CAP`, also its default when the caller passes none (`viola_mcp::wait_timeout`), so an MCP `wait` with no timeout is answered `timed-out` at `WAIT_CAP`: a bound on the driver only (the hidden, plugin-run `viola mcp` serves a driving Claude session, never a person), no key is blocked, refused or delayed by it, and the wheel is untouched. A raised dialog takes a `viola answer` only within the dialog hold (`config.json`'s `dialog_hold_s`, whole seconds 60–3600, default 60, PROVISIONAL bounds; chunk 2026-10-19-dialog-hold-for-the-driver); at the hold's end the wrapper replies `null` and appends one `dialog-expired` line, the dialog renders for the human in the `claude` TUI, and a later `answer` for it is refused `not-delivered` / `unknown-dialog` (exit 13) — the hold bounds the driver, never the human: any human key still hands the held dialog back at once, whatever the hold. A `viola send` on a verified CLI waits in the readiness gate for at most `GATE_MAX_WAIT` (8.5 s) and is then refused `not-delivered` / `input-not-ready` with nothing typed; a key pressed during the wait goes to the child and takes the wheel at once, and the waiting send is then refused `human-typing`. Once typed, a confirmed `viola send` waits for its confirmation within `CONFIRM_WINDOW_FALLBACK` (10 s), and a `/compact` send on a verified CLI within its own `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL, architecture [Delivery Confirmation]); a send not confirmed inside its window is `not-delivered` / `no-prompt-submitted`: a bound on the driver only, the confirm wait takes no key and blocks, refuses or delays none, and the wheel is untouched. A start's window step waits at most 5 s for its own terminal's window to appear in the compositor's client list, each compositor read through the Hyprland seam is bounded at 5 s a reply (the window step, viola open, viola focus), and viola open waits at most 30 s for the instance it started to come live, then reads not-live; each falls on viola's own start or verb, never on a key typed into a session, and never moves the wheel.
- The budget gate on automated sends is removed whole (its thresholds, the `budget-paused` refusal, `release --budget` and its Path 6 guards); exit 11 is retired (was `budget-paused`) and never reused, and an older wrapper's `budget-paused` refusal reads as unknown, exit 14.

---

**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: `.claude/rules/a11y.md`.
