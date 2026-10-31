# Classified — architecture · occupied-resources.b2

## Answers
occupied-resources#26.1 · no · the board scan and the list's two home checks sit in files the session did not reach, and viola-state is not edited
occupied-resources#27.1 · no · budget.json's writer and gate read sit in `src/cmd/hook/statusline.rs` and `src/run/budget.rs`, whose changes are cap-driving test additions only, so the report shows the claim still holds
occupied-resources#28.1 · yes · stale · `src/cmd/verify.rs`, the sole caller of `update_stamps`, is reached whole and the report clears neither its call nor the refusal line it prints
occupied-resources#29.1 · yes · stale · `src/cmd/verify.rs` holds the probe dirs and their drop guards, reached whole and not cleared by the report
occupied-resources#30.1 · yes · stale · `cmd::verify::stop` and the probe-dir drop guards sit in `src/cmd/verify.rs`, reached whole and not cleared, though the change does not name them
occupied-resources#31.1 · yes · add-to · F14 replaces the after-every-test helper this line is extended by with the three witnesses, the enumeration and the writer census, and `tests/support/events.rs` now holds `error_body_hit` and `assert_error_body_clean`
occupied-resources#32.1 · no · the e2e tmpfs base and its keepers sit in test support and the harness, neither reached, and the env vars they name are unchanged
occupied-resources#33.1 · yes · stale · `src/cmd/verify.rs` holds `viola verify --record` and its unclean refusal, reached whole and not cleared by the report
occupied-resources#34.1 · no · the harness outputs under `target/agent-run/` are untouched, as the report finds the harness verbs, CI steps and status shapes unmoved and `.github/workflows/ci.yml` was only read
occupied-resources#35.1 · yes · add-to · F15 concretizes the after-every-test keep to each job's end-to-end run, on `AGENT_RUN_KEEP_HOMES` "1" and `if: always()` in `.github/workflows/ci.yml`, with the `viola-chaos-*` home out (provisional)
occupied-resources#36.1 · no · the icacls grant and `seed_conpty` are test-side and `create_private_dir` is viola-state, neither reached, and viola-state is not edited
occupied-resources#37.1 · no · the profraw census is a dev script under `scripts/` that the session did not reach
occupied-resources#38.1 · no · the mutation scratch is the harness's, under `run --mutants` and `windows-mutants.yml`, neither reached, and the harness verbs are unmoved
occupied-resources#39.1 · no · the DACL of a home viola makes outside the profile is viola-state's, and its read-back oracle sits in a file the session did not reach
occupied-resources#39.2 · no · the temp-dir ACL facts and their `strict::win` witness sit in viola-state, which is not edited
