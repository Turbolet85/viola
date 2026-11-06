@proposal n=1 detector=D-cascade severity=warning file=span-trace-coverage anchor=fct-xrkfnxjv basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: ## 4. Span / Trace Coverage
change: the cli (`viola run`) row's library cell reads «none (portable-pty `=0.9.0` logs through `log`; its target is OFF and failures are captured at the seam, D-11)», the rest of the row unchanged
sidecar: span-trace-coverage — the viola run row's portable-pty version moved 0.8.1 to =0.9.0 (chunk 2026-11-06-version-done-check)
rationale: the report's Dependencies moves portable-pty `=0.8.1` to `=0.9.0` with viola-pty compiling unchanged and pty_backend still openpty, so only the version in the cell moves
@proposal n=2 detector=D-new-detail severity=warning file=span-trace-coverage anchor=fct-6z2owz4q fact=F1 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-npz56o6m
section: ## 4. Span / Trace Coverage › #### Scenario: Budget governor
change: under the Capability ledger item add «- The `statusline-rate-limits` row reads the account's plan state, not the CLI: on claude 2.1.287 a status-line payload with `five_hour` `unknown` and only `seven_day` reads it red (`viola verify` `23 pass  1 fail`), the dev home's stamps unmoved; the `budget-paused` refusal rests on this row, and whether the budget gate stays and what it stands on is 0.2.0's (finding F-06i).»
sidecar: span-trace-coverage — the capability ledger item gains the statusline-rate-limits finding: the row reads the account's plan, owner 0.2.0 (F-06i)
rationale: the report's Spec claims disproved by measurement shows the row read red on an unchanged CLI 2.1.287 because the account payload carries only seven_day, owner 0.2.0 by the founder's word
@proposal n=3 detector=D-cascade severity=warning file=log-coverage anchor=fct-yb5jpaot basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## 6. Log Coverage
change: whole-fact edit (F434, expect_rev=1): the `process-start` row has the header's 2 cells, its `pty_backend` reads (`openpty`) alone, and the `sideload_fallback` field and its Windows degrade clause are gone, every other field unchanged
sidecar: log-coverage — process-start's pty_backend narrowed to openpty and sideload_fallback removed, the row cut to the header's 2 cells (F434, chunk 2026-11-06-version-done-check)
rationale: the report's Expected amendments item 11 orders this F434 edit, and its Outcome states pty_backend() stays openpty on =0.9.0, with no conpty or sideload path on Linux
@remark
the body's section 1 viola-pty entity fct-t2iihkln still names portable-pty =0.8.1; it is a body fact outside this group, and its edit to =0.9.0 is the body's
