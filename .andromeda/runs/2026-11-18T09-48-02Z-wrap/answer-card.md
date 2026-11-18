# Answer card — Q1 — 2026-11-18-trust-record-row-read-live (wrap run 2026-11-18T09-48-02Z-wrap)

One boundary widening: a hidden `viola verify` option, passed by the live rig alone, under which a FAILED `--record`
verify writes the first settled screen of each interactive run that did not reach the input box (Run A's `trust`
excepted), as whole scrubbed rows, refused whole on any leftover path, name or email, kept local and git-ignored,
never committed; a plain `--record` is unchanged and a passing verify writes no such file.

## The founder's own answers it rests on
- «Verify с записью экрана» — 2026-11-18T07:41:36Z, inputs#I11 item 1 (relayed verbatim,
  `../additional/viola-overseer/relay-trust-read-revision3.md`).
- «Строки целиком, только для рига» — 2026-11-18T08:06:08Z, inputs#I13 (relayed verbatim,
  `../additional/viola-overseer/relay-trust-read-p5-review3.md`).

## The operations lying wholly inside those answers (9)
- ops-architecture.md · decision/verify-record-only-clean · established-decisions · fit: fits type 2 — its except: the failed-verify first-screen record
- ops-architecture.md · decision/recorded-fixtures-scrubbed · occupied-resources · fit: fits type 2 — the same except
- ops-architecture.md · decision/screen-fixtures-signature-rows-only · fit: fits type 2 — except: first screens kept as whole scrubbed rows, local only
- ops-architecture.md · contract/viola-cli-arguments/verify · fit: fits type 3 — the hidden option, absent from --help, passed only by the rig
- ops-architecture.md · decision/run-never-reads-screen-content · stack-and-technologies · fit: fits type 1 — the screen's row-text exit also feeds the rig-only record
- ops-security-plan.md · decision/screen-fixture-signature-rows-only · fit: fits type 2 — except: the record, kept local, refused whole when unclean
- ops-security-plan.md · decision/verify-signature-only · threat-model-summary · fit: fits type 2 — the same except
- ops-security-plan.md · decision/verify-screen-read-against-literals · fit: fits type 2 — the same except
- ops-test-plan.md · decision/record-only-at-zero-failing-rows · 7-test-data-fixtures · fit: fits type 2 — except: the rig-only failed-verify record, never committed

Rejected before the card (not asked): none by fit — the design-system, layout-templates, obs-plan and a11y-plan
restatements were rejected at check 2 (one fact, one home).

## The answer
Given by the overseer through the operator dialog, 2026-11-18T10:18:02Z, snapped as inputs#I22 (verbatim):

> Confirm (Recommended) — Confirmed by the overseer at 2026-11-18T10:18:02Z, the card's 9 lines read against his two
> answers (07:41:36Z, 08:06:08Z): each is the same one record — hidden, passed by the rig alone, a failed `--record`
> verify only, whole scrubbed rows, local, never committed. One condition on `architecture ·
> decision/run-never-reads-screen-content`: its text must still say that `run` reads no screen content — the row-text
> exit feeds verify's rig-only record and nothing of `run`; if the landed wording could be read as `run` reading rows,
> set it before the land.

The condition holds as proposed: the value opens «run never reads screen content» and gives the row-text exit to
«verify alone». The confirm is a delegate's (the overseer), PROVISIONAL until the founder's own word at the epoch
boundary; the ratification is his two answers.
