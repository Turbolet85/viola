# Fan-out results — 2026-11-17-booted-wrapper-does-not-outlive-its-test

architecture: add 4 · set 0 · retire 0 · none 0 · ops-architecture.md
security-plan: add 2 · set 0 · retire 0 · none 0 · ops-security-plan.md
design-system: no proposer
layout-templates: no proposer
test-plan: add 2 · set 3 · retire 0 · none 1 · ops-test-plan.md
obs-plan: none · ops-obs-plan.md — reason: the chunk adds no telemetry, log field, event, span or metric; every touched obs-plan row stands true
a11y-plan: none · ops-a11y-plan.md — reason: no interactive surface, CLI output or a11y schema touched; the tied spawn is still a std::process::Command

Expected amendments (check 5): test-plan `decision/one-verify-stamp-per-build` — ops-main-test-plan.md (the proposer's set truncated) · test-plan `decision/wrapper-drop-guard` — ops-test-plan.md set n=1 · test-plan `decision/rust-cache-only-cache` — no operation: the row («rust-cache is the only cache, used in the push and pull-request workflow only») stays true; the chunk's fix holds whatever a restore leaves and changes no cache · architecture §Occupied Resources — ops-architecture.md adds n=1, n=2.
Disproved claim (check 6): «44 hits in 24 files» (research, plan) — no master site (sweep `spawn-sites` master 0); disposed by test-plan add `decision/no-untied-viola-spawn` (the census over every spelling) and P3 curation.

ops-architecture.md
name/other/root-tied-test-child · apply — check 1: routine «Accurate this-chunk addition», the report's Symbols / APIs names the marker; precedent name/other/mcp-serve-test-child
name/other/harness-tied-test-child · apply — check 1: routine «Accurate this-chunk addition», the report's Symbols / APIs names the marker; precedent name/other/mcp-serve-test-child
decision/test-viola-children-tied · reject — check 2: the fact's home is test-plan decision/wrapper-drop-guard
decision/test-verify-stamp-generations · reject — check 2: the fact's home is test-plan decision/one-verify-stamp-per-build

ops-security-plan.md
decision/test-signals-only-recorded-processes · reject — check 2: the fact's home is test-plan decision/pid-start-time-verified-kill
decision/test-children-tied-to-their-test · reject — check 2: the fact's home is test-plan decision/wrapper-drop-guard

ops-test-plan.md
decision/wrapper-drop-guard · apply — check 1: routine «Accurate this-chunk addition»; the plan's expected amendment, the report's tied.rs and BootOptions.tied
decision/one-verify-stamp-per-build · reject — check 7 ahead: the value is cut off mid-clause («the next generation dir is stamped»); main's own set in ops-main-test-plan.md
decision/supervisor-ordinary-child · apply — check 1: routine «Accurate this-chunk addition»; the report's BootOptions.for_session / for_cli
decision/no-untied-viola-spawn · apply — check 1: routine «Accurate this-chunk addition»; the census and the 45th site (report Deviations, Spec claims disproved)
decision/tied-child-spawn-thread · apply — check 1: routine «Accurate this-chunk addition»; the report's Decisions & corrections, measured green

ops-main-test-plan.md
decision/one-verify-stamp-per-build · reject — check 7: not_master type 4 — the added clause retells how the stamp helper decides a stamp counts and names its next generation dir; the code and stamped_home_rebuilds_a_stamp_restored_without_its_files hold it, and the standing rule stays true
