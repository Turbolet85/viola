# a11y · ARIA Patterns & Roles — category read

## Carried
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:26) The moved refusal documents and hints of `viola list`, `viola wait`, `viola last` and `viola verify` must stay asserted by these drivers, in these modes, on every OS leg.
  > trycmd / assert_cmd assert the non-TTY modes, and the portable-pty outer PTY asserts the TTY-only behaviours, on all three OS legs:
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:30) Every new code document and every cause's hint must print with no SGR in these modes.
  > no SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json`;
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:31) A per-cause refusal line must keep its `unable` word unstyled, including under a terminal.
  > and `unable` / `fail` carry no SGR;
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:32) Any session name a cause's hint prints, the unwrapped-name hint's `<name>` first, must reach the stream escaped under this rule, with its witness on all three OS legs.
  > C0/C1 escaped (security excerpt, C0/C1 escaping; ratified T5 for `list` rows), checked with assert_cmd on all three OS legs:
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:36) The exit-cause table's stderr and `--json` assertions must stand as output-discipline stand-ins only, and the chunk must claim no conformance from them.
  > These stand in for SC 1.3.2, 1.4.1 and 3.3.1, with no conformance claim.
- (.andromeda/a11y-plan.md:29) The entity the chunk moves is these verbs in these three modes, so each built cause must be assertable on text and streams in the modes its verb serves.
  > **Entity:** CLI verbs (`list`, `send`, `wait`, `last`, `answer`, `verify`, `pause`, `release`, `link`, `unlink`, `revive`, `plugin install`, `--help`) in human TTY, plain (non-TTY / `NO_COLOR` / `TERM=dumb`) and `--json` modes — assertable: output-discipline assertions on text and streams only; no automated a11y tool reaches a terminal.
- (.andromeda/a11y-plan.md:31) This owed item is named to this chunk: each refusal `run`'s start makes after a passed preflight must print its document under `viola revive --json`, and the owed line must retire when it does.
  > - **Owed:** under `viola revive --json`, a refusal `run`'s start makes after a passed preflight prints no document yet — owed to "Exit-cause code catalogue".
- (.andromeda/a11y-plan.md:53) A refusal after a passed preflight must hold this surface's rules like any other, and each hint must stay plain, static, linear text with its controls escaped.
  > colour is only a second cue; no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`; output is static and linear; C0/C1 control characters are escaped except `\n` and `\t`, which `list` rows escape too.
- (.andromeda/a11y-plan.md:55) A log form given to a `run` start cause must land in `diagnostics/`, and no new code or line may add a viola byte to a terminal the child holds.
  > boundary assertions on the portable-pty outer-PTY driver check that viola emits zero terminal bytes of its own and never blocks or delays human keystrokes; viola's diagnostics for `run` go to `diagnostics/`, never to the terminal
- (.andromeda/a11y-plan.md:107) Each distinct cause must reach the reader as its reason and detail in text, in a linear order, with colour never the only sign of which cause it is.
  > Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text.
- (.andromeda/a11y-plan.md:186) `tests/cross_exit_causes.rs` must assert within this automated reach and on all three OS legs, as agent-runnable output.
  > | cli | assertable (output discipline, no conformance claim) | assert_cmd 2.2.2 + predicates 3.1.4 + trycmd 1.2.1; portable-pty outer-PTY SGR checks; all three OS legs |
- (.andromeda/a11y-plan.md:192) `viola send`'s exit-21 causes fall on this path scenario, so each must keep the `unable` plus `hint:` layout under its pin.
  > **cli:** `[RB]` mirror and `unable <reason> <detail>` + `hint:` via trycmd.
- (.andromeda/a11y-plan.md:257) Every cause's human form must keep this layout, with its own hint as the last stderr line, and the revive and verify exit-1 pairs must keep their one `unable:` line.
  > on the CLI, `unable  <reason>  <detail>` on stderr (`viola revive`'s exit-1 refusals and `viola verify`'s interrupted pair: one `unable: <text>` line), then `hint:` as the last stderr line, with a typed exit code;
- (.andromeda/a11y-plan.md:257) `--json` documents must stay hint-free, and the table's per-cause `hint:` assertion must be reconciled with the fault carve-out for any cause that is a fault and not a refusal, `internal-error` the one to read.
  > faults are not refusals and carry no hint; `--json` gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths (a fixed stderr line, no document).
- (.andromeda/a11y-plan.md:261) Each cause's layout, exit code and hint-free `--json` must be pinned to this verification, with the server-verification row holding the two-leg carve-out.
  > trycmd pins the `unable <reason> <detail>` + final `hint:` layout per refusal and assert_cmd the typed exit codes and a hint-free `--json`, on all three OS legs (the frame verbs' server-verify refusal on windows-2025 and ubuntu-latest only: macOS reports no peer pid).
- (.andromeda/a11y-plan.md:273) Every new or reworded hint, the unwrapped-name hint first, is fixed CLI copy and must be pinned so a copy change fails CI.
  > every fixed CLI string is pinned, by trycmd or, for `viola list`, `viola --help`, `viola plugin install`, the server-verify hint and the `viola link` / `viola unlink` result lines, by literal, so any copy change fails CI until the pinned string is updated on purpose.
- (.andromeda/a11y-plan.md:312) The table's cases must pass inside the existing suites on all three OS legs as part of these invariants.
  > The CLI output-discipline invariants (P4, P6) pass on all three OS legs inside `nextest-integration` / `nextest-e2e` (CI: the `coverage` suite); where they live in crate tests, the epoch-boundary mutation audit (`run --mutants`, test-plan §10) covers them.
- (.andromeda/a11y-plan.md:317) The new table must run under this rule, and the chunk's watched margin must be read under it, no retry and no ignored case.
  > the tests' zero-flakiness rule (`retries: 0`; a flake keeps the chunk red until the root cause is fixed; `#[ignore]` banned)
- (.andromeda/a11y-plan.md:330) No card, test name or plan amendment this chunk writes may claim conformance from the table.
  > NEVER claim WCAG conformance for the cli or the `viola run` TUI (the wrapper-hosted TUI, whether `viola run` or a passed `viola revive` started it). assert_cmd / trycmd / portable-pty results are output-discipline stand-ins for SC 1.3.2 / 1.4.1 / 3.3.1, not a11y-tool evidence.
- (.andromeda/a11y-plan.md:340) The ban holds on every refusal line and document the chunk adds or moves, a post-preflight `viola revive` refusal included.
  > NEVER colour CLI `unable` / `fail`, and never emit SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`.
- (.andromeda/a11y-plan.md:345) Every human-mode refusal among exits 21 and 1 must end with its `hint:` line, because `unknown` is the only excepted refusal and this chunk does not reopen it.
  > NEVER show error without recovery suggestion (per SC 3.3.3). Every human-mode CLI refusal except `unknown` (exit 14, its detail a code word or nothing, design cli) ends with a `hint:` line.
- (.andromeda/a11y-plan.md:346) Each new closed code a human reads must sit beside the plain word `unable`, never bare.
  > NEVER use jargon without a plain alternative. SC 3.1.5 AAA is not claimed, but a kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) never appears without the plain word `unable`.
- (.andromeda/a11y-plan.md:347) The exit-1 internal-error cause must print a code and fixed text, never a stack trace as its only error text.
  > NEVER render a stack trace as the only error text in the CLI.
- (.andromeda/a11y-plan.md:359) `tests/cross_exit_causes.rs` must run under the existing harness, adding no suite value and no second driver.
  > NEVER add a `suite` value (such as `a11y`) or a separate a11y driver outside `scripts/agent-run.* run`.

## Bindings
- **Post-preflight refusal document ↔ the chunk's scope and `verification-matrix.json#v1-39`**: the plan owes the document to this entry and the scope claims the capability with that gap closed; both must retire together, (.andromeda/a11y-plan.md:31)
- **Per-cause refusal layout and pins ↔ the plan's Cognitive Accessibility category and design-system's CLI refusal pattern**: the layout, the hint-last rule and the pinned copy stand in the body's error-recovery and plain-language lines, and a new hint moves both sides, (.andromeda/a11y-plan.md:257) (.andromeda/a11y-plan.md:273)
- **Fault carve-out ↔ test-plan's Exit-cause matrix**: the matrix asks a last `hint:` line for each cause and this plan says a fault carries none; `internal-error`'s row must hold one reading on both sides, (.andromeda/a11y-plan.md:257)
- **SGR bans ↔ the plan's Visual Design Verification category and design-system's colour rules**: no SGR in the listed modes, and `unable` never coloured, on both sides, (.andromeda/a11y-plan-detail/aria-patterns-roles.md:30) (.andromeda/a11y-plan.md:340)
- **Escaping of a printed name ↔ security-plan's C0/C1 escaping**: the same rule binds a hint that prints a name and the plan's output discipline, (.andromeda/a11y-plan-detail/aria-patterns-roles.md:32)
- **Log form of a `run` start cause ↔ `schemas/diag-line.v1.json` and the observability plan's `process-exit` details**: a new closed detail goes to `diagnostics/` under that schema and never to the terminal, (.andromeda/a11y-plan.md:55)
- **Table in the existing suites ↔ test-plan's harness contract and zero-flakiness rule**: no new suite value, no retry, no ignored case, (.andromeda/a11y-plan.md:359) (.andromeda/a11y-plan.md:317)
