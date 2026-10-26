# a11y · A11y Scope Summary — category read

## Carried
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) The CLI verbs this chunk gives a server check to (`send`, `wait`, `last`, `answer`, `pause`, `release`) are an assertable a11y entity in all three output modes. The exit-21 unreachable outcome on a mismatch, or on a check that cannot be made, must hold the output discipline in human TTY, plain and `--json` mode.
  > **Entity:** CLI verbs (`list`, `send`, `wait`, `last`, `answer`, `verify`, `pause`, `release`, `link`, `unlink`, `revive`, `plugin install`, `--help`) in human TTY, plain (non-TTY / `NO_COLOR` / `TERM=dumb`) and `--json` modes.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:10) Any a11y check this chunk adds for the verbs' refusal is an assertion on text and streams. It is not an a11y-tool result.
  > **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:20) Verifying the hook client and the wrapper's endpoint must not add any terminal byte of viola's own in `viola run` or a passed `viola revive`.
  > viola writes zero bytes to the terminal except the child's output;
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:21) A server-verification refusal or a hook that fails open must never block or delay a human keystroke.
  > human keystrokes are never blocked, refused or delayed past the current atomic paste;
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:31) The verbs' changed refusal paths are asserted with the existing CLI drivers, and their results stay machine-parseable through the harness `run` JSON.
  > The fallback is output-discipline assertions on the tests' existing CLI drivers: `assert_cmd` 2.2.2 with `predicates` 3.1.4, `trycmd` 1.2.1, and SGR-only checks through the portable-pty outer PTY (tests excerpt, E2E driver → cli). Their pass/fail results are machine-parseable through the harness `run` JSON.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:32) No WCAG conformance claim may rest on the chunk's refusal-text cases.
  > These are not a11y tools. They produce no a11y-tool JSON, so no WCAG conformance claim may rest on them.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:45) The unreachable refusal must carry no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR` or `TERM=dumb`.
  > there is no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`;
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:46) The server check must not add a spinner or a redraw while it waits on the endpoint or on the start-time read.
  > output is static and linear, with no spinner or redraw;
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:48) The refusal's output discipline must hold on all three OS legs, including the Windows pipe path and the macOS euid-only gap.
  > This must hold on `[windows-2025, macos-latest, ubuntu-latest]` (tests excerpt, Coverage Triggers → multi-platform).
- (.andromeda/a11y-plan.md:37) The hook client (`hook.event` and `hook.dialog`) and the MCP frame tools are not-assertable surfaces. Their fail-open and `instance-unreachable` outcomes carry no a11y assertion.
  > - **Entity:** Claude Code plugin: hooks (`viola hook`, `hook statusline`) and the stdio MCP server; wrapper IPC channel
- (.andromeda/a11y-plan.md:40) This is why the hook and MCP halves sit outside a11y scope.
  > **Reason:** ipc-internal with no UI surface. Machine-to-machine traffic only.
- (.andromeda/a11y-plan.md:73) The chunk's cases must run inside the existing suites. They must not add an `a11y` suite value.
  > **No new suite:** no new `suite` enum value (such as `a11y`) is added without a Decisions Log entry.
- (.andromeda/a11y-plan.md:94) Path 4 covers `pause`, `release` and `send` refused. Their verified-server refusal must be written to stderr as `unable … hint:` with a typed exit code, and colour may only be a second cue.
  > Also CLI output discipline: colour as a second cue only, and the refusal written to stderr as `unable … hint:` with a typed exit code.
- (.andromeda/a11y-plan.md:99) Server verification is a security refusal, so it must never block the human.
  > Security excerpt: "NEVER let a security refusal block the human".
- (.andromeda/a11y-plan.md:107) Path 6 covers `wait` and `last`, so their unreachable refusal must give the reason and the detail in text, in linear order.
  > Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text.
- (.andromeda/a11y-plan.md:192) Path 2 covers `send`, so its refusal layout must be pinned through trycmd.
  > **cli:** `[RB]` mirror and `unable <reason> <detail>` + `hint:` via trycmd.
- (.andromeda/a11y-plan.md:257) In human mode the exit-21 refusal must be `unable` with its detail, then `hint:` as the last stderr line. Under `--json` it must be one document with no hint.
  > on the CLI, `unable  <reason>  <detail>` on stderr (`viola revive`'s exit-1 refusals and `viola verify`'s interrupted pair: one `unable: <text>` line), then `hint:` as the last stderr line, with a typed exit code; the one refusal with no hint is `unknown` (exit 14), whose detail is opaque; faults are not refusals and carry no hint; `--json` gives one document with no hint.
- (.andromeda/a11y-plan.md:261) The changed refusal path of each verb must be pinned by trycmd for its layout and by assert_cmd for its exit code and its hint-free `--json`, on all three OS legs.
  > **Verification:** trycmd pins the `unable <reason> <detail>` + final `hint:` layout per refusal and assert_cmd the typed exit codes and a hint-free `--json`, on all three OS legs.
- (.andromeda/a11y-plan.md:273) If the chunk adds or changes any CLI refusal copy, that string must be pinned, so a copy change fails CI.
  > **Verification:** every fixed CLI string is pinned, by trycmd or, for `viola list`, `viola --help` and `viola plugin install`, by literal, so any copy change fails CI until the pinned string is updated on purpose.
- (.andromeda/a11y-plan.md:340) The refusal's `unable` word must never be coloured.
  > NEVER colour CLI `unable` / `fail`, and never emit SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`.
- (.andromeda/a11y-plan.md:345) The new mismatch outcome of every human-mode verb must end with a `hint:` line.
  > Every human-mode CLI refusal except `unknown` (exit 14, opaque detail, design cli) ends with a `hint:` line.
- (.andromeda/a11y-plan.md:346) Any kebab-case code the refusal shows must sit beside the plain word `unable`.
  > a kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) never appears without the plain word `unable`.
- (.andromeda/a11y-plan.md:347) A verification failure must never surface as a stack trace or an error chain.
  > NEVER render a stack trace as the only error text in the CLI.
- (.andromeda/a11y-plan.md:330) The chunk's cases must make no WCAG claim.
  > NEVER claim WCAG conformance for the cli or the `viola run` TUI (the wrapper-hosted TUI, whether `viola run` or a passed `viola revive` started it).

## Bindings
- **CLI exit-21 unreachable refusal ↔ security-plan / security.md (the client server check, the liveness pre-check's exit 21)**: security owns when the refusal fires (a pid and start-time mismatch, or a check that cannot be made). a11y owns how it reads: `unable` with its detail, `hint:` last, no SGR, and one hint-free document under `--json` (.andromeda/a11y-plan.md:257) (.andromeda/a11y-plan.md:345) (.andromeda/a11y-plan-detail/a11y-scope-summary.md:45).
- **Refusal layout pins ↔ test-plan (trycmd / assert_cmd, the `coverage` suite on all three OS legs)**: a changed refusal path for `send`, `wait`, `last`, `answer`, `pause` or `release` is pinned by trycmd and assert_cmd inside the existing suites, with no new suite value (.andromeda/a11y-plan.md:261) (.andromeda/a11y-plan.md:73) (.andromeda/a11y-plan-detail/a11y-scope-summary.md:31).
- **Refusal copy ↔ design (User-Facing Error Surfaces → CLI refusals)**: any new or changed unreachable copy is a fixed string the design owns, and it is pinned so that a drift fails CI (.andromeda/a11y-plan.md:273).
- **Hook fail-open and the human's wheel ↔ architecture's universal invariant (`viola hook` exits 0 with nothing on stdout or stderr)**: a11y adds that the check writes no terminal byte of viola's own and that no keystroke is blocked or delayed (.andromeda/a11y-plan-detail/a11y-scope-summary.md:20) (.andromeda/a11y-plan-detail/a11y-scope-summary.md:21) (.andromeda/a11y-plan.md:99).
- **Hook and MCP clients ↔ the not-assertable plugin/IPC entity**: their halves of this chunk carry no a11y assertion (.andromeda/a11y-plan.md:37).
