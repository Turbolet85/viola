# Cascade dispositions — 2026-10-09-epoch-3-cleanup

The sweep ran after every body amendment of the pass was applied: `cascade.py sweep` over
`cascade-patterns.toml`, baseline `59e791e9` (the parent of the pre-CI commit). Copied from its listing:
**total (19 patterns) · 67 rows over 20 files** · per class · new 8/5 · standing 36/7 · leaf 21/11 · curation 1/1 ·
base 1/1. Every pattern's control fired on the pre-pass masters. The listing is the trail
`cascade-2026-10-09-epoch-3-cleanup.json`.

## What was searched
The retired claims by their wording and by what they said: `trailing LF` · `not a CR` · `only newlines` ·
`ending in one CR` · `trailing-CR` · `typed as an empty text` · `ends? in a newline|ended in LF|one or more LF` ·
`followed by (a )?newlines?` · `control-character` (every list of refusal details and every refusal order) ·
`only product caller` · `nine labelled cases` · `holds two cases` · `verify_window_` · `package(viola-e2e)` ·
`viola-e2e 30 s` · `when the wrapper returns a refusal` · the stale route numbers `` `:125` `` / `` `:127` ``, `:109`,
`route :93`. Not searched: the plans' Decisions Log stubs and the sidecars (never swept); a claim worded with none of
these tokens.

## Six patterns with 0 rows (the control fired, so the retired wording is gone)
`trailing-cr` · `typed-empty` · `only-caller` · `nine-cases` · `two-cases` · `route-93`.

## Master rows
- `trailing-lf` new, architecture.md:49: this pass's own text ("trailing LF: the founder's ruling … 2026-10-07"), which names the first ruling's width. No change.
- `only-newlines` and `one-cr` in architecture.md:49 and test-plan.md:750 (standing, edited, and one new): the two 2026-10-08 readings, now in the past tense and marked as records of the earlier build. Amended.
- `ends-newline`, architecture.md:81 (×2) and :292: "a wrapped text that ends in a newline" is the CLI's measured behaviour, unchanged; "`send` never types a text that ends in a newline" holds more widely after the strip. No change, read at offsets 1789, 2103 and 1047.
- `by-newlines`, architecture.md:49 and test-plan.md:582: each amended line still says "followed by newlines" and now adds the CR and CRLF clause beside it. Amended.
- `ctrl-char`, 27 master rows:
  - amended (the list or the order gained `empty-text`, or the sentence was widened): architecture.md:133, :136; security-plan.md:226, :234 (new), :543; design-system.md:165 (the next row is the new one), :764 (the next line is the new hint), :822; layout-templates.md:351, :557 (both new text); test-plan.md:233 (new), :582, :589, :638; obs-plan.md:314, :362, :645, :836;
  - security-plan.md:513 (§Error Handling, Channel): it named `control-character` as the one `result.refusal` of this boundary. Folded in this pass: `empty-text` added. It rides the security-plan entry.
  - no change, each read: security-plan.md:227 (the dialog free-text row: `answer` has no empty-text refusal; `validate_paste_text` is unchanged), :402 and :416 (bootstrap-phase history of the `control-character` amendment), :544 (the raw-bytes ban); test-plan.md:314 (the coverage trigger for refused characters, still true), :539 (the `validate_paste_text` unit bullet), :663 (the five control negatives); obs-plan.md:835 (`parse-rejected`'s own detail, a different catalog).
- `verify-window` and `pkg-e2e` and `e2e-30s` in the test-plan key file `bootstrap-phases-derive-for-route-setup-project.md` lines 10, 11, 12 and 20: amended (the class holds three cases; the override order; the kill lines; the 30 s exception). test-plan.md:655 (the fake agent's paste-hint case): no change.
- `wrapper-refuses`, layout-templates.md:351: amended, the client-side clause added after it.

## Leaf rows (21 rows, 11 files) — each re-derived from its amended master in this pass
- `CLAUDE.md:39` (warnings: the typed text) and `:24` (modules: the self-healing entry by title).
- `.claude/rules/events.md:20`, `:34` · `.claude/rules/security.md:25` (the typed text and the empty-text refusal), `:14` (the two Epoch 6 entries by title) · `.claude/rules/api.md:23` (the order).
- `.claude/docs/gotchas.md:84` · `.claude/docs/security-summary.md:63` · `.claude/docs/services/viola-agent-claude.md:24` (both rows) · `services/viola.md:37` · `services/viola-core.md:21` · `services/viola-state.md:6` · `.claude/docs/tests-summary.md:26` (both rows).
- No change: `.claude/docs/security-summary.md:44` (×2, the list of the security plan's architecture amendments, history) · `.claude/docs/gotchas.md:138`, `:139` (`verify_window_`, `package(viola-e2e)`: this chunk's own new entry, already current).
- Leaves read beyond the rows, for a detail list that omits the new value (`unknown-dialog|no-prompt-submitted` over CLAUDE.md, the docs and the rules): none holds one besides the rows above.

## Curation and base rows
- `.claude/rules/testing.md:57` (a Session Addition naming the `verify_window_` prefix): preserve-verbatim, and still true. No change.
- `.andromeda/playbook.md:46` (`:109`): a line number of security-plan in a 2026-09-25 note, not a route citation. No change.

## Registry check
`registry.py check` after the two key-file edits: architecture `0 defect(s)`, test-plan `0 defect(s)`. No label was renamed or added.
