@answer n=1 entity=ci-integration#1.1 v=yes
why: the lint job goes Linux-only and its per-OS wording and the dev-host Windows- and macOS-target check-only clippy recipes lose their purpose (scope item 16)
@answer n=2 entity=ci-integration#1.2 v=yes
why: the supply-chain job's cargo deny, deny-probes 14/14, sole-root tokio ban and channel-tokio-free runs must still pass after the trimmed deny.toml and removed Windows deps (scope item 17)
@answer n=3 entity=ci-integration#1.3 v=yes
why: the unit stage's Windows pwsh agent-run.ps1 line is a Windows CI step that leaves with the two pwsh shim steps (scope item 6)
@answer n=4 entity=ci-integration#1.4 v=no standing=S5
why: its only convention line is rust-cache as the cache in ci.yml, which S5 owns; the Linux integration run names no removed OS and is unchanged
@answer n=5 entity=ci-integration#1.5 v=yes
why: the E2E row runs on all 3 OSes as a per-OS matrix, which shrinks to ubuntu-latest (scope item 1)
@answer n=6 entity=ci-integration#1.6 v=yes
why: the per-OS coverage test job's windows-2025 ConPTY vendor verification steps leave with vendor/conpty, and the conpty_sideload binary leaves the coverage run (scope items 2, 7, 15)
@answer n=7 entity=ci-integration#1.7 v=yes
why: it names windows-mutants.yml and its ten Windows mutants jobs, which the scope removes whole (scope item 3)
@answer n=8 entity=ci-integration#1.8 v=yes
why: the perf job is a per-OS matrix that shrinks to ubuntu-latest (scope item 1)
@answer n=9 entity=ci-integration#1.9 v=yes
why: the ubuntu msrv job must stay green on Rust 1.96 over the unit suite once the e2e free-space floor cases are fixed (scope item 5, task item 18)
@answer n=10 entity=ci-integration#1.10 v=yes
why: the nightly cargo fuzz runs stay on ubuntu when nightly.yml's Windows and macOS jobs leave (task item 4)
@answer n=11 entity=ci-integration#1.11 v=yes
why: the release job is per OS, so its Windows and macOS legs leave, and the release-check probes must still end their required line (scope item 8, task item 17)
@answer n=12 entity=ci-integration#1.12 v=yes
why: the gate tail names windows-mutants pins and tests/contract_windows_mutation_scope.rs, both removed (scope items 3, 4), and the test and perf jq rows that remain on Linux
@answer n=13 entity=ci-integration#1.13 v=yes
why: the 27-binary count on the ci profile's stamped-home filter must be re-counted if the removed test binaries such as conpty_sideload are on that filter (scope item 7)
@answer n=14 entity=ci-integration#1.14 v=yes
why: scripts/junit-os-sets.py names the junit-windows-2025 and junit-macos-latest artifacts that the removed jobs stop producing, so its per-OS sets name gone code (task item 14)
@answer n=15 entity=ci-integration#1.15 v=yes
why: it records per-OS case sets, including the 43 macOS no-peer-pid carve-out cases and the 88 Windows-only cases, which the removed carve-outs and the OS-gated census re-point (task items 11, 14)
