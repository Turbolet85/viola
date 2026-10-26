# a11y · A11y Scope Summary — category read

## Carried
- (.andromeda/a11y-plan.md:37) The chunk's own surface (the wrapper's Windows named-pipe listener: DACL, remote-client rejection, first-pipe-instance, handle inheritance) is the wrapper IPC channel, which carries no a11y assertion: the chunk must add no a11y case, suite or conformance claim for the listener itself.
  > - **Entity:** Claude Code plugin: hooks (`viola hook`, `hook statusline`) and the stdio MCP server; wrapper IPC channel
  >   - **Source:** Arch excerpt, Surfaces ("a Claude Code plugin (hooks + stdio MCP server)"). Tests excerpt, E2E driver → ipc-internal and cli (`viola hook`).
  >   - **Assertability:** not-assertable
  >   - **Reason:** ipc-internal with no UI surface. Machine-to-machine traffic only.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:16) The listener lives inside the `viola run` wrapper (and a passed `viola revive`'s start), so the TUI passthrough boundary binds every admission path the chunk adds: a pipe client rejected as remote, denied by the DACL, or a second listener refused on a held name must write zero bytes of viola's own to the terminal and must never block, refuse or delay a human keystroke.
  > - **Entity:** `viola run` TUI passthrough: the unmodified `claude` TUI hosted in ConPTY/openpty, entered by `viola run` and by a `viola revive` whose preflight passed
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:19) The boundary requirement itself, which the endpoint-admission refusals (automation-side only) must honour.
  > The assertion boundary is viola's wrapper:
  >     - viola writes zero bytes to the terminal except the child's output;
  >     - human keystrokes are never blocked, refused or delayed past the current atomic paste;
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:61) Any diagnostic the chunk records for a rejected or refused pipe client during `run` goes to `diagnostics/`, never to the terminal.
  > viola's diagnostics for `run` go to `diagnostics/`, never to the terminal (arch excerpt, A11y-Relevant Conventions → terminal output discipline).
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:45) If the chunk touches `src/cmd/run.rs`'s squatted-name start refusal, its stderr output must stay free of SGR, glyph and cursor control under the plain modes and `viola run`.
  > there is no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`;
- (.andromeda/a11y-plan.md:257) The squatted-name start refusal (exit 1, word `squatted-name`, unchanged per scope) must keep the human refusal layout: `unable` text on stderr, then `hint:` as the last stderr line, with its typed exit code.
  > on the CLI, `unable  <reason>  <detail>` on stderr (`viola revive`'s exit-1 refusals and `viola verify`'s interrupted pair: one `unable: <text>` line), then `hint:` as the last stderr line, with a typed exit code;
- (.andromeda/a11y-plan.md:345) The squatted-name refusal is not `unknown`, so it must keep ending with a `hint:` line.
  > - NEVER show error without recovery suggestion (per SC 3.3.3). Every human-mode CLI refusal except `unknown` (exit 14, opaque detail, design cli) ends with a `hint:` line.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:48) The output discipline above must hold on the Windows leg as on the other two; the chunk's assertions land on the Windows CI leg.
  > This must hold on `[windows-2025, macos-latest, ubuntu-latest]` (tests excerpt, Coverage Triggers → multi-platform).

## Bindings
- **IPC channel not-assertable ↔ security-plan §Authentication & Authorization (IPC access control, Windows)**: the pipe DACL / `accept_remote(false)` / first-instance work is asserted by security and test cases only; a11y adds no case or suite for it (.andromeda/a11y-plan.md:37).
- **TUI boundary ↔ security rule "the human always wins" (refusals go to automation only)**: an endpoint admission refusal reaches only the rejected client, never the terminal or the human's keystrokes (.andromeda/a11y-plan-detail/a11y-scope-summary.md:19).
- **Run diagnostics ↔ obs-plan event catalog**: a rejected-client record is an obs line under `diagnostics/`, never terminal output (.andromeda/a11y-plan-detail/a11y-scope-summary.md:61).
- **Squatted-name refusal layout ↔ architecture §Conventions (refusals, exit codes)**: `squatted-name`, exit 1, the `unable` / `hint:` pair unchanged (.andromeda/a11y-plan.md:257).
