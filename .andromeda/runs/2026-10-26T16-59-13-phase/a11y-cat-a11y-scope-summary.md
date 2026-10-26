# a11y · A11y Scope Summary — category read

## Carried
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) The frame verbs this chunk gives a strict-modes check before their snapshot read (`send`, `wait`, `last`, `answer`, `pause`, `release`) must keep the refusal shape their entity pins: exit 21, stdout empty, a plain-ASCII `hint:` last on stderr, a hint-free `--json` document, codes and exit codes unchanged.
  > `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` each refuse, before any frame, an endpoint whose serving process is not the recorded wrapper: exit 21, stdout empty and the server-verify `hint:` line last on stderr (plain ASCII, no SGR), or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty (the refusal's text: Cognitive Accessibility › Error recovery); on macOS no peer pid is read and the refusal cannot occur (the recorded gap); their `--json` codes and exit codes are unchanged
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:16) The `link` / `unlink` strict-modes refusal is the in-scope precedent the new frame-verb checks must match: exit 21 on a failed strict-modes check, made before any log opens, with the refusal text under Error recovery.
  > each refuses with exit 21 a driven instance that fails the strict-modes check, before any log opens, a stopped driven instance and a server that is not the recorded wrapper (the entity above), and a pair the wrapper refuses as invalid params reaches it as a fault, exit 20, no hint (the refusals' text: Cognitive Accessibility › Error recovery)
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) Any change to `check_instance` for its Windows proof (CARRY 12/13) must leave `viola revive`'s strict-modes refusal output unchanged: the exact `--json` document with empty stderr, or the byte-identical `unable:` / `hint:` pair.
  > under it each exit-1 refusal of its strict-modes check or its preflight is one document `{"v":1,"error":"<code>","detail":null}` on stdout with empty stderr, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`, and `viola revive <name> --list --json` prints `{"v":1,"ok":{"sessions":[…]}}` in log order (an empty chain the `no-session` document, exit 1); without `--json` `viola revive <name> --list` prints static ASCII rows (`<ts>  <cause>  <id>`, no header, no colour, no ESC byte) and each exit-1 refusal is one `unable:` line and one `hint:` line last on stderr, byte-identical
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) A new strict-modes refusal in `run`'s start, reached after a passed revive preflight, prints no `--json` document in this chunk; that document stays owed to "Exit-cause code catalogue" (the scope's boundary on new exit-cause codes).
  > a refusal `run`'s start makes after a passed preflight prints no document as landed (owed to "Exit-cause code catalogue"); a clap usage error exits 2, and a passed start is the TUI passthrough entity below.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) The `marketplace/` integrity work (CARRY 17) must keep `viola plugin install`'s exit-21 refusal shape.
  > its exit-21 refusal is the home pair with `hint:` last and stdout empty, or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:28) The checks `run` gains (the home, `bin/`, `plugin/`, the pinned-binary re-hash, the per-start plugin rewrite, the diagnostics DACL check) must write no byte to the terminal once the child holds it.
  > viola writes zero bytes to the terminal except the child's output;
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:29) No integrity check this chunk adds to the running wrapper may block, refuse or delay a human keystroke.
  > human keystrokes are never blocked, refused or delayed past the current atomic paste;
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:24) A passed revive start, which now runs the strict-modes checks `run` gains, must still write no line of its own while the child holds the terminal.
  > under revive the clause read by a case of its own is that a passed revive writes no line of its own while the child holds the terminal, `tests/chaos_revive.rs`
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:84) The DACL check before any diagnostics file must not move `run`'s diagnostics onto the terminal: a failed check writes nothing there.
  > viola's diagnostics for `run` go to `diagnostics/`, never to the terminal
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:39) The strict-modes check this chunk adds to the hook client (CARRY 3) and to the MCP frame tools (CARRY 15) must hold the same failure shape: the hook sends no frame and fails open with exit 0 and empty streams under 1.0 s; the MCP tool returns only `instance-unreachable` and sends no frame.
  > a hook event or dialog call that meets such a server sends no frame and fails open: exit 0, empty stdout and stderr, under 1.0 s; an MCP frame tool returns `isError` with the code `instance-unreachable` only (text `refused: instance-unreachable`) and sends no frame. Neither writes a human-facing byte, so the entity stays not-assertable and no human keystroke is touched.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:62) Every new refusal is plain text: no SGR, glyph or cursor control in the modes listed.
  > there is no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`; `viola revive --list` rows and the revive refusal pairs carry no ESC byte;
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:65) The output discipline of the new refusals must hold on all three CI OS legs.
  > This must hold on `[windows-2025, macos-latest, ubuntu-latest]` (tests excerpt, Coverage Triggers → multi-platform).
- (.andromeda/a11y-plan.md:94) A strict-modes refusal of `pause` / `release` / `send` (Path 4's verbs) goes to stderr as `unable … hint:` with a typed exit code, and colour stays a second cue only.
  > Also CLI output discipline: colour as a second cue only, and the refusal written to stderr as `unable … hint:` with a typed exit code.
- (.andromeda/a11y-plan.md:74) The output-discipline cases for the new refusals run inside the existing suites (CI: the per-OS `coverage` suite).
  > **CLI output discipline** runs inside `nextest-integration` / `nextest-e2e` (locally), and in CI inside the per-OS `test` job's single instrumented `coverage` suite, on all three OS legs.
- (.andromeda/a11y-plan.md:73) The chunk's new cases must add no `suite` value.
  > **No new suite:** no new `suite` enum value (such as `a11y`) is added without a Decisions Log entry.

## Bindings
- **Frame-verb strict-modes refusal ↔ body §8 Cognitive Accessibility › Error recovery**: the refusal entity names its text as Error recovery's (`unable` + `hint:` last, a hint-free `--json`, typed exit), (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) and (.andromeda/a11y-plan.md:257).
- **New refusals' cases ↔ body §1 CI integration / tests §9**: they run inside the existing `coverage` suite on all three legs, with no new suite value, (.andromeda/a11y-plan.md:74) and (.andromeda/a11y-plan.md:73).
- **Run-start integrity checks ↔ TUI passthrough boundary (body §1 tui surface)**: zero viola bytes on the terminal and diagnostics only in `diagnostics/`, (.andromeda/a11y-plan-detail/a11y-scope-summary.md:28) and (.andromeda/a11y-plan.md:55).
- **Revive-after-preflight refusal document ↔ "Exit-cause code catalogue" (body §1 Owed)**: no document lands in this chunk, (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) and (.andromeda/a11y-plan.md:31).
- **Hook / MCP strict-modes failure ↔ security.md fail-open rule**: the not-assertable entity keeps exit 0 with empty streams for the hook and the code `instance-unreachable` only for MCP, (.andromeda/a11y-plan-detail/a11y-scope-summary.md:39).
