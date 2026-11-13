# Must-pass readings — check 4 (9), P5

Clone: `git clone --shared` of the project, detached at HEAD `cb0b16abf577bba34326933e4d635756a8142ffb`, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/dd303b0f-08c1-4245-a89b-ded9cce6a7d3/scratchpad/mustpass`.
Input (built by `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/dd303b0f-08c1-4245-a89b-ded9cce6a7d3/scratchpad/mustpass_input.py`,
left uncommitted in the clone): `tests/support/digits.rs` holding `pub fn digit_runs(`; the three
`tests/security_negatives_*.rs` copies renamed away (the server_identity `digit_runs_reads_whole_numbers_only` case
kept); `src/cmd/run.rs`'s `fn parse_name(` renamed away; `derive(Redact, …)` added once in
`crates/viola-agent-claude/src/hook.rs`, `crates/viola-agent-claude/src/dialog.rs` and `crates/viola-mcp/src/tools.rs`.
Each entry fired as itself: `gate.py run --chunk 2026-11-13-rows-proposal-rule-work --docs-root . --entry {n} --project-root {clone}`.

- entry 19 (`grep -rln 'fn digit_runs' tests`, plan rev 1) · `red · last line 'tests/support/digits.rs' ✗ (last: 'tests/security_negatives_server_identity…')`
  — the probe also matched the kept case `fn digit_runs_reads_whole_numbers_only`; re-spelt `fn digit_runs(` (plan rev 2)
- entry 19 (`grep -rln 'fn digit_runs(' tests`, plan rev 2) · `green · exit 0 · 24 B`
- entry 20 (`grep -rn 'fn parse_name' src`) · `green · exit 0 · 86 B`
- entry 23 (`grep -rln 'derive(.*Redact' crates/viola-agent-claude/src crates/viola-mcp/src`) · `green · exit 0 · 108 B`

Entries 21 and 22 assert a count (`last line 0`) over files that stand today; their baseline reds read the subject
this chunk removes, so no must-pass reading is owed for them.
