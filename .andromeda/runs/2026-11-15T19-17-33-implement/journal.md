# Journal — implement 2026-11-15-mutation-and-coverage-baselines-green

One line per long run, written before its wait.

- 2026-11-15T19:21:25Z · P2 block run 1: `gate.py run --chunk 2026-11-15-mutation-and-coverage-baselines-green --run-dir .andromeda/runs/2026-11-15T19-17-33-implement`, backgrounded; non-leg entries 1-12, 15-17 (12 the warm mutants-profile suite, 17 pre-push). Load at fire: 101.47 / 64.07 / 28.58 (another tree's `cargo test --workspace` and test binaries running on the host).
- 2026-11-15T19:26:06Z · P2 control-1.toml (13 controls on entries 5, 7, 10, 11) fired backgrounded through gate.py control; load 18.65 38.95 27.28
- 2026-11-15T19:28:49Z · step 9 planted-hang pair in ~/dev/projects/viola-mutants-scratch/hang-mhCgsY (own target), backgrounded: build, then --profile mutants, then default; load 8.56 27.56 24.90
- 2026-11-15T19:30:27Z · step 10 operator entry 13 (cold parallel baseline under the mutation scratch, own cold target) fired backgrounded via gate.py run --operator 13; df /home 298G; load 62.23 52.96 35.09
- 2026-11-15T19:33:18Z · entry 13 read: recorded exit 0, 3285 passed, 1 failed (viola-channel tokio_client_server_euid_is_this_user, sun_path bind under the scratch TMPDIR). Step 10 operator entry 14 (M2 case under llvm-cov) fired backgrounded; load 12.29 36.61 31.98
- 2026-11-15T19:36:40Z · P2 block run 2 after the fold (inputs#I4): full block backgrounded; entry 15 expected red on the plan wording (product test-module edits); load 44.41 72.64 49.59
- 2026-11-15T19:41:12Z · bound (4): operator entry 13 re-fired after the fold (second cold baseline), backgrounded; df /home 287G; load 33.53 70.03 55.39
- 2026-11-15T19:48:21Z · operator pass: pre-push (entry 17) re-run before the push, backgrounded; load 103.75 45.29 45.70
