# a11y · A11y Scope Summary — category read

## Carried
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:40) The pin race fix lands in `run`'s start, the same start that enters the TUI passthrough for `viola run` and a passed `viola revive`, so its boundary binds every change this chunk makes there.
  > - **Entity:** `viola run` TUI passthrough: the unmodified `claude` TUI hosted in ConPTY/openpty, entered by `viola run` and by a `viola revive` whose preflight passed (the same start; the boundary below binds both verbs
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:43) Whatever the absent-copy write does under a concurrent first start (wait, retry, accept same bytes), it must write no terminal byte of its own, and it must never block or delay a human keystroke.
  > The assertion boundary is viola's wrapper:
  >     - viola writes zero bytes to the terminal except the child's output;
  >     - human keystrokes are never blocked, refused or delayed past the current atomic paste;
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:109) Any new record of the pin race (a wait, a same-bytes acceptance, a failed replace) from `run` goes to `diagnostics/` and never to the terminal.
  > viola's diagnostics for `run` go to `diagnostics/`, never to the terminal (arch excerpt, A11y-Relevant Conventions → terminal output discipline).
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:87) A start that meets the race must emit no SGR, glyph or cursor control under `viola run` or a passed `viola revive`.
  > there is no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`;
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:88) A wait added to the pin write must not show as a spinner or redraw; output stays static and linear.
  > output is static and linear, with no spinner or redraw;
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) `viola verify` reaches `pin_exe` too, so its human TTY, plain and `--json` output keeps the CLI entity's output discipline when a concurrent start holds the pin.
  > - **Entity:** CLI verbs (`list`, `send`, `wait`, `last`, `answer`, `verify`, `pause`, `release`, `link`, `unlink`, `revive`, `plugin install`, `--help`) in human TTY, plain (non-TTY / `NO_COLOR` / `TERM=dumb`) and `--json` modes.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:10) Any new CLI-visible outcome of the fix is checked only by output-discipline assertions on text and streams, never by an a11y tool and with no conformance claim.
  >   - **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.
- (.andromeda/a11y-plan.md:37) The failing agent hook spawn the race causes (`ran:false`, no `session-start`) sits on the plugin surface, so the two-sided witness carries no a11y assertion of its own.
  > - **Entity:** Claude Code plugin: hooks (`viola hook`, `hook statusline`) and the stdio MCP server; wrapper IPC channel
  >   - **Source:** Arch excerpt, Surfaces ("a Claude Code plugin (hooks + stdio MCP server)"). Tests excerpt, E2E driver → ipc-internal and cli (`viola hook`).
  >   - **Assertability:** not-assertable
  >   - **Reason:** ipc-internal with no UI surface. Machine-to-machine traffic only.
- (.andromeda/a11y-plan.md:251) A bounded wait the fix adds at the start (like `pin_companions`' wait on `ERROR_SHARING_VIOLATION`) falls on viola's own start, never on the human.
  > every time bound viola holds — the `viola wait` timeout the caller passes, the MCP `wait` cap, the dialog hold, the `viola send` readiness gate, the confirmation windows, the status-line deadline, the `claude agents --json` read, the budget pause — falls on the driver, the user's own command or viola's own child, never on the human: no key is blocked, refused or delayed by it.
- (.andromeda/a11y-plan.md:257) If the fix adds a human-facing refusal for `run`, `revive`'s start or `verify` (for example, a copy changed while it was in flight), that refusal must take the CLI error-recovery shape.
  > on the CLI, `unable  <reason>  <detail>` on stderr (`viola revive`'s exit-1 refusals and `viola verify`'s interrupted pair: one `unable: <text>` line), then `hint:` as the last stderr line, with a typed exit code; the one refusal with no hint is `unknown` (exit 14), whose detail is opaque; faults are not refusals and carry no hint; `--json` gives one document with no hint.
- (.andromeda/a11y-plan.md:312) The existing P4/P6 output-discipline and TUI boundary cases must stay green on all three OS legs after the `run`-start change.
  > The CLI output-discipline invariants (P4, P6) pass on all three OS legs inside `nextest-integration` / `nextest-e2e` (CI: the `coverage` suite);

## Bindings
- **TUI passthrough boundary ↔ security.md §Local trust boundary (Home) / scope A**: the absent-copy write's concurrency fix (`pin_bytes` / `pin_exe`, `replace_private_shared`) stays inside the zero-viola-bytes, keystroke-never-delayed boundary (.andromeda/a11y-plan-detail/a11y-scope-summary.md:43); its diagnostics go to `diagnostics/` only (.andromeda/a11y-plan-detail/a11y-scope-summary.md:109), which binds with obs-plan's diagnostics routing.
- **Error recovery ↔ design cli refusals / test-plan trycmd pins**: a new start or verify refusal takes the `unable` + final `hint:` layout with a typed exit code and a hint-free `--json` (.andromeda/a11y-plan.md:257), pinned by trycmd or by literal.
- **Plugin not-assertable ↔ test-plan (the two-sided race witness)**: the hook-spawn failure the witness reads is machine-to-machine and carries no a11y assertion (.andromeda/a11y-plan.md:37).
- **SLO invariants ↔ test-plan §10 / CI coverage suite**: P4/P6 cases stay green on `[windows-2025, macos-latest, ubuntu-latest]` with no new suite (.andromeda/a11y-plan.md:312).
- **Scope B (reproducible-build measurement) ↔ this category**: (none) — a build measurement with no UI surface.
