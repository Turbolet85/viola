@answer n=1 entity=ci-integration#1.1 v=yes mode=contradict
why: the session removes the dev-host Windows and macOS check-only clippy runs, and Lint now runs on ubuntu-latest alone, so the rmcp step no longer runs on every OS
@answer n=2 entity=ci-integration#1.2 v=yes mode=add-to
why: the supply-chain job now reads red on RUSTSEC-2026-0441 (F4), carried by a Version done-check, with the ignore list not widened
@answer n=3 entity=ci-integration#1.3 v=yes mode=stale
why: scripts/agent-run.ps1 is deleted, so the pwsh run --unit path on windows no longer exists
@answer n=4 entity=ci-integration#1.4 v=no
why: the integration stage still runs agent-run run --integration on scripts/agent-run.sh, the only harness left, and the session moves no count or OS claim here
@answer n=5 entity=ci-integration#1.5 v=yes mode=stale
why: E2E now runs on ubuntu-latest alone, so the all-3-OSes count is gone
@answer n=6 entity=ci-integration#1.6 v=yes mode=contradict
why: the windows-2025 ConPTY vendor verification step and the conpty_sideload binary in the coverage run are removed with the Windows leg
@answer n=7 entity=ci-integration#1.7 v=yes mode=contradict
why: windows-mutants.yml is deleted, so the Windows mutation leg of the epoch audit no longer exists
@answer n=8 entity=ci-integration#1.8 v=no
why: the perf job keeps its matrix.os form with one leg, its name and steps are unchanged by the session
@answer n=9 entity=ci-integration#1.9 v=no
why: the MSRV job is ubuntu-only, and its toolchain, check and unit steps are untouched by the session
@answer n=10 entity=ci-integration#1.10 v=no
why: fuzz-replay is an ubuntu job whose steps and the nightly fuzz job are unchanged by the session
@answer n=11 entity=ci-integration#1.11 v=no
why: the release job keeps its matrix.os form with one leg, and release-check.sh behaves as before
@answer n=12 entity=ci-integration#1.12 v=yes mode=stale
why: the workflow count moves from three to two with windows-mutants.yml deleted, so the three-workflow list and the three-low zizmor concurrency reading move to two
@answer n=13 entity=ci-integration#1.13 v=yes mode=stale
why: the ci kill line drops from 27 binaries to 26 because binary(conpty_sideload) leaves .config/nextest.toml line 24
@answer n=14 entity=ci-integration#1.14 v=yes mode=contradict
why: scripts/junit-os-sets.py is retired, so the per-OS JUnit reader it describes no longer exists
@answer n=15 entity=ci-integration#1.15 v=yes mode=stale
why: the per-OS JUnit reading is a three-OS measurement from junit-os artifacts the session retires, so it no longer reads the current build
