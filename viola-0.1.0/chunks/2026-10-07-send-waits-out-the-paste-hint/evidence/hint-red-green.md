# The `hint` case, red before the fix and green after

The pair is `send_under_the_paste_hint_on_a_verified_cli` in `tests/cli_send.rs`, cases `hint`
(`--paste-hint-ms 3000`) and `no_hint`. Step 1 turned the `hint` case's expectation to delivered: exit 0,
`ok.cursor` equal to the end offset read before the second send, two prompts in the receipt, the second prompt's
text equal to the second send's text, no `send-refused` record. The hold and the `no_hint` case are as they were.

Command, both readings:
`bash scripts/agent-run.sh run --integration --filter 'test(/send_under_the_paste_hint_on_a_verified_cli/)'`

## Red: the turned case against the untouched product

Read 2026-10-07T12:22:46Z to 12:22:52Z (`date -u`). The tree: HEAD `56e67bb33e63` with `tests/cli_send.rs` as the
only changed file under `src/`, `crates/`, `tests/` and `.config/` (`git status --short`).

The harness document: exit 1, `"ok":false`, suite `nextest-integration` passed 1, failed 1, failures
`["viola::cli_send send_under_the_paste_hint_on_a_verified_cli::case_1_hint"]`.

The nextest status lines, verbatim:

```
        PASS [   4.083s] (1/2) viola::cli_send send_under_the_paste_hint_on_a_verified_cli::case_2_no_hint
        FAIL [   4.128s] (2/2) viola::cli_send send_under_the_paste_hint_on_a_verified_cli::case_1_hint
     Summary [   4.138s] 2 tests run: 1 passed, 1 failed, 337 skipped
```

The failing assertion, verbatim:

```
    thread 'send_under_the_paste_hint_on_a_verified_cli::case_1_hint' (34814) panicked at tests/cli_send.rs:575:5:
    assertion `left == right` failed: stdout: {"v":1,"refusal":"not-delivered","detail":"input-not-ready"}

      left: Some(13)
     right: Some(0)
```

So on the untouched product the second send under the held hint exits 13 with `not-delivered` /
`input-not-ready`, 4.1 s into the case, well before the 3 000 ms hold has ended.

## Green: the same case after the fix, with the keystroke case beside it

Read 2026-10-07T12:28:15Z to 12:28:23Z (`date -u`), after steps 2 to 4 and step 10. Command:
`bash scripts/agent-run.sh run --integration --filter 'test(/send_under_the_paste_hint/)'`, which selects the pair
and the new `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`.

The harness document: exit 0, `"ok":true`, suite `nextest-integration` passed 3, failed 0.

The nextest status lines, verbatim:

```
        PASS [   4.092s] (1/3) viola::cli_send send_under_the_paste_hint_on_a_verified_cli::case_2_no_hint
        PASS [   7.085s] (2/3) viola::cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins
        PASS [   7.089s] (3/3) viola::cli_send send_under_the_paste_hint_on_a_verified_cli::case_1_hint
     Summary [   7.090s] 3 tests run: 3 passed, 337 skipped
```

Durations: `hint` 7.089 s (4.128 s when it was refused: the send now waits out the 3 000 ms hold and is
delivered), the keystroke case 7.085 s, `no_hint` 4.092 s. Both held cases are under the nextest `mutants`
profile's 10 s kill and the CI profile's 20 s kill for `binary(cli_send)`.

## One reading on the way: the key receipts

The keystroke case's first run (12:27:35Z to 12:27:43Z) was red on its last receipt assertion and green on every
product assertion before it (exit 10, the `human-typing` document, `wheel` then `send-refused`). The receipt's
`key` lines read `["0d", "6b"]` where the case expected `["6b"]`: the fake agent receipts the Enter that submits
a prompt as a `key` line of its own (`src/bin/viola-fake-agent.rs`, `Input::plain`), so the long send's own Enter
stands first. The case now waits for the `6b` line and asserts both, in order. The product was not changed
between the two runs.
