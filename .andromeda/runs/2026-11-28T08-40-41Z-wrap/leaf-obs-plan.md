@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `.
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=9 class=stale
leaf-text: four process roles, the instrumentable crates and six must-trace paths make it Standard-class, but upstream bans egress, new listeners, stdout on three roles
source: dec-cvjveydd «why: Upstream bans outbound network calls, reserves stdout on 4 of 5 roles, and bans Tokio and C crates in the sync graph.»
replace-with: five process roles, the instrumentable crates and six must-trace paths make it Standard-class, but upstream bans egress, new listeners, stdout on four of five roles
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=17 class=stale
leaf-text: codes-only role files `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`
source: dec-6rrj6j2v «decision/log-file-basenames-follow-tests · Log file names follow tests — Process-log files are <home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>,console}.ndjson, as tests require.»
replace-with: codes-only role files `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>,console}.ndjson` (`console` home-level, its lines with no instance key)
@leaf_edit n=3 leaf=.claude/docs/obs-summary.md line=18 class=stale
leaf-text: `process` (`run|hook|mcp|cli`), `instance`
source: ctr-4v4eq7hx «contract/process-log-line/process · the process log line · process — One of run, hook, mcp, cli, console; cli is a short-lived verb with a resolved instance, writing cli-<name>.ndjson, and console is the fleet console, writing console.ndjson with no instance.»
replace-with: `process` (`run|hook|mcp|cli|console`), `instance` (absent on `console` lines)
@leaf_edit n=4 leaf=.claude/docs/obs-summary.md line=42 class=omission
leaf-text: `state-recovered` · `parse-rejected` · `trust-record`
source: ctr-pspx3lhv «contract/diag-line-fields/console-key · diagnostic line additive fields · console-key — Logged once per key read with key, one of quit, interrupt or unbound, required; the key's bytes never reach a line.»
replace-with: `state-recovered` · `parse-rejected` · `console-frame` (once per draw: required `cols`, `rows`, `fits`; an unread size logs 0x0, not fitting) · `console-key` (once per key read: required `key`, one of `quit|interrupt|unbound`; the key's bytes never reach a line) · `trust-record`
@leaf_edit n=5 leaf=.claude/docs/obs-summary.md line=67 class=stale
leaf-text: (`internal-error` on `run` and `cli` only: the `mcp` role writes no self `process-exit` line)
source: ctr-n4duyqgy «contract/panic-outcome-by-role/console · panic outcome by role · console — After a dispatch error or caught panic, console restores the terminal first, then logs process-exit{subject:"self", exit_code:1, detail:"internal-error"} in console.ndjson»
replace-with: (`internal-error` on `run`, `cli` and `console` only — `console` restoring the terminal first, then writing it in `console.ndjson`: the `mcp` role writes no self `process-exit` line)
@leaf_edit n=6 leaf=.claude/docs/obs-summary.md line=67 class=omission
leaf-text: a strict-modes refusal made before any process log opens (
source: dec-4nz2tqvl «except: A strict-modes check refused before any log opens, or viola console's terminal check, writes no line; its cause is the fixed stderr pair or the --json document.»
replace-with: `viola console`'s refused terminal check writes no line, its cause the fixed stderr pair; a strict-modes refusal made before any process log opens (
