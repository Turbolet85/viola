# Cascade dispositions — 2026-11-20-permissionrequest-question-answered (wrap 2026-11-20T13-41-57Z)

## Sweep (step 2)
The search: `cascade.py sweep` over `cascade-patterns-architecture-1.toml` (`arch1-2`, fixed `PermissionRequest`,
control `contract/answer-refusals/unknown-dialog` r1), `cascade-patterns-architecture-2.toml` (`arch2-1`, fixed
`PermissionRequest`, control `decision/permission-request-question-stays-null` r4) and main's own
`cascade-patterns.toml` (`main-stays-null` — the retired claim's own verbs: `stays null|left to the human|unanswerable|
first raised by PermissionRequest|first raised through PermissionRequest|question-stays-null`; `main-pq-stamp` — the
kept-stamp row's old `CS or WS`). The security-plan and test-plan dry runs wrote no pattern file (`patterns: none: …
changed prose alone`); their changed rows' deltas are read by step 3's leaves. The first sweep
(`sweep-pass-1.1.txt`, the two tool files alone) is superseded by the second, which adds main's patterns as one call.
Listing `sweep-pass-2.1.txt`, read whole. `total (4 patterns) · 66 rows over 10 files` · `per class · new 5/1 ·
standing 39/4 · leaf 20/5 · curation 2/1 · base 0/0`.

Rows (each `arch1-2` row is also an `arch2-1` row, the same fixed term):
- architecture 155 `decision/permission-request-question-answered` (edited) — amended: this wrap's own row, no retired
  mechanism standing in it (the old «stays null» rule retired).
- architecture 591 `contract/hook-contract/permission-request` (edited) — amended.
- architecture 262 `contract/hook-tiers/…`, 390 `contract/hook-to-kind-map/permissionrequest`, 515
  `contract/channel-methods/hook-dialog-continuation` — no change: each true as worded (the sync tier, the kind map
  and the continuation are unchanged by this chunk; the kind map is founder-forks F2).
- security-plan 20 `decision/driver-permission-policy-outside-viola`, 75, 82 `contract/wrapper-channel-methods/answer`,
  605, 640 — no change: each names PermissionRequest allow as a dialog answer, true as worded (82 is founder-forks F4).
- test-plan 96, 97, 119, 725, 726, 727, 842 — no change: the permission and plan bodies and the hook list, unchanged.
- test-plan 98, 352, 728 — amended (this wrap's sets).
- test-plan 609 `contract/viola-answer/unverified-cli` («the dialog left to the human») — no change: the unverified
  refusal still leaves a dialog to the human, which the new architecture row keeps.
- obs-plan 419 `dec-5ptahhpb` («the reply stays null» at the hold's end) — no change: the expiry, true as worded.
- curation `.claude/docs/session-learnings.md:58` — no change: a plan step's PermissionRequest learning, not this claim.
- leaf rows (20 over 5 files) — step 3's set: `.claude/rules/events.md`, `.claude/docs/gotchas.md`,
  `.claude/docs/security-summary.md` (73: `CS or WS` — stale, the kept stamp now names PQ),
  `.claude/docs/services/viola-agent-claude.md`, `.claude/docs/tests-summary.md` (28: «a question first raised by
  PermissionRequest logged…» — stale wording of the retired rule).
- base rows: none.

## Leaves (step 3)
`cascade.py leaves` — pass mode, epoch «Epoch 4 — Driver reliability: ledger rows and bounds» open (markerless 3);
`total · pass · 9 group(s) · 23 leaves · 407389 B · unmapped 2`; no `not compared` row. One comparer a group, batch
`leaf`, `check` 9 of 9 pass. `leaves --apply --dry-run`: `total · blocks 5 · applied 5`.
- architecture-1 (pass) — 1 finding, applied: `.claude/docs/services/viola-agent-claude.md:20` omission (the PermissionRequest question's `allow` + `updatedInput` body).
- architecture-2 (pass) — 1 finding, applied: `.claude/docs/services/viola.md:41` omission (the held PermissionRequest question).
- architecture-3 (pass) — 1 finding, applied: `.claude/rules/events.md:20` omission (the same, beside the kind map).
- architecture+security-plan, architecture+security-plan+test-plan, architecture+test-plan, test-plan-2 (pass) — 0 findings.
- security-plan (pass) — 1 finding, applied: `.claude/docs/security-summary.md:73` stale («CS or WS» → «CS, WS or PQ»).
- test-plan-1 (pass) — 1 finding, written by hand: `.claude/docs/tests-summary.md:28` stale — the comparer's replace-with dropped the still-true «a late `answer` exit 13»; the hand write keeps it (draft landed by `rec.py harness land`, tx 3442).
- unmapped: `.claude/rules/host-linux.md` (host — a statement no source states, no step-3 edit); `.claude/rules/testing-src.md` (no provenance to a changed master; not reached by these deltas — no edit).
- Counts: applied 4 (the tool's `total · blocks 5 · applied 4 · held 1`) · by hand 1 · declined 0. No leaf now holds a figure the code states from this change (`inputs#I11` item 8): none of the five edits adds a count or a version.
- The curation hit `.claude/docs/session-learnings.md:58` — no change (a plan-step learning, not this claim).
