# Answer card — 2026-11-13-verify-readiness-on-the-input-box (wrap 2026-11-13T06-27-36, Q1)

The founder's own answers to the two P4 forks (F1, F2 of `inputs#I5`), given live at this wrap's P1 through the
overseer's dialog, 2026-11-13T06:32:35Z, recorded `inputs#I9`:
- F1 «Утвердить ignore» — the three reasoned deny.toml ignores stand, rmcp stays at >=3.4.1, <3.5.
- F2 «Утвердить добавление» — the box signature added beside the two footer literals, for verify and for viola run's
  pre-send gate; asked as a possible boundary widening and ratified as such.

Escalated operations main reads as lying wholly inside those answers:

F1 (the reasoned rmcp advisory ignores):
- ops-security-plan.md · decision/ci-cargo-deny-gate — dependency-security — the gate fails on any advisory not on its reasoned ignore list
- ops-security-plan.md · decision/cargo-deny-gate-clean — dependency-security — retired, heir decision/ci-cargo-deny-gate
- ops-security-plan.md · decision/no-ignored-advisories — dependency-security — retired, heir decision/rmcp-advisory-ignores
- ops-security-plan.md · decision/rmcp-advisory-ignores — dependency-security — added: ignores exactly the three, each its disabled feature, until rmcp leaves <3.5
- ops-test-plan.md · decision/no-advisory-ignored — test-scope — `except`: the disabled-rmcp-feature ignore
- ops-test-plan.md · decision/no-ignored-advisories — ci-integration — `except`: the same

F2 (the box signature beside the footer literals, verify and run's gate):
- ops-architecture.md · decision/screen-signatures-compiled — established-decisions — ready on a footer literal or the box's rule-above-prompt rows, no modal
- ops-architecture.md · decision/verify-box-wait-for-literal — established-decisions — verify's waits end on the input box (literal or rows) or a modal
- ops-architecture.md · decision/screen-fixtures-signature-rows-only — occupied-resources — a box row kept as its literal alone
- ops-security-plan.md · decision/screen-fixture-signature-rows-only — data-protection — the same recording form
- ops-security-plan.md · decision/input-box-readiness-from-literals — input-validation — added: the predicate, compiled literals only
- ops-test-plan.md · contract/readiness-gate/gate-max-wait — unit-test-strategy — waits when the box does not read up
- ops-test-plan.md · decision/screen-signatures-only — unit-test-strategy — one predicate over the footer literals or the box rows
- ops-test-plan.md · decision/screen-fixture-box-rows-as-literals — test-data-fixtures — added: box rows recorded as literals, the walk refusing more
- ops-a11y-plan.md · decision/no-screen-content-parsing — r7 — the box's rows parsed, the prompt row only for its opening char

Check 6 groups no further occurrence with them.

## The answer
Q1 answered: "Confirm (Recommended)" — delegate overseer, 2026-11-13 (PROVISIONAL as a confirm; the ratification is the founder's own F1/F2 answers, inputs#I9).
Notes, verbatim: The overseer's delegate confirm, read against the founder's own two answers of 2026-11-13T06:32:35Z (inputs#I9) and the card's fifteen lines: each lies inside F1 (exactly the three reasoned rmcp ignores; the gate still fails on any advisory off that list) or F2 (the box signature beside the footer literals, verify and run's gate, a box row recorded as its literal alone). None outside.
