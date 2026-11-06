@proposal n=1 detector=D-retired severity=warning file=cross-cutting-patterns anchor=fct-k6htnrfj basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-uufjzm4k
section: ## Cross-cutting Patterns
change: in the viola-home bullet, `canonical_home` reads «canonicalises a home that exists (`std::fs::canonicalize`) and keeps `std::path::absolute`'s spelling for one that does not, creating nothing» — the clause «a Windows `\\?\C:\…` written back as `C:\…`, every other verbatim form kept» removed; the rest of the fact unchanged
sidecar: crosscut-config — canonical_home's Windows verbatim-drive write-back removed with plain_drive (chunk 2026-11-06-version-done-check)
rationale: the report's Retired bullet names `plain_drive` and its Windows verbatim-drive handling, and Symbols / APIs states its call in `canonical_home` removed with the signature kept
@proposal n=2 detector=D-cascade severity=warning file=cross-cutting-patterns anchor=fct-wfge3rsz basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-npz56o6m
section: ## Cross-cutting Patterns
change: after «so `budget-paused` now rests on a stamped row.» add «That row reads the account's plan state, not the CLI alone: on an unchanged 2.1.287 the rig home's `viola verify` read it red (`stamped 2.1.287  23 pass  1 fail`, `[21/24] statusline-rate-limits`), the account's status-line payload carrying `five_hour` `unknown` and only `seven_day` (as measured on the Linux dev host at chunk 2026-11-06-version-done-check, `evidence/live-round.md` Session 1); the dev home's stamps are not moved, the ledger stays 24 rows, and what the budget gate stands on is owned by 0.2.0 (the founder's finding F-06i).»
sidecar: crosscut-capability-ledger — the statusline-rate-limits row recorded as reading the account's plan state, red 23/24 on an unchanged 2.1.287; owner 0.2.0 (F-06i)
rationale: the report's Spec claims disproved by measurement shows the row, which this fact names as the stamped ground of `budget-paused`, read red on an unchanged CLI because it reads the account's plan
@remark
F1 and F2 (the ledger row's plan-state reading and the budget-gate OPEN) are established-decisions' facts for [Hook Contract] fct-qwpklckh and [Budget Governor] fct-3hy3yfb4; F3 (dialog_drift_names_a_changed_annotation) has no picked cross-cutting entity to carry it and goes to the new-detail pass.
