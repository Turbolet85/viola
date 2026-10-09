# Cascade dispositions — 2026-10-09-inner-cr-and-crlf-in-a-sent-text

The sweep ran after the last body edit of the pass (`cascade.py sweep`, the pattern set in
`cascade-patterns.toml`, the trail `cascade-2026-10-09-inner-cr-and-crlf-in-a-sent-text.json`). Baseline
`3c5e012b`, the parent of the pre-CI commit. Its last lines, copied:

`total (15 patterns) · 47 rows over 14 files`
`per class · new 20/4 · standing 13/6 · leaf 14/8 · curation 0/0 · base 0/0`

## What was searched
Fifteen patterns, each with its control fired on the pre-pass masters:
- the retired definition: `without its trailing CR and LF` · `and nothing else` · `CR or LF that is not at the very
  end` · `the one removal` · a verb before `trailing` (`drop`, `remov`, `strip`);
- the retired inner-case claim: `covers the ending only` · `not covered by the strip` · `is not confirmable` ·
  `no fix … landed` · `carried on the working route` · `still holds the CR` · `inner newlines intact`;
- the subject by name: `inner CR` and its spellings · `typed_text`;
- the retired count: `seventeen labelled cases`.

Not searched by pattern: the words `typed text` alone (too many true uses; the 25 sites of the first read were
each read by offset window before the edits) and `text_bytes` (every definition of it was read; the others name
the field without defining it).

## Rows, by class
**new (20 rows, 4 files)** — this pass's own text in architecture, security-plan, test-plan and obs-plan. No
disposition owed.

**standing (13 rows, 6 files)**
- `architecture.md:49` (`nothing-else`, `inner-cr`, `typed-text-fn`), `security-plan.md:226` (`nothing-else`,
  `typed-text-fn`), `test-plan.md:233`, `:565`, `:582`, `:750`, `obs-plan.md:819`: amended lines where a swept word
  stands in the new wording ("it holds no CR, and nothing else changes"; the rule's own name). Each re-read by
  window for a second copy of the retired claim on the same line: none.
- `design-system.md:847` (×2) and `layout-templates.md:578`: "and nothing else" said of a lamp colour. A true
  claim sharing the words. No change.

**leaf (14 rows, 8 files)** — each re-derived from its amended master:
- `CLAUDE.md:39` (warnings: the typed text) — re-derived. Also re-derived there, with no row: the capability-ledger
  warning (the one relied-on shape with no row yet) and, on the operator's word (`inputs#I7`), the first line of
  the workflow block, now `**Key commands:**`.
- `.claude/rules/events.md:20` — re-derived (the `prompt-submitted` gloss).
- `.claude/rules/security.md:25` — re-derived (its Input validation body, not its Session Additions).
- `.claude/docs/gotchas.md:84` — re-derived (the local-command gloss).
- `.claude/docs/security-summary.md:63` — re-derived.
- `.claude/docs/services/viola-agent-claude.md:24` — re-derived (the rule, its return, the owed row).
- `.claude/docs/services/viola.md:37` — re-derived.
- `.claude/docs/tests-summary.md:26` — re-derived (Path 2: the inner outcome pinned and confirmed).

**curation 0 · base 0** — no hit in a Session Additions section, in CLAUDE.md's session learnings, in
`docs/session-learnings.md`, in the playbook or in the drift base.

## Zero-row patterns
`ending-only`, `not-covered`, `not-confirmable`, `no-fix`, `seventeen-cases`, `inner-newlines`, `one-removal`:
each control fired on the pre-pass text, so each reads as retired in the masters and the leaves. `one-removal`
printed one case variant, "The one removal", in `.claude/docs/security-summary.md:63` before its re-derivation;
that leaf line no longer holds it.

## Read and left
- `layout-templates.md:557` and `design-system.md:765`: the `empty-text` condition and hint. True as written.
- `.claude/rules/api.md:23`, `.claude/docs/services/viola-core.md:21`: "an empty typed text on `send`". True.
- The registries: 0 rows.
- The two sidecars that quote the retired wording as history (`security-plan-amendments.md`) are history and are
  not swept.

## Binds
test-plan §3 and obs-plan §3: neither changed. The a11y violation schema and the obs log schema: neither changed.
