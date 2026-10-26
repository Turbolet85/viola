# a11y · ARIA Patterns & Roles — category read

## Carried
- (.andromeda/a11y-plan.md:182) The hook client's fail-open on a verification mismatch sits on a not-assertable surface: it must add no a11y assertion, and no a11y case may be written against `viola hook` or the IPC channel.
  > surfaces flagged not-assertable in a11y-scope Section 1 are skipped: plugin/IPC, internal crates
- (.andromeda/a11y-plan.md:191) The CLI verbs that now report the instance unreachable on a server mismatch, or on a check that cannot be made, must print that refusal in the P2 refusal layout on stderr with a typed exit code, held by trycmd.
  > **P2 — Confirmed send and readback, or refusal** (cli).
  >   - **cli:** `[RB]` mirror and `unable <reason> <detail>` + `hint:` via trycmd.
- (.andromeda/a11y-plan.md:193) `viola answer` gains the server verification before its frame; the P3 `unverified-cli` refusal text must still reach the CLI unchanged against a wrapper that passes the check.
  > **P3 — Dialog pending, then `answer`** (cli).
  >   - **cli:** `viola answer` refusal `unverified-cli` text on the CLI.
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:17) `viola send` gains the server verification before its frame; the P4 `human-typing` refusal on stderr with its typed exit code must still pass against a verified wrapper.
  > - **P4 — Human takes the wheel** (tui boundary + cli).
  >   - **cli:** `send` refused with `human-typing` on stderr, with a typed exit code (assert_cmd).
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:19) The P4 tui cases probe the wheel through an `answer` and a verified `send`; both clients now verify the server first, and the cases' exit codes and the key reaching the child must hold on all three OS legs.
  > the wheel probed by an `answer` to no pending dialog (exit 13 driver, 10 human)
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:19) A human key during a verified `send` must still take the wheel with nothing typed; the verification adds no block or delay on the human path.
  > A further case sits under clause (2): a key written while a verified `send` waits in the readiness gate takes the wheel, the send exits 10 `human-typing` with nothing typed and the key reaches the child (the same Tooling paragraph, with its stated limit).
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:26) `viola wait` / `viola last` gain the server verification before their frames; P6's linear static output must hold, the verification adding no stderr line, redraw or spinner, with `waiting: <name>` still exactly once under the outer PTY.
  > - **P6 — CLI board, wait/last, verify** (cli). No ARIA. trycmd / assert_cmd assert the non-TTY modes, and the portable-pty outer PTY asserts the TTY-only behaviours, on all three OS legs:
  >   - linear static output: under the outer PTY, `waiting: <name>` appears exactly once on stderr and the stream contains no `\r` redraw, cursor-movement or erase sequence (no spinner); under non-TTY the `waiting:` line is absent;
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:33) The `wait` / `last` C0/C1 escaping cases run against a `viola-fake-agent` wrapper, which the verified clients must pass, so the escaping assertions keep running and passing.
  > - Setup: human-mode `viola wait` / `viola last` run against a `viola-fake-agent` event whose text carries C0 and C1 control bytes (including ESC / CSI)
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:35) The non-TTY and outer-PTY pass conditions for `wait` / `last` output must still hold after the verification lands.
  > Under non-TTY, `wait` / `last` stdout contains no raw C0 / C1 byte other than `\n` / `\t`, and `list` stdout contains none inside a row.

## Bindings
- **P2 refusal layout ↔ the body's §8 Error recovery**: the unreachable refusal a mismatch produces must take the `unable <reason> <detail>` + final `hint:` layout §8 pins for every human-mode refusal, with a hint-free `--json` document; (.andromeda/a11y-plan.md:191).
- **Not-assertable hook surface ↔ security.md / CLAUDE.md fail-open invariant**: the hook's fail-open on mismatch (exit 0, nothing on stdout or stderr, no frame) is verified by the tests plan, not by an a11y case; (.andromeda/a11y-plan.md:182).
- **P4 wheel cases ↔ the body's §11 Keyboard ban**: the verification sits on the automation clients only and must never block, refuse or delay a human keystroke; (.andromeda/a11y-plan-detail/aria-patterns-roles.md:19).
- **P6 wait/last output ↔ test-plan's CLI suites**: the verified `wait` / `last` keep their output-discipline cases inside `nextest-integration` / `nextest-e2e` (CI: `coverage`) on all three OS legs; (.andromeda/a11y-plan-detail/aria-patterns-roles.md:26).
