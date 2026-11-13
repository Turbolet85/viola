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
- **P3 — dialog pending, then `answer`** — cli (`viola answer`, `unverified-cli` refusal)
- **P4 — human takes the wheel** — tui boundary + cli (`pause`, `release`, `send` refused with `human-typing`)
- **P6 — CLI board, wait/last, verify** — cli output discipline; `viola list`'s `CAPTION` literal pins the caption order, which the terminal fleet console (matrix#v2-14) reuses; the `-- UNWRAPPED - READ-ONLY --` separator stands too, printed only when unwrapped rows exist (the `--help` group labels stand too: board / traffic / wheel / handoff / setup over the served verbs, the `handoff:` row `handoff:  link, unlink` between `wheel:` and `setup:`, `plugin install` built in `setup:` as one entry and a bare `plugin` unserved)

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
- NEVER claim WCAG conformance for the cli or the `viola run` TUI, whether `viola run` or a passed `viola revive` started it.
- NEVER add a `suite` value (such as `a11y`) or a separate a11y driver outside `scripts/agent-run.* run`.
- NEVER parse the rendered `claude` TUI beyond its compiled input-box and modal signatures, or take screenshots as a11y evidence (R7).
- NEVER block, refuse or delay a human keystroke in `viola run`, or in a start a passed `viola revive` made, past the current atomic paste, and never count focus / mouse / resize sequences or terminal replies as editing (P4).

## Critical decisions

- viola has no web or graphical frontend, so no page structure, live-region, announcer or focus decision binds; the terminal fleet console (matrix#v2-14) takes its a11y scope at the route entry «Fleet a11y verification».
- D-A11Y-05 — Lighthouse / pa11y / WAVE dropped (second client, stale, or unable to reach loopback).
- The statusline-bearing start (as landed, chunk 2026-10-10-statusline-pass-through): `passthrough_with_a_statusline_source_adds_no_viola_bytes` (`tests/tui_passthrough.rs`) is its reading of the zero-viola-bytes clause, green in CI (run 38080855246). `viola hook statusline`'s 5 s PROVISIONAL bound falls on the user's own command, never on a keystroke or the human. The one `claude agents --json` read `viola list` and `viola revive` each make (chunk 2026-10-16-unwrapped-sessions-on-the-board) is killed at 5 s, a bound on viola's own child only: a failed, oversize, malformed or timed-out read fails open, and no key is blocked, refused or delayed by it. `viola wait` returns `{timed_out:true}` only for a timeout the caller passes in. The MCP `wait` tool (chunk 2026-10-17-mcp-server-for-drivers) clamps its `timeout_ms` to `WAIT_CAP` 50 000 ms, also its default when the caller passes none (`viola_mcp::wait_timeout`), so an MCP `wait` with no timeout is answered `timed-out` after 50 s: a bound on the driver only (the hidden, plugin-run `viola mcp` serves a driving Claude session, never a person), no key is blocked, refused or delayed by it, and the wheel is untouched. A raised dialog takes a `viola answer` only within the dialog hold (`config.json`'s `dialog_hold_s`, whole seconds 60–3600, default 60, PROVISIONAL bounds; chunk 2026-10-19-dialog-hold-for-the-driver); at the hold's end the wrapper replies `null` and appends one `dialog-expired` line, the dialog renders for the human in the `claude` TUI, and a later `answer` for it is refused `not-delivered` / `unknown-dialog` (exit 13) — the hold bounds the driver, never the human: any human key still hands the held dialog back at once, whatever the hold. A `viola send` on a verified CLI waits in the readiness gate for at most `GATE_MAX_WAIT` (8.5 s) and is then refused `not-delivered` / `input-not-ready` with nothing typed; a key pressed during the wait goes to the child and takes the wheel at once, and the waiting send is then refused `human-typing`. Once typed, a confirmed `viola send` waits for its confirmation within `CONFIRM_WINDOW_FALLBACK` (10 s), and a `/compact` send on a verified CLI within its own `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL, architecture [Delivery Confirmation]); a send not confirmed inside its window is `not-delivered` / `no-prompt-submitted`: a bound on the driver only, the confirm wait takes no key and blocks, refuses or delays none, and the wheel is untouched.
- The budget pause (as landed, chunk 2026-10-10-budget-governor) is a bound on the driver only: an automated `send` is refused `budget-paused` (exit 11) until the window's reset or `viola release <name> --budget`, and no key is blocked, refused or delayed. Its reading: `path6_a_reading_over_the_threshold_refuses_the_send` (`tests/cli_budget.rs`) — a human key reaches the child under a closed gate and the outer PTY holds none of viola's literals, green in CI (run 38091081413). `answer`, `wait`, `last` and `pause` are not gated: read in the diff, and held since chunk 2026-10-10-session-links by `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` (each gives under a closed gate the exit code and document it gives under an open one).

---

**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: `.claude/rules/a11y.md`.
