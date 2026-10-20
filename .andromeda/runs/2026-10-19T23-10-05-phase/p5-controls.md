# P5 controls — chunk 2026-10-19-newer-claude-cli-rows (2026-10-19, HEAD 50e2f761)

Each control fired AS THE ENTRY (`gate.py run --plan … --entry {n} --project-root {clone}`) over a scratch clone
`$TMPDIR/{scratchpad}/p5clone` (`git clone --shared` of the project at HEAD 50e2f761), the plan copied in. Only the
verdicts are recorded here; the inputs lived in the clone.

## Gate 24 — the preservation guard
- clone unmodified → green, exit 0 (as on the real tree).
- `crates/viola-agent-claude/src/ledger.rs` and `tests/support/home.rs` edited, `fixtures/claude/2.1.302/Screen.turn.json`
  minted → green, exit 0 (listed files and the new set pass).
- that, plus `Cargo.toml` edited → red, exit 1.
- that (Cargo.toml restored), plus an untracked `src/new_file.rs` → red, exit 1.

## Gate 20 — the fixture guard (fired with `--void 20:control`)
- clone unmodified → red, exit 1 (no 2.1.302 set yet; the real tree's baseline reading too).
- `fixtures/claude/2.1.302/Screen.turn.json` minted → green, exit 0.
- that, plus `fixtures/claude/2.1.287/Screen.turn.json` modified → red, exit 1.

## Gates 7, 8, 22 — the name filters (typed controls, the filter operand alone differing, through `/usr/bin/bash -o pipefail -c`)
- `bash scripts/agent-run.sh run --unit --filter 'test(/check_input_box_signature_/)'` → exit 0, `"ok":true`,
  nextest-unit passed 1 failed 0 (an existing case): the filter form selects and reads green.
- `bash scripts/agent-run.sh run --integration --filter 'test(/verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal/)'`
  → exit 0, `"ok":true`, nextest-integration passed 1 failed 0.
