@leaf_group group=security-plan+obs-plan
@leaf_read leaf=.claude/rules/ci.md lines=37
last: - 2026-11-20: A WATCH's green run read from CI counts only o
@leaf_stands n=1 leaf=.claude/rules/ci.md line=11
because: Dependency Security still holds every action pinned by full commit SHA with a version comment (dec actions-pinned-by-sha), the least workflow permissions, the cargo-deny and zizmor audits and the workflow bans, as the pointer says.
@leaf_stands n=2 leaf=.claude/rules/ci.md line=12
because: CI Integration still has every gate fail closed (ci-gates-fail-closed), G2, G4 and the secret scan run after a failed step (gates-run-after-failed-tests, secret-scan-always-runs), uploads only after a passing scan (ci-artifacts-gated-on-secret-scan) and the perf scan output uploaded only when that scan fails (perf-uploads-scan-gated).
