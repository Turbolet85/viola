@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=74
last: - 2026-11-17: A change to what the fake agent draws under `--
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=28 class=omission
leaf-text: `viola verify --home` against it with
source: dec-tgem7lte «Every test-started viola verify and every harness boot stamping runs under a test-owned user home whose config file is seeded empty and owner-only, and the fake agent is told that config file.»
replace-with: `viola verify --home` under a test-owned user home (its config file seeded empty and owner-only, the fake agent told that config file by `--trust-config`) against it with
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=42 class=omission
leaf-text: taken against the fake agent at 2.1.287 over `fixtures/claude`
source: dec-tgem7lte «Every test-started viola verify and every harness boot stamping runs under a test-owned user home whose config file is seeded empty and owner-only, and the fake agent is told that config file.»
replace-with: taken against the fake agent at 2.1.287 over `fixtures/claude` under a test-owned user home whose config file is seeded empty and owner-only, the fake agent told that config file (a `--record` verify keeps the real HOME and points a kept `CLAUDE_CONFIG_DIR` at the test-owned user home instead)
@leaf_edit n=3 leaf=.claude/rules/verification-harness.md line=42 class=omission
leaf-text: finds no untied viola spawn in any test source)
source: dec-miolq5qd «A census over the test sources holds every test-started viola verify start tied to a test-owned home, with a closed exempt list whose every entry carries its reason, and is proven red on a planted untied start.»
replace-with: finds no untied viola spawn in any test source; `every_test_verify_runs_under_a_test_home` holds every test-started `viola verify` start tied to a test-owned home, with a closed exempt list whose every entry carries its reason, proven red on a planted untied start)
@leaf_edit n=4 leaf=.claude/rules/verification-harness.md line=41 class=omission
leaf-text: `--settings <file>` is accepted, its value consumed and ignored.
source: ctr-kxqcdc74 «contract/fake-agent-cli/trust-config · viola-fake-agent command line · --trust-config — The fake agent reads a trust record from the named config file by its own key rule, the nearest ancestor holding a git entry else the canonical cwd, through a bounded read; any failure reads untrusted.»
replace-with: `--settings <file>` is accepted, its value consumed and ignored. `--trust-config <file>` reads a trust record from that config file by its own key rule (the nearest ancestor holding a git entry, else the canonical cwd) through a bounded read; any failure reads untrusted.
@leaf_edit n=5 leaf=.claude/rules/verification-harness.md line=41 class=omission
leaf-text: (1 000 ms at most, holding the Stop hook's receipt to force a race window open).
source: ctr-5opdrt5r «contract/fake-agent-cli/trust-record-blank · viola-fake-agent command line · --trust-record-blank — The test-only hold --trust-record-blank makes a start trusted by the trust record alone draw no screen, holding verify's Run F window open for a stop case.»
replace-with: (1 000 ms at most, holding the Stop hook's receipt to force a race window open), and the test-only hold `--trust-record-blank` (a start trusted by the trust record alone draws no screen, holding verify's Run F window open for a stop case).
