# The wrap's light gate (run dir `2026-10-04T22-27-20-wrap`) — two reds and their operator fixes

Both resolved on the overseer's founder-delegated word, relayed by the operator in the wrap session (2026-10-04):
"wait for the prompt receipt before each count in tests/cli_send.rs and tests/tui_wheel.rs. Show red-before-green on
the race (or name why it cannot be forced), then pre-push, the fix commit, push, the CI read, and resume at P7.1" ·
"a dated plan correction on the operator word, recorded in the report and the commit body. Keep both atoms. Show that
it still discriminates: run the new form once on a scratch copy holding one `:82` citation and see it read red, then
run it on the tree and see it green".

## 1. Entry `bash scripts/agent-run.sh run` — a receipt-count race in two new tests

- Light-gate reading: `red · exit 0 ✗ (exit 1)`; `cli_send::send_after_a_confirmed_send_is_turn_running_until_turn_ended`
  panicked at `tests/cli_send.rs:354` `nothing typed` — left 0, right 1. The same tree read green at /implement (two
  full-suite runs) and in five pre-push coverage runs.
- Cause (test-side): the fake agent writes a `prompt` receipt only after its UserPromptSubmit hook returns, and the
  hook delivers its `hook.event` (which confirms the send / appends `prompt-submitted`) before it returns. A count taken
  right after the send's reply, or right after the harness `prompt-submitted` line, can read 0.
  `tui_wheel::path5_harness_turns_never_take_the_wheel` counted the same way.
- Forced open (red before green): a temporary test-only hold — `std::thread::sleep(800 ms)` before the fake agent's
  `prompt` receipt write, never committed — then
  `run --integration --filter 'test(=send_after_a_confirmed_send_is_turn_running_until_turn_ended) |
  test(=path5_harness_turns_never_take_the_wheel)'`:
  - unfixed tests: exit 1, 2 run, 0 passed — `tests/tui_wheel.rs:266` left 0 / right 1 and `tests/cli_send.rs:354`
    left 0 / right 1 (the light gate's own signature);
  - fixed tests (each count now waits for at least one prompt receipt — `prompts_at_least` in `cli_send`,
    `fake::wait_for` in `tui_wheel`): exit 0, 2 passed.
  - Hold removed; `git diff --quiet src/bin/viola-fake-agent.rs` exit 0 (identical to `93a5cbf`).

## 2. Entry `git grep -F -e '`:82`' -- tests src | wc -l` — a plan defect

- Light-gate reading: `red · exit 0 ✗ (exit 1)` with `last line 0` ✓ — the gate shell runs `pipefail`, and `git grep`
  exits 1 on zero matches, so the `exit 0` atom could never hold on the green subject.
- Corrected (plan.md, entry 8, dated in its `note`): `{ git grep -F -e '`:82`' -- tests src || true; } | wc -l`, both
  atoms kept.
- Discrimination, the form run as the gate runs it (`bash -o pipefail -c`):
  - a scratch git repo holding one `` `:82` `` citation under `tests/`: exit 0 · last line 1 — red against `last line 0`;
  - the project tree: exit 0 · last line 0 — green;
  - the project tree, the old form: exit 1 · last line 0 — the defect reproduced.
