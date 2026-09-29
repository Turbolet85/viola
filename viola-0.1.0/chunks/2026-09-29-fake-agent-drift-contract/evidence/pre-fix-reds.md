# Pre-fix reds — the three witnesses, before their fixes (2026-09-29)

Gate run `2026-09-29T13-30-05-implement`, `gate.py run --only 4,5,6`, on the tree with the witnesses and step 3's
`stdin_hex` landed and steps 2, 4 and the `HARNESS_PREFIXES` edit NOT yet applied. Verdict line:
`entries 18 · green 0 · red 3 (4,5,6)`.

## Entry 4 — step 6 table against the unchanged `HARNESS_PREFIXES`
`prompt_origin_files_the_cross_session_tag_as_harness`: 5 run, 3 passed, 2 failed.
- FAIL `case_1_cross_escaped` — `origin` read `human`, expected `harness` (`text` already the plain tag).
- FAIL `case_2_cross_plain` — `origin` read `human`, expected `harness`.
- PASS `case_3_cross_escaped_inner`, `case_4_cross_plain_inner`, `case_5_leading_space`.

## Entry 5 — the drift contract before step 2
`contract_fake_agent_drift`: 4 run, 3 passed, 1 failed.
- FAIL `fake_agent_print_turn_matches_every_recorded_set` — `2.1.283: drift` left `["UserPromptSubmit"]`, right `[]`.
  Hook order and `ran:true` / `exit_code:0` held (the assertions before the drift check passed).
- PASS `drift_is_empty_for_the_recorded_set_in_spine_order`, `drift_names_a_swapped_order` (→ `["order"]`),
  `drift_names_the_event_whose_fixture_differs_by_one_byte` (→ `["Stop"]`).

**Byte-equality hypothesis (plan §Implementation notes) — measured, holds.** The same pre-fix fake agent
(`target/harness/debug`, built by this run) driven by hand in print mode over the recorded set, its `stdin_hex`
decoded and compared per event: SessionStart, Stop, SessionEnd equal; UserPromptSubmit differs by exactly one
thing — `fixture = sent + b'\n'`. Re-serialising the `preserve_order` object reproduces the recorder's bytes up to
the recorder's trailing newline, and nothing else.

## Entry 6 — the key-free resize witness before step 4
`pty_resize_reaches_a_child_that_reads_no_key`: FAIL at 7.24 s — `timed out waiting for resized with no key`, watch
report `receipt lines 4 kinds start,cwd,env,size` (the 80×24 line only; no byte read, so no size sampled). It does
not pass vacuously.
