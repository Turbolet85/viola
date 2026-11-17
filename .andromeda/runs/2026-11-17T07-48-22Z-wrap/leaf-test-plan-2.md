@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=73
last: - 2026-11-14: A test-only recorder that replaces every off-l
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=41 class=omission
leaf-text: Exits on `\x03` or at stdin EOF, first waiting for a hook still running and starting no other;
source: ctr-iujs54sa «The fake agent exits on \x03 or at stdin EOF, and exits 1 on Enter on the trust form's No; before it exits it waits for a hook still running and starts no other.»
replace-with: Exits on `\x03` or at stdin EOF, and exits 1 on Enter on the trust form's No, first waiting for a hook still running and starting no other;
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=41 class=omission
leaf-text: `--settings <file>` is accepted, its value consumed and ignored.
source: ctr-nfyktr7z «Under --screens with an untrusted cwd the fake agent draws claude 2.1.325's unnumbered trust form over the set's recorded modal, the selector first on No, exit directly above the recorded trust row»
replace-with: `--settings <file>` is accepted, its value consumed and ignored. Under `--screens` with an untrusted cwd it draws claude 2.1.325's unnumbered trust form over the set's recorded modal (selector first on No, directly above the recorded trust row, row 16 with no recorded modal); arrows move the selector, Enter on No exits 1, and the launch SessionStart fires only after Enter on the trust row, which draws the ready screen.
@leaf_edit n=3 leaf=.claude/rules/verification-harness.md line=13 class=omission
leaf-text: the `scripts/agent-run.*` shims, `viola-harness`, the rstest fixture chains and the fake agent.
source: dec-k4rtddnt «Live rounds against the real claude reuse one committed test-only rig, viola-live-rig in viola-e2e: its rehearsals over the fake agent run in CI, its live starts run by hand under the founder's start cap»
replace-with: the `scripts/agent-run.*` shims, `viola-harness`, the rstest fixture chains and the fake agent, and the one standing test-only live rig `viola-live-rig` in viola-e2e (its rehearsals over the fake agent run in CI; its live starts run by hand under the founder's start cap per chunk, each ledgered with its clock time first; its live arm refuses under CI; it reads the child's screen by compiled literals only into closed words its rehearsal asserts on, and passes a read trust dialog under its key grant).
