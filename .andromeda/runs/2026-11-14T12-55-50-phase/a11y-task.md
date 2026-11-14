@task_prose part=chunk
Test-only: pins the one missing case on both MAX_FRAME bounds of the viola-hyprland seam — a reply of exactly MAX_FRAME bytes is accepted, one byte more is oversize.
The bounded read (read_within) is driven through the fake hyprctl with exactly MAX_FRAME bytes (not oversize) and MAX_FRAME + 1 bytes (oversize).
The dispatch parse (parse_dispatch) is given exactly MAX_FRAME bytes (Refused, not oversize) and MAX_FRAME + 1 bytes (oversize).
Each new guard test carries a one-off remove-the-guard control (the site mutated > to >=, the case named by the test alone; it holds when that case ran and failed), so the next mutation audit catches both survivors; no product line changes unless a test proves one wrong.
The parse_clients bound is already pinned by its existing exact-frame test and is not re-pinned.
@task_file n=1 path=crates/viola-hyprland/src/read.rs mark=edits
what: the Hyprland seam's bounded read of a hyprctl reply, an internal crate's code and its in-crate tests — edits: its test module, a new exact-frame test of read_within through the fake hyprctl
@task_file n=2 path=crates/viola-hyprland/src/reply.rs mark=edits
what: the Hyprland seam's reply parsing (parse_dispatch, parse_clients), an internal crate's code and its in-crate tests — edits: its test module, a new exact-frame test of parse_dispatch
@task_item n=1 part=needs
The viola-hyprland crate is an internal crate: its MAX_FRAME bounds are invariants held in crate tests, not user-facing output, so the new tests are crate tests rather than a11y assertions.
@task_item n=2 part=needs
The mutation audit covers invariants held in crate tests: the chunk closes the audit's two missed mutants (> to >= on each MAX_FRAME bound), and each new test must be one that kills that mutant.
@task_item n=3 part=needs
The new tests run inside the existing test suites under nextest; a failure of one is a nextest failure that fails the build, with no separate driver or suite value.
@task_item n=4 part=needs
The CI test gate allows no skips: neither new test may be ignored, skipped or gated off in CI.
@task_item n=5 part=needs
Zero flakiness, no retries: the exact-frame cases must be deterministic, the read's case driven only through the fake hyprctl with no live desktop, no real hyprctl and no live claude start.
@task_item n=6 part=needs
Live readings never gate: nothing measured on a live compositor enters the tests; the bounded read's behaviour is replayed by the fake.
@task_item n=7 part=needs
The gate requires coverage and doctest: the new tests run in the coverage suite the gate reads and must leave it green.
@task_item n=8 part=needs
The oversize refusal and the Refused reply are asserted by their error variant (HyprlandError::Oversize, Refused), with no change to any error text or code word the seam carries.
@task_item n=9 part=needs
The bounded read's time bound stays on the driver: the exact-frame case through the fake hyprctl must stay inside the read's existing deadline and add no wait that falls on a human.
@task_standing n=1
convention: every new test runs in the existing nextest suites inside the coverage run, and its failure fails the build
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: every new test is deterministic, never retried and never skipped in CI
home: 10-slo-invariants-a11y-budgets
@task_standing n=3
convention: every new test passes the gate's coverage and doctest requirement
home: ci-integration
@task_item n=1 part=not_needed
CLI output contracts — refusal unable/hint lines, --json failure documents, exit codes, list board, verify, wait, link, revive and help output: the chunk prints nothing and changes no CLI surface.
@task_item n=2 part=not_needed
TUI passthrough contracts and the keyboard-test harness rules on human keystrokes, terminal bytes and the wheel: the chunk touches no PTY or terminal path.
@task_item n=3 part=not_needed
Colour and SGR rules, NAME truncation, stale-row dim and visual design: no rendered output is added or changed.
@task_item n=4 part=not_needed
Screen-reader targets, manual Orca and keyboard passes and their test setups: no live desktop step and no human-facing surface in this chunk.
@task_item n=5 part=not_needed
config.json keys, the budget gate, dialog hold, the statusline hook and the MCP tools: the chunk reaches none of them.
@task_item n=6 part=not_needed
Fixed CLI copy, hint wording and reading-level rules of cognitive accessibility: no string a user reads is added or changed.
@task_item n=7 part=not_needed
Absences (web or graphical frontend, touch surface, internationalization, rendered page, user time limit): nothing in the chunk adds such a surface.
@task_item n=8 part=not_needed
The structured violation JSON rows and any a11y artifact of its own: the chunk adds no a11y case and no artifact.
