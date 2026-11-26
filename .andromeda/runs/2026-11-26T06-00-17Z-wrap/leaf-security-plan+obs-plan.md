@leaf_group group=security-plan+obs-plan
@leaf_read leaf=.claude/rules/ci.md lines=37
last: - 2026-11-20: A WATCH's green run read from CI counts only o
@leaf_stands n=1 leaf=.claude/rules/ci.md line=11
because: Dependency Security still pins every action by full commit SHA with a version comment (decision/actions-pinned-by-sha) and holds the least permissions, the cargo-deny and zizmor audits and the workflow bans the line names.
@leaf_stands n=2 leaf=.claude/rules/ci.md line=12
because: CI Integration still has every gate fail closed (decision/ci-gates-fail-closed), G2, G4 and the secret scan run after a failed step, and every upload gated on a successful scan, with the perf scan output uploaded only on its failure (decision/perf-uploads-scan-gated).
