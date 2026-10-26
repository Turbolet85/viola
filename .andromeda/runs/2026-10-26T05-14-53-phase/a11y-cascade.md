# a11y · cascade read

## Carried
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) Every CLI verb's output in all three modes must stay byte-identical across the tokio bump: the chunk's no-behaviour-change boundary is this entity, and a deprecation-forced source edit (the safe `AsyncFd` constructors) must change no byte any verb prints in human, plain or `--json` mode.
  > CLI verbs (`list`, `send`, `wait`, `last`, `answer`, `verify`, `pause`, `release`, `link`, `unlink`, `revive`, `plugin install`, `--help`) in human TTY, plain (non-TTY / `NO_COLOR` / `TERM=dumb`) and `--json` modes.
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:26)
  > assert_cmd checks the typed exit codes (`10`–`14`, `20`, `21`) and that `--json` carries no hint text. These run on all three OS legs.
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:35)
  > Any copy change therefore fails CI until the pinned string is updated on purpose.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:16) The `viola run` / passed `viola revive` boundary must hold unchanged after the bump: the run pump stays std threads with no Tokio, so the bump reaches no keystroke path, and the existing outer-PTY boundary cases are re-run on all three OS legs, passing unaltered.
  > `viola run` TUI passthrough: the unmodified `claude` TUI hosted in ConPTY/openpty, entered by `viola run` and by a `viola revive` whose preflight passed
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:20)
  > viola writes zero bytes to the terminal except the child's output;
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:21)
  > human keystrokes are never blocked, refused or delayed past the current atomic paste;
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:57)
  > Boundary assertions run on the tests' portable-pty `=0.8.1` outer-PTY driver (tests excerpt, E2E driver → tui). They check that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed.
  (.andromeda/a11y-plan-detail/aria-patterns-roles.md:19)
  > **tui:** three portable-pty outer-PTY nextest cases on all three OS legs, one per Section 1 boundary clause
  (.andromeda/a11y-plan.md:334)
  > NEVER block, refuse or delay a human keystroke in `viola run`, or in a start a passed `viola revive` made, past the current atomic paste
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:8) The MCP `wait` cap must keep its 50 s driver-only bound on the bumped Tokio runtime that `viola-mcp` builds: the `wait` tool's clamp and default stay `WAIT_CAP`, no key is blocked, refused or delayed by it, and the wheel stays untouched.
  > The MCP `wait` tool (chunk 2026-10-17-mcp-server-for-drivers) clamps its `timeout_ms` to `WAIT_CAP` 50 000 ms, which is also its default when the caller passes none (`viola_mcp::wait_timeout`), so an MCP `wait` with no timeout is answered `timed-out` after 50 s; that bound falls on the driver only (the hidden, plugin-run `viola mcp` serves a driving Claude session, never a person): no key is blocked, refused or delayed by it, and the wheel is untouched.
  (.andromeda/a11y-plan.md:251)
  > every time bound viola holds — the `viola wait` timeout the caller passes, the MCP `wait` cap, the dialog hold, the `viola send` readiness gate, the confirmation windows, the status-line deadline, the `claude agents --json` read, the budget pause — falls on the driver, the user's own command or viola's own child, never on the human: no key is blocked, refused or delayed by it.

## Bindings
- **scope-entities (CLI verbs) ↔ cognitive-errors / cognitive-plain-language**: the byte-identical output boundary of the chunk is witnessed by the pinned refusal layouts and typed exit codes (.andromeda/a11y-plan-detail/cognitive-accessibility.md:26) and the pinned fixed strings (.andromeda/a11y-plan-detail/cognitive-accessibility.md:35); a red on either is a behaviour change the chunk must not make, never a pin to update (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8).
- **scope-entities (tui passthrough) ↔ path-p4-wheel ↔ the body's Keyboard ban**: the boundary clauses (.andromeda/a11y-plan-detail/a11y-scope-summary.md:20) are re-read by the three P4 outer-PTY cases (.andromeda/a11y-plan-detail/aria-patterns-roles.md:19), which must pass unaltered on all three OS legs, holding the ban (.andromeda/a11y-plan.md:334).
- **cognitive-timeouts ↔ the not-assertable MCP surface**: the MCP server is machine-to-machine with no UI (.andromeda/a11y-plan.md:40), so the a11y obligation the Tokio move carries there is only that the `wait` cap stays a driver-only bound (.andromeda/a11y-plan-detail/cognitive-accessibility.md:8), as every time bound must (.andromeda/a11y-plan.md:251).
