# Fan-out results — 2026-10-26-tokio-rustsec-2026-0386-lock-bump

Batches: stage1 (13: doc 7 · task 6; the architecture and test-plan task writers re-spawned once on
`[UNTOUCHED_REACHED]`, stage1-r1 2 of 2 pass) · classify (62; `test-plan-cls-mocking-stubbing-discipline.b1`
re-spawned once on `[EVERY_ENTITY]`, classify-r1 1 of 1 pass) · strong (3 of 3 pass) · newdetail none (every fact
placed). Sample: `sample: no · wrap · … · draw 1 of 10`. Proposals (the `check` lines): architecture 1 ·
security-plan 0 · design-system 0 · layout-templates 0 · test-plan 1 · obs-plan 0 · a11y-plan 0; cascade:
architecture 4 · security-plan 2 · test-plan 3.

- architecture: 1 proposals · fanout-architecture.md · 4 cascade proposals · fanout-architecture--cascade.md · 2 facts · 2 placed · 0 no home
  note: the stack-and-technologies.md:11 site is a standing detail group, the cascade's; D-arch-resources no drift; D-retired 0 hits in architecture.
- security-plan: 0 proposals · fanout-security-plan.md · 2 cascade proposals · fanout-security-plan--cascade.md · 0 facts · 0 placed · 0 no home
  note: the body states neither tokio's version nor RUSTSEC-2026-0386; both sit in dependency-security.md (the cascade's).
- design-system: 0 proposals · fanout-design-system.md · 0 facts · 0 placed · 0 no home
  note: no UI element, no output literal; D-retired grep exit 1.
- layout-templates: 0 proposals · fanout-layout-templates.md
  note: no cli wording or surface change; tokio / rustsec / supply-chain / advisor 0 hits.
- test-plan: 1 proposals · fanout-test-plan.md · 3 cascade proposals · fanout-test-plan--cascade.md · 0 facts · 0 placed · 0 no home
  note: the three category-file sites are the cascade's; the body holds no 1.53.1 or RUSTSEC-2026-0386 literal.
- obs-plan: 0 proposals · fanout-obs-plan.md · 0 facts · 0 placed · 0 no home
  note: no symbol, hot-path op, logging or telemetry dependency; no tokio version or advisory line in obs-plan.
- a11y-plan: 0 proposals · fanout-a11y-plan.md · 0 facts · 0 placed · 0 no home
  note: no new surface or schema; no tokio / RUSTSEC / supply-chain text.

## Dispositions

fanout-architecture.md
1. apply — check 1: Accurate this-chunk addition (the report's Dependencies bullet moves tokio `=1.53.1` → `=1.53.3`, containment unchanged); check 5: the plan's expected amendment «architecture §Inherited Defaults»

fanout-architecture--cascade.md
1. apply — check 1: Accurate this-chunk addition (Dependencies; `tokio-ver` row `stack-and-technologies.md:11`); check 5: the plan's expected amendment «architecture §Stack and Technologies»
2. apply-merged → fanout-architecture.md 1 — the same §Inherited Defaults line 442 and the same move
3. reject — check 1: Registry over-reach (the stack table tracks technologies at category grain; one advisory's patched band for one crate is per-item realization, and the advisory's record is security-plan's, whose open-advisory line this pass retires with its sidecar entry); check 7: architecture-newgroup-stack-and-technologies.md · contradiction — the contradicted text is line 11's «Tokio 1.53.1», which proposal 1 of this file moves to 1.53.3 in this pass (check 2: no contradiction stands after it)
4. reject — check 1: Registry over-reach (four deprecated constructors of a dependency's API are per-symbol realization inside the already-registered Tokio row; clippy `-D warnings` is the gate that holds the fact, per the report's Symbols / APIs bullet); check 7: architecture-newgroup-stack-and-technologies.md · distinct

fanout-security-plan--cascade.md
1. apply — check 1: Accurate this-chunk addition (Dependencies; `tokio-ver` row `dependency-security.md:14`); check 5: the plan's expected amendment «the `deps` group's tokio `=1.53.1`»
2. apply — check 1: D-retired whose item the report's Retired bullet names (routine); check 5: the plan's expected amendment «the `deps-supply-chain` group's open-advisory line retired» · written by main: apply-security-plan.md refused it (the retirement empties the whole group, which an in-line block cannot), so main removed the group `<!-- detail: values · deps-supply-chain -->` holding the one line (`dependency-security.md` lines 55–60, the file now ending at line 54 `<!-- /detail -->`) with the Edit tool; `detail.py check` 0 defects, `classify.py check` PASS; the `deps-supply-chain` subject stays (three other groups carry it)

fanout-test-plan.md
1. apply — check 1: Accurate this-chunk addition (Dependencies; `tokio-ver` row of the §3 Bootstrap phases key file line 24); check 5: the plan's expected amendment «test-plan §3 … the Bootstrap phases key file's tokio version»

fanout-test-plan--cascade.md
1. apply — check 1: Accurate this-chunk addition; check 5: the plan's expected amendment «test-plan §7 … tokio `=1.53.1`» · as: replace `=1.53.1` with `=1.53.3` in the parenthesis and keep its features clause; write no history clause into the body (no «the pin moved from `=1.53.1`») — the parenthesis reads «(as landed at chunk 2026-10-17-mcp-server-for-drivers, the version as of chunk 2026-10-26-tokio-rustsec-2026-0386-lock-bump: tokio `=1.53.3` with default features off, `viola-mcp` `rt`, `viola-channel` `io-util` and dev `rt`, `viola-e2e` `rt` + `process`)»; the old value goes to the sidecar entry
2. apply — check 1: Accurate this-chunk addition; check 5: the plan's expected amendment «test-plan §8 … twice» (line 8)
3. apply — check 1: Accurate this-chunk addition; check 5: the plan's expected amendment «test-plan §8 … twice» (line 17, the «in no manifest yet» qualifier kept)

Check 5 (the plan's `Expected amendments (wrap)`): every entry matched — architecture 442 (architecture 1), stack row 11 (cascade 1), security deps 14 and the retired line 58 (security cascade 1, 2), test-plan §8 ×2 and §7 (test cascade 2, 3, 1), the §3 key file (test-plan 1); the Tier-3 docs `.claude/docs/stack.md:10,47` and `.claude/docs/services/viola-mcp.md:11` are leaves — Cascade step 3. Check 6: the report's Spec claims disproved is none. Checks 2, 3, 4: no opposing pair, no divergence from intent (scope record: none), no absence claim. Escalations: 0. Rejected for a source the report does not carry: 0.
