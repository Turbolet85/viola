# Operator pass — 2026-09-28-cli-output-tokens

Driven on the operator's word (AskUserQuestion, 2026-09-28: "Drive it, commit + push"; note "overseer: yes; report the
final hygiene verdict line verbatim").

| entry | run | reading |
|---|---|---|
| 26 | `bash scripts/agent-run.sh pre-push` (uncommitted tree) | green: exit 0; `"cmd":"pre-push","ok":true` ×1; `"stage":"union"` ×1; legs ubuntu-latest and windows-2025 both `ok:true`, `verdict:"counted"`, 7 tested; union gate `breaches: []`; Linux and Windows coverage gates `ok:true` — `operator-26-pre-push.json` |
| 25 | `python -X utf8 …/andromeda-tools/scripts/gate.py hygiene` | green: exit 0; final line, re-run after this file was first written: `hygiene: clean — read 37 (runs 29 · evidence 8) · trails 12 not read · binary 0 not read by P1` |
| — | pre-CI commit | `c2dccf6` `chore(2026-09-28-cli-output-tokens): operator pre-CI commit, for the run this chunk's verdict reads` |
| 27 | `git diff --quiet && git diff --cached --quiet && git push origin build/viola-0.1.0` | exit 0; `c04e332..c2dccf6  build/viola-0.1.0 -> build/viola-0.1.0` |
| 28 | `python -X utf8 …/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 5400` | green: exit 0; `c2dccf60c7db verdict: green · checks 18/18 · wall 285 s · runs ci#36404931982 completed/success` |

## CI reads behind entry 28 (ci#36404931982)
- **Mutation union (gate 17's reconciliation):** `mutants (ubuntu-latest)` caught `src/cmd/run.rs:373:5: replace
  refuse_stale with ()` and missed `refuse_batch_script`; `mutants (windows-2025)` caught `refuse_batch_script` and
  missed `refuse_stale`; `mutants-verdict` printed `{"v":1,"cmd":"gate","ok":true,"breaches":[]}`. The union
  matches the local pre-push union exactly.
- **`cli_output_plain` on three OSes:** all four cases PASS in `test (ubuntu-latest)`, `test (windows-2025)` and
  `test (macos-latest)`, including `help_is_plain_on_a_terminal` under each OS's PTY.
