@proposal n=1 detector=D-retired severity=warning basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: §1 Test Scope Summary
change: fct-njspb5nv's viola-pty entity reads «the seam over portable-pty `=0.9.0` (spawn, read, write, resize, wait, kill)», the rest unchanged.
sidecar: §1 viola-pty entity — portable-pty pin `=0.8.1` → `=0.9.0` (chunk 2026-11-06-version-done-check, Dependencies).
rationale: report fct-xc746owh moves `portable-pty` `=0.8.1` → `=0.9.0`; fct-uufjzm4k retires the `=0.8.1` pin; fct-ngwjb7xx item 10 names fct-njspb5nv.
@proposal n=2 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: §3 → 5-command-implementation
change: fct-zxaumenb boot step 5 reads «It owns one portable-pty (`=0.9.0`, via the `viola-pty` seam) outer PTY per instance», the rest unchanged.
sidecar: §3 5-command implementation boot step 5 — the supervisor's portable-pty pin `=0.8.1` → `=0.9.0`.
rationale: the same retired `=0.8.1` pin (report fct-xc746owh, fct-uufjzm4k) restated in the keyed contract.
@proposal n=3 detector=D-retired severity=warning basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-u65xlcax
section: §1 Test Scope Summary
change: fct-hlx2zkbt's supply-chain trigger reads «`cargo deny check` (no advisory ignored, `[advisories] ignore = []`)» in place of «(only ignore RUSTSEC-2017-0008)».
sidecar: §1 supply-chain trigger — the RUSTSEC-2017-0008 ignore left with the `serial` crate; the ignore list is empty.
rationale: report fct-xc746owh (`deny.toml` `ignore = []`, the RUSTSEC-2017-0008 entry left with `serial`), fct-u65xlcax (ignore entries 1 → 0), fct-uufjzm4k retires the ignore; fct-ngwjb7xx item 10 names fct-hlx2zkbt.
@proposal n=4 detector=D-retired severity=warning basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-uufjzm4k
section: §1 Test Scope Summary
change: fct-engo5ir7's CLI and env bullet reads «a leading-slash argument is never taken as a prompt», the «(§6 rewritten-path warning)» pointer removed.
sidecar: §1 CLI and env trigger — the retired Git Bash rewritten-path warning no longer named; the leading-slash rule stands.
rationale: report fct-uufjzm4k retires the Git Bash rewritten-path warning (emitter, unit table, CLI witness); fct-zmvefc57 removes `warn_if_rewritten` / `looks_rewritten` and `send_rewritten_path_argument_draws_the_warning`; fct-fiaocty5 keeps the stdin/`--file` rule.
@proposal n=5 detector=D-retired severity=warning basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: §1 Test Scope Summary
change: fct-u3dkdc2w reads «always the 11-name identity floor measured on the retired Windows host», the rest unchanged.
sidecar: §1 inherited-credentials entity — the Windows-host measurement marked as the retired host's.
rationale: report fct-ngwjb7xx item 10 names fct-u3dkdc2w among the Windows wording to move; the product is Linux-only (fct-uufjzm4k, Windows handling retired).
@proposal n=6 detector=D-retired severity=warning basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: §10 Quality Gates & Coverage Targets
change: fct-vxnii5yh's host reading reads «Host reading at chunk 2026-09-28-hook-perf-gate (the retired Windows dev host, release build)», the rest unchanged.
sidecar: §10 perf Status — the 2026-09-28 reading marked as the retired Windows host's.
rationale: report fct-ngwjb7xx item 10 names fct-vxnii5yh among the Windows wording to move.
@proposal n=7 detector=D-retired severity=warning basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: §10 Quality Gates & Coverage Targets
change: fct-yoazurel's row reads «`viola hook session-end` (synthetic payload via `--input`, live wrapper)», «per OS» removed: the perf job runs on `ubuntu-latest` alone.
sidecar: §10 perf table — the session-end row's «per OS» dropped (Linux-only).
rationale: report fct-ngwjb7xx item 10 names fct-yoazurel among the Windows wording to move.
@proposal n=8 detector=D-retired severity=warning basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: §10 Quality Gates & Coverage Targets
change: fct-2ztzeoct reads «Coverage below threshold for tier (the `test` job on `ubuntu-latest`)» in place of «(per OS)».
sidecar: §10 build failure conditions — coverage «per OS» → the one `ubuntu-latest` test job.
rationale: report fct-ngwjb7xx item 10 names fct-2ztzeoct among the Windows wording to move.
@proposal n=9 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: §10 Quality Gates & Coverage Targets
change: fct-plbmhdnw reads «Test failure (any level)», «any OS» removed.
sidecar: §10 build failure conditions — «any OS» dropped (Linux-only).
rationale: the same multi-OS framing as fct-2ztzeoct (report fct-ngwjb7xx item 10).
@proposal n=10 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: §3 → bootstrap-phases-derive-for-route-setup-project
change: fct-da3wgwes quality-gate-config-emit reads «the coverage thresholds in §10 (lines 85 / functions 95 / regions 80)», «, per OS» removed.
sidecar: §3 bootstrap phases quality-gate-config-emit — coverage «per OS» dropped.
rationale: the same multi-OS framing as fct-2ztzeoct restated in the keyed contract (report fct-ngwjb7xx item 10).
@proposal n=11 detector=D-retired severity=warning basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: §10 Quality Gates & Coverage Targets
change: fct-3mj52n2x reads «…the harness const `COVERAGE_IGNORE`, a separator-agnostic form kept from the retired Windows target, where llvm-cov reported paths with `\` (as measured at chunk 2026-09-24-quality-gates, research.md M5)»; the regex unchanged.
sidecar: §10 coverage exclusion — the Windows separator reason marked as the retired target's; the four trees byte-identical.
rationale: report fct-ngwjb7xx item 10 names fct-3mj52n2x; fct-qnniwhgc: comments only in coverage.rs, the regex's four trees byte-identical.
@proposal n=12 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-qnniwhgc
section: §11 Test Anti-Patterns
change: fct-b4avhaux reads «The separator class, kept from the retired Windows target, is not a widening, because the four trees are unchanged», the rest unchanged.
sidecar: §11 CI ban — the separator class's Windows reason marked as the retired target's.
rationale: the same Windows-path claim as fct-3mj52n2x (report fct-ngwjb7xx item 10, fct-qnniwhgc).
@remark
Not proposed: fct-ngwjb7xx item 10's trycmd/assert_cmd re-word to literal pins (CARRY 1) — the report states no target wording; detail-file sites (fct-vmzc7an2, fct-tr6c4sml, fct-zwsm4wxk, fct-lzdm6e7w plain_drive_*/viola.exe/rewritten-path, fct-b2j3wr36, fct-hhwc4gim) are the cascade's; D-tests-coverage/framework/obs-harness/count-literals hold (new contract test at e2e tier, nextest unchanged, harness unchanged, LEDGER_ROWS 24 and deny-probes 14/14 unchanged).
