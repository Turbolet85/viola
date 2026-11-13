# Fan-out results — 2026-11-13-rows-proposal-rule-work

architecture: add 4 · set 3 · retire 0 · none 0 · ops-architecture.md (re-spawn propose-r1 after [EVERY_TASK_FACT]; the first file kept as .raw-ops-architecture.md)
security-plan: add 1 · set 1 · retire 0 · none 0 · ops-security-plan.md
design-system: add 0 · set 2 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 2 · set 0 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 3 · set 8 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 1 · set 2 · retire 0 · none 1 · ops-obs-plan.md
a11y-plan: add 2 · set 0 · retire 0 · none 0 · ops-a11y-plan.md

One home per fact (check 2): the hint wording is design-system's (`contract/refusal-hints/*`) and layout-templates' per-verb output (`contract/viola-answer-output/*`); the payload-type redaction is obs-plan's `decision/payload-types-derive-redact`; the trusted-state link refusal is security-plan's `decision/strict-modes-check`; the private append open is security-plan's `decision/logs-owner-only` and obs-plan's `decision/private-diagnostics-files`; the record email is security-plan's `decision/fixture-scrub-and-refusal`. The same fact offered in another master is rejected there.

ops-architecture.md
decision/mcp-dependencies · reject — check 7: not_master type 6 — the added veil edge retells the crate manifest's dependency keys; the row stays true, nothing follows
decision/recorded-fixtures-scrubbed · reject — check 2: the record email's home is security-plan decision/fixture-scrub-and-refusal, which states it already; this set lengthens a row still true
contract/human-stderr/refusal · reject — check 4: the row is the fixed `unable:` / `hint:` pair writer, which always prints its hint; the report's every-refusal-but-unknown fact is the mirror `unable  name  reason` line, another writer — the set would make a true row false
decision/payload-types-derive-redact · reject — check 2: the fact's home is obs-plan decision/payload-types-derive-redact (inputs#I2 item 1)
decision/strict-modes-refuses-symlink · reject — check 2: security-plan decision/strict-modes-check states the link refusal
decision/private-append-no-follow · reject — check 2: security-plan decision/logs-owner-only and obs-plan decision/private-diagnostics-files hold it
contract/human-hint-lines/unverified-cli · reject — check 2: the hint wording's home is design-system contract/refusal-hints/unverified-cli

ops-security-plan.md
decision/logs-owner-only · apply — check 1: Accurate this-chunk addition (routine); report F6: the append open refuses a link and narrows through the handle for the event log, heartbeat and diagnostics files
decision/content-payload-types-redact-debug · reject — check 2: the fact's home is obs-plan decision/payload-types-derive-redact (inputs#I2 item 1)

ops-design-system.md
contract/refusal-hints/refusal-line · apply — check 1: Accurate this-chunk addition (routine); D-design-cli-wording, report F1: send and answer share each hint
contract/exit-21-json-cause/unwrapped-name · apply — check 4: the row's symlink clause is false since the strict-modes check now refuses a symlinked instance directory (report F5)

ops-layout-templates.md
contract/viola-answer-output/not-delivered-hints · apply — check 1: Accurate this-chunk addition (routine); D-design-cli-wording, report F1, beside the standing contract/viola-answer-output/unknown-dialog
contract/send-answer-hints/unverified-cli · reject — check 2: the hint wording's home is design-system contract/refusal-hints/unverified-cli; a new boundary for one string repeats it

ops-test-plan.md
decision/snapshot-replay-from-log · reject — check 4: the set drops the budget leg's e5_the_replay_reads_what_the_wrapper_wrote, which still stands until «Budget gate removed»; main's ops-main-test-plan.md names both
decision/one-shared-test-helper · reject — check 7: not_master type 5 — the set lengthens the helper list, an inventory the test support module holds; the rule («no root test file defines its own copy») stays true, nothing follows
decision/pty-tested-on-real-openpty · apply — check 5: the plan's expected amendment, report F7
decision/hygiene-reports-class-only · reject — check 7: not_master type 5 — the set lengthens the class list; its because names the row false (a closed list of three classes, email now a fourth), so main writes the shorter set in ops-main-test-plan.md
decision/driver-hints-never-name-release · reject — check 7: not_master type 5 — the set lengthens a list the hints table holds; the row, scoped to its two reasons, stays true, nothing follows
decision/strict-modes-two-rules · reject — check 2: security-plan decision/strict-modes-check states the link refusal; this set also drops the row's «content-bearing paths are held to the read rule»
decision/diagnostics-files-owner-only · reject — check 2: security-plan decision/logs-owner-only and obs-plan decision/private-diagnostics-files hold the no-follow open
decision/hook-payloads-recorded-by-verify · reject — check 2: the record email's home is security-plan decision/fixture-scrub-and-refusal; this set lengthens a row still true
decision/payload-types-derive-redact · reject — check 2: the fact's home is obs-plan decision/payload-types-derive-redact (inputs#I2 item 1)
contract/viola-answer-human-mode/unable · reject — check 2: the hint wording's home is design-system and layout-templates; test-plan's contract/cli-human-lines/unable stands
decision/control-isolates-its-own-case · reject — check 3: a process learning of this chunk's controls, no project decision; carried to curation through the report's Decisions & corrections

ops-obs-plan.md
decision/payload-types-derive-redact · reject — check 1: the operator's wrap directive (inputs#I2 item 1) brings the rule cell to the fact; main's ops-main-obs-plan.md sets the rule, not an exception
decision/veil-redact-payload-types · apply — check 1: Accurate this-chunk addition (routine); its rule says every payload type derives Redact, false for the three content-free MCP inputs without its exception
decision/strict-modes-refuses-symlink · reject — check 2: security-plan decision/strict-modes-check states the link refusal

ops-a11y-plan.md
contract/cli-refusal-hint/answer-not-delivered · reject — check 2: the hint wording's home is design-system contract/refusal-hints/* and layout-templates contract/viola-answer-output/*; a11y-plan's decision/refusal-ends-with-hint carries the a11y rule
contract/cli-refusal-hint/unverified-cli · reject — check 2: the hint wording's home is design-system contract/refusal-hints/unverified-cli

ops-main-obs-plan.md
decision/payload-types-derive-redact · apply — check 1: the operator's recorded direction settles it, inputs#I2 item 1 (relayed verbatim by the operator's wrap directive, operator, 2026-11-13)

ops-main-test-plan.md
decision/snapshot-replay-from-log · apply — check 5: the plan's expected amendment, the guard gains e5_the_replay_reads_the_link_the_wrapper_wrote, the budget leg's case kept
decision/hygiene-reports-class-only · apply — check 7: the shorter set for a not_master correction (the closed class list false since the email class)

ops-main-security-plan.md
contract/instance-unreachable-detail/unwrapped-name · apply — cascade step 2 (sweep m-symlink-unwrap): the symlink clause is false since the strict-modes check refuses a symlinked instance directory first (report F5)

ops-main-a11y-plan.md
contract/instance-unreachable-detail/unwrapped-name · apply — cascade step 2 (sweep m-symlink-unwrap): the symlink clause is false since the strict-modes check refuses a symlinked instance directory first (report F5)

Expected amendments not carried by an operation (check 5): a11y-plan decision/refusal-ends-with-hint, security-plan decision/fixture-scrub-and-refusal, decision/cli-names-through-name-check, decision/strict-modes-check, test-plan decision/fixture-hygiene-walk, decision/fd-test-reads-live-child-table, decision/no-fd-count-assertion, decision/fixtures-walked-for-hygiene, obs-plan decision/private-diagnostics-files — each row's text already states the rule this chunk built (the landing kept the rule, `fates: divergences:rule`); the divergence is the landing's `verified` meta (`v: stale`), which no row operation writes. Architecture's crate-dependency direction: decision/mcp-dependencies carries viola-mcp's veil edge; no architecture row lists viola-agent-claude's third-party dependencies (decision/agent-claude-pure names viola-state only).
Spec claims disproved (check 6): obs-plan decision/payload-types-derive-redact — disposed by ops-main-obs-plan.md; plan step 9 — no master site (a plan, read-only here).
