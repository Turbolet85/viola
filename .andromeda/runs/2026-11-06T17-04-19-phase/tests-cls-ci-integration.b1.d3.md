@answer n=1 entity=ci-integration#1.1 v=yes
why: the lint gates fmt, clippy -D warnings and per-crate cargo check are re-read green, and the Cargo.toml pin move and Windows remnant removals reach them
@answer n=2 entity=ci-integration#1.2 v=yes
why: the supply-chain job's cargo deny and deny probes are where RUSTSEC-2026-0441 must clear with no ignore-list widening
@answer n=3 entity=ci-integration#1.3 v=yes
why: the unit run is the test gate the chunk re-reads green, and it covers the edited test files and harness code
@answer n=4 entity=ci-integration#1.4 v=yes
why: the integration run is the test gate re-read green and carries tests/cli_output_plain.rs, whose HELP literal must keep holding
@answer n=5 entity=ci-integration#1.5 v=yes
why: the E2E run on ubuntu-latest is the test gate re-read green and covers the viola-e2e session.rs and EXE_SUFFIX edits
@answer n=6 entity=ci-integration#1.6 v=yes
why: the test job's coverage run and its gate are one of CI's seven jobs re-read green and carry the item 7 coverage floors and COVERAGE_IGNORE edits
@answer n=7 entity=ci-integration#1.7 v=no
why: no push or pull-request mutation job or gate exists, and the mutation run is the epoch code audit, which is not this chunk's work
@answer n=8 entity=ci-integration#1.8 v=yes
why: the perf job is one of CI's seven jobs the chunk re-reads green, with its G2 and gate --require perf steps
@answer n=9 entity=ci-integration#1.9 v=yes
why: the MSRV 1.96 job is one of the seven jobs re-read green, and the new portable-pty release must build on the 1.96 floor
@answer n=10 entity=ci-integration#1.10 v=yes
why: the fuzz-replay job of ci.yml is one of the seven jobs re-read green, and a dependency move must keep corpus replay green
@answer n=11 entity=ci-integration#1.11 v=yes
why: the release job's release-check probes and clean control are re-read green, and the pin move and removed Windows code reach the release build
@answer n=12 entity=ci-integration#1.12 v=no standing=S8
why: the workflow conventions (permissions {} at top, contents read per job, SHA-pinned uses, rust-cache only, no concurrency block) that every workflow change owes; the chunk makes no workflow change, and the gate-reading its coverage rule rests on is carried by #1.6
@answer n=13 entity=ci-integration#1.13 v=yes
why: the test re-read runs under the ci nextest profile, whose 20 s kill over 26 binaries decides whether the test gate reads green
@answer n=14 entity=ci-integration#1.14 v=yes
why: it names the recorded supply-chain red RUSTSEC-2026-0441, owned by the CARRY on Version done-check, whose remedy moves portable-pty off =0.8.1 without widening the ignore list
