# arch extract

## Relevance
relevant — the chunk's code sits in the root bin's human writers, the clap definition and the `run` pump's outer-terminal path, and its colour clause runs into arch's exhaustive environment-variable rule.

## Constraints
- architecture §Cross-cutting Patterns → Config management requires that no `viola` build reads any environment variable beyond the `VIOLA_*` plumbing and the two `cfg(feature = "fake-agent")` test seams, and says "a dependency or feature that reads an environment variable breaks this rule like a direct read does". Reading `NO_COLOR` / `TERM` / `COLORTERM` for the colour decision therefore changes that sentence. It is a boundary widening (CARRY 2) and needs a Decisions Log entry and the founder's own ratification before it ships. It is not a lean.
- architecture §Stack and Technologies (the CLI parser row) requires clap 4.6.7 with `default-features = false` and the feature set `std`, `derive`, `help`, `usage`, `error-context`, `suggestions`, "its default set minus `color`", so that help and usage output stays plain and no colour or terminal crate is in the graph. The grouped `--help` has to be built inside that feature set. Turning `color` back on, or adding anstream / anstyle-query, is the env-read route that §Cross-cutting Patterns → Config management records as closed at chunk 2026-09-28-cli-output-tokens.
- architecture §Cross-cutting Patterns → Diagnostic output channels requires that, while the child runs, `run` writes nothing to the terminal except the child's own output. Short-lived CLI verbs may put their human output on stderr. This is the arch basis for the chunk's zero-wrapper-bytes clause. It also covers any colour or VT-enable sequence and any human line on `run`'s outer terminal. Whether the pump already holds this byte for byte is research's question.
- architecture §Established Decisions [Human Takeover / Wheel] makes several requirements:
  - A bridge send is one `ESC[200~…ESC[201~` + CR write.
  - Human bytes that arrive during that write are held in the pump and passed on straight after.
  - The stdin observer returns every byte whole and unchanged.
  - `send` reads the wheel and the running turn a second time once the readiness gate's wait returns. A key typed during that wait is refused `human-typing` (exit 10) with no `send-issued` and nothing typed. The gate's wait is not interrupted when the wheel moves, so the driver hears the refusal when the wait ends.

  The chunk's unblocked-keys case has to assert exactly this ordering. A test that expects the send to end early does not.
- architecture §Established Decisions [PTY] requires that viola never holds, queues or re-orders a human key behind a resize. It also requires `HostTerminal` to save the host terminal's console modes / termios and restore them on Drop. A Windows VT-output enable for colour on a non-`run` verb must keep the same save-and-restore discipline. It must be its own mode set, never viola-pty's raw/VT mode (CARRY 2: `ENABLE_VIRTUAL_TERMINAL_PROCESSING` only, never `0x000c`).
- architecture §Infrastructure Patterns → Crate dependency direction places the colour layer, the help groups and the escaper in the `viola` root bin (`src/human.rs`, `src/human/board.rs`, the clap definition). viola-pty "knows no agent" and takes no human-output role. The root bin's windows-sys use is enumerated per feature (`Win32_System_Console` is there today for `SetConsoleCtrlHandler`). Whether a console-mode read/write needs a further windows-sys feature or a manifest change is research's question. Any change gets recorded in that key.
- architecture §Cross-cutting Patterns → Untrusted upstream text requires a driven session's `Stop` text and dialog payloads to be forwarded as escaped strings and never interpreted. That is the arch basis for `viola wait` / `viola last` human-mode text showing every C0 / C1 byte other than `\n` / `\t` escaped. Whether those verbs already escape is research's question.

## Patterns to follow
- The `viola verify` human output shape is fixed in architecture §Established Decisions [CLI Version Compatibility]. Verify prints one stdout step line per relied row, `[NN/MM] <row id> <row words>  pass|fail`, where `MM` is `LedgerRow::relied().len()` (23 on Unix, 20 on Windows). Then it prints the `stamped <ver>  N pass  M fail` summary. The chunk's appended-step-lines clause reads against this. It does not invent a new form.
- Refusal text on stderr is the fixed `unable:` / `hint:` pair with no path, pid or id (architecture §Conventions, CLI exit codes: `run`'s collision / pinned-copy / batch-script / squatted-name pairs, `revive`'s strict-modes and preflight pairs, `verify`'s refusal pairs and its interrupted pair). The `cli` role's internal error is exactly `error: internal error` once, from the `main` catch site, and no dispatch arm prints it. `send`'s `input-not-ready` line is the founder's wording in `send_hint` (`src/human/hints.rs`), per §Established Decisions [CLI Version Compatibility]. These pairs are the stderr writers CARRY 1's obs-plan sentences have to name, and colour never touches them (`unable` / `fail` carry no SGR).
- `--home` per test home is the isolation point (architecture §Established Decisions [CI/CD]; §Cross-cutting Patterns → Config management). Outer-PTY byte checks run against the fake agent named after `--`, with one home per test.
- The measurement that closed clap's env route is recorded in architecture §Cross-cutting Patterns → Config management: `CLICOLOR_FORCE=1 viola --help` into a pipe gave 0 ESC bytes after the change. That is the existing witness shape for a help that stays SGR-free once its group labels land.

## Anti-patterns to avoid
- Re-enabling clap's `color` feature, or adding any crate or feature that reads `CLICOLOR` / `CLICOLOR_FORCE` / `NO_COLOR` / `TERM` / `COLORTERM` / `CI`, to get colour or help styling. It breaks the exhaustive env rule just as a direct read does (architecture §Cross-cutting Patterns → Config management; §Stack and Technologies, the CLI parser row).
- Any viola-originated byte on `run`'s outer terminal while the child runs (a status line, a colour enable, a refusal echo), and any hold, queue or re-order of a human key beyond the current atomic paste (architecture §Cross-cutting Patterns → Diagnostic output channels; §Design Philosophy, "The human always wins the wheel").
- Writing human output with `print!` / `eprintln!` instead of going through the human writers. The workspace bans `print_stdout` / `print_stderr` / `dbg_macro` (architecture §Stack and Technologies, the Code quality row).

## Contract bindings
- The arch env sentence (architecture §Cross-cutting Patterns → Config management) binds to security-plan Secret Management. The `NO_COLOR` / `TERM` / `COLORTERM` widening amends both, under one Decisions Log entry and the founder's ratification (CARRY 2).
- The arch stderr discipline (architecture §Cross-cutting Patterns → Diagnostic output channels; §Conventions, the CLI exit codes `unable:` / `hint:` pairs) binds to the obs-plan §3 intro and §11 Logs "only human-stderr writer" sentences (CARRY 1). The wrap's amendments should name the writers the chunk leaves.
- The arch wheel ordering (architecture §Established Decisions [Human Takeover / Wheel]) binds to the a11y-plan P4 cases and to the tests' outer-PTY harness (`tests/tui_wheel.rs`, `tests/support/outer_pty.rs`).
- architecture §Established Decisions [PTY] records two things the a11y-plan P4 Windows check depends on:
  - `tests/support/outer_pty.rs` spawns on the inbox ConPTY.
  - The ConPTY host itself writes spawn preambles: inbox `ESC[?9001h ESC[?1004h`, sideloaded `ESC[1t ESC[c ESC[?1004h ESC[?9001h`.

  So "viola's own literals absent on Windows ConPTY" has to set those host bytes apart from viola's.
- The clap feature set (architecture §Stack and Technologies) binds to the layout-templates `--help` and the design-system §Tokens (platform-specific): the group labels are text, never SGR.

## Acceptance criteria contributions
- No new environment-variable read in any `viola` build, and no colour or terminal crate (anstream / anstyle-query, or clap `color`) in the root graph, unless a Decisions Log entry and the founder's ratification of the `NO_COLOR` / `TERM` / `COLORTERM` widening are recorded, with the env sentence amended to name them (per architecture §Cross-cutting Patterns → Config management).
- Under `viola run` against the fake agent, the outer-PTY stream holds no viola-originated byte. On Linux / macOS it matches an unwrapped fake-agent run byte for byte; on Windows ConPTY, viola's own literals are absent (per architecture §Cross-cutting Patterns → Diagnostic output channels).
- A human key written while a verified `send` waits in the readiness gate reaches the child unchanged. The send ends `human-typing` (exit 10) with no `send-issued` line and nothing typed (per architecture §Established Decisions [Human Takeover / Wheel]).
- Every `--json` document and every typed exit code stays byte-identical and SGR-free. The grouped `viola --help` holds zero ESC bytes even with `CLICOLOR_FORCE=1` into a pipe, and the chunk's code lives in the root bin with no new viola-crate edge (per architecture §Established Decisions [CLI Conventions]; §Stack and Technologies, the CLI parser row; §Infrastructure Patterns → Crate dependency direction).
