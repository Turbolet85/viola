# Answer card — 2026-10-19-dialog-hold-for-the-driver

The founder's own answers to the chunk's P4 forks, given live in the overseer's dialog 2026-10-19T08:04Z and relayed
verbatim by the operator (`inputs#I2`, `viola-0.1.0/chunks/2026-10-19-dialog-hold-for-the-driver/inputs/I2-relay-2.md.txt`):

- **Q1 «Ключ, по умолчанию 60»** — the hold is the `config.json` key `dialog_hold_s`, whole seconds 60–3600, default
  60, a bad value 60 with one `parse-rejected`; `hooks.json` and the hook's bound compiled at the 3600 s ceiling.
  The widening ratified by him.
- **Q2 «Отдать человеку»** — `null` at expiry, the CLI's own menu; a late `answer` `unknown-dialog`.
- **Q3 «Verify + 1 старт, 11 мин»** — one `viola verify`, one wrapped start answered after about 11 minutes.

Escalated proposals read as lying wholly inside these answers (numbered as `fanout-results.md`):

- architecture 1 — [Hook Contract]: the hold `dialog_hold_s` (60–3600, default 60, `DIALOG_HOLD_DEFAULT` / `_MIN` /
  `_MAX`), the hook bound 3605 s and the `timeout` 3615 at the ceiling, `null` + one `dialog-expired` at the hold's end,
  the 671 s live reading (Q1, Q2, Q3)
- architecture 5 — §Occupied Resources `config.json`: + `dialog_hold_s` and its read and fallback (Q1)
- architecture 6 — §Occupied Resources `schemas/config.v1.json`: + `dialog_hold_s`; `CONFIG_KEYS` six (Q1)
- architecture 7 — §Cross-cutting Patterns Config management: Settings + `dialog_hold_s` (Q1)
- security-plan 1 — §Input Validation Configuration values row: + `dialog_hold_s` into `DialogHold` (Q1)
- test-plan 4 — §5 CLI controls table: the `dialog_hold_s` row and its booted case, `SETTINGS` ten (Q1)
- obs-plan 6, 7, 8 — §3 → OTel SDK init steps 5–6: the start's `dialog_hold_s` read, its one `parse-rejected`,
  `CONFIG_KEYS` six (Q1)
- a11y-plan 1 — §8 Timeout extensions: the configurable hold, its bounds, `dialog-expired` at the hold's end, any
  human key still hands back (Q1, Q2)

## The confirm (2026-10-19, in this session)

Asked as one question with the list above in its preview. Answer, as given: «Confirm all (Recommended)», with the note,
verbatim: «Overseer, 2026-10-19: a fit check; each records the dialog_hold_s key, its bound and the live 671 s reading
he set at P4..»

Authority: the confirm is the Viola overseer's (delegate overseer, 2026-10-19), not the founder's own word, so it is
PROVISIONAL, to be superseded by his own word at the Epoch 5 boundary batch. The ratification of the widening stays
the founder's own answer to Q1 (`inputs#I2`), named as his in each owning sidecar entry.
