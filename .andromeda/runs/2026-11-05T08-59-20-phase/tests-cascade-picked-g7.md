@account n=1 entity=ci-integration#1.1 v=carried
why: the lint job is one of the ci.yml jobs whose Windows and macOS legs leave, and its dev-host Windows- and macOS-target clippy runs lose their purpose
@account n=2 entity=ci-integration#1.2 v=carried
why: deny.toml's Windows dependency entries leave, and cargo deny and the deny probes must still end their required lines over the trimmed manifests
@account n=3 entity=ci-integration#1.3 v=carried
why: the unit row names the pwsh agent-run.ps1 shim on windows, the harness step the scope leaves to P3 and P4 to decide
@account n=4 entity=ci-integration#1.5 v=carried
why: the E2E row runs on all 3 OSes and the chunk shrinks the matrix to ubuntu-latest
@account n=5 entity=ci-integration#1.6 v=carried
why: the per-OS test job's ConPTY vendor verification and the conpty_sideload binary leave with vendor/conpty and the Windows job, and the coverage gates must hold on the ubuntu job
@account n=6 entity=ci-integration#1.7 v=carried
why: windows-mutants.yml, the audit's Windows mutation leg it describes, is removed whole
@account n=7 entity=ci-integration#1.8 v=carried
why: the per-OS perf job is one of the ci.yml jobs whose Windows and macOS legs leave
@account n=8 entity=ci-integration#1.9 v=carried
why: the msrv job is the job the chunk must turn green by fixing the two host-disk harness cases
@account n=9 entity=ci-integration#1.10 v=not_in_scope
why: the ubuntu fuzz-replay job and nightly.yml's ubuntu fuzz job are units that only keep holding, a re-check every change owes
@account n=10 entity=ci-integration#1.11 v=carried
why: the per-OS release job is one of the ci.yml jobs whose Windows and macOS legs leave, and its release-check probes must still end their lines
@account n=11 entity=ci-integration#1.12 v=carried
why: its workflow list, windows-mutants.yml's pins and the contract test asserting them, and the per-workflow concurrency count all lose windows-mutants.yml
@account n=12 entity=ci-integration#1.13 v=carried
why: the ci kill line names binary(conpty_sideload), so removing that test file moves the 27-binary count
@account n=13 entity=ci-integration#1.14 v=carried
why: the per-OS JUnit reader downloads the windows and macos JUnit artifacts the chunk's removed jobs no longer produce
@account n=14 entity=ci-integration#1.15 v=carried
why: the per-OS JUnit reading records the three-OS case sets and the macOS carve-out that the chunk's census of OS-gated tests re-points or retires
@account n=15 entity=quality-gates-coverage-targets#1.1 v=carried
why: the coverage-merge guarantee rests on the fake agent's per-command process group, a cfg(unix) arm the chunk makes unconditional keeping its Linux behaviour
@account n=16 entity=quality-gates-coverage-targets#2.0 v=carried
why: its host-excluded cfg(windows) mutants, the windows-mutants.yml dispatch and the macOS-compiled-out harness tests are all removed or ungated by the chunk
@account n=17 entity=quality-gates-coverage-targets#3.1 v=carried
why: the timed hook path holds check_endpoint_dir and peer_admitted as real checks on Unix only, arms the chunk ungates while the perf gate stays SPINE_DEADLINE on the ubuntu perf job
