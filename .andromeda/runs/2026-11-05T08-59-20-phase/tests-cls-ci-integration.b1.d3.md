@answer n=1 entity=ci-integration#1.1 v=yes
why: the lint job's per-OS wording and the Windows- and macOS-target check-only clippy runs, which lose their purpose (task item 16), bind.
@answer n=2 entity=ci-integration#1.2 v=yes
why: the deny-probes 14/14 count and the deny.toml bans move with the removed Windows dependency entries (task item 17).
@answer n=3 entity=ci-integration#1.3 v=yes
why: the Unit row's pwsh agent-run.ps1 run on windows is one of the two pwsh shim steps the chunk removes from ci.yml.
@answer n=4 entity=ci-integration#1.4 v=no
why: the Integration stage's agent-run run --integration names no OS and no edited unit, so it keeps holding without a change.
@answer n=5 entity=ci-integration#1.5 v=yes
why: the E2E stage runs on all 3 OSes through a per-OS matrix, which the chunk shrinks to ubuntu-latest (task item 1).
@answer n=6 entity=ci-integration#1.6 v=yes
why: the per-OS test job's windows-2025 ConPTY vendor verify and probe steps leave with vendor/conpty and the Windows job, and the coverage run's conpty_sideload binary goes with its file.
@answer n=7 entity=ci-integration#1.7 v=yes
why: windows-mutants.yml is removed whole, so its ten windows-2025 mutants jobs and the dispatch-only leg they belong to leave (task item 3).
@answer n=8 entity=ci-integration#1.8 v=yes
why: the perf job is a per-OS matrix job in ci.yml, and its Windows and macOS legs are among the jobs the chunk removes.
@answer n=9 entity=ci-integration#1.9 v=yes
why: the scope names the msrv job (ubuntu, Rust 1.96, unit suite) as the check that must be green again through the e2e free-space fix (task items 5 and 18).
@answer n=10 entity=ci-integration#1.10 v=yes
why: the scope names the nightly time-boxed cargo fuzz runs as staying on ubuntu while nightly.yml's Windows and macOS jobs leave (task item 4).
@answer n=11 entity=ci-integration#1.11 v=yes
why: the release job in ci.yml is per OS, and its Windows and macOS legs are among the jobs the chunk removes.
@answer n=12 entity=ci-integration#1.12 v=no
why: the gate's reading and exit contract over junit, run-summary, llvm-cov and hyperfine JSON names no OS and no unit the chunk edits, so it is a re-check every change owes.
@answer n=13 entity=ci-integration#1.13 v=yes
why: the chunk removes the integration binaries tests/conpty_sideload.rs and tests/contract_windows_mutation_scope.rs, which the ci filter's count of 27 binaries may name, so that count must be re-checked.
@answer n=14 entity=ci-integration#1.14 v=yes
why: the reader downloads junit-windows-2025 and junit-macos-latest artifacts, which leave with the per-OS artifact names (task item 1).
@answer n=15 entity=ci-integration#1.15 v=no
why: a dated measurement at ci#41918694243 that the chunk neither re-measures nor re-pins, so its counts stand as history.
