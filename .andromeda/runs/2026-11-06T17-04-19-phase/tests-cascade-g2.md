@carried n=1 label=test-plan/test-strategy fact=fct-7osf2h3d
why: the chunk's re-read of the test gate green must run through nextest, because a plain cargo test reads three known-cause reds.
quote: nextest is the runner; a plain `cargo test` run (`cargo test --bin viola`) is not supported, and under it the root-bin cases `run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes`, `obs::tests::log_config_rejection_writes_one_warn_line` and `case_2_sigterm` are known-cause reds, recorded with no owner.
@carried n=2 label=test-plan/test-strategy fact=fct-hlf5klew
why: item 1 re-measures at HEAD that no manifest holds assert_cmd, so the masters-agree check decides between the tool and the text, and this level's wording follows.
quote: using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.
@carried n=3 label=test-plan/test-strategy fact=fct-pe3lhjgn
why: the E2E layer drives the outer PTY through portable-pty, so every E2E path must still read green on the release that replaces =0.8.1.
quote: They run against harness-booted `viola run … -- viola-fake-agent`, using the outer PTY (portable-pty) and the rmcp client.
@carried n=3 run=2 label=test-plan fact=fct-q3ukwekc
quote: against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
@carried n=4 label=test-plan/test-strategy fact=fct-akm552hs
why: the fuzz corpus replay is one of CI's seven jobs that must read green at this chunk's wrap.
quote: cargo-fuzz 0.13.2 targets run as a time-boxed ubuntu nightly job, plus a corpus replay gate
@carried n=5 label=test-plan/test-strategy fact=fct-zx23mmpw
why: the perf job is one of the seven jobs re-read green, and its one ubuntu-latest leg is the target that item 17's per-OS perf wording must follow.
quote: the `perf` CI job, on its one `ubuntu-latest` leg, are built; §10)
@carried n=6 label=test-plan/test-strategy fact=fct-zinazvq2
why: the portable-pty move under crates/viola-pty must keep the seam that the mockall faults fake, and keep the no-EOF child's exit read from the handle.
quote: a no-EOF child), plus mockall 0.15.0 faults on the `viola-pty` seam.
@carried n=6 run=2 label=test-plan fact=fct-2yi4u2mc
quote: faults come from process and file operations and from mockall faults on the `viola-pty` seam.
@carried n=7 label=test-plan/test-strategy fact=fct-cz5jn5fs
why: freight item 8 is this route entry's reading of the survivor, so the survivor must be recorded against the untestable zone and matrix v1-44 without the mutation gate being run here.
quote: A known survivor stands against the `missed` rule: a mutant replacing a client seam's `peer_admitted` read (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) with an always-admit survives every committed test by construction (§1 `viola-channel`, **As landed (the euid wiring, untestable)**), and the mutation audit reads that survivor at the route entry "Version done-check".
@carried n=8 label=test-plan/test-strategy fact=fct-hckuzazj
why: item 1 re-measures that no tests/cmd exists, so the masters-agree check decides between the tool and the text, and this directory line and the verify-mode line follow that decision.
quote: **trycmd cases:** `tests/cmd/*.toml`.
@carried n=8 run=2 label=test-plan fact=fct-gcer5i4i
quote: trycmd only in verify mode.
@carried n=9 label=test-plan/test-strategy fact=fct-df3r2j7c
why: v1-53's claim reads this census and the ubuntu-latest JUnit alone, so the census must run in the nextest run whose JUnit the claim reads, and that live run must not be replaced by a fake-agent reading.
quote: `tests/contract_os_gated_tests.rs` is a root `contract_` binary of the sync suites, auto-discovered with no `[[test]]` table; the E2E layer's regex selects it, so it runs under `run --e2e`, in suite `nextest-e2e`.
@carried n=9 run=2 label=test-plan fact=fct-elca5edh
quote: Rust suites give an exit code plus nextest JUnit (`target/nextest/ci/junit.xml`).
@carried n=9 run=3 label=test-plan fact=fct-egysilpo
quote: NEVER treat a green fake-agent run as proof of real-CLI behaviour.
@carried n=10 label=test-plan/test-strategy fact=fct-gldxhydz
why: item 17's remnant removal must leave every remaining OS gate a GATES row with no windows or macos predicate, keeping the census count the chunk re-reads consistent.
quote: §9 Matrix builds (one leg, `ubuntu-latest`); §6 Contract suite (root `contract_os_gated_tests.rs`: every OS-gated test function and gated file under `tests/` and `crates/*/tests/` a `GATES` row with its reason, an unnamed or stale gate failing the run, and a gate whose predicate names `windows` or `target_os = "macos"` a fault even when a row names it)
@carried n=11 label=test-plan/test-strategy fact=fct-peucakqx
why: RUSTSEC-2026-0441 must clear through the portable-pty move inside these stages with no ignore-list widening, and every stage must be re-read green at the wrap.
quote: §9 Lint stage (sync-crate `cargo check`, `scripts/orphans-check.sh`) + Supply-chain stage (cargo deny incl. the `fuzz/Cargo.lock` audit, sole-root tokio ban, ban probes, zizmor) + Release build stage (`scripts/release-check.sh`) + `nightly.yml` weekly advisories (root and fuzz lockfiles)
@carried n=11 run=2 label=test-plan/test-strategy fact=fct-hckuzazj
quote: It is never a root of the tokio ban: the ban runs `deny-sync.toml` once per crate in `scripts/sync-crates.txt`, each as the sole root
@carried n=12 label=test-plan/test-strategy fact=fct-34amt7nu
why: item 7 brings the founder the socket-directory pair's wording, and these named refusals are what that ruling governs.
quote: root `security_negatives_socket_dir.rs`, Unix: a socket directory at 0755 or a symlink to 0700 refusing a start, the eight CLI frame verbs and the hook, each left as found;
@carried n=13 label=test-plan/test-strategy fact=fct-mqdaf5bf
why: item 7 asks the founder's word on the board's read rule on every OS, and this test witnesses that rule.
quote: root `cli_list.rs` `list_refuses_a_home_another_user_may_read_with_and_without_an_instance`: a home another user may read refusing `viola list`, with and without an instance)
@carried n=14 label=test-plan/test-strategy fact=fct-svqwosl6
why: item 15 measures on Linux the channel-handle no-leak property that Scenario E2 witnesses, so the child-spawn trace names the witness that the security-plan rewording must agree with.
quote: §5 process boundary; §6 Scenario E2
@carried n=15 label=test-plan/test-strategy fact=fct-s3ozhmcp
why: v1-13's and v1-36's dispositions read their CLI halves as built through Scenario E3, and the founder rules on the link clauses no chunk can prove.
quote: §6 Scenarios 2, E3 + Cross-surface parity
@binding n=1
**supply-chain trace ↔ Cargo.toml's portable-pty pin and the section 9 build-failure lines**: the portable-pty move must clear RUSTSEC-2026-0441 in cargo deny (root and fuzz lockfiles) and CI's supply-chain job, with RUSTSEC-2017-0008 still the only ignore, and the deny-probes, sole-root tokio ban, channel-tokio-free, zizmor and release-check lines must still end green (test-plan/test-strategy fct-peucakqx) (test-plan fct-hlx2zkbt) (test-plan fct-ys3szg34) (test-plan fct-thnlptgc).
@binding n=2
**viola-pty seam faults and E2E outer PTY ↔ the section 1 viola-pty entity and section 5 Module ↔ PTY**: the release that replaces =0.8.1 must keep the six-method seam, keep exit read from the handle and never from EOF, and keep the resize-then-key spawns green, and the plan's `=0.8.1` text must follow the new pin (test-plan/test-strategy fct-zinazvq2) (test-plan/test-strategy fct-pe3lhjgn) (test-plan fct-njspb5nv) (test-plan fct-nvtf2boy) (test-plan fct-sbml5vi6).
@binding n=3
**trycmd directory line ↔ section 5 human-mode layouts, section 7 expected output and the assert_cmd wording**: one masters-agree decision, either tool or text, must govern every trycmd and assert_cmd statement together, so that no master keeps naming `tests/cmd/*.toml` or a manifest-less tool once the decision lands (test-plan/test-strategy fct-hckuzazj) (test-plan/test-strategy fct-hlf5klew) (test-plan fct-klrvzf5j) (test-plan fct-puo2hazj) (test-plan fct-gcer5i4i).
@binding n=4
**multi-os-compat census ↔ item 17's remnant removal ↔ v1-53's claim**: removing the remnants must leave the census with no windows or macos predicate and a count that agrees with its GATES rows, and v1-53's claim must read the census and the test job's `junit-${{ matrix.os }}` JUnit, fetched with gh run view and gh run download (test-plan/test-strategy fct-gldxhydz) (test-plan/test-strategy fct-df3r2j7c) (test-plan fct-elca5edh) (test-plan fct-kprehror).
@binding n=5
**mutation survivor ↔ section 1 untestable zone ↔ matrix v1-44**: the survivor must be read as the cross-OS-user untestable zone's by-construction survivor, with no seam added and no mutation gate run by this chunk (test-plan/test-strategy fct-cz5jn5fs) (test-plan fct-upolcsa3) (test-plan fct-lkgqv65p).
@binding n=6
**child-spawn trace ↔ section 1 IPC trigger ↔ Scenario E2**: the Linux measurement of `/proc/<child pid>/fd` under the fake agent must show what the IPC trigger requires ("no inherited channel handles in the child") and what E2 verifies (the child holds no viola socket), and only then may security-plan fct-p7rzex2i be reworded (test-plan/test-strategy fct-svqwosl6) (test-plan fct-3znwz3ga) (test-plan fct-tenptmlu).
@binding n=7
**perf job's one ubuntu-latest leg ↔ section 10 performance rows**: item 17's follow-through must bring section 10's "per OS" session-end row and its Windows dev-host reading in line with the one Linux leg (test-plan/test-strategy fct-zx23mmpw) (test-plan fct-yoazurel) (test-plan fct-vxnii5yh).
@binding n=8
**unit runner rule ↔ the coverage test job**: the re-read of the test gate must be the test job's one instrumented nextest run, ending in `viola-harness gate`, never a plain cargo test (test-plan/test-strategy fct-7osf2h3d) (test-plan fct-onifqrhg).
@binding n=9
**socket-directory refusals and the board's read rule ↔ item 7's founder word**: the wording the founder confirms for the socket-directory pair and for the board's read rule must govern these named tests' plan text, and his word must not be presumed (test-plan/test-strategy fct-34amt7nu) (test-plan/test-strategy fct-mqdaf5bf) (test-plan fct-w5xcngvx).
