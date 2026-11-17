@task_prose part=session
The supply-chain gate reads green again: the cargo-deny advisory ignore list gains RUSTSEC-2026-0543 (chrono unmaintained, no patched version, brought in by rmcp and schemars through viola-mcp), admitted on a second ground, an unmaintained upstream with no safe upgrade, on the founder's word, re-read at each epoch boundary; the list goes from three ids to four and the comment above it states both grounds.
Every booted viola-e2e test process now ends with its test: the in-test boot's tied stamping viola verify takes SIGTERM at its test thread's end, so verify's own stop ends its children and removes its own run dirs, while the supervisor keeps SIGKILL.
The in-test boots of viola-e2e's harness lifecycle and CLI tests are now tied, held by a new census over viola-e2e's test sources that finds no untied boot (planted positive first).
A new kill witness proves a killed test leaves no stamping verify: process and dir reads alone could not separate a tied verify from an untied one (an orphaned untied verify finishes inside the wait bound and removes its dirs), so the witness also reads the stamp's absence and waits until verify holds a child inside a viola-verify run dir.
The one shared source-scan masker, taken by path, now also serves viola-e2e; its 14 cases became a labelled table in one plain test instead of rstest cases, since viola-e2e takes no rstest; the test count moved 3510 to 3462 by that alone (cases unchanged).
CI on the pre-CI commit reads green, 7 of 7 checks, supply-chain among them; no product symbol, command, status shape, log format or dependency changed.
@task_file n=1 path=deny.toml
what: the cargo-deny supply-chain gate's policy — the advisory ignore list with each entry's reason, the bans, licences, sources and graph settings
@task_file n=2 path=crates/viola-e2e/src/harness/boot.rs
what: viola-e2e's in-test boot of a harness session — makes the home, runs the stamping viola verify and starts the supervisor, tying each to the test thread by a parent-death signal
@task_file n=3 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: viola-e2e's harness lifecycle tests — boot, readiness timeout and supervisor stop of a booted session, each boot now tied
@task_file n=4 path=crates/viola-e2e/tests/cli.rs
what: viola-e2e's harness CLI tests — status, logs and cleanup driving a booted session, each boot now tied
@task_file n=5 path=crates/viola-e2e/tests/harness_tied.rs
what: viola-e2e's tied-children kill witnesses — a killed test leaves no harness session and no stamping verify, and the census that finds no untied in-test boot
@task_file n=6 path=tests/support/source_scan.rs
what: the one shared source-scan masker taken by path by the root census binaries (bounded readers, error bodies, OS-gated tests, killed-test cross) and now by viola-e2e's tied-children tests, with its own masking cases
@task_item n=1 part=touches
decision/no-advisory-ignored (section 1): its exception holds only the rmcp ground; it now takes the second ground, an unmaintained upstream with no patched version and no safe upgrade, admitted on the founder's word, its reason on the entry and re-read at each epoch boundary (RUSTSEC-2026-0543, chrono).
@task_item n=2 part=touches
decision/no-ignored-advisories (section 9): the same second ground joins its exception; any cell naming exactly three ignored ids now holds four (RUSTSEC-2026-0457, RUSTSEC-2026-0461, RUSTSEC-2026-0462, RUSTSEC-2026-0543).
@task_item n=3 part=touches
Any row whose cell holds the advisory ignore list's ids or its count (three) is stale: the list is now four ids.
@task_item n=4 part=touches
Indirect, through the gate policy: decision/cargo-deny-gate, decision/cargo-deny-supply-chain-gate, decision/supply-chain-ci-gates and decision/supply-chain-fail-closed rule over the supply-chain gate whose advisory list widened; the gate stays fail-closed and reads green with the widened list.
@task_item n=5 part=touches
decision/weekly-advisories-run and decision/weekly-advisory-scan: the fuzz workspace's advisory audit now reports four advisory-not-detected warnings, not three; a touch only where a row's cell holds that count.
@task_item n=6 part=touches
decision/supervisor-ordinary-child: it says the in-test boot ties the supervisor and its stamping viola verify to the test thread by a parent-death kill; now the stamping verify takes SIGTERM and only the supervisor takes SIGKILL.
@task_item n=7 part=touches
decision/tied-child-spawn-thread: the tied children are still spawned on a thread that outlives them; true of either signal, but its wording may name the kill signal of the stamping verify.
@task_item n=8 part=touches
decision/no-untied-viola-spawn: the root-test rule now has a viola-e2e counterpart, a census that finds no untied in-test boot in viola-e2e's test sources.
@task_item n=9 part=touches
decision/rstest-case-tables: an exception — the shared masker's cases, carried by path into every taker, are a labelled table in one plain test, since viola-e2e takes no rstest and adds no dev-dependency.
@task_item n=10 part=touches
decision/one-shared-test-helper and decision/shared-e2e-helpers: the one masker stays the single copy and gains viola-e2e as a taker; no census defines its own copy.
@task_item n=11 part=touches
decision/bounded-remove-the-guard-witness: a new kill witness with its controls (verify given SIGKILL again, verify untied, census blinded) reads red under each and green as shipped.
@task_item n=12 part=touches
contract/verify-stop/signal (SIGINT or SIGTERM): the tied stamping verify now relies on verify's own SIGTERM stop to end its children and remove its run dirs; a stopped verify writes no stamp.
@task_item n=13 part=touches
decision/stamps-written-only-by-verify, decision/verify-sole-stamp-writer and decision/one-verify-stamp-per-build: the witness reads the stamp file's absence after the kill, relying on an interrupted verify never writing stamps.
@task_item n=14 part=touches
decision/within-7s-wait-bound: every new wait of the witness and census sits under WITHIN.
@task_item n=15 part=touches
decision/killed-test-child-uninstrumented: the new witness spawns a victim test entry that is killed mid-stamp, a killed test child of this kind.
@task_item n=16 part=touches
decision/verify-driven-tests-20s-kill and decision/ci-stamped-home-kill-line: viola-e2e's lifecycle, CLI and tied-children tests drive a stamping verify through the boot; measured slowest cases 20.644 s (the readiness-timeout case, waiting out the 20 s readiness deadline), 5.991 s and 1.014 s, so a 20 s kill line would kill the lifecycle binary's readiness-timeout case; the finding's placement is the route's, not amended by this session.
@task_item n=17 part=touches
decision/stamping-verify-passes-transcript: the stamping verify the boot runs still gets its transcript; only its parent-death signal changed.
@task_fact n=1 slug=integration-test-strategy
fact: No viola-e2e test boots a harness session untied: a census over viola-e2e's test sources finds every in-test boot tied, read red on a planted untied boot first.
@task_fact n=2 slug=test-strategy
fact: A kill witness for a tied stamping verify waits until verify holds a child inside a viola-verify run dir, asserts an owned dir stood at the kill, and reads the stamp's absence besides processes and dirs, since an untied verify orphaned by the kill finishes inside the wait bound and removes its own dirs.
@task_fact n=3 slug=2-test-strategy
fact: A shared test helper taken by path carries its own cases into every taker, so its cases are a labelled table in one plain test, never rstest cases, as the E2E crate takes no rstest.
@task_item n=1 part=untouched
Product CLI, MCP, wrapper channel, hook, event log, snapshot and exit-code contracts: no product symbol changed.
@task_item n=2 part=untouched
The harness log line format and the agent-run status document shape: neither changed.
@task_item n=3 part=untouched
Crate bans, licence and source rules, the tokio bans, the serde_json depth ban, the rmcp feature limit and the ban probes: unchanged, every ban still fires.
@task_item n=4 part=untouched
Coverage floors, mutation gate thresholds and cadence, and performance budgets: no threshold moved.
@task_item n=5 part=untouched
Toolchain, MSRV and tool version pins and dependency rows: no dependency, lockfile or tool version changed.
@task_item n=6 part=untouched
Fake agent command line, receipt, script and recorded fixture rows: the fake agent and fixtures did not change.
@task_item n=7 part=untouched
CI workflow structure, artifacts, permissions, caches and SHA pinning: no workflow changed.
@task_item n=8 part=untouched
Fuzz targets, corpora and the fuzz replay gate: untouched beyond the audit's warning count.
