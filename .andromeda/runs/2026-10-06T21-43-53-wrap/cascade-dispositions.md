# Cascade dispositions — 2026-10-06-local-command-and-paste-framing-rows

The sweep: `cascade.py sweep --patterns-file cascade-patterns.toml`, 18 patterns, run twice — after every body was
applied (101 lines) and again after the leaves were re-derived (90 lines). Every pattern's known-positive control
fired on the pre-pass masters (baseline `2fbc9545`). The trail is `cascade-2026-10-06-local-command-and-paste-framing-rows.json`.
Long lines were read by offset windows (a scratch site finder over the same files; `architecture.md:91` is 7 614
chars after the pass).

## What was looked for
- the retired counts: `fourteen`, `/14]`, `14 pass`, `` `/14` ``, `[&str; 14]`, `is 14 today`, `12 pass`, `15th row`,
  `five argv options`, `seven 300 ms`;
- the retired mechanisms and claims: `byte for byte` (the unwrap keeping the ends), `No local-command row`,
  `one bracketed paste`, `backslash after` (the general tag-escaping claim), `paste-wrap` (the fake agent's
  threshold), `session transcript`, `the spine and the dialog` / `spine fixtures and dialog variants`,
  `local-command rows` as something still to land;
- the retired owner name `Local-command and paste-framing rows`, the flag list token `--dialogs`, the version
  token `2.1.288`, and `drift-only`.
Not looked for: `unconfirmable` as a design target (unchanged by this chunk; `send` is untouched), `four
interactive` (still four PTY runs), `six spawn` (unchanged).

## Master and key-file rows (second listing)
- `fourteen` · new 3 — `architecture.md:91`, `security-plan.md:608`, `test-plan.md:561`: this pass's own text
  ("a stamp holding exactly the fourteen older ids … reads unverified"). No change.
- `n14`, `12pass`, `15th`, `five-argv`, `no-lc-row`, `old-owner`, `spine-dialog`, `lc-rows-land` · 0 master rows:
  every site was amended.
- `ends-kept` · standing 6 — `architecture.md:81` (amended; now "everything else outside a pair is kept byte for
  byte", true); `test-plan.md:438` ×4, `:1031`, `:1055`, `a11y-plan.md:573` and the a11y key file
  `keyboard-test-harness.md:8`: fixture and stream comparisons, a different claim sharing the phrase. No change.
- `one-paste` · standing 1 — `test-plan.md:233`: `send` writes one bracketed paste + Enter; true, no change
  (the same line's `unconfirmable` clause was amended).
- `dialogs-flag` · standing 9 — `architecture.md:91` (amended, `--dialogs --framing`), `:355` (the option's own
  description; `--framing` added beside it), `test-plan.md:438`, `:654`, `:664`, `:1077`, `obs-plan.md:737`, key
  files `architecture/ci-cd-approach.md:20`, `test-plan/5-command-implementation.md:8`: each amended, the token
  stands beside `--framing`.
- `v288` · new 1, standing 7 — `test-plan.md:664` (this pass: drift-only); `architecture.md:48`, `:75`,
  `security-plan.md:275`: dated measurements on 2.1.288, true; `architecture.md:91` ×2 (dated measurements),
  `:412` ×2 (amended: drift-only, no framing variant); `test-plan.md:438` ×2 (12 dialog variants under each set,
  true; the framing tier added), `:1085` ×3 (the hygiene walk's sets, true; the framing variants added).
- `seven-settles` · standing 1 — key file `test-plan/bootstrap-phases-derive-for-route-setup-project.md:10`:
  amended to "seven … when it was sized" with the ten waits and the CI reading beside it. The two source comments
  that say the same (`.config/nextest.toml:19`, `tests/support/verify.rs:390`) are source: pinned at route-resolve.
- `escape-claim` · standing 1 — `architecture.md:89`: amended; the phrase now reads "a backslash after `<` in a
  `pasted_content` tag the user typed". True.
- `wrap-threshold` · new 8 — the row id `long-paste-wrapper` in this pass's own text (the pattern `paste-wrap`
  matches inside it); `test-plan.md:738`'s "paste-wrap threshold" is gone.
- `transcript` · standing 1 — `architecture.md:100`: amended ("session transcripts … Run B two").
- `drift-only` · standing 2 — `architecture.md:412`, `test-plan.md:664`: amended.

## Leaves (re-computed, cascade step 3)
Re-derived from the amended masters: `CLAUDE.md` (`GENERATED:setup:warnings`, the ledger line),
`.claude/docs/commands.md`, `security-summary.md`, `tests-summary.md`, `gotchas.md`, `services/viola.md`,
`services/viola-agent-claude.md`, `.claude/rules/verification-harness.md`, `.claude/rules/security.md`. Rows left
after the second listing, each read:
- `security-summary.md:38` `fourteen` / `wrap-threshold` — the re-derived text itself. No change.
- `verification-harness.md:46`, `services/viola-agent-claude.md:42`, `tests-summary.md:21` ×2 `ends-kept` —
  fixture comparisons. No change.
- `verification-harness.md:28`, `tests-summary.md:21` `dialogs-flag` — re-derived, `--framing` beside it.
  `commands.md:25` still read `--dialogs` alone in its CI clause: re-derived after the second listing.
- `gotchas.md:74`, `services/viola-agent-claude.md:31` `v288` — a dated measurement. `tests-summary.md:21` ×3 —
  re-derived.
- `tests-summary.md:21` `drift-only` — re-derived.
Not re-derived, read and left: `design-summary.md`, `obs-summary.md`, `a11y-summary.md`, `stack.md`,
`conventions.md`, `workflow.md` (no row; none states a row count, the flag list or Run B's pastes).

## Curation homes and judgment bases
- 0 rows in `CLAUDE.md` `USER:session-learnings`, the rule files' `## Session Additions` and
  `docs/session-learnings.md`; 0 rows in `playbook.md` and `drift-base.md`.

## Lateral binds
- test-plan §3 ↔ obs-plan §3 / §4: boot step 4's argv gained `--framing` on both sides (key file
  `test-plan/5-command-implementation.md:8`, `obs-plan.md:737`). The status shape and the log format did not move.
- a11y-plan schema ↔ obs-plan schema: untouched.
- `registry.py check` over architecture, test-plan, obs-plan and a11y-plan: 0 defects each.
