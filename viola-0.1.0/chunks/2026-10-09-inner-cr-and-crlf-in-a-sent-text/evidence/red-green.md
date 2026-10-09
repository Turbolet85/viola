# The new cases, red on the present rule and green on the new one (plan.md steps 4 and 7)

A one-shot pair. Both sides ran the two filter entries of the gate block by hand, through the harness, on the same
test sources; between them only `typed_text` and its two callers changed (plan.md steps 5 and 6). Times are UTC,
2026-10-09, read from the clock.

## Red: the present rule (`typed_text` removes the ending and nothing else)

| entry | when | exit | document |
|---|---|---|---|
| `bash scripts/agent-run.sh run --unit --filter 'test(/cr_is_one_lf/)'` | 18:55:27Z to 18:55:33Z | 1 | `"ok":false`; `nextest-unit` passed 1, failed 17 |
| `bash scripts/agent-run.sh run --integration --filter 'test(/cr_is_one_lf/)'` | 18:55:40Z to 18:55:41Z | 1 | `"ok":false`; `nextest-integration` passed 0, failed 3 |

Where each failed, read from the runs' own panic lines:

| cases | count | failed at | the assertion |
|---|---|---|---|
| `hook::tests::typed_text_every_inner_cr_is_one_lf`, every case that feeds a CR | 9 of 10 | the table's one assertion | the typed text is not the literal with LF |
| `hook::tests::typed_text_prop_holds_no_cr_and_every_inner_cr_is_one_lf` | 1 | its first assertion, the typed text holds no CR | minimal failing input: one CR, then one space |
| `run::send::tests::send_text_whose_inner_cr_is_one_lf_is_typed_and_confirmed` | 5 of 5 | `confirmed_as`, the paste assertion | the paste is the sent text, CR included |
| `run::send::tests::send_a_prompt_that_keeps_the_cr_while_its_inner_cr_is_one_lf_claims_nothing` | 2 of 2 | its paste assertion | the paste is the sent text, CR included |
| `cli_send::send_text_whose_inner_cr_is_one_lf_is_typed_with_lf_and_confirmed` | 3 of 3 | the receipt's bytes hold no CR | the fake agent receipted a CR |

The one case that passed is the table's `no_cr`, which feeds no CR. Every new case that feeds a CR read red, so
stop rule S2 did not fire. The prediction held as written: the unit cases failed at the paste assertion, the
cross-process cases at the receipt's bytes, the property failed, and the no-CR case passed.

## Green: the new rule (every CR LF pair and every other CR as one LF, then no CR or LF ending)

| entry | when | exit | document |
|---|---|---|---|
| `bash scripts/agent-run.sh run --unit --filter 'test(/cr_is_one_lf/)'` | 18:56:46Z to 18:56:51Z | 0 | `"ok":true`; `nextest-unit` passed 18, failed 0 |
| `bash scripts/agent-run.sh run --integration --filter 'test(/cr_is_one_lf/)'` | 18:56:51Z to 18:56:52Z | 0 | `"ok":true`; `nextest-integration` passed 3, failed 0 |

The same 18 and the same 3 cases, selected by the same filter.

## Outside the filter

Five cases were added to existing tables and carry no `cr_is_one_lf` in their names; they pin what the rule must
leave alone and read the same on both rules, so they are not part of this pair:

- `send_refusal_order`: a refused character beside an inner CR LF is `control-character`;
- `send_empty_text_is_refused_first_after_control_character`: CR LF CR is `empty-text`;
- `refusal_of_reads_a_refused_character_before_an_empty_text`: an inner CR is content, CR CR LF CR is
  `empty-text`, a refused character beside an inner CR LF is `control-character`.

One test was added with the rule and has no red side on the present rule, because it reads the new return type:
`typed_text_copies_only_a_text_with_an_inner_cr` (a text with no inner CR is typed as a slice of the received
text; one with an inner CR is copied). The whole unit entry of the block runs all six.
