# a11y · cascade read

## Carried
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) The six CLI verbs the chunk gives server verification are members of this entity, so their refusal on a mismatched or unverifiable server must be asserted in human TTY, plain and `--json` modes, on text and streams only.
  > CLI verbs (`list`, `send`, `wait`, `last`, `answer`, `verify`, `pause`, `release`, `link`, `unlink`, `revive`, `plugin install`, `--help`) in human TTY, plain (non-TTY / `NO_COLOR` / `TERM=dumb`) and `--json` modes.
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:10)
  > assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:20) The hook client runs inside the `viola run` passthrough, so its fail-open on a mismatch or an unmakeable check must write no byte of viola's own to the terminal and must never block, refuse or delay a human keystroke.
  > viola writes zero bytes to the terminal except the child's output;
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:21)
  > human keystrokes are never blocked, refused or delayed past the current atomic paste;
  (.andromeda/a11y-plan.md:37)
  > **Entity:** Claude Code plugin: hooks (`viola hook`, `hook statusline`) and the stdio MCP server; wrapper IPC channel
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:47) The new refusal path of the CLI verbs is cli-surface output: any text it prints must escape C0/C1 controls and stay static and linear, with no spinner or redraw.
  > C0/C1 control characters are escaped except `\n` and `\t`
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:46)
  > output is static and linear, with no spinner or redraw;
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:57) A hook fail-open case run under `viola run` that the plan may add is a tui boundary assertion on the outer-PTY driver, and the hook's verification failure and its diagnostics must go to `diagnostics/`, never the terminal.
  > Boundary assertions run on the tests' portable-pty `=0.8.1` outer-PTY driver (tests excerpt, E2E driver → tui). They check that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed.
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:61)
  > viola's diagnostics for `run` go to `diagnostics/`, never to the terminal (arch excerpt, A11y-Relevant Conventions → terminal output discipline).
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:18) `send`, `pause` and `release` sit on the P4 path, so their refusal on a mismatched server must stay a stderr refusal with a typed exit, and a passed verification must leave the `human-typing` refusal reachable and distinct; on P2 a refused `send` must never read as an `[RB]` readback, and on P3 a refused `answer` must stay distinct from `unverified-cli`.
  > `send` refused with `human-typing` on stderr, with a typed exit code (assert_cmd).
  (.andromeda/a11y-plan.md:90)
  > - **Surfaces involved:** tui (boundary-only) and cli (`pause`, `release`, `send` refused with `human-typing`).
  (.andromeda/a11y-plan.md:192)
  > **cli:** `[RB]` mirror and `unable <reason> <detail>` + `hint:` via trycmd.
  (.andromeda/a11y-plan.md:194)
  > **cli:** `viola answer` refusal `unverified-cli` text on the CLI.
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:27) `wait` / `last` refused by the new check before any frame must keep linear static output, and the `waiting:` line must appear only after a passed verification, never on the refusal path.
  > linear static output: under the outer PTY, `waiting: <name>` appears exactly once on stderr and the stream contains no `\r` redraw, cursor-movement or erase sequence (no spinner); under non-TTY the `waiting:` line is absent;
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:8) Every time bound the verification adds (connect, peer lookup, start-time read) in the MCP tools, the CLI verbs and the hook must fall on the driver or the hook only, never on a human key or the wheel, and the statusline arm whose snapshot read the chunk edits must still exit 0 with empty stderr in every case.
  > that bound falls on the driver only (the hidden, plugin-run `viola mcp` serves a driving Claude session, never a person): no key is blocked, refused or delayed by it, and the wheel is untouched.
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:9)
  > `viola hook statusline` runs the user's own status line command under `STATUSLINE_DEADLINE` (5 s, PROVISIONAL, `src/cmd/hook.rs`) and exits 0 with empty stderr in every case.
  (.andromeda/a11y-plan.md:251)
  > - **CLI:** every time bound viola holds — the `viola wait` timeout the caller passes, the MCP `wait` cap, the dialog hold, the `viola send` readiness gate, the confirmation windows, the status-line deadline, the `claude agents --json` read, the budget pause — falls on the driver, the user's own command or viola's own child, never on the human: no key is blocked, refused or delayed by it.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19) The unreachable refusal a CLI verb gives on a failed or unmakeable verification, the held-fork outcome for an unreadable or newer `snapshot.json` included, must be `unable  <reason>  <detail>` on stderr with `hint:` last and a typed exit, and under `--json` one document with no hint.
  > **CLI:** `unable  <reason>  <detail>` on stderr, then `hint:` as the last stderr line, with a typed exit code. The one refusal with no hint is `unknown` (exit 14): its detail is opaque, so design prints no hint rather than a guess. Faults (`error: wrapper fault`, `error: internal error`) are not refusals and carry no hint. `--json` gives one document with no hint.
  (.andromeda/a11y-plan.md:270)
  > - Every kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
  (.andromeda/a11y-plan.md:271)
  > - The CLI never shows a stack trace or bare status number as the only error text.
- (.andromeda/a11y-plan.md:334) The hook client's new verification runs in the hook a `viola run` child fires, so neither its check nor its fail-open may block, refuse or delay a human keystroke.
  > NEVER block, refuse or delay a human keystroke in `viola run`, or in a start a passed `viola revive` made, past the current atomic paste, and never count focus / mouse / resize sequences or terminal replies as editing (P4)
- (.andromeda/a11y-plan.md:345) The human-mode refusal on a failed verification is not `unknown`, so it must end with a `hint:` line.
  > NEVER show error without recovery suggestion (per SC 3.3.3). Every human-mode CLI refusal except `unknown` (exit 14, opaque detail, design cli) ends with a `hint:` line.
- (.andromeda/a11y-plan.md:346) The unreachable code the new refusal prints must sit beside the plain word `unable`, never alone.
  > NEVER use jargon without a plain alternative. SC 3.1.5 AAA is not claimed, but a kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) never appears without the plain word `unable`.

## Bindings
- **scope-entities (hook in the passthrough) ↔ cognitive-timeouts**: the hook's fail-open writes zero terminal bytes (.andromeda/a11y-plan-detail/a11y-scope-summary.md:20) and its verification bound falls on the hook, never the human (.andromeda/a11y-plan-detail/cognitive-accessibility.md:8), held together by the keyboard ban (.andromeda/a11y-plan.md:334).
- **scope-surfaces (tui) ↔ path-p4-wheel**: a hook fail-open case added under `viola run` is an outer-PTY boundary case (.andromeda/a11y-plan-detail/a11y-scope-summary.md:57), and the P4 cli refusals of `send` / `pause` / `release` keep their typed exits beside it (.andromeda/a11y-plan-detail/aria-patterns-roles.md:18).
- **cognitive-errors ↔ scope-entities ↔ path-p4-wheel / P2 / P3**: one refusal layout (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19) holds in every mode of the verbs (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) and stays distinct from the `[RB]` readback and the `unverified-cli` refusal (.andromeda/a11y-plan.md:192), (.andromeda/a11y-plan.md:194), with `hint:` last (.andromeda/a11y-plan.md:345) and the code beside `unable` (.andromeda/a11y-plan.md:346).
- **path-p6-board ↔ scope-surfaces (cli)**: `wait` / `last` refused before the frame keep linear static output with no `waiting:` line (.andromeda/a11y-plan-detail/aria-patterns-roles.md:27), the surface's static-output and C0/C1 rule over the same text (.andromeda/a11y-plan-detail/a11y-scope-summary.md:46).
