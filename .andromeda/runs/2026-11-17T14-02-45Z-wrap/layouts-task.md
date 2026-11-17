@task_prose part=session
The supply-chain gate reads green again: the cargo-deny advisory ignore list gains a fourth id, RUSTSEC-2026-0543 (chrono unmaintained, no safe upgrade, brought in through the MCP server's rmcp and schemars), admitted on the founder's unmaintained-upstream ground beside the existing rmcp ground.
Test-only work: every booted end-to-end test's stamping viola verify now ends with its test thread by SIGTERM (it was SIGKILL); the harness supervisor still takes SIGKILL.
Two tests were added: a kill witness proving a killed test leaves no stamping verify, its run dirs or its stamp behind, and a census proving no in-test boot in the e2e crate is untied.
The shared source-scan masker's 14 rstest cases became one labelled-table test, so the reported test count fell by 48 with no case lost.
No command, CLI output, status shape, exit code, refusal or log format changed; no product symbol, dependency or crate moved; nothing retired.
@task_file n=1 path=deny.toml
what: the supply-chain policy: the cargo-deny advisory ignore list, its bans, licences and sources
@task_file n=2 path=crates/viola-e2e/src/harness/boot.rs
what: the test harness's in-test boot of a wrapper session with a fake agent, and its parent-death tie of the supervisor and the stamping viola verify to the test thread
@task_file n=3 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: the harness lifecycle tests: booted sessions started, readied and stopped through the in-test boot
@task_file n=4 path=crates/viola-e2e/tests/cli.rs
what: the e2e CLI tests driving a booted session through status, logs and cleanup
@task_file n=5 path=crates/viola-e2e/tests/harness_tied.rs
what: the tied-harness kill tests: a killed test leaves no harness session and no stamping verify, and the census of untied in-test boots
@task_file n=6 path=tests/support/source_scan.rs
what: the shared source-scan masker the census and contract tests take, with its own masking cases
@task_item n=1 part=touches
Indirect, through the stamping viola verify the booted tests now stop by SIGTERM: rows on how viola verify reports being stopped (the verify stop line and the verify-stopped refusal) now describe the path every booted test relies on; their wording is not changed, but a picker should check they still say a stopped verify ends its children and removes its own run dirs.
@task_item n=2 part=touches
Indirect: a stopped viola verify records no stamp (it ends interrupted before its stamps are updated); any row over verify's record or summary that implies a stamp on a stopped run is now relied on by the new kill witness and must stay true.
@task_item n=3 part=touches
Indirect: the report measured that a viola verify whose stdout pipe has died swallows its result writes and runs to its normal end; rows on verify's exit-0 or exit-1 output and on results going to stdout are untouched in wording but this behaviour sits beside them.
@task_item n=4 part=touches
Indirect: the MCP server surface (viola mcp) brings in the unmaintained chrono through rmcp and schemars, now an admitted advisory ignore; rows on viola mcp and its tools are untouched in behaviour, only their supply chain gains the ground.
@task_item n=1 part=untouched
CLI exit codes, refusal lines, hints and the --json error documents: no code, cause, word or hint changed.
@task_item n=2 part=untouched
The viola list board, the BAY line, column widths, styling, colour depth and token rendering: no output changed.
@task_item n=3 part=untouched
viola send, wait, last, answer, link, unlink, pause and release output, the readback mirror and its vocabulary: unchanged.
@task_item n=4 part=untouched
viola run and viola revive starts, their preflight order, refusals and --json documents: unchanged.
@task_item n=5 part=untouched
viola --help, per-verb usage, viola plugin install output and the hidden verbs: unchanged; no verb added or removed.
@task_item n=6 part=untouched
The web-spa absence and the absence of deep links in the CLI: no surface added.
