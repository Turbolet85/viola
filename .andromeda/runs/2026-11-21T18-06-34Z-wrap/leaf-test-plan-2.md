@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=80
last: - 2026-11-20: A plan's non-leg entry that reads a file a rou
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=41 class=stale
leaf-text: `--settings <file>` is accepted, its value consumed and ignored.
source: ctr-r2ad2s23 «--settings <file> is accepted; the fake agent reads its cross-session inbox only when the value holds the accept settings, and otherwise ignores it.»
replace-with: `--settings <file>` is accepted; the fake agent reads its cross-session inbox only when the value holds the accept settings, and otherwise ignores it (its cross-session sender resolves its target by the uds socket address alone, delivers only to its sibling and refuses an address not of the uds shape).
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=38 class=omission
leaf-text: and under `--stop-crons` each Stop's `session_crons`.
source: ctr-j4oujm2y «save the fields a prompt, a resume, a compact replay, the launch SessionStart under --transcript or a Stop under --stop-crons sets, and the cross-session sender's synthetic send-tool hooks.»
replace-with: and under `--stop-crons` each Stop's `session_crons`; the cross-session sender's PreToolUse and PostToolUse hooks carry synthetic payloads naming the send tool, never recorded bytes.
@leaf_edit n=3 leaf=.claude/rules/verification-harness.md line=45 class=omission
leaf-text: which `contract_fake_agent_drift` replays byte for byte (contract suite);
source: dec-kj6lptge «For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.»
replace-with: which `contract_fake_agent_drift` replays byte for byte (contract suite; a hook whose payload names the send tool is left out of the dialog tier, so a side sender's hooks shift no position, every other dialog-tier hook still compared by position, event and bytes);
@leaf_edit n=4 leaf=.claude/rules/verification-harness.md line=28 class=omission
leaf-text: `viola verify --home` under a test-owned user home (its config file seeded empty and owner-only, the fake agent told that config file by `--trust-config`)
source: dec-3nvt7nvr «Every test or harness viola verify that stamps runs with a short owner-only runtime dir the test or harness owns for the stamp's life.»
replace-with: `viola verify --home` under a test-owned user home (its config file seeded empty and owner-only, the fake agent told that config file by `--trust-config`) and a short owner-only runtime dir the harness owns for the stamp's life (verify reads `XDG_RUNTIME_DIR` once for the cross-session address)
@leaf_edit n=5 leaf=.claude/rules/verification-harness.md line=42 class=omission
leaf-text: under a test-owned user home whose config file is seeded empty and owner-only
source: dec-3nvt7nvr «Every test or harness viola verify that stamps runs with a short owner-only runtime dir the test or harness owns for the stamp's life.»
replace-with: under a test-owned user home whose config file is seeded empty and owner-only and a short owner-only runtime dir the test owns for the stamp's life
