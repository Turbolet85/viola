# P5 controls — 2026-10-19-overseer-switch-to-viola

Scratch clone: `$TMPDIR/{scratchpad}/clone10`, `git clone --shared` of the project at HEAD `f23586fb`, the plan copied
in. Each entry was fired through `gate.py run --entry {n} --project-root <clone>` unless marked typed.

- **Entry 6** (the new unit cases by filter): real tree red, `nextest-exit-4`, 0 tests run, 2377 skipped (the cases do
  not exist yet; the subject steps 1–2 build). Control, typed: the same filter form over the existing
  `hook_dialog_expires_at_the_deadline_on_the_injected_clock` reads exit 0, `"ok":true`, nextest-unit 1 passed
  0 failed.
- **Entry 10** (the preservation guard):
  - clone unmodified: green, matching the real tree.
  - must-pass, green: `crates/viola-agent-claude/src/dialog.rs` and `tests/hook_events.rs` edited, and
    `evidence/switch-round.md` minted.
  - must-fail, red (exit 1): `Cargo.toml` edited.
  - must-fail, red (exit 1): an untracked `src/new_file.rs`, with `Cargo.toml` restored.
- **Entry 23** (the seven ungated acts): real tree red, exit 2, file absent. Must-pass in the clone: green, last
  line 7, over eight act lines, `takeover` among them (the `-E` set excludes it).
- **Entry 24** (the stamp line): real tree red, exit 2, file absent. Must-pass in the clone: green, last line 1, over
  the literal as `src/cmd/verify.rs:277` prints it (`stamped {version}  {} pass  {failed} fail`, two spaces).
- **Entry 25** (the gated takeover reader), typed, because the tool does not fire a gated entry: real tree exit 2,
  file absent. Clone with one `takeover` line: exit 0, last line 1.
