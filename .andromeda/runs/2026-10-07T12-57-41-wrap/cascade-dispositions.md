# Cascade dispositions — 2026-10-07-send-waits-out-the-paste-hint

Written from the one `cascade.py sweep` listing of this pass (the trail `cascade-2026-10-07-send-waits-out-the-
paste-hint.json`), run after every body of the pass was applied and before any sidecar entry. Baseline `56e67bb3`,
the parent of the pre-CI commit.

## The search
Seventeen patterns, `cascade-patterns.toml`, each control fired on the pre-pass text: the old value beside the
constant (`gate-5s`, `gate-near-5s`), the constant by name (`max-wait-name`), the unbuilt-remedy sentence
(`not-built`), the wheel residual's phrasings (`residual`), the immediate refusal on a failed signature check
(`either-check`), "the run gate is unchanged" (`unchanged-by`), the refused-at-once outcome (`refused-fast`), the
8 000 ms cap in three spellings (`cap-8000`), the old kills and floor (`kills`), the 6 s hold (`hold-6s`), the
single-pass refusal order (`order`), the two-valued verdict (`verdict-2`), every mention of the paste hint
(`hint`, case-insensitive), `send`'s blocking sum (`blocks`), the readback metric (`confirm-ms`) and the tui
keystroke sentence (`unblocked`). Read: the seven masters, every `.andromeda/registries/**` file, the three
curation homes, the two judgment bases and the leaf bodies.

Not looked for: test totals (no master states them); the `--version` 5 s kill, the heartbeat's 5 s flip, the
`FAKE_AGENT_PUMP_DELAY_MS` 5 s cap and the 15 s SSE keep-alive, which are other subjects sharing a number (read
at the report's sweep and by the architecture and security-plan detectors, and left).

## Zero-row patterns (each control fired pre-pass, so the retired wording is gone)
`not-built` · `either-check` · `unchanged-by` · `hold-6s` · `verdict-2`.

## Rows, by pattern
- `gate-5s`: `test-plan.md:654` new (this pass's own "8.5 s maximum") — amended. The key file
  `bootstrap-phases-…md:10`, `:11` standing, edited — amended (each now reads "8.5 s maximum").
- `gate-near-5s`: `architecture.md:48` ×2, `:49`, `:70`, `:91`, `test-plan.md:654`, `a11y-plan.md:823`, the key
  file `:10`, `:11` — all new, this pass's text. `security-plan.md:244` standing — no change: the `--version`
  reads killed at 5 s, another subject. Leaf `.claude/docs/services/viola.md:20` — no change: the version gate's
  `--version` kill.
- `max-wait-name`: `architecture.md:48` (×3 and a further hit), `:70`, `:91` (×3 and a further hit) standing,
  edited — amended where the value stood; the hits that read the constant by name (the settle rule, `box_wait`)
  are true and left. `test-plan.md:566`, `obs-plan.md:763`, `a11y-plan.md:823` new. Leaf
  `.claude/docs/services/viola-agent-claude.md:11` (`GATE_MAX_WAIT` 5 s) — re-derived to 8.5 s, with the verified
  verdict's waiting arm.
- `residual`: `architecture.md:70` standing, edited, one hit left at the line's later half — no change: it is the
  SessionEnd-appended-directly residual, a different one, read by window (`[6450:6800]`).
- `refused-fast`: `architecture.md:91` new ×2 — this pass's text (the refusal at the bound, and the dated
  0.63 s reading kept as the state before the chunk).
- `cap-8000`: masters 0 rows. Leaf `.claude/rules/verification-harness.md:42` ("8 s at most") — re-derived to
  10 s (the rule's body, above its Session Additions).
- `kills`: the key file `:11` new ×2 (the 45 s kill, this pass's text). `:18` ("a 21 s baseline") and `:20` ("a
  30 s kill" for `viola-e2e`) standing — no change: cargo-mutants' measured baseline and the harness override,
  neither moved by this chunk.
- `order`: `architecture.md:136` and `test-plan.md:588` standing, edited — amended (the second reads added after
  the order). Leaf `.claude/rules/api.md:23` — re-derived.
- `hint`: `architecture.md:48` ×2, `:70`, `obs-plan.md:763`, the a11y key file `:8`, the test-plan key file `:10`
  new (the chunk's marker and this pass's text). `architecture.md:81` standing — no change: "a long or a repeated
  text pasted while the paste hint stands" is still unmeasured. `architecture.md:91` standing, edited (×5 and a
  further hit) — amended; the hits left are dated measurements that stand (the 8.0 s timer, the `viola wait`
  reading in the window, the unverified-CLI reading). `architecture.md:355`, `test-plan.md:654`, `:750`, `:1077`,
  the test-plan key file `:11` standing, edited — amended. Leaves: `.claude/rules/verification-harness.md:42`
  re-derived (above); `.claude/docs/services/viola.md:6` no change (verify's `box_wait` and the 8.0 s hint, still
  true); `.claude/docs/tests-summary.md:21` re-derived (the cap, 10 s) and `:26` re-derived (the pair delivered,
  the keystroke case with its limit).
- `blocks`: `architecture.md:49` standing, edited — amended (18.5 s).
- `confirm-ms`: `obs-plan.md:763` standing, edited — amended.
- `unblocked`: `a11y-plan.md:573` standing, edited — amended (the further case appended; the three clauses
  stand). The a11y key file `:8` standing, edited (×2 and a further hit) — amended.
- Curation homes: 0 rows. Judgment bases: 0 rows.

## Leaves re-derived (step 3)
Beyond the sweep's `leaf` rows, a read of every leaf body for the moved claims (regex over `.claude/docs/**`,
`.claude/rules/*` above their Session Additions, and CLAUDE.md's generated blocks) found one more:
`.claude/docs/gotchas.md:79` ("not quiet within 5 s refuses") — re-derived to 8.5 s, with the verified gate's
wait. CLAUDE.md's generated blocks state none of the moved facts: recomputed, unchanged (124 lines).

| leaf | from | what changed |
|---|---|---|
| `.claude/docs/services/viola-agent-claude.md` | architecture [Screen Model] | `GATE_MAX_WAIT` 8.5 s; the verified verdict waits for the input box |
| `.claude/docs/gotchas.md` | architecture [Screen Model] | the partial gate's bound 8.5 s; the full gate's wait |
| `.claude/docs/tests-summary.md` | test-plan §6, §7 | the cap 10 s; the pair delivered; the keystroke case |
| `.claude/rules/api.md` | architecture §Conventions | `send`'s second wheel and turn reads |
| `.claude/rules/verification-harness.md` | test-plan §7 | the cap 10 s |

## Binds (step 2)
`test-plan §3 ↔ obs-plan §3`: the harness commands, status shape and log format did not move; the test-plan key
file's edit is nextest configuration, which obs-plan §3 does not state. `a11y-plan schema ↔ obs-plan schema`: not
touched.
