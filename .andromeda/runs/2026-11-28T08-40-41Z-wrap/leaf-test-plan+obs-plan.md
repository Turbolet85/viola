@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=19 class=stale
leaf-text: `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`
source: dec-6rrj6j2v «decision/log-file-basenames-follow-tests · Log file names follow tests — Process-log files are <home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>,console}.ndjson, as tests require.»
replace-with: `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>,console}.ndjson` (`console` home-level, its lines carrying no `instance` key)
@leaf_edit n=2 leaf=.claude/rules/observability.md line=36 class=omission
leaf-text: `mcp` main-thread → no `process-exit` line (`obs::internal_error_exit_line` covers `Run` and `Cli` only), then exit 1 with nothing on stdout.
source: ctr-n4duyqgy «contract/panic-outcome-by-role/console · panic outcome by role · console — After a dispatch error or caught panic, console restores the terminal first, then logs process-exit{subject:"self", exit_code:1, detail:"inte»
replace-with: `mcp` main-thread → no `process-exit` line (`obs::internal_error_exit_line` covers `Run` and `Cli` only), then exit 1 with nothing on stdout; `console` → restores the terminal first, then `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` in `console.ndjson`, `error: internal error` once on stderr and exit 1 (the catch site classes it as `cli`).
@leaf_edit n=3 leaf=.claude/rules/observability.md line=21 class=omission
leaf-text: with no role file opened and the directory's mode left as found.
source: ctr-rjgdvz3d «contract/strict-modes-refusal/console · Strict-modes refusal · viola console — Exit 1 with the home pair, made before any log opens: no line, the mode left as found.»
replace-with: with no role file opened and the directory's mode left as found. `viola console` runs its strict-modes check before the log init too: a refusal is exit 1 with the home pair, no line and the mode left as found.
