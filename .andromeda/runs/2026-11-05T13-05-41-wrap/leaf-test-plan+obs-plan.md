@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=20 class=retired
leaf-text: a file to the read rule on Unix and the write rule on Windows
source: fct-bprhan45 «a symlink is refused, a folder and a file are held to the read rule, and an absent path passes»
replace-with: a file to the read rule
@leaf_edit n=2 leaf=.claude/rules/observability.md line=28 class=stale
leaf-text: `schemas/diag-line.v1.json` closes `process-exit`'s at 20 values
source: fct-dsupowm6 «`schemas/diag-line.v1.json` closes the `process-exit` `detail` at 19 values»
replace-with: `schemas/diag-line.v1.json` closes `process-exit`'s at 19 values
@leaf_edit n=3 leaf=.claude/rules/observability.md line=38 class=retired
leaf-text: (`batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `unclean-fixture`)
source: fct-dsupowm6 «`viola verify` writes its refusal's code on the `process-exit` it writes under a parsed `VIOLA_NAME`: `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`»
replace-with: (`program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `unclean-fixture`)
@leaf_edit n=4 leaf=.claude/rules/observability.md line=38 class=retired
leaf-text: (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break)
source: fct-dsupowm6 «a `viola verify` stopped by Ctrl-C or TERM (`SIGINT` / `SIGTERM`) before its rows are checked ends with one `process-exit{subject:"self", exit_code:1, detail:"interrupted"}`»
replace-with: (`SIGINT` / `SIGTERM`)
@leaf_edit n=5 leaf=.claude/rules/observability.md line=38 class=stale
leaf-text: `detail` enum `schemas/diag-line.v1.json` closes at 20 values
source: fct-dsupowm6 «`schemas/diag-line.v1.json` closes the `process-exit` `detail` at 19 values»
replace-with: `detail` enum `schemas/diag-line.v1.json` closes at 19 values
@leaf_edit n=6 leaf=.claude/rules/observability.md line=40 class=retired
leaf-text: the peer pid and start-time comparison on Linux and Windows; `server-verify-failed` reaches Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out
source: fct-2tnvi4o3 «The peer pid and start-time comparison runs on Linux, the one target.»
replace-with: the peer pid and start-time comparison, run on Linux, the one target
