# tests · picked accounting · group 6 of 8

## Picked
e2e-test-strategy#5.2 · not in scope · Path 4's answer steps drive the decision bodies the task rules out; the chunk bounds the dialog's inputs, not its decision bodies
e2e-test-strategy#5.3 · not in scope · Path 4's insta decision-body signals, ruled out by the task's own exclusion of Path 4
e2e-test-strategy#7.1 · carried · step 1 names the statusline settings sources (`statusline-source.json`, `settings.json`) whose reads the chunk caps and enumerates
e2e-test-strategy#7.2 · carried · the statusline arm's bounded command run, its `rate_limits`-only write and the malformed `resets_at` reading are input bounds of a reader the chunk caps
e2e-test-strategy#7.3 · not in scope · records the landed governor cases of `tests/cli_budget.rs`; the chunk adds no budget behaviour and the task counts only Path 6's config bounds and file reads
e2e-test-strategy#8.1 · not in scope · Path 7's unverified CLI version gate, ruled out by the task
e2e-test-strategy#8.2 · not in scope · Path 7's verdicts and the `stamped … pass … fail` verify line, ruled out by the task
e2e-test-strategy#11.1 · carried · names the six product readers of `snapshot.json`, the set the chunk's reader enumeration must name with their caps
e2e-test-strategy#12.1 · carried · the MCP schema-violation cases and the hook fail-open matrix's oversize stdin and malformed JSON rows are the chunk's MCP and hook stdin witnesses
e2e-test-strategy#13.1 · not in scope · standing S9 · every E2E test is followed by the error-sanitisation and secret scan and the diagnostics schema conformance check
e2e-test-strategy#14.1 · carried · a malformed revive `--id` refused as clap usage exit 2 is the session-id shape check the chunk re-verifies before a path join; the matrix itself is owed to "Exit-cause code catalogue"
e2e-test-strategy#15.1 · carried · the raw-channel `link` / `unlink` name validation is the channel `driver` / `driven` name witness the task names
e2e-test-strategy#17.1 · carried · the forced vt100 panic, the torn-line count and the forward-compat skip counts are chaos cases the chunk re-verifies
e2e-test-strategy#18.1 · carried · the property suite and fuzz targets over the paste rule, the frame codec at `MAX_FRAME` ± 1, the hook stdin, statusline and agents parsers and the vt100 feed are the chunk's per-boundary property witnesses
e2e-test-strategy#19.1 · carried · the tolerant fixture schema's closed enums witness the closed hook event and decision types the chunk re-verifies
