@task_prose part=session
The supply-chain gate reads green again: the cargo deny advisory ignore list grows from three ids to four, adding RUSTSEC-2026-0543 (chrono unmaintained, no patched version, brought in by rmcp and schemars through viola-mcp beside viola's and viola-state's own chrono use), admitted on the founder's unmaintained-upstream ground and re-read at each epoch boundary.
The comment over the ignore list now states two grounds: the rmcp advisory ground and the unmaintained-upstream ground; bans, licences, sources and the dependency graph are unchanged, and no dependency was added or removed.
The fuzz audit's advisory-not-detected warnings move from three to four; cargo deny, the fuzz audit and the ban probes (14 of 14 banned) all pass, and CI reads green 7 of 7 with supply-chain among its checks.
In the test-only e2e harness, the in-test boot's tied stamping viola verify now takes SIGTERM at its test thread's end (it was SIGKILL), so verify's own stop ends its children and removes its own run dirs; the supervisor keeps SIGKILL.
New integration tests witness that a killed test leaves no stamping verify (read by the absent stamp, since a stopped verify ends Interrupted before writing stamps) and a census reads that every in-test boot of the viola-e2e tests is tied.
No product code, command, status shape, log line, detail code or log format changed; the shared source-scan masker's 14 cases became one labelled test, so the pre-push test count moved 3510 to 3462 with no case lost.
@task_file n=1 path=deny.toml
what: the cargo deny configuration the supply-chain CI gate checks — the advisory ignore list and its grounds, the banned telemetry and logging dependencies, licences and sources
@task_file n=2 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e test harness boot — an in-test booted session whose supervisor and stamping viola verify are tied to the test thread by a parent-death signal
@task_file n=3 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: the harness lifecycle tests — boot, readiness timeout and supervisor stop of a booted session under an integration-test home
@task_file n=4 path=crates/viola-e2e/tests/cli.rs
what: the harness CLI tests — status, logs and cleanup driven over a booted session
@task_file n=5 path=crates/viola-e2e/tests/harness_tied.rs
what: the kill-witness tests that a killed test leaves no harness session and no stamping viola verify, and the census that no in-test boot is untied
@task_file n=6 path=tests/support/source_scan.rs
what: the one shared source-scan masker the census and contract tests take (the OS-gated test census, bounded readers, error bodies, killed-test census)
@task_item n=1 part=touches
Obs dependencies pass cargo deny: the advisory ignore list the deny gate admits now holds four ids, the new one an unmaintained-upstream advisory on chrono, with no safe upgrade.
@task_item n=2 part=touches
Direct chrono dependency for timestamps: chrono now stands under an admitted unmaintained advisory (RUSTSEC-2026-0543), ignored rather than replaced; any row naming chrono as clean or advisory-free is stale.
@task_item n=3 part=touches
cargo deny failure is a failure: the supply-chain gate was red on RUSTSEC-2026-0543 and is green again through a widened ignore, not through a dependency change.
@task_item n=4 part=touches
rmcp stdio MCP server: rmcp (and schemars) bring chrono in through viola-mcp, the path the new advisory ignore names.
@task_item n=5 part=touches
Banned telemetry and logging dependencies: the ban list is unchanged and every ban still fires (14 of 14); a row on bans holds as it stands.
@task_item n=6 part=touches
Millisecond UTC timestamps: the timestamp source is the chrono crate the new advisory concerns; the timestamp format itself is unchanged.
@task_item n=7 part=touches
verify process-exit detail interrupted and verify stop: the tied stamping verify in the test harness is now stopped by SIGTERM, ending Interrupted through its own stop path, which the new kill witness relies on.
@task_item n=8 part=touches
verify is the only stamps writer: the new witness reads the stamps file's absence to tell a stopped verify from one that ran to its end.
@task_item n=9 part=touches
CI verify against the fake agent only: the in-test booted sessions and their stamping verify run against the fake agent; their parent-death signal changed.
@task_item n=10 part=touches
Integration-test homes under one root and homes kept until gates run: the new witness creates and removes its victim's home and verify run dirs under the scratch root.
@task_item n=11 part=touches
OS-gated test census: the shared masker the census takes changed its test shape (one labelled table test for its 14 cases, no rstest), its cases unchanged.
@task_item n=12 part=touches
No sleep-based log waits: the new waits poll process and dir reads under the WITHIN bound rather than sleeping a fixed time.
@task_item n=13 part=touches
Gates run even after failed tests and CI gates fail closed: the supply-chain check is one of the seven CI checks now green.
@task_fact n=1 slug=bootstrap-phases-derive-for-route-setup-project
fact: The direct chrono dependency stands under RUSTSEC-2026-0543 (chrono unmaintained, no patched version), admitted in the cargo deny ignore list on the unmaintained-upstream ground and re-read at each epoch boundary; alternatives jiff and time are noted, none adopted.
@task_item n=1 part=untouched
Product diagnostics line schema, its fields, event enum and corr rules: no log line, field or format changed.
@task_item n=2 part=untouched
Detail codes and exit codes of run, cli, hook and mcp: no code was added, renamed or retired.
@task_item n=3 part=untouched
Span and trace coverage, span naming and span attributes: no span changed.
@task_item n=4 part=untouched
PII scrubbing, redaction, veil and the secret scan: no redaction rule or scan scope changed.
@task_item n=5 part=untouched
Panic capture and error reporting rows, and the absences of exporters, OTel, Sentry and network egress: none moved.
@task_item n=6 part=untouched
Metric coverage, heartbeat liveness and perf budgets of hooks: no budget, threshold or derived metric moved.
@task_item n=7 part=untouched
MCP tool contracts and channel JSON-RPC contracts: no tool, frame or refusal changed.
