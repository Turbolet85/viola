@leaf_group group=test-plan
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=109
last: - 2026-11-06: A sweep for a `cfg` predicate reads the predic
@leaf_read leaf=.claude/rules/verification-harness.md lines=72
last: - 2026-10-20: `target/debug/viola` is a hard link cargo re-p
@leaf_edit n=1 leaf=.claude/rules/testing.md line=28 class=stale
leaf-text: Time only through the injected `viola_core::Clock` / mock_instant (sync; in no manifest yet), `start_paused` (Tokio; tokio `test-util` in no manifest yet, and `viola-mcp` owns no timer — its `wait` bound is the pure `wait_timeout(given)`, a unit case)
source: dec-d6bwecvy «Time is faked only through the injected viola_core Clock, with no mock_instant; the MCP adapter runs no tokio timer, so no paused test clock, and a pure decision takes its instant as an argument.»
replace-with: Time only through the injected `viola_core::Clock` (no mock_instant; `viola-mcp` runs no tokio timer, so no paused test clock — its `wait` bound is the pure `wait_timeout(given)`, a unit case)
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=25 class=stale
leaf-text: Every command prints exactly one JSON document on stdout starting
source: ctr-bzimviou «Every harness command but logs and supervise prints exactly one JSON document on stdout, starting with v 1, cmd naming the command and ok, in that key order; logs streams ndjson lines unless it fails»
replace-with: Every command but `logs` (which streams ndjson lines unless it fails) and `supervise` (which prints nothing) prints exactly one JSON document on stdout starting
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=16 class=stale
leaf-text: each prints one JSON document `{"v":1,"cmd":…,"ok":…}`
source: ctr-bzimviou «Every harness command but logs and supervise prints exactly one JSON document on stdout, starting with v 1, cmd naming the command and ok, in that key order; logs streams ndjson lines unless it fails»
replace-with: each but `logs` prints one JSON document `{"v":1,"cmd":…,"ok":…}` (`logs` streams ndjson lines unless it fails)
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=33 class=stale
leaf-text: property (6 parsers)
source: dec-dqjttp65 «proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, statusline and agents parsers, the vt100 feed and prompt-submitted»
replace-with: property (7 parsers)
