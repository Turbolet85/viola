
## 2026-10-05-dialog-rows-and-re-probe — fourteen rows, recorded dialog variants, the verify_window_ class grown
**Section:** §1 contract trigger · §2 Test pyramid → Contract · §4 Dialog mapping · §5 `cli_verify` · `contract_ledger_probes` · §6 Path 3 / Path 4 · §7 Fixture library · Recorded hook payloads · Fake agent modes · Fixture hygiene · §3 → 5-command implementation · §3 → Bootstrap phases
**Change:**
- Rows: ten → fourteen; `[NN/10]` → `[NN/14]`; `contract_ledger_probes` ends `14 pass  0 fail` under `--dialogs`; `cli_verify` adds the no-replay four-dialog-row fail (the stale case count dropped); boot step 4 passes `--dialogs` (four runs); `LEDGER_ROWS: [&str; 14]`, the live firing at fourteen rows owed to "First live test and self-drive".
- Dialog tier: was relayed from the viola-lab prototype until this entry's re-probe; now recorded by verify's Runs C / D as `<Event>.<stem>-<n>.json` (12 per version), scrubbed and `unclean`-checked; the drift contract replays every variant; hygiene walks them; the relayed set superseded and kept.
- Fake agent: argv options three → five (`--dialogs`, `--stop-receipt-hold-ms`, capped at 1 000 ms).
- §4 / §6: the S7 approve is `allow` + `updatedInput` echoing the tool's input (was a bare `allow`); the `permission` end to end re-owed to "Permission end to end" over the recorded `PermissionRequest.permission-1.json`; `cli_answer` adds `v1-15`.
- Bootstrap phases: `[profile.ci]` gains two overrides — `verify_window_` tests 45 s (15 s × 3), the twelve verify-driven binaries 20 s (10 s × 2); a verify a test drives has no test-side bound; `WITHIN` 7 s stays elsewhere. The mutants `verify_window_` rationale reads four maximum waits (was ~11 s, two); kills unchanged.
**Why:** the chunk landed the rows and the re-probe. The 7 s bound gave way for verify-driven tests on the overseer's founder-delegated decision: four runs make a designed floor (about 1.0 s → 2.1 s median on the ubuntu coverage leg, seven 300 ms settles), not a regression. The approve form and the permission split are the founder's live rulings of 2026-10-05.
**Ref:** .andromeda/runs/2026-10-05T14-27-34-wrap/
