# fanout-results — 2026-11-14-status-line-interception-removed

architecture: add 1 · set 12 · retire 27 · none 0 · ops-architecture.md
security-plan: add 3 · set 20 · retire 15 · none 0 · ops-security-plan.md
design-system: add 2 · set 0 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 1 · set 0 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 7 · set 13 · retire 19 · none 0 · ops-test-plan.md
obs-plan: add 6 · set 19 · retire 29 · none 0 · ops-obs-plan.md
a11y-plan: add 3 · set 2 · retire 2 · none 0 · ops-a11y-plan.md

ops-architecture.md
absence/status-line-interception · apply — check 1: Accurate this-chunk addition (routine) — the report's Changes carry it
decision/hooks-fail-open · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/permission-request-question-stays-null · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/snapshots-read-through-replay · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/statusline-wrapped-per-session · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/tolerant-external-payloads · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/probe-capture/statusline-shell-file · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/protocol-version-bump-rule · escalate → Q1 — check 1: Boundary widening the founder answered himself to a P4 fork (rul-bznresq5): his own word on this chunk's `v` fork, relayed verbatim by overseer1, inputs#I3; the widened except goes on the answer card (directive inputs#I10 item 1) · resolved: apply (Q1) — delegate the driving overseer, 2026-11-15, PROVISIONAL (the card's confirm), inputs#I11: «Option 1 — Confirm.» — the exception itself is the founder's own answer, relayed verbatim by overseer1, inputs#I3: «То же исключение (Recommended)»
contract/hook-to-kind-map/hook-statusline · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/no-shell-out · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/statusline-check-before-log · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-deadline · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-hook-exempt · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-only-shell-out · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-source-in-viola-home · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-wrapper-not-a-hook · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/hook-statusline/capture · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/hook-statusline/exit · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/hook-statusline/refused-check · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/hook-statusline/stdout · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/instance-snapshot/data · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/instance-snapshot/statusline-command · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/statusline-source/command · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/claude-flag-literals-in-agent-crate · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/fixture-keeps-own-repository-remote · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/per-session-settings-override · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/probe-capture-arms-inert · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/refused-statusline-check-no-trace · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/settings-override-safe-path-only · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-source-precedence · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/child-argv/settings · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/claude-child-argv/settings · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/instance-settings-json/statusline · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/settings-override/plain-path-only · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/settings-override/status-line · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/statusline-capture-shell-file/verdict · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/board-read-model-shape · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/hook-exits-on-untrusted-instance · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/statusline-source-follows-home · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/hook-statusline-output/stdout · apply — check 1: D-retired, routine — the report's Retired bullet names the subject

ops-security-plan.md
absence/status-line-interception · reject — check 2: the fact's home is architecture absence/status-line-interception
decision/no-shell-out · reject — check 2: the fact's home is architecture decision/no-shell-out
decision/snapshot-holds-no-command · reject — check 2: the fact's home is architecture decision/protocol-version-bump-rule (its except, readers skip what was removed)
decision/hook-unknown-event-silent-exit · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/replayed-snapshot-yields-nothing · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/cli-state-root/home-flag · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/cli-state-root/viola-dir · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/strict-modes-check · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/capture-arms-isolated · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/home-strict-modes-before-read · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/hook-instance-check-first · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/max-frame-shared-cap · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/replayed-snapshot-fallbacks · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/replayed-snapshot-inert · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/code-bearing-files-rewritten-each-start · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/fixture-scrub-and-refusal · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/plugin-files-rewritten-each-start · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/user-content-only-to-diagnostics · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/hook-silent-exit-zero · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/hook-strict-modes-first · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/absolute-viola-path-in-commands · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/fixture-hygiene · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/snapshot-written-only-by-wrapper · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/statusline-command-integrity · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-only-shell-out · reject — check 2: its heir decision/no-shell-out is rejected in this master (the fact's home is architecture decision/no-shell-out); the row is retired with no heir by ops-main-security-plan.md
decision/statusline-sources-untrusted-read · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/no-statusline-command-in-logs · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/single-shell-out · reject — check 2: its heir decision/no-shell-out is rejected in this master (the fact's home is architecture decision/no-shell-out); the row is retired with no heir by ops-main-security-plan.md
decision/statusline-capture-arm · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-command-trust · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-deadline · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-payload-fails-silent · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-shell-out · reject — check 2: its heir decision/no-shell-out is rejected in this master (the fact's home is architecture decision/no-shell-out); the row is retired with no heir by ops-main-security-plan.md
decision/statusline-source-read-only · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-source-selection · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/settings-override-plain-path · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-override-plain-path-only · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-only-from-strict-home · apply — check 1: D-retired, routine — the report's Retired bullet names the subject

ops-design-system.md
absence/status-line-interception · reject — check 2: the fact's home is architecture absence/status-line-interception
contract/hook-verb/unknown-event · reject — check 2: the fact's home is architecture contract/hook-to-kind-map/hook-statusline

ops-layout-templates.md
absence/status-line-interception · reject — check 2: the fact's home is architecture absence/status-line-interception

ops-test-plan.md
contract/viola-hook/exit-code · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/viola-hook/stdout · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/fake-agent-contract-suite · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/fixtures-recorded-by-live-verify · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/bounded-readers-keep-the-cap · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/viola-hook/exit-and-stderr · reject — check 2: the fact's home is architecture contract/hook-to-kind-map/hook-statusline (the set lengthens a row still true with that fact)
decision/hook-deadline-driver · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/generated-plugin/hook-commands · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/hook-instance-check-before-config · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/fake-agent-children-own-process-group · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/fixture-hygiene-walk · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/fake-agent-receipt/kind · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/no-production-data-in-tests · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/plugin-rewritten-on-start · reject — check 7: its heir decision/plugin-dir-rewritten-on-start stands under this row; main sets the row instead (ops-main-test-plan.md, .2.md)
decision/statusline-command-needs-strict-home · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/viola-hook-statusline/passthrough · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-passthrough · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-shell-out-frame-cap · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/instance-settings/statusline-override · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/harness-boot/statusline-echo · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-command-after-strict-modes · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/hook-statusline/passthrough · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-command-never-runs-unsafe · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-command-shell-and-bound · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/statusline-hook/stdout · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/statusline-settings-source/source-file · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/viola-run-start/settings-override · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/recorder-scrub · reject — check 7: its heir decision/recorder-scrubs-paths-and-usernames stands under this row; main sets the row instead (ops-main-test-plan.md, .2.md)
decision/statusline-source-read-only · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/fake-agent-hooks/status-line-command · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-source-written-by-test · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/fake-agent-receipt/statusline · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
absence/status-line-interception · reject — check 2: the fact's home is architecture absence/status-line-interception
decision/no-shell-out · reject — check 2: the fact's home is architecture decision/no-shell-out
decision/plugin-dir-rewritten-on-start · reject — check 7: stands as decision/plugin-rewritten-on-start — main sets that row's name and rule instead (ops-main-test-plan.md, .2.md)
contract/viola-run-start/settings · apply — check 1: Accurate this-chunk addition (routine) — the report's Changes carry it
decision/retired-snapshot-key-read-skipped · reject — check 2: the fact's home is architecture decision/protocol-version-bump-rule (its except, readers skip what was removed)
decision/recorder-scrubs-paths-and-usernames · reject — check 7: stands as decision/recorder-scrub (also decision/hook-payloads-recorded-by-verify) — main sets decision/recorder-scrub's rule and except instead (ops-main-test-plan.md, .2.md)
contract/fake-agent-cli/settings · apply — check 1: Accurate this-chunk addition (routine) — the report's Changes carry it

ops-obs-plan.md
decision/statusline-command-confined · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-command-duration-exit-only · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-command-string-diagnostics-only · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-command-never-in-home-logs · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-command-only-in-diagnostics · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-shell-out-instrumentation · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/hook-stdout/statusline · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/hook-output/statusline-stdout · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-command-never-logged · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-deadline · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-stdout-passthrough · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/no-statusline-settings-in-telemetry · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/rejected-statusline-stdin-runs-nothing · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-settings-steps-unlogged · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-silent-without-instance · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/budget-span-attributes/statusline-shell-out · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/statusline-decision-detail/deadline · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/statusline-decision-detail/malformed-json · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/statusline-decision-detail/oversize-stdin · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/statusline-log-fields/hook-decision · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/statusline-log-fields/hook-invoked · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/statusline-log-fields/parse-rejected · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/statusline-log-fields/statusline-shell · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/statusline-spans/span-chain · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-command-deadline · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/diag-subject/statusline-shell · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/hook-decision-detail/statusline-codes · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
decision/statusline-command-out-of-home-logs · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/replayed-snapshot/live-fields · reject — check 7: its heir contract/replayed-snapshot/endpoint-pid-started-at stands under this row; main sets the row's promise instead (ops-main-obs-plan.md)
decision/hook-stdout-decision-body-only · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/hook-instance-check-before-obs-init · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/hook-latency-budget · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/hook-stdout/capture-arms · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/span-naming · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/instance-check-before-log-init · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/capture-arms-uninstrumented · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/no-endpoint-over-replayed-snapshot · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/hidden-capture-arms-uninstrumented · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/hook-capture-arms-unlogged · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/span-kinds/client · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/span-kinds/server · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/child-spawn-log/subject · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/diag-line-fields/hook-events · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/diag-line-fields/process-exit · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/diag-line-fields/process-start · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/sensitive-content-detail-file-only · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
contract/stdout-by-role/hook · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/log-schema-additive-only · reject — check 7: not_master type 8 — the except retells what the removal took out of diag-line v1, history the route and the schema's diff hold; escalated (Q2), not asked
absence/status-line-interception · reject — check 2: the fact's home is architecture absence/status-line-interception
absence/shell-out · reject — check 2: the fact's home is architecture decision/no-shell-out
contract/hook-unknown-event/statusline · reject — check 2: the fact's home is architecture contract/hook-to-kind-map/hook-statusline
contract/snapshot-compat/statusline-command · reject — check 2: the fact's home is architecture decision/protocol-version-bump-rule (its except, readers skip what was removed)
contract/ledger-stamps-compat/retired-rows · reject — check 2: the fact's home is architecture decision/protocol-version-bump-rule (its except, readers skip what was removed)
contract/replayed-snapshot/endpoint-pid-started-at · reject — check 7: stands as contract/replayed-snapshot/live-fields — main sets that row's promise instead (ops-main-obs-plan.md)

ops-a11y-plan.md
decision/time-bounds-never-on-human · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/time-bounds-fall-on-the-driver · apply — check 1: Accurate this-chunk addition (routine) — the report's Retired / Symbols bullets carry the change, the row's invariant kept
decision/statusline-output-passed-byte-for-byte · apply — check 1: D-retired, routine — the report's Retired bullet names the subject
contract/viola-hook-statusline/exit · reject — check 2: its heir contract/viola-hook-event-word/statusline is rejected (the fact's home is architecture contract/hook-to-kind-map/hook-statusline); the row is retired with no heir by ops-main-a11y-plan.md
decision/no-settings-layer-on-child · reject — check 2: the fact's home is architecture absence/status-line-interception (with architecture decision/global-statusline-untouched)
contract/viola-hook-event-word/statusline · reject — check 2: the fact's home is architecture contract/hook-to-kind-map/hook-statusline
absence/status-line-interception · reject — check 2: the fact's home is architecture absence/status-line-interception

ops-main-architecture.md
name/other/hook-statusline · apply — check 5: raised by main, the sweep's row the proposers left; Retired (4) names the hook statusline arm (routine, D-retired)

ops-main-obs-plan.md
decision/verify-lines-carry-no-content · apply — check 5 / check 7: raised by main (the sweep's row the proposers left, or the row fit read a proposed heir standing under); Retired (4) and (6) carry it (routine)
contract/replayed-snapshot/live-fields · apply — check 5 / check 7: raised by main (the sweep's row the proposers left, or the row fit read a proposed heir standing under); Retired (4) and (6) carry it (routine)

ops-main-test-plan.md
decision/plugin-rewritten-on-start · apply — check 7: main sets the row fit read the proposed heir standing under; Retired (1) and (9) carry it (routine)
decision/recorder-scrub · apply — check 7: main sets the row fit read the proposed heir standing under; Retired (1) and (9) carry it (routine)

ops-main-security-plan.2.md
decision/user-content-only-to-diagnostics · apply — cascade step 2: main's fix of a stale clause the sweep found on a row tx 1287 changed; Retired (1) and (3) carry it (routine)
decision/absolute-viola-path-in-commands · apply — cascade step 2: main's fix of a stale clause the sweep found on a row tx 1287 changed; Retired (1) and (3) carry it (routine)

ops-main-test-plan.2.md
decision/plugin-rewritten-on-start · apply — check 7: main's second cell of the same row, after ops-main-test-plan.md lands; Retired (1) and (9), the P5 card's approved row (routine)
decision/recorder-scrub · apply — check 7: main's second cell of the same row, after ops-main-test-plan.md lands; Retired (1) and (9), the P5 card's approved row (routine)

ops-main-security-plan.md
decision/statusline-only-shell-out · apply — check 2: main's heir-less retire of a row whose proposed heir was rejected; Retired (4) names the shell-out (routine, D-retired)
decision/single-shell-out · apply — check 2: main's heir-less retire of a row whose proposed heir was rejected; Retired (4) names the shell-out (routine, D-retired)
decision/statusline-shell-out · apply — check 2: main's heir-less retire of a row whose proposed heir was rejected; Retired (4) names the shell-out (routine, D-retired)

ops-main-a11y-plan.md
contract/viola-hook-statusline/exit · apply — check 2: main's heir-less retire of a row whose proposed heir was rejected; Retired (4) names the arm and its deadline (routine, D-retired)
