# Fan-out results — 2026-11-06-version-done-check (wrap 2026-11-06T20-57-05)

sampler: off
proposals rejected for a source the report does not carry: 0

architecture: 6 proposals · fanout-architecture.md · 21 cascade proposals · fanout-architecture--cascade-g1.md, fanout-architecture--cascade-g2.md, fanout-architecture--cascade-new-cross-cutting-patterns.md (none) · 3 facts · 2 placed · 1 no home
security-plan: 4 proposals · fanout-security-plan.md · 14 cascade proposals · fanout-security-plan--cascade-g1.md, fanout-security-plan--cascade-g2.md, fanout-security-plan--cascade-g3.md · 3 facts · 3 placed · 0 no home
design-system: 1 proposals · fanout-design-system.md · 2 cascade proposals · fanout-design-system--cascade.md · 1 facts · 1 placed · 0 no home
layout-templates: 0 proposals · fanout-layout-templates.md
test-plan: 12 proposals · fanout-test-plan.md · 22 cascade proposals · fanout-test-plan--cascade-g1.md … -g4.md · 2 facts · 2 placed · 0 no home
obs-plan: 2 proposals · fanout-obs-plan.md · 4 cascade proposals · fanout-obs-plan--cascade-g1.md (none), -g2.md, -g3.md · 1 facts · 1 placed · 0 no home
a11y-plan: 2 proposals · fanout-a11y-plan.md (re-spawned once, NO_REPOINT; raw twin .raw-fanout-a11y-plan.md) · 11 cascade proposals · fanout-a11y-plan--cascade.md · 0 facts

Remarks carried: architecture — fct-zhr3wx2i's Git Bash rationale left to main (taken by cascade-g1 7); F3 (dialog_drift witness) no home in architecture, placed in test-plan (cascade-g3 6). security-plan, obs-plan, a11y-plan, design-system — the body/detail sites outside their groups named (raised below where the plan lists them).

The CARRY 1 direction (D1), the one text main authors for every trycmd / assert_cmd site: «the CLI tests drive the `viola` binary through `std::process::Command` and pin each fixed string by a literal in its test file; no manifest holds `assert_cmd`, `predicates`, `trycmd` or `snapbox`, and there is no `tests/cmd/`» (basis: the plan's Provenance lean «`trycmd|assert_cmd|snapbox` has 0 hits over every `Cargo.toml`, `tests/cmd/` is absent»; re-read at this wrap, `git grep -e assert_cmd -e trycmd -e snapbox -e '^predicates' -- '*Cargo.toml'` 0 rows, `tests/cmd` absent). A version number of those tools (`assert_cmd 2.2.2`, `predicates 3.1.4`, `trycmd 1.2.1`) leaves with them.

The statusline-rate-limits finding (report Spec claims disproved, first bullet; `inputs#I7` item 1, F-06i): its homes are the ledger row's own fact (architecture fct-qwpklckh), the cross-cutting ledger fact naming `budget-paused` on it (fct-wfge3rsz), the budget gate's OPEN (fct-ikuc6e4y), and the two masters that state the row as relied on every build (security-plan fct-m6zdbeoz, test-plan fct-hpqcf7r7). A copy at any other site restates one of these.

fanout-architecture.md
1. apply — check 1: expected amendment 1 (P5-approved, names the `=0.9.0` move and the re-measured rationale); routine (accurate this-chunk addition)
2. apply — check 1: expected amendment 1 (fct-zegjs5rw)
3. apply — check 1: accurate this-chunk addition (Dependencies)
4. apply-merged → fanout-architecture.md 7 — the pin rides the F434 whole-fact edit of fct-irhgihut
5. apply — check 1: accurate this-chunk addition (Dependencies, Cross-project ci#42288148289)
6. apply — check 1: expected amendment 4 (T2)
7. apply — check 5 (raised by main): §Infrastructure Patterns → project-directory-structure, fct-irhgihut, F434 whole-fact edit (`@fact id=fct-irhgihut expect_rev=4 whole=true`, `inputs#I7` item 4): the seven `(removed …; recoverable at the tag multi-os-final)` pointer lines removed; the `cmd/*.toml` trycmd line removed (no `tests/cmd/`, D1); the `viola-pty/` line reads `portable-pty =0.9.0`; every other line byte-identical
8. apply — check 5 (raised by main): expected amendment 2 — conventions fct-nhsg4k7g, body fct-vf75c7sp, standard-contracts fct-ekks4dfz, occupied-resources fct-kekl3p46 follow T1 (`viola-mcp`'s `ServeError`, `viola-state`'s `StampsError`, `PinError`, `StampsReadError` and `Refused` ratified as named exceptions, the founder's own word via the overseer's T1, `inputs#I2`; no CARRY owns them) and the census rule (`thiserror::Error` derives over the product sources, `crates/viola-e2e` outside the count, 9); where a fact states neither the divergence nor the census, it is left as it reads
9. apply — check 5 (raised by main): expected amendment 5 — occupied-resources fct-i4wacyrp and body fct-6e4hf2ai: where the text names the ceiling's owner «Version done-check», it reads «owner: 0.2.0's first entry», PROVISIONAL kept (`inputs#I7` item 2); a fact naming no owner is left as it reads

fanout-architecture--cascade-g1.md
1. apply — check 1: expected amendment 1 (fct-kxiazeyf)
2. apply-merged → fanout-architecture.md 2 — the same fact fct-zegjs5rw
3. apply — check 1: expected amendment 1 (fct-ad44nn2e)
4. apply-merged → fanout-architecture.md 1 — the same fact fct-omwrdnlc
5. apply — check 1: expected amendment 1 «fct-co2wsjkw retired»
6. apply — check 1: Retired (T2) — the Git Bash reason leaves the [Hook Transport] rationale
7. apply — check 1: Retired (T2), fct-zhr3wx2i (the doc agent's remark)
8. apply — check 1: expected amendment 5 (kept; owner 0.2.0's first entry, `inputs#I7` item 2)
9. apply — check 1: expected amendment 5
10. apply — check 1: expected amendment 5 (fct-4nt6qtyp)
11. apply — check 7: architecture-newgroup-established-decisions.md · distinct; check 6: the founder's question F-06i recorded as an OPEN, owner 0.2.0's intent (`inputs#I7` item 1)
12. apply — check 7: architecture-newgroup-established-decisions.md · distinct; check 6: the report's first disproved claim disposed at its home fct-qwpklckh
13. apply — check 1: expected amendment 2 (T1) · as: the five read «ratified as named exceptions (the overseer's T1, founder-delegated, on the founder's own word «Принять всё», 2026-11-06, `inputs#I2`)»
14. apply — check 1: expected amendment 2 (fct-sha7o35z)
15. apply — check 1: expected amendment 2 (census fct-gctodeyz; `inputs#I7` item 4, 9 stays)
16. apply — check 1: expected amendment 5 (Measurement B, the founder's own word, `inputs#I2` dialog 7/7)
17. apply — check 1: expected amendment 5 (fct-c3wssf3l)
18. apply — check 1: expected amendment 4 (fct-onw4pywk)
19. apply-merged → fanout-architecture.md 6 — the same fact fct-nvft5tqc

fanout-architecture--cascade-g2.md
1. apply — check 1: expected amendment 4 (fct-k6htnrfj's `\\?\C:\…` clause leaves with `plain_drive`)
2. apply — check 6: the first disproved claim, at the ledger fact naming `budget-paused` on the row

fanout-security-plan.md
1. apply — check 1: expected amendment 7 (fct-serp72ds)
2. apply — check 1: expected amendment 7 (fct-uydjlfhj)
3. apply — check 1: Retired (the swap trigger spent)
4. apply — check 1: expected amendment 6 · as: the mechanism reads exactly «portable-pty closes every inherited descriptor in the child» (`inputs#I7` item 4), witnessed by E2 and the live fd table (fds 0–2 on the pty, no socket that is viola's endpoint or the wrapper's, no file under the home); «recorded, not established» and the portable-pty-psmux clause leave

fanout-security-plan--cascade-g1.md
1. apply — check 1: expected amendment 9 (the founder's own word, `inputs#I2` dialog 6/7)
2. reject — check 2: the statusline finding's homes are named above; the `~/.viola/` access-control row restates none of them
3. apply — check 6: the master stating the row as relied on every build (fct-m6zdbeoz)
4. apply — check 1: Measurement B, the founder's own word (`inputs#I2` dialog 7/7, card row 7)
5. apply-merged → fanout-security-plan.md 4 — the same fact fct-p7rzex2i
6. apply — check 1: expected amendment 7 (fct-6nwwowx3)
7. apply-merged → fanout-security-plan--cascade-g1.md 6 — check 7: security-plan-newgroup-dependency-security.md · contradiction · fct-6nwwowx3 · rewritten by fanout-security-plan--cascade-g1.md 6
8. apply — check 1: expected amendment 7 (fct-zj6omeqt)
9. apply-merged → fanout-security-plan--cascade-g1.md 8 — check 7: security-plan-newgroup-dependency-security.md · contradiction · fct-zj6omeqt · rewritten by fanout-security-plan--cascade-g1.md 8 (the graph delta rides the rewritten entity)
10. apply-merged → fanout-security-plan.md 1 — the same fact fct-serp72ds
11. apply-merged → fanout-security-plan.md 2 — the same fact fct-uydjlfhj
12. apply-merged → fanout-security-plan.md 3 — the same fact fct-4r3p2tgk

fanout-security-plan--cascade-g2.md
1. apply — check 1: expected amendment 8 (T1) · as: «ratified as named exceptions (the overseer's T1, founder-delegated, on the founder's own word, 2026-11-06, `inputs#I2`)»
2. apply — check 1: expected amendment 8 (the census rule, 9)

fanout-design-system.md
1. apply — check 1: expected amendment 13 (fct-7u3hdiu2, T2)

fanout-design-system--cascade.md
1. apply — check 1: expected amendment 13 (fct-bmfbeece's OPEN closed, the founder's own word, `inputs#I2` dialog 6/7)
2. reject — check 2: the statusline finding's homes are named above; the verify surface's printed lines are unchanged (layout-templates holds the verify transcript), and design-system states no row as relied

fanout-test-plan.md
1. apply — check 1: expected amendment 10 (fct-njspb5nv)
2. apply — check 1: accurate this-chunk addition (Dependencies)
3. apply — check 1: expected amendment 10 (fct-hlx2zkbt)
4. apply — check 1: Retired (T2)
5. apply — check 1: expected amendment 10 (fct-u3dkdc2w)
6. apply — check 1: expected amendment 10 (fct-vxnii5yh)
7. apply — check 1: expected amendment 10 (fct-yoazurel)
8. apply — check 1: expected amendment 10 (fct-2ztzeoct)
9. apply — check 1: expected amendment 10 (the same Windows wording)
10. apply — check 1: expected amendment 10
11. apply — check 1: expected amendment 10 (fct-3mj52n2x)
12. apply — check 1: expected amendment 10
13. apply — check 5 (raised by main): expected amendment 10, CARRY 1 — every `trycmd` / `assert_cmd` / `predicates` / `snapbox` / `tests/cmd` site of test-plan (pattern `trycmd`, its test-plan rows: body fct-a6gfb3bv, fct-hyr7b53t, fct-4ignlk4w, fct-gcer5i4i, fct-s42l2rwd, fct-klrvzf5j, fct-w5xcngvx, fct-gnzxb6ex, fct-iqthlzz5, fct-puo2hazj, fct-rvltlmnj, fct-ajzn4txb, fct-qxrlql3x; detail fct-ennvbmje, fct-fbkvffnw, fct-aajqyzhy, fct-lzdm6e7w (its trycmd lines), fct-sik7tvk4, fct-fnukifof, fct-hlf5klew, fct-hckuzazj, fct-yhevpdmp; keys fct-hgewdmj2, fct-2j6rht5l) · as: D1; a rule that names a tool's API (`assert_cmd` `.assert()`) reads the same rule over `std::process::Command`
14. apply — check 5 (raised by main): expected amendment 14 — fct-upolcsa3's owner «Version done-check» for the always-admit survivor reads «the version's known limit, read at the epoch-boundary audit (matrix#v1-44)»

fanout-test-plan--cascade-g1.md
1. reject — check 2: the statusline finding's test-plan home is fct-hpqcf7r7 (cascade-g1 5)
2. apply — check 1: expected amendment 10 (fct-b2j3wr36)
3. apply — check 1: expected amendment 14 (fct-cz5jn5fs)
4. apply — check 1: accurate this-chunk addition (the new contract case)
5. apply — check 6: the master stating the row as relied on every build (fct-hpqcf7r7)

fanout-test-plan--cascade-g2.md
1. apply — check 1: expected amendment 10 (fct-vmzc7an2)
2. apply — check 1: expected amendment 10 (`plain_drive_*`)
3. apply — check 1: Retired (the `.exe` trim)
4. apply — check 1: Retired (T2)
5. reject — check 2: the statusline finding's test-plan home is fct-hpqcf7r7

fanout-test-plan--cascade-g3.md
1. apply — check 1: expected amendment 10 (P5-approved, names the re-word to the literal pins; its severity=escalate is the wording, settled by D1) · as: D1 — no `assert_cmd` / `predicates` named
2. apply — check 1: expected amendment 10 (fct-tr6c4sml)
3. apply — check 1: expected amendment 10 · as: D1
4. apply — check 1: expected amendment 6 · as: the mechanism reads exactly «portable-pty closes every inherited descriptor in the child» (`inputs#I7` item 4)
5. apply — check 1: expected amendment 10 (fct-hhwc4gim)
6. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
7. apply — check 1: expected amendment 10 · as: D1 (snapbox redactions leave with trycmd)

fanout-test-plan--cascade-g4.md
1. apply — check 1: accurate this-chunk addition (the remnant rule)
2. apply — check 1: expected amendment 10 (per-OS wording)
3. apply — check 1: expected amendment 10 (per-OS wording)
4. apply — check 1: accurate this-chunk addition (Dependencies, Cross-project)
5. apply — check 1: expected amendment 14 (fct-ocdr3xdn)

fanout-obs-plan.md
1. apply — check 1: expected amendment 11 (fct-t2iihkln)
2. apply — check 1: accurate this-chunk addition (Dependencies)

fanout-obs-plan--cascade-g2.md
1. apply — check 1: expected amendment 11 (fct-xrkfnxjv)
2. reject — check 2: the statusline finding's homes are named above; obs-plan's budget scenario restates architecture's
3. apply — check 1: expected amendment 11 · as: the F434 whole-fact form `@fact id=fct-yb5jpaot expect_rev=1 whole=true` (`inputs#I7` item 4); the row at the header's 2 cells, `pty_backend` (`openpty`) alone, `sideload_fallback` gone

fanout-obs-plan--cascade-g3.md
1. apply — check 1: expected amendment 11 (fct-j5jj3gux) · as: «as re-measured at chunk 2026-11-06-version-done-check» in place of «as measured at chunk 2026-10-31-sanitised-error-surfaces»

fanout-a11y-plan.md
1. apply — check 1: expected amendment 12 (fct-4ktfiloi)
2. apply — check 1: expected amendment 12 (fct-dlpkde76) · as: D1 on the key's tui and cli tool lines too
3. apply — check 5 (raised by main): expected amendment 12, CARRY 1 — the remaining a11y-plan `trycmd` / `assert_cmd` sites (pattern `trycmd`, its a11y-plan rows: body fct-xmkdtzxa, fct-zj2wxt2x, fct-6py5v5ks, fct-3xiq4dqd, fct-jkkyxmll, fct-cyo3rpos, fct-s4vibfo3, fct-ms7djxet; detail fct-dx4kie52, fct-es7le4as; keys fct-trh3q5nm, fct-avr5o2ub) · as: D1

fanout-a11y-plan--cascade.md
1. apply — check 1: expected amendment 12 · as: D1 (no `assert_cmd` 2.2.2 / `predicates` 3.1.4 named)
2. apply — check 1: expected amendment 12 (fct-twqx5x46)
3. apply-merged → fanout-a11y-plan.md 1 — the same fact fct-4ktfiloi
4. apply — check 1: expected amendment 12 · as: D1
5. apply — check 1: expected amendment 12 · as: D1
6. apply — check 1: expected amendment 5 (kept, owner 0.2.0's first entry)
7. apply — check 1: expected amendment 12
8. apply — check 1: expected amendment 12
9. apply — check 1: expected amendment 12
10. apply — check 1: expected amendment 12
11. apply — check 1: expected amendment 12 · as: D1 (no `assert_cmd` 2.2.2 / `predicates` 3.1.4 named)

## Checks across the set
- check 2: the statusline finding's ten proposals reduced to its five homes (5 rejected as restatements); no two proposals edit one section in opposing directions.
- check 3: scope record none; the report's Deviations each on a recorded word (`inputs#I5`, `inputs#I6`, `inputs#I7`).
- check 4: the CARRY 1 absence claim (no tool in any manifest) rests on the plan's lean and this wrap's `git grep`, 0 rows.
- check 5: every expected amendment carried (1–13 by proposals or main's raises above; 14 the v1-44 ledger note at P7.3 plus fct-upolcsa3 / fct-cz5jn5fs / fct-ocdr3xdn; 15 the sidecar histories at the entry writers, `marks-discharged.md`).
- check 6: disproved claim 1 disposed at its five homes; disproved claim 2 (the plan's step-10 premise) has no master site — curation.
