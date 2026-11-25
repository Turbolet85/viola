@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=38 class=omission
leaf-text: (`program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `unclean-fixture`)
source: ctr-aewul3as «Under a parsed VIOLA_NAME, verify writes its refusal code: program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture, untrusted-folder or interrupted;»
replace-with: (`program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `unclean-fixture`, `untrusted-folder`)
@leaf_edit n=2 leaf=.claude/rules/observability.md line=39 class=omission
leaf-text: a refused scan writes no line. `viola switch`
source: ctr-a7m5isdk «Written on the process-exit exit 1 of viola verify and of viola up under a parsed VIOLA_NAME, and on parse-rejected ledger-stamps from the version gate of run, at warn, with no path;»
replace-with: a refused scan writes no line, and a home tree refused after the scan ends that log with `process-exit{exit_code:1, detail:"strict-modes-failed"}`. `viola switch`
