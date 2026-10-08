# The hint line, pins first (step 1, 2026-10-08)

The line, as the founder ruled it (inputs#I7):
`{name} was not ready for input; send again, and if it repeats a human must look at the session`

| order | tree | command | reading |
|---|---|---|---|
| 1 | the two pins changed (`src/human.rs`, the expected stderr of `write_send_unable_puts_the_hint_last_in_one_write` and the `input_not_ready` case of `send_hint_is_the_design_string`), the producer not | `bash scripts/agent-run.sh run --unit` (06:33Z) | exit 1, `"ok":false`; suite `nextest-unit` 1368 passed, 2 failed, 0 skipped. The two failed: `human::tests::send_hint_is_the_design_string::case_2_input_not_ready` and `human::tests::write_send_unable_puts_the_hint_last_in_one_write` |
| 2 | the producer changed too (`send_hint`, the `input-not-ready` arm) | the fence's unit entry, through the gate tool (06:35Z) | exit 0, `"ok":true`; suite `nextest-unit` 1370 passed, 0 failed, 0 skipped |

The census after the producer changed: `grep -c 'was not ready for input; viola wait' src/human.rs` prints 0
(exit 1); `grep -c` of the new line prints 3 (exit 0). The forecast was 3 → 0 and 0 → 3.

Nothing else in the file changed: no code, no exit, no detail, no other hint. `cargo fmt --all --check` exits 0;
the formatter put the producer's `format!` on three lines, the literal whole on one.
