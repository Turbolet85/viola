# Fan-out results — 2026-11-15-mutation-and-coverage-baselines-green (wrap 2026-11-15T20-06-27-wrap)

architecture: add 1 · set 1 · retire 0 · none 0 · ops-architecture.md
security-plan: add 2 · set 0 · retire 0 · none 0 · ops-security-plan.md
design-system: no proposer
layout-templates: no proposer
test-plan: add 2 · set 4 · retire 0 · none 0 · ops-test-plan.md
obs-plan: none · ops-obs-plan.md — reason: test-only chunk; no obs-plan row moved
a11y-plan: none · ops-a11y-plan.md — reason: test-only chunk; no a11y surface or schema touched

ops-architecture.md
decision/root-waits-share-one-bound · apply — check 1: the plan's P5-approved Expected amendment «architecture decision/root-waits-share-one-bound — the one bound is 12 s» names the change; routine (Accurate this-chunk addition)
decision/test-sockets-under-short-tmp-root · apply — check 1: the plan's P5-approved Expected amendment «architecture §Occupied Resources — the test-owned socket root» names it; check 2: this master is the fact's home (the plan's entry), the security-plan and test-plan adds rejected; check 3: the fold's widening carries the operator's word (inputs#I4)

ops-security-plan.md
decision/test-sockets-in-short-tmp-dir · reject — check 2: the fact's home is architecture decision/test-sockets-under-short-tmp-root
decision/record-test-home-no-home-component · reject — check 2: the fact's home is test-plan decision/record-home-names-no-home

ops-test-plan.md
decision/root-wait-bound · apply — check 1: the plan's P5-approved Expected amendment «test-plan decision/root-wait-bound 7 s → 12 s, below the 15 s kill» names the change
decision/mutants-kill-below-cargo-mutants-timeout · reject — check 5: the plan's entry requires the departure recorded (inputs#I2); main's own set in ops-main-test-plan.md carries it
decision/send-window-paste-hint-30s-kill · apply — check 1: routine (Accurate this-chunk addition): the why's «10 s default kill» literal is this chunk's moved value, the class's 30 s kill and its reason unchanged
decision/within-7s-wait-bound · apply — check 1: the plan's P5-approved Expected amendment «test-plan decision/within-7s-wait-bound — WITHIN is 12 s» names the change
decision/record-home-names-no-home · apply — check 1: the plan's P5-approved Expected amendment «test-plan §7 — a home given to viola verify --record in a test comes from the record-home helper» names it; check 2: this master is the fact's home
decision/test-sockets-under-short-tmp-root · reject — check 2: the fact's home is architecture decision/test-sockets-under-short-tmp-root

ops-main-test-plan.md
decision/mutants-kill-below-cargo-mutants-timeout · apply — check 1: the plan's P5-approved Expected amendment «test-plan decision/mutants-kill-below-cargo-mutants-timeout — the default kill is 15 s, below the 20 s floor, with the departure recorded (inputs#I2, delegate overseer)» names the change
