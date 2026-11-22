@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=80
last: - 2026-11-20: A plan's non-leg entry that reads a file a rou
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=41 class=omission
leaf-text: `--vt100-panic-bytes` (after its `start` receipt it writes `e4 b8 ad` once to its terminal; under a 24×1 PTY that panics vt100's feed),
source: ctr-l5pnzopa «contract/fake-agent-cli/stop-reading · viola-fake-agent command line · --stop-reading — The argv-only --stop-reading makes the fake agent never read stdin, parked forever with its terminal left raw, so a paste into it blocks.»
replace-with: `--vt100-panic-bytes` (after its `start` receipt it writes `e4 b8 ad` once to its terminal; under a 24×1 PTY that panics vt100's feed), the argv-only `--stop-reading` (never reads stdin, parked forever with its terminal left raw, so a paste into it blocks),
