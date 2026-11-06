@answer n=1 entity=ci-integration#1.1 v=yes
why: the lint gates (fmt, clippy -D warnings, per-crate tokio-free check, orphans gate, rmcp features) must stay green over the Cargo.toml pin move and the Windows-remnant edits.
@answer n=2 entity=ci-integration#1.2 v=yes
why: RUSTSEC-2026-0441 must clear in the supply-chain job under the portable-pty move, with deny.toml, deny probes, channel-tokio-free and zizmor read green.
@answer n=3 entity=ci-integration#1.3 v=yes
why: the local unit run (agent-run run --unit) is the test gate the chunk re-reads green over the viola-pty dependency move and the edited tests.
@answer n=4 entity=ci-integration#1.4 v=yes
why: the integration run carries the real openpty spawn boundary (item 3) that the portable-pty move must keep green.
@answer n=5 entity=ci-integration#1.5 v=yes
why: the e2e run exercises viola-e2e's harness, which the chunk edits (EXE_SUFFIX joins, session.rs), inside the test gate re-read.
@answer n=6 entity=ci-integration#1.6 v=yes
why: the test job's coverage gate (lines 85, functions 95, regions 80 on ubuntu-latest) and its doctest run are read green, and COVERAGE_FLOORS and COVERAGE_IGNORE sit in the harness the chunk edits.
@answer n=7 entity=ci-integration#1.7 v=no
why: no mutation job or gate runs in CI, and running the mutation gate is not this chunk's work (task not_needed n=14).
@answer n=8 entity=ci-integration#1.8 v=yes
why: the perf job is one of CI's seven jobs the chunk must read green, and its G2, schema-check and gate --require perf steps run over the changed tree.
@answer n=9 entity=ci-integration#1.9 v=yes
why: the MSRV 1.96 job is named among the seven CI jobs that must read green, and a new portable-pty release must still build on 1.96.
@answer n=10 entity=ci-integration#1.10 v=yes
why: the fuzz-replay job is among the seven CI jobs that must read green and runs the corpus replay gate.
@answer n=11 entity=ci-integration#1.11 v=yes
why: the release job's probes (5/5 refused with a clean control) and the "viola only" verdict are read green over the dependency move.
@answer n=12 entity=ci-integration#1.12 v=yes
why: viola-harness gate is the last step of the test job, where the coverage floors and the JUnit census the chunk re-measures are judged.
@answer n=13 entity=ci-integration#1.13 v=yes
why: the 20 s kill bound over the ci profile's 26 binaries binds the test gate the chunk re-reads, since the portable-pty move reruns real openpty spawns inside those binaries.
@answer n=14 entity=ci-integration#1.14 v=yes
why: it records the live supply-chain red on RUSTSEC-2026-0441 owned by this chunk's CARRY, with the portable-pty move as the remedy and the ignore list unwidened.
