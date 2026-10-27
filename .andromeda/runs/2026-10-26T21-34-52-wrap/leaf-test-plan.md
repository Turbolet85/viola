group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 105 · «- 2026-10-26: An OS-gated test module in product code stacks»
read: .claude/rules/verification-harness.md · 72 · «- 2026-10-20: `target/debug/viola` is a hard link cargo re-p»

- leaf: .claude/docs/tests-summary.md:20   class: retired
  leaf-text: «the Epoch 6 entry owns closing it»
  source: .andromeda/registries/contracts/test-plan/test-data-bootstrap.md:11 «making the home and the seed dir through `viola_state::fs::create_private_dir` first; the companions are hard links to one shared seed file»
  replace-with: «`seed_conpty` makes the home and the seed dir through `viola_state::fs::create_private_dir` first, so a seeded home passes the instance check; its companions are hard links to one shared seed file, so a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded»

- leaf: .claude/docs/tests-summary.md:25   class: omission
  leaf-text: «duplicate / squatted / tampered-exe starts exit 1»
  source: .andromeda/test-plan.md:135 «a second `run` of the same name, a squatted endpoint, a tampered pinned exe or a refused home tree exits 1.»
  replace-with: «the home-tree strict-modes check runs before the wrapper log opens; duplicate / squatted / tampered-exe starts and a refused home tree exit 1»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «A check-guarded case that must pass on every OS starts from `stamped_home`: on Windows x64 a home `seed_conpty` made is refused by the instance check»
  source: .andromeda/test-plan-detail/test-data-fixtures.md:10 «`seed_conpty` makes the home and the seed dir `target/conpty-seed/<key>/` through `viola_state::fs::create_private_dir` before `bin/<key>/conpty/`, so a seeded home passes `check_instance`»
  replace-with: «A case that needs a home viola created on every OS starts from `stamped_home`; a home `seed_conpty` made passes the instance check on Windows x64, and one a test makes with `create_dir_all` is refused»

- leaf: .claude/docs/tests-summary.md:32   class: omission
  leaf-text: «snapshot corruption → replay;»
  source: .andromeda/test-plan.md:595 «**As landed: built.** Steps 1–3 run as `tests/chaos_snapshot_replay.rs` (`a_start_over_an_unreadable_snapshot_replays_the_log`, `a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live`)»
  replace-with: «snapshot corruption → replay (steps 1–3 built as `tests/chaos_snapshot_replay.rs`: a start over an unreadable or `v:99` snapshot replays the log, and a fresh beat over the replayed snapshot refuses the start `already-live`; `read_snapshot` has no product caller under `src/`);»

- leaf: .claude/rules/testing.md:33   class: stale
  leaf-text: «a seeded home fails `hook statusline`'s instance check on Windows, so a case that drives a check-guarded path on every OS starts from `stamped_home`»
  source: .andromeda/test-plan-detail/test-data-fixtures.md:10 «`seed_conpty` makes the home and the seed dir `target/conpty-seed/<key>/` through `viola_state::fs::create_private_dir` before `bin/<key>/conpty/`, so a seeded home passes `check_instance`»
  replace-with: «`seed_conpty` makes the home and the seed dir through `viola_state::fs::create_private_dir` first, so a seeded home passes the instance check; its companions are hard links to one shared seed file, so a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded; a case that needs a home viola created on every OS starts from `stamped_home`»

- leaf: .claude/rules/testing.md:33   class: omission
  leaf-text: «so viola creates it (Windows protected DACL)»
  source: .andromeda/test-plan.md:634 «a test that makes a home itself makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`: a plain home is one the product refuses»
  replace-with: «so viola creates it (Windows protected DACL); a test that makes a home itself makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`, whose home the product refuses (Unix by the read rule; Windows by an inherited Users write ACE)»

- leaf: .claude/rules/testing.md:18   class: omission
  leaf-text: «the one complete-lines reader in `ndjson.rs`»
  source: .andromeda/test-plan-detail/test-strategy.md:42 «`home.rs` holds the home-widening helpers `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted)»
  replace-with: «the home-widening helpers `widen` · `grant_read` · `unwiden` · `still_widened` in `home.rs` (Unix modes; on Windows `icacls`, its exit asserted), through which a root test widens or read-grants a home path, restores it and reads whether it is still widened, the one complete-lines reader in `ndjson.rs`»
