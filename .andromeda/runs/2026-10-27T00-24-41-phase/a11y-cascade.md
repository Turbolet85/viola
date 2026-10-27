# a11y · cascade read

## Carried
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:43) The pin step of `run`'s start (and of a passed revive's start) must write no byte of viola's own to the terminal, and any wait the fix adds must sit before the child's spawn or off the input path, so no human keystroke is blocked, refused or delayed past the current atomic paste.
  > **Reason:** The rendered content belongs to the vendor (`claude`) and is not traced. The assertion boundary is viola's wrapper:
  >   - viola writes zero bytes to the terminal except the child's output;
  >   - human keystrokes are never blocked, refused or delayed past the current atomic paste;
  (.andromeda/a11y-plan-detail/aria-patterns-roles.md:19)
  > **tui:** three portable-pty outer-PTY nextest cases on all three OS legs, one per Section 1 boundary clause (Section 3 → Keyboard test harness → Tooling): (1) zero viola-originated bytes on the outer-PTY stream (byte for byte against an unwrapped fake-agent run on Linux/macOS; viola's own literals absent on Windows ConPTY); (2) human keystrokes delivered unblocked past the current atomic paste;
  (.andromeda/a11y-plan.md:334)
  > NEVER block, refuse or delay a human keystroke in `viola run`, or in a start a passed `viola revive` made, past the current atomic paste
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:109) Any wait, retry or failure in the concurrent pin must go to `diagnostics/` or the wrapper log, never to the terminal, and the hook-spawn and plugin-rewrite side of the fix, like the pin change inside `viola-state`, must add no a11y assertion of its own.
  > **Notes:** viola's diagnostics for `run` go to `diagnostics/`, never to the terminal (arch excerpt, A11y-Relevant Conventions → terminal output discipline).
  (.andromeda/a11y-plan.md:37)
  > - **Entity:** Claude Code plugin: hooks (`viola hook`, `hook statusline`) and the stdio MCP server; wrapper IPC channel
  >   - **Source:** Arch excerpt, Surfaces ("a Claude Code plugin (hooks + stdio MCP server)"). Tests excerpt, E2E driver → ipc-internal and cli (`viola hook`).
  >   - **Assertability:** not-assertable
  >   - **Reason:** ipc-internal with no UI surface. Machine-to-machine traffic only.
  (.andromeda/a11y-plan.md:42)
  > - **Entity:** Internal crates (modular monolith)
  >   - **Source:** Arch excerpt, Project Intent ("one binary with compiler-enforced internal crates"). Tests excerpt, E2E driver → library-only.
  >   - **Assertability:** not-assertable
  >   - **Reason:** Library-only with no UI surface.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:87) No output the pin path adds under `viola run` or a passed revive may carry SGR, glyph or cursor control, and any `verify` output it touches must stay static and linear, with no spinner or redraw while a pin waits.
  > there is no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`; `viola revive --list` rows and the revive refusal pairs carry no ESC byte;
  >       - output is static and linear, with no spinner or redraw;
  (.andromeda/a11y-plan-detail/visual-design-verification.md:9)
  > viola emits zero SGR of its own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`, and the revive refusal pairs and `--list` rows carry no ESC byte. Under `viola run` on Windows the outer-PTY stream also carries the ConPTY host's own SGR, cursor and query bytes (not viola's: the inbox host's as measured at chunk 2026-09-25-pty-wrapper-on-windows research fact 4, the sideloaded `OpenConsole.exe`'s `ESC[1t ESC[c ESC[?1004h ESC[?9001h` as measured at chunk 2026-09-29-sideloaded-conpty), so the windows-2025 `viola run` check is that viola's literals are absent, on both backends. `unable` / `fail` are never coloured.
  (.andromeda/a11y-plan.md:340)
  > NEVER colour CLI `unable` / `fail`, and never emit SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`.
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:28) A change to `pin_exe` inside `viola verify` must keep its step lines in their `[NN/NN] … pass|fail` form and order, with `unable` and `fail` uncoloured.
  > `[NN/NN] … pass|fail`;
  (.andromeda/a11y-plan-detail/aria-patterns-roles.md:31)
  > and `unable` / `fail` carry no SGR;
  (.andromeda/a11y-plan.md:106)
  > Output is linear and static: `waiting: <name>` once on stderr (TTY only), `verify` step lines `[NN/NN] … pass|fail`.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) A pin wait or failure inside `viola verify` must keep its interrupted path to the one `unable:` / `hint:` pair with no step line and no summary, and a refusal the fix adds to `run`'s start after a passed revive preflight prints no `--json` document until the owed catalogue lands.
  > a refusal `run`'s start makes after a passed preflight prints no document as landed (owed to "Exit-cause code catalogue"); a clap usage error exits 2, and a passed start is the TUI passthrough entity below. A `verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only one `unable: verify was stopped before it finished` line and one `hint: run viola verify again to stamp this CLI` line last on stderr (plain ASCII), and exits 1 (chunk 2026-10-13-interrupted-verify-cleanup).
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:32) The fix's absent-copy write, and any lock or temp file it adds under `bin/<key>/` or `plugin/<key>/`, must come after the home-tree strict-modes refusal of `run`, a revive start and `verify`, which stays before any log with its fixed pair and starts no child.
  > `viola run`, a `viola revive` start and `viola verify` over a home tree that fails the strict-modes check: each refuses before any log opens with exit 1, stdout empty and the home-state pair on stderr, `hint:` last, and starts no child
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:37)
  > a home tree that fails the strict-modes check (the home, the instance's files, `ledger/`, `bin/`, `plugin/`, the pinned copy and its plugin folders) is refused with exit 1 before any log opens, no line written and no child started
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:43)
  > a home whose tree (the home, `ledger/`, `bin/`, `plugin/`) fails the strict-modes check is refused with exit 1 before its log opens, with no step line, no summary and no line written: the fixed home-state `unable` / `hint:` pair on stderr, `hint:` last; a `ledger/` that `update_stamps`'s own check refuses gives the same pair.
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:49)
  > a home path under a regular file is a strict-modes refusal with its hint on Unix (`verify`: the home-state pair, exit 1; the six frame verbs: the instance-state pair, exit 21); on Windows such a path stats `NotFound`, the check skips it as absent, and the `cli` catch site's fault line stands, with no hint.
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:81)
  > `run_refuses_a_widened_home_tree_before_its_log` (`tests/run_cli.rs`) reads `run`'s exit 1 with the `HOME_PAIR` or `INSTANCE_PAIR` literal, stdout empty, no line and no child; `verify_refuses_a_widened_ledger_before_any_step` and `verify_under_a_regular_file_refuses_the_home` (`tests/cli_verify.rs`, the latter's oracle per OS: the home-state pair on Unix, the catch-site line on Windows) read `verify`'s home-state pair with no step line or summary. Unlike the server-verify cases, these run on all three OS legs.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:9) A bounded wait or retry the plan adds to the pin (a lock, or waiting out a sharing violation) is one of viola's own time bounds: it must fall on viola's own start, never on a keystroke or the human, and needs no warn-and-extend pattern.
  > `viola list` and `viola revive` each make one `claude agents --json` read (a direct spawn, stdout through `take(MAX_FRAME + 1)`, killed at 5 s; chunk 2026-10-16-unwrapped-sessions-on-the-board): a failed, oversize, malformed or timed-out read leaves `list`'s board as `board::read` gave it and passes `revive`'s preflight (fail open). That bound falls on viola's own child only, never on a keystroke or the human: no key is blocked, refused or delayed by it.
  (.andromeda/a11y-plan.md:251)
  > every time bound viola holds — the `viola wait` timeout the caller passes, the MCP `wait` cap, the dialog hold, the `viola send` readiness gate, the confirmation windows, the status-line deadline, the `claude agents --json` read, the budget pause — falls on the driver, the user's own command or viola's own child, never on the human: no key is blocked, refused or delayed by it.
  (.andromeda/a11y-plan.md:344)
  > NEVER set harsh timeouts without warning (no warn-extend pattern per SC 2.2.1). v1 has none.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19) A refusal or failure exit the fix adds to `run`'s start or `viola verify` (a pin that cannot be completed) must print `unable  <reason>  <detail>` on stderr with `hint:` last and a typed exit code, one hint-free document under `--json`, and a fault no hint; its layout and exit code pinned per refusal.
  > **CLI:** `unable  <reason>  <detail>` on stderr, then `hint:` as the last stderr line, with a typed exit code. The one refusal with no hint is `unknown` (exit 14): its detail is opaque, so design prints no hint rather than a guess. Faults (`error: wrapper fault`, `error: internal error`) are not refusals and carry no hint. `--json` gives one document with no hint.
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:68)
  > **cli:** trycmd cases pin the `unable <reason> <detail>` + final `hint:` layout per refusal
  (.andromeda/a11y-plan.md:345)
  > NEVER show error without recovery suggestion (per SC 3.3.3). Every human-mode CLI refusal except `unknown` (exit 14, opaque detail, design cli) ends with a `hint:` line.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:90) Any new or changed fixed CLI string from the pin path (a refusal line, its hint, a `verify` step line) must be pinned by trycmd or by literal beside the existing `run` and `verify` pairs, and any new reason or detail code must sit beside the plain word `unable`, never a stack trace or bare status number alone.
  > the instance-state and home-state pairs as the six frame verbs and `run` print them, pinned by the `INSTANCE_PAIR` / `HOME_PAIR` literals in `tests/security_negatives_strict_modes.rs` and `tests/run_cli.rs` (`verify`'s home-state pair by literal in `tests/cli_verify.rs`). Any copy change therefore fails CI until the pinned string is updated on purpose.
  (.andromeda/a11y-plan.md:270)
  > - Every kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
  >   - The CLI never shows a stack trace or bare status number as the only error text.

## Bindings
- **scope-entities (the TUI passthrough boundary) ↔ cognitive-timeouts**: any wait or retry the pin fix adds must both stay off the terminal and off the keystroke path (.andromeda/a11y-plan-detail/a11y-scope-summary.md:43) and fall on viola's own start as one of its own bounds, never on the human (.andromeda/a11y-plan-detail/cognitive-accessibility.md:9).
- **scope-surfaces (tui diagnostics) ↔ scope-entities (the TUI passthrough boundary)**: a pin wait, retry or failure is reported only to `diagnostics/` or the wrapper log (.andromeda/a11y-plan-detail/a11y-scope-summary.md:109), which is what keeps viola's zero terminal bytes, witnessed by the P4 outer-PTY cases (.andromeda/a11y-plan-detail/aria-patterns-roles.md:19).
- **scope-entities (strict-modes refusal) ↔ cognitive-errors (strict-modes refusals and their cases)**: the home-tree refusal of `run`, a revive start and `verify` stays before any log and before the pin write (.andromeda/a11y-plan-detail/a11y-scope-summary.md:32), with its fixed pairs (.andromeda/a11y-plan-detail/cognitive-accessibility.md:37, .andromeda/a11y-plan-detail/cognitive-accessibility.md:43) held by the run and verify cases (.andromeda/a11y-plan-detail/cognitive-accessibility.md:81).
- **cognitive-errors (error recovery) ↔ cognitive-plain-language (pinned copy)**: a new pin refusal takes the `unable` / `hint:` form with a typed exit (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19), and its fixed text is pinned by trycmd or literal so a copy change fails CI (.andromeda/a11y-plan-detail/cognitive-accessibility.md:90).
- **path-p6-board (verify step lines) ↔ visual-sgr / scope-surfaces (no SGR, static output)**: `verify`'s `[NN/NN] … pass|fail` lines keep their order with `fail` / `unable` uncoloured (.andromeda/a11y-plan-detail/aria-patterns-roles.md:28), and no pin output adds SGR, cursor control or a redraw (.andromeda/a11y-plan-detail/visual-design-verification.md:9, .andromeda/a11y-plan-detail/a11y-scope-summary.md:87).
- **scope-entities (CLI verbs: verify's stop pair) ↔ cognitive-errors (error recovery)**: a pin wait inside `verify` interrupted by Ctrl-C or TERM still ends in the one fixed `unable:` / `hint:` pair with no step line (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8), the form the error-recovery pattern pins (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19).
