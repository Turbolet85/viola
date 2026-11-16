# P5 controls — 2026-11-16-survey-key-keeps-the-wheel (validation check 4 (9))

Scratch clone `git clone --shared` of the project at HEAD `498b45333fd9`,
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/1935997b-f457-4092-be9c-b16de930e4c8/scratchpad/ctl-clone`,
each entry fired as itself through `gate.py run --chunk 2026-11-16-survey-key-keeps-the-wheel --docs-root <root>
--entry {n} --project-root <clone>`.

- entry 9, unmodified: `green · exit 0` (last line 0) — as on the tree.
- entry 9, a minted untracked `fixtures/claude/2.1.287/Screen.control-minted.json`: `red · last line '0' ✗ (last: '1')`.
- entry 10, unmodified: `green · exit 1` (last line 0) — as on the tree.
- entry 10, a minted `src/control_minted.rs` naming `CLAUDE_FORCE_DISPLAY_SURVEY`: `red · last line '0' ✗ (last: '1')`.
- entry 12 on the tree: `red · exit 2` — the rig does not exist yet (an absent input): it proves only that the entry
  can read red. The rig is an instrument this chunk builds; its known-verdict controls are its own `rehearse` arms
  (plan step 1: the refused cap, the refused version, no key without the literal, the removal on a normal end, a
  refusal and an injected exception), run as entry 12 before the first live start.
