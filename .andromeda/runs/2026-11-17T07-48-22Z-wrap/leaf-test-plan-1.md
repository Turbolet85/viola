@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_read leaf=.claude/rules/testing.md lines=119
last: - 2026-11-16: A CI red read at the operator pass that is no
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=50 class=omission
leaf-text: NEVER parse the rendered child screen; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt.
source: dec-j2mfetpl «Rendered child screen is never parsed — Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads»
replace-with: NEVER parse the rendered child screen; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt (the one exception: the test-only live rig reads its child's screen by compiled literals only into closed words its rehearsal asserts on).
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=53 class=omission
leaf-text: NEVER write a test that auto-approves a dialog.
source: dec-iq4nst55 «No blanket-approval tests — No test auto-approves a dialog. · except: The live rig passes a read trust dialog under its key grant, in its rehearsals and its live starts.»
replace-with: NEVER write a test that auto-approves a dialog (the one exception: the live rig passes a read trust dialog under its key grant, in its rehearsals and its live starts).
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=21 class=stale
leaf-text: only the local `run --local-live`, refused under `CI` (`live-in-ci`)
source: dec-k4rtddnt «One standing committed live rig — Live rounds against the real claude reuse one committed test-only rig, viola-live-rig in viola-e2e: its rehearsals over the fake agent run in CI, its live starts run by hand»
replace-with: only the local `run --local-live`, refused under `CI` (`live-in-ci`), and the live starts of the one standing test-only rig `viola-live-rig` in viola-e2e, run by hand under the founder's start cap for each chunk, each ledgered with its clock time first, its live arm refused under CI and its rehearsals over the fake agent run in CI
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: `--screens` / `--trusted-root` / `--turn-stop` write `Screen.modal|ready|turn`
source: ctr-nfyktr7z «viola-fake-agent screens · trust dialog — Under --screens with an untrusted cwd the fake agent draws claude 2.1.325's unnumbered trust form over the set's recorded modal, the selector first on No»
replace-with: `--screens` / `--trusted-root` / `--turn-stop` write `Screen.modal|ready|turn` (under `--screens` with an untrusted cwd it draws claude 2.1.325's unnumbered trust form over the set's recorded modal, the selector first on No; arrows move the selector, Enter on No exits 1, and SessionStart fires only after Enter on the trust row, which draws the ready screen)
@leaf_edit n=5 leaf=.claude/rules/testing.md line=38 class=omission
leaf-text: never a parse of the rendered screen.
source: dec-j2mfetpl «Rendered child screen is never parsed — Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads»
replace-with: never a parse of the rendered screen. The one exception: the test-only live rig reads its child's screen by compiled literals only into closed words its rehearsal asserts on.
@leaf_edit n=6 leaf=.claude/rules/testing.md line=41 class=omission
leaf-text: No test auto-approves a dialog;
source: dec-iq4nst55 «No blanket-approval tests — No test auto-approves a dialog. · except: The live rig passes a read trust dialog under its key grant, in its rehearsals and its live starts.»
replace-with: No test auto-approves a dialog (the one exception: the live rig passes a read trust dialog under its key grant, in its rehearsals and its live starts);
