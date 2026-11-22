@leaf_group group=security-plan+obs-plan
@leaf_read leaf=.claude/rules/ci.md lines=36
last: - 2026-11-20: A WATCH's green run read from CI counts only o
@leaf_edit n=1 leaf=.claude/rules/ci.md line=12 class=stale
leaf-text: every upload only when the job's secret scan succeeded)
source: dec-rwjzbyas «decision/perf-uploads-scan-gated · perf job uploads gated by secret scan — The perf job's hyperfine exports and diagnostics are uploaded only when its own secret scan succeeds, the scan output only when it fails»
replace-with: every artifact upload only when the job's secret scan succeeded, save the perf job's scan output, uploaded only when that scan fails)
