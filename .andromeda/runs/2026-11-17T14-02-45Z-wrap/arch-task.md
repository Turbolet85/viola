@task_prose part=session
The supply-chain gate reads green again: the cargo-deny advisory ignore list widens from three ids to four, adding RUSTSEC-2026-0543 (chrono unmaintained, no safe upgrade, brought in by rmcp and schemars through viola-mcp beside viola's and viola-state's own chrono use).
The new ignore stands on a second, widened ground: an unmaintained upstream with no safe upgrade, admitted on the founder's word, re-read at each epoch boundary; the list's comment now states both grounds (the first is the rmcp advisory response). Bans, licences, sources and the dependency graph are unchanged.
The fuzz workspace's advisory audit now reports four advisory-not-detected warnings, up from three.
In viola-e2e's in-test harness boot, the tied stamping viola verify now takes SIGTERM when its test thread ends (it was SIGKILL), so verify's own stop ends its children and removes its own probe dirs; the tied supervisor still takes SIGKILL.
Every in-test boot in viola-e2e's lifecycle and CLI tests is now tied, held by a new census over viola-e2e's tests; a new kill witness proves a killed test leaves no stamping verify, no probe dir and no stamp written.
The shared source-scan masker's own cases became one labelled-table test so viola-e2e, which has no rstest, can take it; no dependency added, CI green on all seven checks.
@task_file n=1 path=deny.toml
what: the cargo-deny dependency policy — its advisory ignore list, each id with its reason, and the comment stating the grounds an ignore may stand on
@task_file n=2 path=crates/viola-e2e/src/harness/boot.rs
what: viola-e2e's in-test harness boot — the booted wrapper session with its supervisor and stamping viola verify, tied to the test thread by a parent-death signal
@task_file n=3 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: viola-e2e's harness lifecycle tests, booting sessions through the in-test boot, now every boot tied
@task_file n=4 path=crates/viola-e2e/tests/cli.rs
what: viola-e2e's CLI end-to-end tests driving a booted session, now every boot tied
@task_file n=5 path=crates/viola-e2e/tests/harness_tied.rs
what: viola-e2e's tied-boot kill witnesses — a killed test leaves no harness session and no stamping verify — and the census that finds no untied in-test boot
@task_file n=6 path=tests/support/source_scan.rs
what: the one shared source-scan masker the root census tests (bounded readers, error bodies, OS-gated tests, killed-test cleanup) and now viola-e2e's tied census take
@task_item n=1 part=touches
The cargo-deny dependency policy rows: the advisory ignore list now holds exactly four ids (RUSTSEC-2026-0457, RUSTSEC-2026-0461, RUSTSEC-2026-0462, RUSTSEC-2026-0543); any row naming exactly three ids, or the rmcp advisories alone, is stale.
@task_item n=2 part=touches
Any row ruling that an advisory ignore stands only on the rmcp/Critical-CVE response ground (never widen an ignore): a second ground, an unmaintained upstream with no safe upgrade, now admits an ignore.
@task_item n=3 part=touches
The fuzz lockfile's separate advisory audit and the weekly advisory audit: the fuzz audit's advisory-not-detected warnings moved from three to four; a row holding that count is stale.
@task_item n=4 part=touches
The chrono timestamp decisions: chrono now carries an unmaintained advisory, ignored with its reason rather than replaced (alternatives jiff and time not taken).
@task_item n=5 part=touches
The rmcp MCP server decisions and viola-mcp's dependency rows, indirectly: rmcp 3.4.1 and schemars 1.2.2 bring chrono into viola-mcp, which is what the new ignore answers.
@task_item n=6 part=touches
The CI supply-chain job and the gate-steps-fail-closed rule: the supply-chain check, red since the advisory appeared, reads green; cargo-deny re-read at its CI pin 0.20.2.
@task_item n=7 part=touches
The every-dependency-ban-proven rule: the ban probes re-read 14 of 14 banned with a clean control after the policy change.
@task_item n=8 part=touches
Rows on how an in-test harness boot ties its children to the test thread by a parent-death kill: the stamping verify now takes SIGTERM, the supervisor SIGKILL; a row saying both take a kill is stale.
@task_item n=9 part=touches
The verify probe-dir rows (removed by drop guard, verify removes only its own dirs): the tied stamping verify now ends by TERM, the way its own guard removes its run dirs; SIGKILL remains the way that leaves them for hand removal.
@task_item n=10 part=touches
The verify-sole-stamps-writer and stamps rows, indirectly: the new kill witness reads the stamps file absent to prove a stopped verify wrote no stamp.
@task_item n=11 part=touches
The test-evidence-survives-kill, test-homes-cleaned-by-owner and test process hygiene rows: a killed viola-e2e test now leaves no stamping verify process, probe dir or harness session behind.
@task_item n=12 part=touches
The name HARNESS_TIED_TEST_CHILD: the tied-boot test child marker is now also read by a stamped victim entry that runs without it.
@task_item n=13 part=touches
The viola-e2e-is-test-only and workspace dependency pin rows: viola-e2e still carries no dev-dependencies; the shared masker was reshaped instead of adding rstest.
@task_item n=14 part=touches
The root census rules over bounded readers, error bodies, OS-gated tests and killed-test cleanup, indirectly: the one shared masker they take now runs its cases as one labelled-table test (per-case test count down, cases unchanged).
@task_fact n=1 slug=build-system
fact: An advisory ignore may stand on one of two grounds — the rmcp advisory response, or an unmaintained upstream with no safe upgrade admitted on the founder's word — each ignore carrying its reason and re-read at each epoch boundary.
@task_fact n=2 slug=build-system
fact: RUSTSEC-2026-0543 (chrono unmaintained, no patched version) is ignored on the unmaintained-upstream ground, brought in by rmcp and schemars through viola-mcp beside viola's and viola-state's own chrono use.
@task_fact n=3 slug=occupied-resources
fact: An in-test harness boot ties its stamping viola verify to the test thread by parent-death SIGTERM, so verify's own stop removes its probe dirs and writes no stamp, and ties its supervisor by SIGKILL.
@task_fact n=4 slug=occupied-resources
fact: Every in-test harness boot in viola-e2e passes tied, held by a census over viola-e2e's tests that finds no untied boot.
@task_fact n=5 slug=conventions
fact: A shared test helper taken by path carries its own test cases into every taker, so its cases use no dev-dependency a taker lacks.
@task_item n=1 part=untouched
Product runtime behaviour: run, revive, send, wait, answer, list, verify's own steps and exit codes, the hook and MCP servers are unchanged.
@task_item n=2 part=untouched
Wire and on-disk contracts: channel frames and methods, CLI and MCP output documents, event kinds and data, snapshots, stamps envelope shape, config and diagnostics formats.
@task_item n=3 part=untouched
Root (non-e2e) tests' tie and wrapper freeze: ROOT_TIED_TEST_CHILD, the root SIGSTOP guard and root test waits are unchanged.
@task_item n=4 part=untouched
The nextest CI kill line for verify-driven tests: the finding that it omits some viola-e2e binaries is placed at route-resolve, not changed by this session.
@task_item n=5 part=untouched
Toolchain, MSRV, CI tool versions other than the cargo-deny re-read, action pinning, release, perf, fuzz replay and mutation testing.
@task_item n=6 part=untouched
Crate layout and dependency direction: no crate added or removed, no Cargo manifest or lockfile delta, no new binary, command or environment name.
