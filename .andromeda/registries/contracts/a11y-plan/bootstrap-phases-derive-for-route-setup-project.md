### Bootstrap phases (derive for route / setup-project)

The downstream skills derive the following bootstrap phases from
the contract above. Listed for explicitness — route may reorder /
combine, setup-project may add stack-specific intermediate steps.

- **a11y-tooling-install:** none in 0.1.0. The cli / tui cases use the tests' own assert_cmd, predicates, trycmd and portable-pty pins; the browser a11y tooling left with the browser leg, and the frontend's is a 0.2.0 input, its technology not chosen.
- **focus-management-library-install:** none.
- **aria-component-library-install:** none. The frontend is deferred to 0.2.0; its technology is not chosen.
- **contrast-verification-harness-setup:** none in 0.1.0: no page renders a token pair.
- **screen-reader-test-spec-setup:** none in 0.1.0; the manual NVDA / VoiceOver / Orca pass stays founder-owned (Section 1 → Screen reader test pattern).
- **a11y-ci-gate-wire:** none of its own: the cli / tui cases run inside the per-OS `test` job's `coverage` suite under `gate --require coverage,doctest`, and no a11y lint, aggregation or upload step exists.
- **violation-json-emission-wire:** none in 0.1.0 — cli / tui emit no a11y violation rows (Section 3 → Structured violation JSON schema: output-discipline failures surface only as nextest failures in `suites[].failures[]`), and obs's closed `process` enum is `run · hook · mcp · cli`.

route uses this list to plan phase ordering (typically:
a11y-tooling-install → focus-management-library-install →
aria-component-library-install → contrast-verification-harness-setup
→ screen-reader-test-spec-setup → a11y-ci-gate-wire →
violation-json-emission-wire). setup-project uses this list to
materialize each phase's bootstrap script + dependency list +
verification command.

---
