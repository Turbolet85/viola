# Fan-out results — 2026-11-13-epoch-8-cleanup wrap

architecture: add 1 · set 0 · retire 0 · none 3 · ops-architecture.md
security-plan: add 1 · set 0 · retire 1 · none 1 · ops-security-plan.md
design-system: none · ops-design-system.md — reason: no UI element and no CLI output wording added or changed; the census filter change leaves `decision/no-internals-in-errors` true as written
layout-templates: none · ops-layout-templates.md — reason: no CLI output wording, no user-facing surface or region; the verify output rows stand (their test files only split)
test-plan: add 1 · set 4 · retire 1 · none 3 · ops-test-plan.md
obs-plan: add 2 · set 6 · retire 2 · none 1 · ops-obs-plan.md
a11y-plan: none · ops-a11y-plan.md — reason: no UI element; every surface a11y-plan states stands; neither a11y detector fires

Check 5 — the plan's Expected amendment «architecture §Infrastructure Patterns — the directory tree names
`src/bin/viola-fake-agent/`, `crates/viola-agent-claude/src/ledger/test_cases_*.rs` and `tests/cli_verify/`»: no
operation — architecture holds no row at that depth (`rec.py find --root . --doc architecture --regex
"src/bin/|tests/support|ledger\.rs|ledger/"`: 2 rows, both `ledger/stamps.json`, neither a tree row; the proposer's
`@row_none` F4 says the same). Superseded: no site to amend.
Check 5 — the plan's Expected amendment «test-plan §2 — one-shared-test-helper names the shared scanner and the
private-dir helper»: proposed, then rejected by fit (`not_master` type 5, an inventory the code holds); the delegate's
P5 approval (inputs#I4) is no operator word for check 7's exception. The two helpers reach the testing rule's helper
list at P3.

ops-architecture.md
decision/os-gated-cases-in-root-test-file · reject — check 2: the same fact is proposed in test-plan (`decision/os-gated-cases-in-binary-root`) and obs-plan (`decision/os-gated-test-census`); one thing stands under one key, and a test binary's file layout is the test plan's (it holds `decision/os-gated-tests-registered`), not an architecture decision

ops-security-plan.md
contract/artifact-canary-scan-classes/cookie-header · apply — check 1: the plan's Expected amendment names this retire (security-plan §Secret Management — remove `contract/artifact-canary-scan-classes/cookie-header`), P5-approved; the report's Retired bullet carries the cookie class
contract/artifact-canary-scan-classes/unreadable · apply — check 1: accurate this-chunk addition — the report carries the `unreadable` class's planted case (Spec claims disproved 5; v2-06's acceptance names it); the class itself stood in code at HEAD (`secret_scan.rs`, `hit(…, "unreadable")`) and the boundary's other three classes are listed; a contract row inside an existing boundary of a master with no keyed-contract section (`rec.py contracts security-plan`: `n/a — it carries no keyed-contract section`), so no NEW keyed contract (inputs#I5 item 5); it narrows nothing and crosses no boundary

ops-test-plan.md
decision/logs-carry-no-secrets · apply — check 1: the plan's Expected amendment (test-plan §1 — drop the Cookie clause), P5-approved; the rest of the rule kept
decision/no-secrets-in-logs · apply — check 1: the plan's Expected amendment (test-plan §1 — drop the Cookie clause); the mode clause kept
decision/secrets-never-in-logs · apply — check 1: the plan's Expected amendment (test-plan §6 — drop the Cookie clause)
decision/one-shared-test-helper · reject — check 7: not_master type 5 (`test-plan-fit.md`: «an inventory of test-support items that the code holds»); the P5 approval was a delegate's (inputs#I4), PROVISIONAL, no exception
contract/secret-scan-class/cookie · apply — check 1: the plan's Expected amendment (remove `contract/secret-scan-class/cookie`), the report's Retired bullet
decision/os-gated-cases-in-binary-root · apply — check 1: accurate this-chunk addition — the report's Deviations 3 and Decisions & corrections carry the measured census walk that kept the cli_verify gated cases in the root file; the test plan is its home (check 2 keeps it here alone)

ops-obs-plan.md
decision/secret-scan-before-upload · apply — check 1: the plan's Expected amendment (obs-plan §8 — drop the Cookie clause); the unreadable class the report carries named
decision/no-secrets-in-log-lines · apply — check 1: the plan's Expected amendment (obs-plan §1 — drop the Cookie clause)
decision/secrets-never-logged · apply — check 1: the plan's Expected amendment (obs-plan §8 — drop the Cookie clause)
decision/content-only-in-instance-detail-file · apply — check 5: the report's sweep names this sixth obs-plan cookie site (`dec-5d6fg64z`, beyond the plan's list); the cookie word goes with the retired class, the rest of the rule kept
decision/os-gated-test-census · reject — check 2: the same fact lands in test-plan (`decision/os-gated-cases-in-binary-root`); one thing stands under one key, the test plan's
decision/error-bodies-clean · reject — check 3: the fake agent is a test-only binary, so «every product error writer is named» already leaves it out; which test-only files the census filter skips is code inventory (`tests/contract_error_bodies.rs`), not a master fact, and the row has no `except` cell to set
decision/cookie-header-never-logged · apply — check 1: the plan's Expected amendment (obs-plan §11 — drop the Cookie clause; the whole row is that clause)
decision/claude-env-and-cookie-never-logged · reject — check 7: its heir `decision/claude-env-never-logged` reads `stands as` this row; the row stays and its rule and name are set in `ops-main-obs-plan.md` (the Cookie clause dropped, the key kept)
decision/claude-env-never-logged · reject — check 7: stands as `decision/claude-env-and-cookie-never-logged` (`obs-plan-fit.md`); that row is set instead
decision/library-test-scoped-subscriber · reject — check 3: one test's mechanism; obs-plan's `decision/subscriber-only-in-root-bin` and the one-global-tracing-install rule already state the boundary it keeps — the how-to is curation's (P3), not a master row

Note: ops-main-obs-plan.md landed in tx 949; it stood renamed for one fit render (wave 3, which reads this file name alone) and is back under its landed name.
ops-main-obs-plan.md
decision/claude-env-and-cookie-never-logged · apply — check 1: the plan's Expected amendment (obs-plan §11 — drop the Cookie clause of decision/claude-env-and-cookie-never-logged); the fit reader named this row as where the CLAUDE* rule stands

Note: the name set — judged `fits` by wave 3 (obs-plan-fit.w3.md) while it stood as ops-main-obs-plan.md; rows-land refused that name (`landed at tx 949 under another text`), so it lands under its own.
ops-main-obs-plan-name.md
decision/claude-env-and-cookie-never-logged · apply — check 1: the name follows the rule set landed in tx 949
