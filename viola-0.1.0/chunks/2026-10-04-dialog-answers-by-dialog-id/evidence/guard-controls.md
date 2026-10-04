# Remove-the-guard controls (testing.md 2026-09-25) — this chunk's new guards

Each guard was neutralised in place by a one-shot script (kept outside the tree), its tests run, and the file
restored byte for byte (the script asserts the restore). Linux host, 2026-10-04, after P1. Selectors are nextest
filtersets.

| guard | neutralisation | tests run | reading neutralised | after restore |
|---|---|---|---|---|
| strict-modes on `run`'s stamps read (`crates/viola-state/src/strict.rs` `check_stamps`) | returns `Ok(())` before any check | `strict::tests::check_stamps*`, `stamps::tests::read_stamps_strict*`, `run_with_world_writable_stamps_is_unverified_whatever_they_hold` | 6 of 9 FAIL: the four widened-mode cases, `read_stamps_strict_refuses_a_world_writable_stamps_file`, and the root `cli_version_gate` world-writable case | green in the gate run |
| `control-character` first in the wrapper's `answer` (`src/run/dialog.rs`) | the `validate_paste_text` refusal filtered away | `run::dialog::tests::answer_refusal_order` | 2 of 9 FAIL: `control_character_before_unverified`, `control_character_in_an_annotation` | green |
| the continuation (`src/run/dialog.rs` `hook_dialog`) | the armed-match branch skipped (raised as a new dialog) | `run::dialog::tests::continuation*` (nextest `mutants` profile, 10 s kill) | 4 of 6 red: 2 FAIL (an extra event logged), 2 TIMEOUT (a raised plan held forever on the fixed test clock) | green |
| the hook's dialog read bound (`src/cmd/hook.rs` `ask`) | the reply deadline pushed 10 s later | `cmd::hook::tests::handle_dialog_stops_waiting_at_its_read_deadline`, `ask_reads_the_three_outcomes` | 2 of 2 FAIL (each waited out the 3 s server hold) | green |

Note: the first attempt ran the continuation control under the default profile; the held dialog never ended, the
run reached its time bound and was killed before the script's restore. The one neutralised line it left
(`if raise.continuation && raise.data.is_empty()`) was restored by hand and confirmed absent (`grep -c` 0) before
the control was re-run under the `mutants` profile's kill line.
