@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=44
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/security.md line=12 class=stale
leaf-text: `unwrapped-name` for anything else
source: ctr-qimnrfzc «contract/instance-unreachable-detail/unwrapped-name · instance-unreachable detail · unwrapped-name — A cause of exit 21 and of MCP's instance-unreachable result: no endpoint with anything else at the instance path (absent or a regular file)»
replace-with: `unwrapped-name` for an absent path or a regular file
@leaf_edit n=2 leaf=.claude/rules/security.md line=14 class=omission
leaf-text: (pinned `bin/<version>-<hash>/viola` is 0700);
source: dec-sigujrfo «decision/logs-owner-only · Logs owner-only — The event log, heartbeat and diagnostics files are 0600 inside 0700 directories, and their owner-only append open never follows a symbolic link: a link at the path is a failed open»
replace-with: (pinned `bin/<version>-<hash>/viola` is 0700); the event log, heartbeat and diagnostics files' owner-only append open never follows a symbolic link (a link at the path is a failed open) and sets 0600 through the open handle;
