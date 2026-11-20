# CI red after the wrap's push — recorded before any re-run

- Run: ci#44698471269 on `6935e0f0` (the wrap commit), `test (ubuntu-latest)` failed; the other six jobs green. `ci.py conclusion --sha HEAD --wait 1800`: `6935e0f0aed0 verdict: red · checks 7/7 · first-fail +599 s test (ubuntu-latest) · wall 599 s`.
- The test: `viola-e2e harness::supervise::tests::stop_presses_ctrl_c_again_when_the_first_is_lost` — `FAIL [ 2.000s] (3141/3727)`; first diagnostic: `panicked at crates/viola-e2e/src/harness/supervise.rs:277:9: assertion left == right failed — left: Number(1) right: 0` (the stopped fake agent's `exit_code` read 1, where 0 is asserted). Summary: `3727 tests run: 3726 passed (11 slow), 1 failed, 0 skipped`.
- Basis, not this chunk's: `git diff --stat fa113635 6935e0f0 -- crates src tests fuzz Cargo.toml Cargo.lock scripts .github` is empty — the wrap commit changed no code — and that same code tree read green 7/7 in ci#44691332735 (the operator pass, entry 38). The test and `supervise.rs` are outside this chunk's diff (last touched 2026-11-18). No earlier red of it in this project's records.
- Cause: not read. No re-run fired; a green re-run would not close it (testing.md, 2026-11-16).
- Owner: a `WATCH:` on «A quoted modal never blocks a send» (route-resolve's Recurrence-watch row), pinned after the chunk commit in a follow-up bookkeeping commit.
