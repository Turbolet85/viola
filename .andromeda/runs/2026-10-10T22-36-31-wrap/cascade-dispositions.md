# Cascade step 2 — sweep dispositions, wrap of 2026-10-10-budget-governor

**The search.** `cascade.py sweep` over `cascade-patterns.toml` (28 patterns), run twice: once after every master
body was applied (106 rows over 25 files), and again after the four stale standing rows it found were amended and
the leaves were re-derived. This file dispositions the SECOND listing, the tree as it is committed. Its last lines,
copied:

    total (28 patterns) · 94 rows over 25 files
    per class · new 13/6 · standing 57/11 · leaf 24/14 · curation 0/0 · base 0/0

The baseline was `396c108b`, the parent of the pre-CI commit. Every pattern's control fired on the pre-pass text.
Each pattern is keyed on what a retired claim SAID. The sections read whole beside the listing: architecture
[Budget Governor], [Session Liveness], the Channel methods, the Hook contract, the viola home files; security-plan's
`~/.viola/` access control row (by offset, a line of 10 549 chars after the pass) and the two Input Validation rows;
obs-plan §4 Scenario 1 and Scenario: Budget governor, §6's catalog, §10's exemptions; test-plan §6 Path 6 and E5.

**What was NOT looked for:** a pattern on `budget.json`, `rate_limits` or `budget-paused` as bare subject tokens
(dozens of true rows each; the claims about them are covered by `trusted-list`, `rows-owed`, `gate-rung` and
`release-result`); "the sixth exemption" as a count of obs-plan §10's list (no master states the list's length;
the leaf `rules/observability.md` names exemptions six and seven by number and was re-derived); the word "one
invocation" (the hypothesis stood only on the route, a frozen line).

A master line over 2 000 chars was read by offset (`cascade.py window`, or the exact text at the row's `@c`).

## Zero-row patterns (seven; each control fired, so the claim's wording stands nowhere in the swept set)

`four-readings` · `not-measured` · `third-reading` · `waits-26` · `thirteen` · `span-chain` · `read-in-src`. A
zero-row pattern is a statement about that pattern, never an absence proof; the first listing's rows for the first
three were each read and amended (below).

## Rows the first listing held and the second does not (amended or re-derived between the two)

- `four-readings` `.claude/docs/services/viola.md:21` leaf → re-derived (three readings, the check ahead).
- `not-measured` `.claude/rules/security.md:28`, `.claude/docs/security-summary.md:67` leaf → re-derived (the order
  is measured and fixed).
- `third-reading` `obs-plan.md:731` standing → amended: the fallback is the preflight's second reading (a site no
  proposal named).
- `owner-bg` leaf rows: `CLAUDE.md:43`, `.claude/rules/security.md:28`, `.claude/docs/commands.md:30`,
  `.claude/docs/security-summary.md:67`, `.claude/docs/services/viola.md:23`, `.claude/docs/tests-summary.md:30` →
  each re-derived; no leaf names "Budget governor" as an owner now.
- `smf` leaf rows: `.claude/rules/observability.md:21`, `.claude/rules/security.md:28`,
  `.claude/docs/conventions.md:33`, `.claude/docs/obs-summary.md:70`, `.claude/docs/services/viola.md:21` →
  re-derived.
- `log-first` `.claude/docs/services/viola.md:23` leaf → re-derived.

Three more standing rows were stale in the first listing and are `edited` in the second: `design-system.md:787`
(`smf`: "its strict-modes refusal reuses `strict-modes-failed` at exit 1" → no log line, no detail code),
`test-plan.md:281` (`release-result`: the result holds `override_until` when a pause stood), `test-plan.md:556`
(`check-instance`: "Two `cfg(unix)` root cases" → three, read in `tests/cli_revive.rs`).

## The 94 rows, by pattern (s = standing, n = new, l = leaf, e = edited by this pass)

- **check-instance** (16): `architecture.md:327` se, `:406` se and its second match → amended (A6, A7, A8; the
  check's place and the two call sites). `security-plan.md:213` se, `:241` se → amended (S1 to S3, S6).
  `security-plan.md:236` s → no change: "before its log opens and before any stdin … use" holds as landed (the check
  is first). `test-plan.md:556` se → amended (above). `obs-plan.md:835` n, `:841` n,
  `K:obs-plan/otel-sdk-init.md:21` n → this pass's text. `K:architecture/project-directory-structure.md:54` s → no
  change: the crate inventory names the check, true. Leaves `services/viola.md:21`, `:23`, `services/viola-state.md:6`,
  `rules/security.md:28`, `:29` → re-derived or true (`viola-state.md:6` states what the check reads, unchanged).
- **clock-seam** (1): `test-plan.md:1121` se → amended (T3).
- **config-once** (1): `K:obs-plan/otel-sdk-init.md:17` se → amended (O23).
- **echo-flag** (8): `architecture.md:361` s, `test-plan.md:1091` s → no change: the fake agent's mode word
  `statusline-echo`, which exists. `test-plan.md:839` se, `K:test-plan/5-command-implementation.md:4` se → amended
  (T13, T14). Leaves `commands.md:30`, `tests-summary.md:30` → re-derived; `tests-summary.md:21`,
  `rules/verification-harness.md:42` → no change: the mode word.
- **four-of-seven** (1): `obs-plan.md:838` n → this pass's text (the count before the chunk, named as past).
- **gate-rung** (1): `architecture.md:72` se → amended (A19): the rung's order follows the sentence.
- **hand-write** (7): `test-plan.md:450` se, `:1311` se → amended (T9, T11). `design-system.md:840` s,
  `a11y-plan.md:1013` s, leaf `rules/events.md:20` → no change: "comes only from" about other subjects. Leaves
  `rules/testing.md:35`, `tests-summary.md:52` → re-derived.
- **lifted-line** (4): `design-system.md:803` se, `layout-templates.md:457` se, `:469` n → amended or new (EA 14,
  L1, L2). Leaf `services/viola.md:24` → re-derived (new bullet).
- **log-first** (1): `security-plan.md:236` s → no change (as under check-instance).
- **mock-instant** (8): `test-plan.md:582`, `:1070`, `:1121`, `:1136` se,
  `K:test-plan/bootstrap-phases-derive-for-route-setup-project.md:23` se → amended (T1, T3 to T6).
  `test-plan.md:1352` s → no change: the ban on an in-process clock for another process stands, and its pointer to
  the §4 unit case holds. Leaves `rules/testing.md:29`, `stack.md:50` → re-derived.
- **no-diag-line** (1): `K:obs-plan/otel-sdk-init.md:24` se → amended (O16): the sentence now holds for the arm.
- **not-exempt** (1): `obs-plan.md:1071` se → amended (O8).
- **owner-bg** (2): `architecture.md:470` n → this pass's text (the row moved off the entry). `obs-plan.md:339` s →
  no change: a source label ("creator must-work "Budget governor""), not an owner.
- **preflight-exit** (4): `architecture.md:362` se, `obs-plan.md:835` se → amended (A13, O1). Leaves
  `conventions.md:33`, `rules/observability.md:40` → re-derived.
- **release-result** (4): `architecture.md:285` n, `test-plan.md:853` n → this pass's text. `test-plan.md:281` se →
  amended. `test-plan.md:827` s → no change: Path 5's plain `release` with no pause standing answers
  `budget_paused:false`, true under the gate's recorded state.
- **rows-owed** (3): `architecture.md:79` se, `:470` se → amended (A21, A22). Leaf `CLAUDE.md:43` → re-derived.
- **smf** (17): `architecture.md:406` se, `security-plan.md:213` se, `:241` se, `test-plan.md:992` se,
  `obs-plan.md:838` se, `:841` se, `design-system.md:787` se → amended. `obs-plan.md:835` n,
  `K:obs-plan/otel-sdk-init.md:24` n → this pass's text. No change, each a true claim sharing the token:
  `architecture.md:253`, `:410` (the `ledger-stamps` refusal); `obs-plan.md:532` (the coverage map's pointer to the
  catalog), `:660` (the liveness-only pre-check), `:735` (`ledger-stamps`), `:836` (exit 21), `:839`
  (`ledger-stamps` in the `parse-rejected` list). Leaf `obs-summary.md:68` → re-derived (exit 21 keeps the word).
- **src-unresolved** (1): `K:test-plan/5-command-implementation.md:4` se → amended (T14: dropped, no owner).
- **strict-mode-fail** (1): `obs-plan.md:465` se → amended (O3); the exit-21 half of the line stands.
- **thresholds** (6): `architecture.md:399` se, `security-plan.md:237` n, `K:obs-plan/otel-sdk-init.md:17` n →
  amended or new. `architecture.md:464` s (Config management names the keys and defaults, true),
  `security-plan.md:32` s (the threat summary's "holds budget thresholds", true) → no change. Leaf
  `conventions.md:45` → re-derived.
- **trusted-list** (8): `architecture.md:407` n, `security-plan.md:213` se and its second match, `:241` se →
  amended or new (A5, S4, S6). Leaves `rules/security.md:29`, `:30`, `security-summary.md:68`, `:69` → re-derived;
  each writes the `budget.json` refusal as NOT ratified.

## Curation homes and judgment bases

0 rows in either class. `.andromeda/playbook.md` gained one rule in this pass through the propose, approve, append
channel (the operator's word, `inputs#I11`), not through the cascade.

## Leaves re-derived at step 3

`CLAUDE.md` (four lines: the `viola-core`, `viola-state` and root-bin module lines, the capability-ledger warning) ·
`.claude/rules/security.md` · `rules/observability.md` · `rules/events.md` · `rules/testing.md` · `rules/api.md` ·
`.claude/docs/conventions.md` · `commands.md` · `stack.md` · `security-summary.md` · `tests-summary.md` ·
`obs-summary.md` · `a11y-summary.md` · `services/viola.md` · `services/viola-state.md` · `services/viola-core.md`.
Each was read at the sentences its master's amendment reaches and rewritten from the amended master. Not re-derived:
`design-summary.md` (it holds no verb line), `gotchas.md` (no amended section feeds it).
