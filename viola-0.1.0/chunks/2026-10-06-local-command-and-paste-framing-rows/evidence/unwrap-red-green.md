# Step 1 — the unwrap's framing newlines, red before green

Both readings are the same command, run from the repository root on the Linux dev host:

```
bash scripts/agent-run.sh run --unit --filter 'test(/prompt_text_/)'
```

It selects 20 tests in `viola-agent-claude`: the eight new cases of
`prompt_text_drops_the_cli_framing_around_a_pair`, the eleven cases of
`prompt_text_unwraps_the_cli_pair_then_unescapes` (its `cli_pair` case moved to the new expected text) and the
reworked property `prompt_text_prop_round_trips_a_wrapped_paste`.

## Red — the cases written first, `unwrap_pastes` untouched (its HEAD `2fbc954` body)

Started 2026-10-06T20:21:11Z. Exit 1, `"ok":false`, `"suite":"nextest-unit","passed":12,"failed":8`.
nextest: `20 tests run: 12 passed, 8 failed, 1282 skipped`.

| test | reading |
|---|---|
| `…around_a_pair::case_1_measured_long` | FAIL — left `"\n\nA long paste\n"`, right `"A long paste"` |
| `…around_a_pair::case_2_measured_tag` | PASS — step 0's tag prompt holds no pair; the un-escape alone returns the text |
| `…around_a_pair::case_3_third_newline_before_stays` | FAIL |
| `…around_a_pair::case_4_second_newline_after_stays` | FAIL |
| `…around_a_pair::case_5_one_newline_before_stays` | FAIL |
| `…around_a_pair::case_6_text_then_framed_pair` | FAIL |
| `…around_a_pair::case_7_two_framed_pairs` | FAIL |
| `…around_a_pair::case_8_unmatched_open_keeps_its_newlines` | PASS — no matched pair, nothing is removed before or after the change |
| `…then_unescapes::case_01_cli_pair` | FAIL |
| `…then_unescapes` cases 02-11 | PASS (10) |
| `prompt_text_prop_round_trips_a_wrapped_paste` | FAIL — left `"\n"`, right `""`; minimal input `text = "", id = "A", lead = ("", "")` |

Six of the eight new cases are red on the untouched unwrap. The two that pass are the two whose expected text is
what HEAD already returns: they pin that the change removes nothing there.

The property recorded its failing seed, which is committed:
`crates/viola-agent-claude/proptest-regressions/hook.txt`, `cc 42da5b24…6d4`.

## Green — after the change to `unwrap_pastes`

Started 2026-10-06T20:21:39Z. Exit 0, `"ok":true`, `"suite":"nextest-unit","passed":20,"failed":0`.
nextest: `20 tests run: 20 passed, 1282 skipped`.

The change, in `crates/viola-agent-claude/src/hook.rs`: for a matched same-id pair the lead loses a final
two-newline run (`strip_suffix("\n\n")`) and the tail loses one leading newline (`strip_prefix('\n')`). The lead
is read from what is left after the previous pair's tail was taken, so a newline is taken at most once.
