# Red first, then green — the send half (steps 1 to 4)

Times are `date -u`, 2026-10-07. Every run is the harness command as the plan lists it, on the Linux dev host,
under the fake agent. No `claude` process was started (inputs#I1, inputs#I4).

The remedy these cases prove is PROVISIONAL: the overseer's delegate answer of 2026-10-07 on the P4 card
(inputs#I2), the founder's to confirm or overturn. `send` validates the text as received and types it without
its trailing LF characters; the list, the paste, `text_bytes` and the exact claim read that one typed text.

## The tree at the red readings
Steps 1, 3 and 4 were written and step 2 was not: `typed_text` and its nine-case table stood in
`crates/viola-agent-claude/src/hook.rs`, the seven send tests in `src/run/send.rs` and the cross-process case in
`tests/cli_send.rs`, and `send` still typed the text as received. The one existing case
`a_trailing_newline("/clear\n")` had already left its table.

## Unit, red — 14:23:26Z
`bash scripts/agent-run.sh run --unit --filter 'test(/trailing_newline/)'` → exit 1, `"ok":false`, suite
`nextest-unit` passed 10, failed 6, archived `target/run-archive/729`.

Passed, 10: the nine cases of `hook::tests::typed_text_drops_every_trailing_newline_and_nothing_else` (the rule
exists) and `send_a_refused_character_before_a_trailing_newline_is_control_character`. That last case cannot be
read red by removing step 2: LF is an allowed character, so the refusal is the same whether the text is
validated before or after the newlines are taken off. It pins the outcome, not the order.

Failed, 6, each on the value the paste recorded (`left` is what was typed, `right` what the case expects):

| case | first failed assertion | left | right |
|---|---|---|---|
| `send_trailing_newlines_are_not_typed_and_the_send_is_confirmed::case_1_one` | `src/run/send.rs:885` | `["canary-chain-value-5c1e\nsecond line\n"]` | `["canary-chain-value-5c1e\nsecond line"]` |
| `…::case_2_two` | `src/run/send.rs:885` | `["canary-chain-value-5c1e\nsecond line\n\n"]` | `["canary-chain-value-5c1e\nsecond line"]` |
| `send_a_prompt_that_keeps_the_trailing_newline_claims_nothing` | `src/run/send.rs:966` | `["canary-chain-value-5c1e\nsecond line\n"]` | `["canary-chain-value-5c1e\nsecond line"]` |
| `send_local_command_decision_reads_the_text_without_its_trailing_newline` | `src/run/send.rs:1528` | `["/clear\n"]` | `["/clear"]` |
| `send_local_command_without_a_post_condition_and_a_trailing_newline_is_unconfirmable` | `src/run/send.rs:1551` | `["/remote-control\n"]` | `["/remote-control"]` |
| `send_local_command_clear_with_a_trailing_newline_is_confirmed_by_a_new_session` | `src/run/send.rs:1582` | `["/clear\n"]` | `["/clear"]` |

Each case ends by its own assertion and none parks: the cases that wait for a prompt or a new session run on a
clock that stands still until the hook's line is on disk and jumps after it, so an unconfirmed send expires.

## Cross-process, red — 14:23:45Z
`bash scripts/agent-run.sh run --integration --filter 'test(/ending_in_newlines/)'` → exit 1, `"ok":false`, suite
`nextest-integration` passed 0, failed 2, archived `target/run-archive/730`. Both cases failed at the receipt's
bytes, `tests/cli_send.rs:306`, after `viola send --json` had already exited 0 with an `ok` document:

- `case_1_one`: `a newline was typed last:` then the receipt's `hex`, which ends `…6c696e652074776f0a`;
- `case_2_two`: the same message, the `hex` ending `…6c696e652074776f0a0a`.

This is the reading that tells the remedy from its absence. Under the fake agent the exit code and the records
are a floor: it echoes what it is typed, so a text ending in a newline was confirmed before the build too.

## Green after step 2
- 14:24:12Z, the unit filter → exit 0, `"ok":true`, `nextest-unit` passed 16, failed 0 (`target/run-archive/731`).
- 14:24:14Z, the integration filter → exit 0, `"ok":true`, `nextest-integration` passed 2, failed 0
  (`target/run-archive/732`).

## The rule's own table, with the rule neutralised — 14:31:39Z
The nine-case table calls `typed_text` directly, so its control removes the rule inside that function: its body
returned the text unchanged. The unit filter then read exit 1, passed 6, failed 10: the four cases of the table
that take a newline off (`one`: left `"x\n"`, right `"x"`; `three`: `"x\n\n\n"` / `"x"`;
`a_cr_before_it_stays`: `"x\r\n"` / `"x\r"`; `only_newlines`: `"\n\n"` / `""`) and the same six send cases as
above. The five table cases that expect the text unchanged passed, as they must.

The body was restored (`text.trim_end_matches('\n')`, read back at `hook.rs:204`) and the filter read exit 0,
passed 16, failed 0 at 14:31:51Z (`target/run-archive/739`).

## What the cross-process case reads beyond the plan's list
The acceptance sentence says a delivered send of a text ending in a newline prints the `[RB] read back` line.
The case's second send, in human mode, therefore ends in the same newlines as its first, so the sentence is
true as written of the send that prints the line. The case also asserts that no `wheel` record follows the
first send's cursor before the second send, and that the receipt's second prompt is the second typed text.

## A name the plan did not see
`hook.rs`'s test module already held a proptest strategy named `typed_text` (`hook.rs:718`, used at `:815`),
which shadows the new function inside that module. The table calls `super::typed_text`; the strategy and every
existing case are untouched.
