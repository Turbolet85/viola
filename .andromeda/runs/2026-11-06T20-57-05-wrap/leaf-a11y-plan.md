@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=13 class=stale
leaf-text: with assert_cmd / trycmd / portable-pty
source: fct-ms7djxet «`std::process::Command` / literal-pin / portable-pty results are output-discipline stand-ins for SC 1.3.2 / 1.4.1 / 3.3.1, not a11y-tool evidence.»
replace-with: with `std::process::Command` / literal-pin / portable-pty
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=18 class=stale
leaf-text: assert_cmd + trycmd + portable-pty outer-PTY output-discipline checks
source: fct-yzqlnwsx «`std::process::Command` + literal pins; portable-pty outer-PTY SGR checks; the `ubuntu-latest` CI leg»
replace-with: `std::process::Command` + literal pins + portable-pty outer-PTY output-discipline checks
@leaf_edit n=3 leaf=.claude/docs/a11y-summary.md line=33 class=stale
leaf-text: the cli / tui cases use the tests' own assert_cmd, predicates, trycmd and portable-pty pins
source: fct-trh3q5nm «The cli cases drive the `viola` binary through `std::process::Command`, and the tui cases use the tests' own portable-pty pin»
replace-with: the cli cases drive the `viola` binary through `std::process::Command`, and the tui cases use the tests' own portable-pty pin
@leaf_edit n=4 leaf=.claude/rules/a11y.md line=25 class=stale
leaf-text: the tests' own assert_cmd / predicates / trycmd / portable-pty cases
source: fct-dlpkde76 «Output discipline is checked by tests that drive the `viola` binary through `std::process::Command` and pin each fixed string by a literal in its test file»
replace-with: the tests' own `std::process::Command` / literal-pin / portable-pty cases
