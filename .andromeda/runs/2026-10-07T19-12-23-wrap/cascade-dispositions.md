# Cascade dispositions — 2026-10-07-a-send-ending-in-a-newline-is-confirmed

The sweep ran once, after every body of this pass was applied and before any sidecar entry (`cascade.py sweep`,
14 patterns in `cascade-patterns.toml`, baseline `9f2bebe5`, the trail `cascade-2026-10-07-a-send-ending-in-a-
newline-is-confirmed.json`). Every pattern's known-positive control fired on the pre-pass masters. Hits on lines
over 2 000 chars were read by char offset, not from the row's clip. Read over: the seven masters, every file
under `.andromeda/registries/`, the three curation homes, the two judgment bases and the leaf bodies.

What was not looked for: a restatement of the retired claim that uses none of the fourteen patterns' words. The
three masters whose detectors returned nothing were also read by their detectors for the claim however worded
(their twins name the lines).

## Per pattern
- `byte-short` (`one byte short`) — standing 2: `architecture.md:49` at c2811 and `:292`. Both are the kept CLI
  measurement in this pass's own sentences (the newline is dropped; "came back one byte short"). No change.
- `last-newline` — new 12, standing 6, leaf 2.
  - new 12: this pass's own text in architecture (`:286`), security-plan (`:73`, `:74`, `:223` ×2, `:540`),
    test-plan (`:233` ×2, `:238`, `:565`, `:582`, `:750`) and obs-plan (`:636`, `:819`). Amended.
  - standing, edited: `architecture.md:49` (×4), `:70` (×2), `:81` (×4), `:292` (×4). Each is the amended
    passage: the kept measurement beside the new rule. Re-read for an intra-line duplicate of the retired
    mechanism ("not relabelled", "exact match fails", "No remedy is built"): none stands (the `mechanism` and
    `exception` patterns read 0 master rows).
  - standing, not edited: `test-plan.md:564` (the two `hook.rs` last-newline pins, which stay) and `:1063` (a
    fixture file's own trailing newline, another subject). No change.
  - leaf: `.claude/rules/verification-harness.md:39` (the fixture's trailing newline, another subject, no
    change); `.claude/docs/services/viola-agent-claude.md:24` (stale: "a `send` ending in a newline is not
    claimed … unfixed") — re-derived.
- `exception` (`measured exception|unfixed|No remedy is built`) — masters 0. Leaf 1:
  `.claude/docs/services/viola-agent-claude.md:24` — re-derived (the same sentence).
- `mechanism` — 0 rows, control fired at `architecture.md:49`: the retired mechanism's four phrasings stand
  nowhere.
- `sent-text` — new 6, standing 3. New: this pass's "the sent text without its trailing LF characters". Standing:
  `architecture.md:292` (edited, the same phrase), `design-system.md:781` and `layout-templates.md:557` ("A hint
  never quotes the sent text": hint content, true). No change.
- `exact-match` — new 1, standing 3, leaf 1. Standing, all edited: `architecture.md:49` c2834 (past tense, the
  measurement's history), `:286` ("exactly equals the in-flight `send`'s typed text"), `security-plan.md:223`
  c1331 (the match the typed text is compared in). Leaf: `.claude/docs/gotchas.md:84` (stale: "by exact text
  equality (no trim, …)") — re-derived.
- `no-trim` — standing 3, leaf 2. `architecture.md:70` c2704, `security-plan.md:592` and `test-plan.md:564` speak
  of a harness prompt's raw start, another claim, true. Leaves: `gotchas.md:84` (re-derived, above);
  `services/viola-agent-claude.md:24` (the raw prefix, true, no change).
- `never-strip` — standing 4, leaf 3. `architecture.md:136` and `test-plan.md:314`: refused control characters
  are never stripped, true, no change. `security-plan.md:223` c717 and `:540`: edited, the rule kept for refused
  characters. Leaves: `CLAUDE.md:39`, `.claude/rules/security.md:25`, `.claude/docs/security-summary.md:63`
  (×2, it carried the retired reason "stripping would break exact-match confirmation") — all three re-derived.
- `fn-count` (`five functions|13 cases`) — 0 rows, control fired at `test-plan.md:581`: the old count stands
  nowhere.
- `intact` — standing 1: `test-plan.md:238`, edited ("its inner newlines intact, no trailing LF").
- `text-bytes` — new 3, standing 5, leaf 1. Standing: `obs-plan.md:636`, `:641`, `:819` edited; `:865` and
  `:937` say only the count is logged, true, no change. Leaf: `services/viola-pty.md:23` lists the field name,
  true, no change.
- `profile` — new 3, standing 2, curation 1. Standing: `test-plan.md:1080` (an earlier, different measured
  mechanism, true) and `:1209` (edited: the earlier sentence kept, this chunk's mechanism added). Curation:
  `.claude/docs/session-learnings.md:12` (the method that reads a refused profile's own counters). It is not
  stale: the method stands. This chunk met its precondition failing (the refused file was not on the run), which
  goes to curation as an extension of that entry, never a cascade edit.
- `pty-write` — 0 rows, control fired at `security-plan.md:73`: the old entry-point wording stands nowhere.
- `start-test` — the id is a misnomer; the pattern is `scripts/npm-audit.sh`, the last line of the keyed
  `scripts/` tree before this pass, swept to find every place that enumerates the scripts. Standing 16, leaf 8.
  The sixteen master and registry rows speak of the npm audit itself; the one that enumerates scripts,
  `project-directory-structure.md:74`, is edited (the tree gains the census script). Leaves:
  `.claude/docs/commands.md:62` sits in a list of the scripts' commands — re-derived, one line added for the
  census script; `commands.md:14`, `rules/security.md:30`, `security-summary.md:68` and `stack.md:33`, `:34`,
  `:42`, `:49` speak of the audit or of host tools, no change.

## Leaves re-derived (Cascade step 3)
Recomputed from the amended sections, not from the sweep alone; each leaf was also read for the amended
subjects by a second pattern run over `CLAUDE.md`, `.claude/rules/*.md` and `.claude/docs/**`.
- `CLAUDE.md` `GENERATED:setup:warnings` — the bound-every-input line: never strips a refused character; `send`
  types a validated text without its trailing LF. 124 lines.
- `.claude/rules/security.md` §Input validation — the `validate_paste_text` line (outside `## Session
  Additions`).
- `.claude/rules/events.md` — the `prompt-submitted` relabel is matched on the send's typed text (outside
  `## Session Additions`).
- `.claude/docs/security-summary.md` — the paste-text line.
- `.claude/docs/gotchas.md` — the local-command list is consumed by the typed text.
- `.claude/docs/services/viola-agent-claude.md` — `typed_text`, and the dropped newline as a behaviour no `send`
  relies on.
- `.claude/docs/services/viola.md` — `send.rs` types the text without its trailing LF.
- `.claude/docs/tests-summary.md` — Path 2 gains the trailing-newline case.
- `.claude/docs/commands.md` — the census script's line.
- Read and left: `.claude/docs/obs-summary.md` and `.claude/rules/observability.md` (neither names
  `text_bytes`), `.claude/docs/conventions.md`, `.claude/rules/api.md` (the refusal order, unchanged),
  `.claude/rules/testing.md`, `.claude/docs/stack.md`, the other `services/*.md`.

## Binds
- test-plan §3 ↔ obs-plan §3: no harness command, status shape or log format changed; neither §3 was edited.
- a11y-plan schema ↔ obs-plan schema: neither was edited.
- The judgment bases (`playbook.md`, `drift-base.md`): 0 rows on every pattern.
