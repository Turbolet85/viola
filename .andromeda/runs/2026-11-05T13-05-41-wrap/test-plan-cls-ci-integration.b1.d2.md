@answer n=1 entity=ci-integration#1.1 v=yes mode=contradict
why: the dev-host Windows-target and macOS-target check-only clippy recipes are gone with those legs, and the rmcp step's "every OS" reading is stale
@answer n=2 entity=ci-integration#1.2 v=yes mode=add-to
why: F4 — the supply-chain job reads red on RUSTSEC-2026-0441 (pin-utils through portable-pty =0.8.1 and nix 0.25.1), owned by a CARRY on Version done-check, with the ignore list not widened
@answer n=3 entity=ci-integration#1.3 v=yes mode=stale
why: the windows pwsh scripts/agent-run.ps1 parenthetical names a file the session deleted
@answer n=4 entity=ci-integration#1.4 v=no
why: the Integration stage's agent-run command, parallelism and cache are not among the session's ci.yml changes, which are the ConPTY, pwsh shim, runner.os and windows-2025 coverage steps
@answer n=5 entity=ci-integration#1.5 v=yes mode=stale
why: the E2E stage runs agent-run run --e2e on all 3 OSes, and it now runs on ubuntu-latest only
@answer n=6 entity=ci-integration#1.6 v=yes mode=contradict
why: the windows-2025 ConPTY vendor verification step that runs first and the conpty_sideload binary in the coverage run are removed
@answer n=7 entity=ci-integration#1.7 v=yes mode=contradict
why: windows-mutants.yml is deleted, so the Windows mutation leg and its ten mutants jobs no longer exist
@answer n=8 entity=ci-integration#1.8 v=no
why: the perf job keeps its strategy.matrix.os form with one leg, and its steps, hyperfine pin and jq handling are unchanged
@answer n=9 entity=ci-integration#1.9 v=no
why: the msrv job is ubuntu-only, and the session did not change its toolchain steps, agent-run call or 1.96 floor
@answer n=10 entity=ci-integration#1.10 v=no
why: the fuzz-replay job is ubuntu-only, and neither its ci.yml steps nor nightly.yml's fuzz job changed
@answer n=11 entity=ci-integration#1.11 v=no
why: the release job keeps its strategy.matrix.os form with one leg, and its release-check steps are unchanged
@answer n=12 entity=ci-integration#1.12 v=yes mode=stale
why: the workflow list is ci.yml and nightly.yml only, and zizmor's concurrency-limits suppressions are 2 (one per workflow), not 3
@answer n=13 entity=ci-integration#1.13 v=yes mode=stale
why: the ci kill-line binary(...) filter drops from 27 names to 26 with binary(conpty_sideload) removed
@answer n=14 entity=ci-integration#1.14 v=yes mode=contradict
why: scripts/junit-os-sets.py is retired with the junit-os artifacts, so the per-OS JUnit reader no longer exists
@answer n=15 entity=ci-integration#1.15 v=yes mode=stale
why: the per-OS JUnit set comparison reads windows-2025 and macos-latest legs the session removed, so its OS sets are stale
