# Cascade dispositions — the 2026-10-13T06-38-35 0-pending wrap (the Epoch 4 boundary)

The pass writes the founder's batch of 2026-10-13 (`relay-1.md` §4, his own answers relayed verbatim by the Viola
overseer; the operator's scoping answers in `relay-2.md`) into five masters, and moves the explicit-link hold's
placement in two:

- architecture.md: `:71` (the hold, now on the Epoch 6 entry, his answer «Не делать до Эпохи 6»), `:139` (list's
  exit 1 for an unlistable `instances/` his; an entry whose type cannot be read, a failed log start and the no-row
  read error not ruled), `:147` and `:407` (list's whole-board refusal his; on `:407` the statusline arm's no-trace
  shape his, `revive`'s start-arm no-line shape still the operator's answer), `:408` (`budget.json` widening
  ratified), `:471` (`rate_limits`: the build stays, row and probe on "Paste newline ledger row").
- security-plan.md: `:213` (list's refusal shape; the statusline arm's no-trace shape; the `budget.json` widening
  ratified), `:241` (the same widening ratified).
- design-system.md: `:724`, `:750`, `:834` (list's exit-21 refusal his), `:749` (WHEEL 23 and the five board forms
  his), `:805` (the two budget-governor lines his wording).
- layout-templates.md: `:397` (WHEEL 23, the five forms), `:406`, `:588` (list's refusal; list's unlistable-dir exit 1
  his, the rest not ruled), `:471` (the two lines).
- obs-plan.md: `:374` (statusline no line his), `:394` (statusline and list his; `revive` and the `budget.json` read
  still the operator's word), `:581`, `:841`, `:1072` (list and statusline his, `revive` not).
- test-plan.md: `:925` (the hold's entry now in Epoch 6).

**The search** (`cascade-patterns.toml`, run by `cascade.py sweep` after the last body edit; baseline `dffad3e0`;
the listing is `cascade-listing.txt`): eight patterns over the seven masters, every `.andromeda/registries/**` file,
the three curation homes, the two judgment bases and the leaf bodies — `notratified` (`not ratified|unratified`),
`opword` (`operator's (word|answer)`), `notruled` (`not ruled|has not ruled|not having ruled`), `wordowed` (`wording
is owed|word is owed|not worded by the founder|has not worded`), `acceptedop` (`accepted by the operator`),
`linkhold` (`Explicit session links`), `refusedriver` (the widening's verb, `refus\w* a driver`), `ratelimits`
(`rate_limits` within a sentence of `founder|operator`). Every control fired on the pre-pass masters.

Counts, copied from the listing: `total (8 patterns) · 80 rows over 15 files` · `per class · new 2/2 · standing
49/6 · leaf 22/7 · curation 1/1 · base 6/1`.

Not searched for: the sidecars (outside the sweep; their three card-confirm sites were read by `sed -n` at
`security-plan-amendments.md:499`, `:509`, `:519`); the working route (P5's); "PROVISIONAL" (measured at P1: in the
masters it marks only the two deadline constants).

## Master rows

| row | disposition |
|---|---|
| `architecture.md:71` notratified ×2, opword, notruled | amended (the hold sentence): the widening stays NOT ratified by his answer, as the batch says; the five implicit-link shapes on the operator's word are in no batch item — stand |
| `architecture.md:139` opword @c1231, notruled ×2, acceptedop | `revive`'s refusal with no line — the operator's answer, not in the batch (`relay-2.md` q1) — stands; the list clause amended, its unruled remainder named |
| `architecture.md:147` opword | amended — his |
| `architecture.md:247` opword, notruled | stands — the `since: "unknown"` implicit-link shape, no batch item |
| `architecture.md:407` opword ×3, notruled | amended — list's refusal his (c1734); the arm sentence split: statusline his (c3421), `revive` the operator's (c3543, notruled) |
| `architecture.md:408` opword, refusedriver | amended — ratified; the operator's word kept as superseded history |
| `architecture.md:471` opword ×3, ratelimits ×2 | amended (c3056: his ruling); c1547 (`claude --resume`'s row owed, the operator's answer at the revive wrap) and c2855 (the landing on the operator's word, kept as history beside his ruling) stand |
| `security-plan.md:213` opword ×4, notruled, refusedriver ×2 | amended: list (c2825), statusline (c3977), `budget.json` (c8042); c1959 / notruled is `revive`'s no-line shape — stands (q1); refusedriver c7199 is the gate's description — a true claim |
| `security-plan.md:241` refusedriver | amended — ratified |
| `design-system.md:724`, `:749`, `:750`, `:805` opword | amended — his |
| `design-system.md:839` opword | stands — `revive` beside `run` in `--help`, the operator's answer at the revive wrap, no batch item |
| `layout-templates.md:397`, `:406`, `:471` opword | amended — his |
| `layout-templates.md:588` opword, acceptedop, notruled (new) | amended — list's refusal his; the exit-1 clause names the unruled remainder |
| `test-plan.md:552` opword, notruled | stands — the implicit-link `since: "unknown"` shape |
| `test-plan.md:925` notratified (new), linkhold | amended — the hold on the Epoch 6 entry |
| `test-plan.md:1076`, `:1313` opword | stand — the revive fake-agent payload; the state-file planting rule: no batch item |
| `obs-plan.md:374`, `:581` opword | amended — his |
| `obs-plan.md:394` opword ×3, notruled | amended — statusline and list his; `revive` and the `budget.json` read stand on the operator's word (`relay-2.md`, invocation answer 1 and q1) |
| `obs-plan.md:465` opword, notruled | stands — `revive`'s strict-mode fail with no `detail` code and no line (q1) |
| `obs-plan.md:606`, `:729`, `:1071` opword | stand — the statusline start steps with no span; the implicit-link shapes; `revive --list`'s exemption 6: no batch item |
| `obs-plan.md:841` opword, notruled | amended — statusline and list his, `revive` the operator's |
| `obs-plan.md:1072` opword ×3, notruled | amended — exemption 7 split the same way |

The `wordowed` pattern prints 0 rows after the pass: every "his wording is owed" / "word is owed" mark it reached is
gone. No master row carries a retired claim beside an amended one on the same line (each `mixed` row read whole at
its offsets).

## Judgment bases and curation homes

- `playbook.md:62` — the superseded rule, its `note:` now opening `SUPERSEDED 2026-10-13 (by the rule Boundary
  widening the founder answered himself to a P4 fork) —` (directive §3, the operator's word); `:70` the new rule;
  `:58`, `:64`, `:66` provenance lines — no change. `:66`'s "marked not ratified" is the "Held items" rule's own
  text, true of its class — no change.
- `.claude/rules/testing.md:87` (curation) — a PID-1 note, no batch item — no change.

## Leaf rows (step 3's set, by the whole mode)

`.claude/rules/security.md:28` (`revive` — true, stands), `:30` (the `budget.json` widening — stale);
`.claude/docs/security-summary.md:67` (list's refusal — stale), `:69` (the widening — stale);
`.claude/rules/observability.md:40`, `:41`, `.claude/docs/obs-summary.md:70`, `:72` (no-line shapes — stale in part);
`.claude/rules/events.md:26` (implicit-link shapes — true); `.claude/docs/tests-summary.md:32` (the hold's entry by
title — true); `.claude/rules/ci.md:18` (the mutation dispatch — true). Every leaf is re-derived by the whole-mode
comparison below (directive §1); these rows only join its set.

## Leaves

Mode: **whole** (`cascade.py leaves --all`; the founder's ruling F252 D2, directive §1 — the first wrap since `leaves`
deployed). 13 groups, 28 leaves, one comparer each in one batch; the path check read exactly 13 `A` lines
(`leaf-{group}.md`), index and HEAD unchanged (before `tree 2f5444c1…`, after `tree 703631e3…`). Unmapped, no
comparer: `.claude/rules/host-linux.md` (the host leaf — no master states it) and `.claude/rules/testing-src.md` (a
pointer file into `testing.md`'s sections, repeating no master text). Apply listing: `leaves-apply.txt`.

Tool total: `total · blocks 80 · applied 77 · reshaped 2 · held 1`.

| group | findings | applied | by hand | declined |
|---|---|---|---|---|
| architecture-1 | 20 | 19 | 1 — `conventions.md:40` (reshaped: the pipe between `pause` and `release` read as a cell boundary); written as `viola pause <target>`, `viola release <target> [--budget]` (arch `:176`: only `release` takes `--budget`) | 0 |
| architecture-2 | 15 | 15 | 0 | 0 |
| architecture+security-plan | 5 | 5 | 0 | 0 |
| architecture+security-plan+design-system+layout-templates | 3 | 2 | 1 — `frontend.md:39` (reshaped: a backslash escape the line did not carry); written with the escape form named in words | 0 |
| masters | 1 | 1 | 0 | 0 |
| architecture+test-plan+obs-plan | 9 | 9 | 0 | 0 |
| security-plan | 3 | 3 | 0 | 0 |
| security-plan+obs-plan | 0 | 0 | 0 | 0 |
| design-system+layout-templates | 5 | 5 | 0 | 0 |
| test-plan | 12 | 12 | 0 | 0 |
| test-plan+obs-plan | 2 | 2 | 0 | 0 |
| obs-plan | 3 | 3 | 0 | 0 |
| a11y-plan | 2 | 1 | 0 | 1 — `a11y-summary.md:31` (held): the board's as-built state and its test literals, a detail of a11y-plan §P6 that stays in its master; the leaf's row already points to "cli output discipline" |

Checked before the write: every `tree` finding against the tree (`crates/viola-mcp/` and `crates/viola-ui/` absent;
`tests/tui_pty_seam.rs`, `tests/cli_program_resolution.rs` present); `viola-channel.md:11` against
`crates/viola-channel/Cargo.toml` (no `sysinfo`); the rest at their cited source lines. Of the pass's own amendments,
the leaves now carry: `security.md:30` and `security-summary.md:69` (the `budget.json` widening ratified),
`security-summary.md:67`, `obs-summary.md:70`, `:72`, `observability.md:40`, `:41` (the board's and the statusline
arm's no-line shapes his, `revive`'s not). `testing.md:34` now states the stamped-home rule for a check-guarded case
(test-plan §5, the generated body) — P3's candidate on it reads that body for dedup.
