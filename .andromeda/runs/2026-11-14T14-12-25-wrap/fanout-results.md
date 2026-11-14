# Fan-out results — 2026-11-14-hyprland-exact-frame-bound-pinned

architecture: no proposer
security-plan: none · ops-security-plan.md — reason: test-only; every picked row still reads true and the task brings no fact
design-system: no proposer
layout-templates: no proposer
test-plan: add 0 · set 2 · retire 0 · none 0 · ops-test-plan.md
obs-plan: none · ops-obs-plan.md — reason: test-only; every picked obs-plan row stays true, no new operation, telemetry dependency or user-data logging
a11y-plan: no proposer

ops-test-plan.md
decision/fakes-for-external-services · apply — check 1: playbook «Accurate this-chunk addition» (routine) — the report's Changes names the read test's two planted `hyprctl` stubs in viola-hyprland's own test module, and Decisions & corrections (fct-gpbtjh4c) the reason; the row's «the real hyprctl by the same binary as hyprctl» needs the named exception, its invariant (tests never reach the real services) holding
decision/fake-agent-as-claude · reject — check 2: the fact's home is test-plan decision/fakes-for-external-services — the same exception text set on two rows of one master is one fact twice; the mocking-and-stubbing row answers what replaces hyprctl in every test, while this row (area 5-integration-test-strategy) states the integration tier's boundary, which the crate's unit-tier stub does not change
