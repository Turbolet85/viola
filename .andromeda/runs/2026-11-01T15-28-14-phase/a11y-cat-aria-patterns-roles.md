# a11y · ARIA Patterns & Roles — category read

## Carried
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:10) The board's home check made one rule must leave a board that passes it in this exact structural order; a refusal replaces the board, never reorders it.
  > As built (chunk 2026-10-11-the-board-viola-list; the rack, chunk 2026-10-16-unwrapped-sessions-on-the-board): BAY line → caption line → wrapped rows → (only when unwrapped items exist) the `-- UNWRAPPED - READ-ONLY --` separator and the unwrapped rows
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:18) `send` must keep this refusal shape beside the client's peer euid check, and that check's own refusal must follow it: on stderr, with a typed exit code, asserted by assert_cmd.
  > - **cli:** `send` refused with `human-typing` on stderr, with a typed exit code (assert_cmd).
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:19) A wrapper refusing a connecting client on a peer euid mismatch while the child holds the terminal must write no byte of its own to the outer PTY and must not hold back a human key; these clauses must still pass with the check in place.
  > (1) zero viola-originated bytes on the outer-PTY stream (byte for byte against an unwrapped fake-agent run on Linux/macOS; viola's own literals absent on Windows ConPTY); (2) human keystrokes delivered unblocked past the current atomic paste;
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:26) The board's refusal and a client refusal reached through `list`, `wait`, `last` or `verify` must be asserted by these drivers, with no ARIA claim and no other driver.
  > - **P6 — CLI board, wait/last, verify** (cli). No ARIA. trycmd / assert_cmd assert the non-TTY modes, and the portable-pty outer PTY asserts the TTY-only behaviours, on all three OS legs:
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:30) Every refusal this chunk adds must carry no SGR in these modes.
  > - no SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json`;
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:31) Every refusal this chunk adds must leave the word `unable` uncoloured under a terminal too.
  > under the outer PTY, every SGR span sits beside its word (amber only on `DIALOG`, dim only on stale rows, bold only on NAME column values), and `unable` / `fail` carry no SGR;
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:33) These root cases must still hold on their fixtures' homes once Unix strict modes and the board's one home rule stand, a fixture's mode never loosening the assertion.
  > the root cases `list_human_board_is_the_literal_rows` and `list_of_a_stopped_wrapper_is_an_empty_board` assert no ESC and no non-ASCII byte on either stream.
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:36) The cases pinning this chunk's refusal text are output-discipline stand-ins and must claim no conformance.
  > - These stand in for SC 1.3.2, 1.4.1 and 3.3.1, with no conformance claim.
- (.andromeda/a11y-plan.md:31) A refused socket directory at a wrapper's start is a refusal of the start `revive` shares with `run`, so under `viola revive --json` it must print the one document this line names, and P3's reading of its exit must be squared with this line's exit 1.
  > - **Under `viola revive --json`:** every refusal of the start it shares with `run` prints one exit-1 document, `batch-script-child`, `pinned-hash-mismatch` and `squatted-name` among them and a program not found `internal-error`; a `pump_child` error prints none: the child has held the terminal, and viola writes nothing past it.
- (.andromeda/a11y-plan.md:33) The wrapper-side euid check runs under this boundary: no terminal byte of viola's own and no key held back when it refuses a client.
  > viola writes zero bytes to the terminal except the child's output, human keystrokes are never blocked, refused or delayed past the current atomic paste
- (.andromeda/a11y-plan.md:37) The socket channel itself carries no a11y assertion: the directory and euid checks are asserted at the surfaces where their refusals print, never on the channel's traffic.
  > - **Entity:** Claude Code plugin: hooks (`viola hook`, `hook statusline`) and the stdio MCP server; wrapper IPC channel
  >   - **Source:** Arch excerpt, Surfaces ("a Claude Code plugin (hooks + stdio MCP server)"). Tests excerpt, E2E driver → ipc-internal and cli (`viola hook`).
  >   - **Assertability:** not-assertable
  >   - **Reason:** ipc-internal with no UI surface. Machine-to-machine traffic only.
- (.andromeda/a11y-plan.md:53) A refusal naming a socket directory or an instance must escape any control character in the path it prints.
  > C0/C1 control characters are escaped except `\n` and `\t`, which `list` rows escape too.
- (.andromeda/a11y-plan.md:55) A wrapper-side refusal raised once the child holds the terminal must land in `diagnostics/`, never on the terminal.
  > viola's diagnostics for `run` go to `diagnostics/`, never to the terminal;
- (.andromeda/a11y-plan.md:94) Each refusal this chunk adds must read in this form on stderr, colour no part of its meaning.
  > Also CLI output discipline: colour as a second cue only, and the refusal written to stderr as `unable … hint:` with a typed exit code.
- (.andromeda/a11y-plan.md:99) The directory and euid checks are security refusals and must stop a client or a bind, never the human at the keyboard.
  > - Security excerpt: "NEVER let a security refusal block the human".
- (.andromeda/a11y-plan.md:186) The new refusals' cases must use this reach; a refusal that exists on Unix only must name the legs it holds on rather than read as all three.
  > | cli | assertable (output discipline, no conformance claim) | assert_cmd 2.2.2 + predicates 3.1.4 + trycmd 1.2.1; portable-pty outer-PTY SGR checks; all three OS legs | NVDA + Windows Terminal/conhost; VoiceOver + Terminal.app; Orca + GNOME Terminal |
- (.andromeda/a11y-plan.md:192) A frame verb refused by the client's peer euid check must hold this layout.
  >   - **cli:** `[RB]` mirror and `unable <reason> <detail>` + `hint:` via trycmd.
- (.andromeda/a11y-plan.md:257) Each closed code this chunk adds must be a printed word with plain detail and a final `hint:` line, and one document with no hint under `--json`, never a fault standing where a refusal is owed.
  > - **Pattern:** every error is a printed word followed by plain detail, and a recovery instruction where recovery exists; on the CLI, `unable  <reason>  <detail>` on stderr (`viola revive`'s exit-1 refusals and `viola verify`'s interrupted pair: one `unable: <text>` line), then `hint:` as the last stderr line, with a typed exit code; the one refusal with no hint is `unknown` (exit 14), whose detail is a code word or nothing, opaque to a reader; faults are not refusals and carry no hint; `--json` gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths (a fixed stderr line, no document).
- (.andromeda/a11y-plan.md:261) This chunk's refusals must be pinned this way, and the macOS carve-out must stand as it does here: the server-verify refusal holds on windows-2025 and ubuntu-latest only, no test reading macOS as a comparison.
  > - **Verification:** trycmd pins the `unable <reason> <detail>` + final `hint:` layout per refusal and assert_cmd the typed exit codes and a hint-free `--json`, on all three OS legs (the frame verbs' server-verify refusal on windows-2025 and ubuntu-latest only: macOS reports no peer pid).
- (.andromeda/a11y-plan.md:270) Each new kebab-case code must print beside `unable`.
  >   - Every kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
- (.andromeda/a11y-plan.md:271) The socket-directory failure must leave the CLI as a closed refusal, never as a stack trace or a bare status number.
  >   - The CLI never shows a stack trace or bare status number as the only error text.
- (.andromeda/a11y-plan.md:273) Each new refusal's hint must be pinned by trycmd or by literal, the row of `tests/cross_exit_causes.rs` included in what pins it, and this line must move if a literal joins its list.
  > - **Verification:** every fixed CLI string is pinned, by trycmd or, for `viola list`, `viola --help`, `viola plugin install`, the server-verify, `unwrapped-name` and `endpoint-lost` hints and the `viola link` / `viola unlink` result lines, by literal, so any copy change fails CI until the pinned string is updated on purpose.
- (.andromeda/a11y-plan.md:340) No new refusal may be coloured, and no SGR may appear under `viola run` or a passed `viola revive` from a check this chunk adds.
  > - NEVER colour CLI `unable` / `fail`, and never emit SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`.
- (.andromeda/a11y-plan.md:345) Each new refusal must end with a `hint:` line, none of them taking the `unknown` exception.
  > - NEVER show error without recovery suggestion (per SC 3.3.3). Every human-mode CLI refusal except `unknown` (exit 14, its detail a code word or nothing, design cli) ends with a `hint:` line.

## Bindings
- **Refusal layout ↔ the Cognitive Accessibility category**: the layout P2 and P4 assert on the CLI is detailed under Error recovery; both must read the same for each code this chunk adds, (.andromeda/a11y-plan.md:192) with (.andromeda/a11y-plan.md:255).
- **P4 boundary cases ↔ §3 harness contract**: the outer-PTY clauses the wrapper-side euid check must keep passing stand under Section 3's Keyboard test harness, read by contract key, (.andromeda/a11y-plan-detail/aria-patterns-roles.md:19) with (.andromeda/a11y-plan.md:172).
- **Security refusals ↔ the human path**: the security plan's checks on the socket directory and the peer euid bind with the rule that a security refusal never blocks the human, (.andromeda/a11y-plan.md:99).
- **Refusal copy ↔ design plan**: the `unable` / `hint:` form and the `unknown` exception are the design plan's CLI refusal surface, and a new code's copy must hold on both sides, (.andromeda/a11y-plan.md:345).
- **macOS gap ↔ tests plan**: the server-verify legs named here must match the legs the test plan's cases hold on, the macOS match with no comparison never read as a pass of the comparison, (.andromeda/a11y-plan.md:261).
