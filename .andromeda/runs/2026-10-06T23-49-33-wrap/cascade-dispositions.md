# Cascade dispositions — 2026-10-06-local-command-send-outcomes

Written from the sweep listing `cascade-sweep.txt` (`cascade.py sweep`, 13 patterns of `cascade-patterns.toml`,
baseline `11c77f7c`, exit 0), after every body of the pass was applied and before any sidecar entry.

## The search
- Read: the seven masters, every `.andromeda/registries/**` file, the three curation homes, the two judgment bases,
  the leaf bodies.
- Patterns, each with a control that fired on the pre-pass text: the retired claim's wording (`does not consume`,
  `Local-command send outcomes`), its mechanism (`local command … not-delivered`), the option count and its
  neighbour in every option list (`seven argv`, `--paste-hint-ms`), the record shape (`send-confirmed` + `{cursor}`),
  the sole-confirmer claim (``matching `prompt-submitted` ``; the hook named as user-prompt-submit in three
  spellings), `confirm_window`, `send.confirm_ms`, the three-line mirror list, `the driver relabel`, `turn screen
  shows a modal`.
- Not looked for: a restatement of the sole-confirmer claim that names neither `prompt-submitted` nor the
  user-prompt-submit hook; a count of mirror lines written as a number ("three lines"). The detectors read their
  masters for both and reported none.
- `confirm-win` prints `case-variants +3`: the compiled `CONFIRM_WINDOW_FALLBACK`, another token, not swept.

## Rows
| row | disposition |
|---|---|
| `seven-argv` · 0 rows | amended at both pre-pass sites (`architecture.md:355`, `test-plan.md:1077`); none left |
| `mirror-list` · 0 rows | amended at its one site (the key file `project-directory-structure.md:21`) |
| `architecture.md:4` · local-nd, matching-ps · standing | no change — it already states both confirmers ("a matching `prompt-submitted` or, for a local command on the ledger, its measured post-condition") |
| `architecture.md:49` · matching-ps · edited | amended (A1); the hit is the entry's opening sentence, now scoped by the entry's own "describe a send that is not a listed command" |
| `architecture.md:355` · paste-hint-opt · edited | amended (A6): the eighth option follows it |
| `test-plan.md:654` · paste-hint-opt, turn-modal · edited | amended (T7); the `--paste-hint-ms` clause is a true claim about another test |
| `test-plan.md:1077` · paste-hint-opt · edited | amended (T6) |
| `design-system.md:11`, `:23`, `:850` · matching-ps · edited | amended (E9): each now names the post-condition beside the prompt |
| `design-system.md:149` · matching-ps · standing | no change — it already reads "matching `prompt-submitted`, or ledger post-condition" |
| `layout-templates.md:50`, `:221` · matching-ps · standing | no change — each already gives `session-start` cause `clear` as `/clear`'s trigger (windows read at the hit) |
| `layout-templates.md:351` · matching-ps · edited | amended (L1) |
| `obs-plan.md:312`, `:313`, `:631`, `:642` · ups-hook · edited | amended (O6, O5, O4, O3) |
| `obs-plan.md:313`, `:632`, `:648` · confirm-win · edited (`×2` on `:632`, `:648`) | amended (O5, O2, O1); the second match on each line is this pass's own sentence about the unconfirmable send |
| `obs-plan.md:637` · confirm-win · standing | no change — "`run.confirm_window`: `window_ms`" holds whenever the span opens |
| `obs-plan.md:763` · confirm-ms · edited | amended (O8) |
| `registries/contracts/obs-plan/snapshot-paste-to-ai-integration.md:13` · ups-hook, confirm-win · edited | amended (O7) |
| `registries/contracts/architecture/project-directory-structure.md:28` · driver-relabel · edited | amended (A8) |
| `.claude/docs/gotchas.md:83` · local-nd · leaf | no change — the "What breaks" line states the hazard, which stands |
| `.claude/docs/gotchas.md:84` · not-consume, owed-entry, local-nd · leaf | re-derived from architecture [Delivery Confirmation] |
| `.claude/docs/tests-summary.md:26` · owed-entry, local-nd · leaf | re-derived from test-plan §6 Path 2 |
| `.claude/docs/tests-summary.md:21` · paste-hint-opt · leaf | re-derived from test-plan §7: the option list gains `--tag-turn-screen` |
| `.claude/rules/verification-harness.md:42` · paste-hint-opt · leaf | re-derived from test-plan §7 (above `## Session Additions`, which is untouched) |
| `.claude/rules/events.md:16` · sc-cursor · leaf | re-derived from architecture Event `data` per kind |
| `.claude/docs/design-summary.md:7` · matching-ps · leaf | re-derived from design-system §Brand Identity |
| `.claude/docs/services/viola.md:37` · driver-relabel · leaf | re-derived from the key file's `send.rs` line |

Curation homes: 0 rows. Judgment bases: 0 rows.

## Leaves recomputed with no change
- `CLAUDE.md` `GENERATED:setup:*` blocks: the architecture paragraph already reads "confirmed after the fact
  (`prompt-submitted` read-back or a ledger post-condition), never presumed"; the modules, warnings and pointer
  table state nothing this pass moved (the ledger still gates seventeen rows).
- `.claude/docs/obs-summary.md`: it lists the send events and states neither the confirm window's closing rule
  nor the `confirm_ms` derivation (`grep 'confirm_window\|confirm_ms'`: 0 hits).
- `.claude/docs/commands.md`, `conventions.md`, `stack.md`, `security-summary.md`, `a11y-summary.md`,
  `.claude/rules/api.md`, `frontend.md`, `a11y.md`, `observability.md`, `testing.md`, `services/viola-core.md`,
  `services/viola-agent-claude.md`: read at their hits for `unconfirmable`, `local command`, `post-condition`,
  `read back`, `human.rs`, `argv option`; each line is still true (`unconfirmable` is an `ok` payload, never a
  refusal detail; `/clear`'s post-condition as stated).

## Binds
- test-plan §3 ↔ obs-plan §3: neither harness contract changed; the one §3 key file edited is obs-plan's
  "Snapshot / paste-to-AI integration", which test-plan does not mirror. `registry.py check`: 0 defects for
  architecture, obs-plan, test-plan and a11y-plan.
- a11y-plan schema ↔ obs-plan schema: untouched.
