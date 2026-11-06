@proposal n=1 detector=D-cascade severity=warning file=mocking-stubbing-discipline anchor=fct-63urz52n basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-zqhrblwm
section: ## 8. Mocking & Stubbing Discipline
change: in the Process faults row, «A stale holder is a real wrapper under the `Frozen` guard (`tests/support/frozen.rs`, its sole home, shared through `tests/support/mod.rs`; on Unix a `SIGSTOP`)» reads «… shared through `tests/support/mod.rs`; a `SIGSTOP`)» — the rest of the row unchanged
sidecar: 2026-11-06-version-done-check — Process faults row: the Frozen guard's «on Unix» qualifier dropped (Linux-only tree, Windows remnants retired)
rationale: the report retires the Windows remnants across 47 files with comments re-worded to the Linux reading (Changes, Files; Retired), so a per-OS qualifier on the SIGSTOP mechanism no longer distinguishes anything
@proposal n=2 detector=D-cascade severity=warning file=ci-integration anchor=fct-bfdvmti3 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-q2lwjk3d
section: ## 9. CI Integration
change: the Perf stage's «**Parallel:** per OS» reads «**Parallel:** one `ubuntu-latest` leg (the job name keeps its `${{ matrix.os }}` form)»; Runs and Cache unchanged
sidecar: 2026-11-06-version-done-check — Perf stage: «Parallel: per OS» re-worded to the one ubuntu-latest leg
rationale: the plan's expected amendment 10 re-words the per-OS wording (fct-yoazurel, fct-2ztzeoct) and the report's CI reading is one perf job on ubuntu-latest (`perf`, artifacts `perf-ubuntu-latest`, ci#42288148289)
@proposal n=3 detector=D-cascade severity=warning file=ci-integration anchor=fct-2qyicniz dependent_of=D-cascade basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-q2lwjk3d
section: ## 9. CI Integration
change: the Release build stage's «**Parallel:** per OS» reads «**Parallel:** one `ubuntu-latest` leg»; Runs (its `release-check probes: 5/5 refused, control clean` line) and Cache unchanged
sidecar: 2026-11-06-version-done-check — Release build stage: «Parallel: per OS» re-worded to the one ubuntu-latest leg
rationale: same per-OS remnant as the Perf stage; the report reads `release` green on ci#42288148289 and release-check probes 5/5 refused (Outcome gates), so only the Parallel line moves
@proposal n=4 detector=D-cascade severity=warning file=ci-integration anchor=fct-zwsm4wxk basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: ## 9. CI Integration
change: - **The supply-chain red, closed:** RUSTSEC-2026-0441 (`pin-utils` unmaintained, through `portable-pty =0.8.1` → `nix 0.25.1`), measured red at ci#42030025050, is closed by chunk 2026-11-06-version-done-check moving `portable-pty` to `=0.9.0` (`nix 0.28.0`; `cargo tree -i pin-utils` exit 101, no package); `deny.toml` `[advisories]` reads `ignore = []` (the RUSTSEC-2017-0008 entry left with `serial`), and the `supply-chain` job is green at ci#42288148289; the ignore list was never widened.
sidecar: 2026-11-06-version-done-check — the recorded supply-chain red re-worded closed: portable-pty =0.9.0, ignore = [], supply-chain green at ci#42288148289; its CARRY owner «Version done-check» discharged
rationale: the report's Dependencies state RUSTSEC-2026-0441 closed and `ignore = []`, its Retired bullet names the `=0.8.1` pin and the RUSTSEC-2017-0008 ignore, and the CI reading is seven jobs green incl. supply-chain (fct-q2lwjk3d); the row's red, owner and remedy are all spent
@proposal n=5 detector=D-cascade severity=warning file=quality-gates-coverage-targets anchor=fct-ocdr3xdn basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## 10. Quality Gates & Coverage Targets
change: in the always-admit survivor paragraph, «The mutation audit reads that same survivor at the route entry "Version done-check".» reads «That survivor stands as version 0.1.0's known limit, read at each epoch-boundary audit (matrix#v1-44).» — the rest of the fact unchanged
sidecar: 2026-11-06-version-done-check — mutation gate: the peer_admitted always-admit survivor's owner «Version done-check» re-worded to the version's known limit read at the epoch-boundary audit (matrix#v1-44, CARRY 8)
rationale: the report's expected amendment 14 states matrix#v1-44's always-admit survivor stands as the version's known limit, read at the epoch-boundary audit (CARRY 8), naming fct-ocdr3xdn among the three facts that name this entry as owner
@remark
the same v1-44 owner re-wording reaches test-plan body fct-upolcsa3 (§1 Untestable zones, group test-scope-summary) and the hold-ceiling owner title rows lie outside group 4; task facts F1 (e2e-test-strategy) and F2 (test-scope-summary) belong to other groups
