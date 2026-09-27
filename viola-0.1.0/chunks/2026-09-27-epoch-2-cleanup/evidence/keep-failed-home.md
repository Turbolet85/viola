# A failed lifecycle test keeps its home (plan step 10) — one-shot control pair

The mechanism: `crates/viola-e2e/tests/harness_lifecycle.rs` gives each booted test a `Booted` guard. On drop, even after a failed
assertion, the guard runs `cleanup(.., keep_homes = true)`, which stops every recorded process and keeps the home. It then removes
the home's `viola-session-*` dir unless `keep_home(keep_failed, std::thread::panicking())`. That is the root fixture chain's
`keep_decision` (`tests/support/home.rs:21–23`) without `AGENT_RUN_KEEP_HOMES`: only a failing test under `AGENT_RUN_KEEP_FAILED=1`
keeps its home. The tests' own `cleanup` calls now keep the home (`home_removed:"kept"`), and the guard removes it.
`run --mutants` still sets `AGENT_RUN_KEEP_FAILED=0` (`run_mutants_never_keeps_test_homes` unchanged).

## The pair (temporary tests, removed after the reading)

Two throwaway tests shared one body: boot a `builder` session, take a `Booted` guard, write the guard's home dir name to
`%TEMP%/zz-control-<label>.txt`, then `assert!(!fail)`. `zz_control_fails` passed `fail = true` (the deliberately failing test),
and `zz_control_passes` passed `fail = false` (its passing twin).

Run: `AGENT_RUN_KEEP_FAILED=1 bash scripts/agent-run.sh run --integration --filter 'binary(harness_lifecycle) & test(/zz_control/)'`,
this host, 2026-09-27 ~15:02Z.

- Document: exit 1, `ok:false`, nextest-integration 1 passed / 1 failed, failures `["viola-e2e::harness_lifecycle zz_control_fails"]`
  (the deliberate one).
- `zz_control_fails` (failing, `AGENT_RUN_KEEP_FAILED=1`): its home dir `target/e2e-home/viola-session-NETvJg` **exists** after the
  run and holds `home/` (`bin`, `diagnostics`, `instances`, `plugin`).
- `zz_control_passes` (passing twin, same env): its home dir `target/e2e-home/viola-session-L80SeE` **does not exist** after the run.
- Processes: a `Win32_Process` census right after the run showed no `viola.exe`, `viola-harness.exe` or `viola-fake-agent.exe` from
  this workspace's `target/harness` bins. The guard's `cleanup` stopped the failed test's session before keeping its home. (Two
  `viola-fake-agent.exe` from an old `%TEMP%/cargo-mutants-viola-Bk11Ik.tmp` copy, started 13:16Z, predate this session and are
  unrelated.)

The two temporary tests were then removed (`grep -c zz_control` = 0). The kept home `viola-session-NETvJg` stays under the
gitignored `target/e2e-home/` for the operator to remove.

The standing guard of this mechanism: `keep_home_is_a_failed_test_under_keep_failed_only` (all four cases), and
`harness_session_boots_reports_logs_and_tears_down`, which asserts `home_removed:"kept"` after its cleanup and the home gone once
its guard drops. Both run in the integration filter entry.
