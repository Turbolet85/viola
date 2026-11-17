@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=119
last: - 2026-11-16: A CI red read at the operator pass that is not
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: passes it `--transcript` beside `--framing` (the framing-free verify helper passes none)
source: dec-tgem7lte «Every test-started viola verify and every harness boot stamping runs under a test-owned user home whose config file is seeded empty and owner-only, and the fake agent is told that config file.»
replace-with: passes it `--transcript` beside `--framing` (the framing-free verify helper passes none), and every test-started `viola verify` and every harness boot stamping runs under a test-owned user home whose config file is seeded empty and owner-only, the fake agent told that config file by `--trust-config` (a `--record` verify keeps the real `HOME` and points a kept `CLAUDE_CONFIG_DIR` at the test-owned user home instead)
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: `--screens` / `--trusted-root` / `--turn-stop` write
source: ctr-kxqcdc74 «The fake agent reads a trust record from the named config file by its own key rule, the nearest ancestor holding a git entry else the canonical cwd, through a bounded read; any failure reads untrusted.»
replace-with: `--trust-config <file>` reads a trust record from that config file by its own key rule (the nearest ancestor holding a git entry else the canonical cwd, through a bounded read; any failure reads untrusted), `--screens` / `--trusted-root` / `--turn-stop` write
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: holds a cleared screen after the long paste's turn;
source: ctr-5opdrt5r «The test-only hold --trust-record-blank makes a start trusted by the trust record alone draw no screen, holding verify's Run F window open for a stop case.»
replace-with: holds a cleared screen after the long paste's turn; the test-only hold `--trust-record-blank` makes a start trusted by the trust record alone draw no screen, holding verify's Run F window open for a stop case;
@leaf_edit n=4 leaf=.claude/rules/testing.md line=34 class=omission
leaf-text: and no component of its path is named `home`.
source: dec-tgem7lte «Every test-started viola verify and every harness boot stamping runs under a test-owned user home whose config file is seeded empty and owner-only, and the fake agent is told that config file.»
replace-with: and no component of its path is named `home`. Every test-started `viola verify` and every harness boot stamping runs under a test-owned user home whose config file is seeded empty and owner-only, and the fake agent is told that config file (`--trust-config`); a `--record` verify, whose scrub reads `HOME`, keeps the real `HOME` and points a kept `CLAUDE_CONFIG_DIR` at the test-owned user home instead.
@leaf_edit n=5 leaf=.claude/rules/testing.md line=34 class=omission
leaf-text: A home a test gives to `viola verify --record`
source: dec-miolq5qd «A census over the test sources holds every test-started viola verify start tied to a test-owned home, with a closed exempt list whose every entry carries its reason, and is proven red on a planted untied start.»
replace-with: A census over the test sources (`every_test_verify_runs_under_a_test_home`) holds every test-started `viola verify` start tied to a test-owned home, with a closed exempt list whose every entry carries its reason, proven red on a planted untied start. A home a test gives to `viola verify --record`
