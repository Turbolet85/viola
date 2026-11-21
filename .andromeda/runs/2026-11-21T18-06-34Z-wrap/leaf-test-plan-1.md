@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=127
last: - 2026-11-21: A whole re-record of a fixture set moves its r
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=21 class=stale
leaf-text: `--settings <file>` accepted, its value consumed and ignored
source: ctr-r2ad2s23 «--settings <file> is accepted; the fake agent reads its cross-session inbox only when the value holds the accept settings, and otherwise ignores it.»
replace-with: `--settings <file>` accepted, its cross-session inbox read only when the value holds the accept settings and the value otherwise ignored
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: and writes an ndjson `--receipt`;
source: dec-dv3yfye5 «The fake agent's cross-session sender resolves its target by the uds socket address alone, delivers only to its sibling and refuses an address not of the uds shape; its receiver reads its inbox only»
replace-with: and writes an ndjson `--receipt`; its cross-session sender resolves its target by the uds socket address alone, delivers only to its sibling and refuses an address not of the uds shape (its PreToolUse and PostToolUse hooks synthetic payloads naming the send tool), and its receiver reads its inbox only under the accept settings;
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: replays every dialog variant and every framing variant byte for byte
source: dec-hc4i3vva «A hook whose payload names the send tool is left out of the dialog tier, so a side sender's hooks shift no position»
replace-with: replays every dialog variant (a hook whose payload names the send tool left out of the dialog tier, so a side sender's hooks shift no position) and every framing variant byte for byte
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: the fake agent told that config file by `--trust-config`
source: dec-3nvt7nvr «Every test or harness viola verify that stamps runs with a short owner-only runtime dir the test or harness owns for the stamp's life.»
replace-with: the fake agent told that config file by `--trust-config`, and every stamping `viola verify` of a test or the harness given a short owner-only runtime dir (`XDG_RUNTIME_DIR`) it owns for the stamp's life (the rig rehearsal sets its own only where the host sets none)
@leaf_edit n=5 leaf=.claude/rules/testing.md line=34 class=omission
leaf-text: and the fake agent is told that config file (`--trust-config`);
source: dec-3nvt7nvr «Every test or harness viola verify that stamps runs with a short owner-only runtime dir the test or harness owns for the stamp's life.»
replace-with: and the fake agent is told that config file (`--trust-config`); every test or harness `viola verify` that stamps runs with a short owner-only runtime dir (`XDG_RUNTIME_DIR`) the test or harness owns for the stamp's life (verify reads it once to compose the cross-session address; the rig rehearsal sets its own only where the host sets none);
@leaf_edit n=6 leaf=.claude/rules/testing.md line=35 class=stale
leaf-text: scrubbed of absolute paths and usernames
source: dec-lrjctiyu «Fixtures never come from real prompts and are checked for home, drive-letter and user-folder paths and usernames.»
replace-with: scrubbed of home, drive-letter and user-folder paths and usernames
