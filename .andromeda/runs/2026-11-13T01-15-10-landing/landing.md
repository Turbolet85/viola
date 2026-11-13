# The rows landing of 2026-11-12T03-23-49

## The dry run's head

    rows-convert · proposal 2026-11-12T03-23-49 · word 76f59d6ac300f77d5e8feffa1e02d5865bda97e766fc55fec6e0b9318a5e3b6d · of 0922fbab47d9 · head 2d3788258131 · pin 744

    re-check 1 · the masters' text pin (744, the proposal's 766): ok
    re-check 2 · the store is the list: ok
    re-check 3 · the files a row's verdict stands on (0 row(s) moved since 8834195b0683): ok
    re-check 4 · the word: ok
    re-check 5 · the proposal's recorded tool (rec v1.32): ok
    re-check 6 · the install reads rows: ok
    re-check 7 · no edge from outside the family: ok
      chunks: 0 pending · 0 gated
      masters: architecture text · security-plan text · design-system text · layout-templates text · test-plan text · obs-plan text · a11y-plan text
      rec.py check: clean
      tree: nothing uncommitted
      proposal 2026-11-12T03-23-49 · finished · every file by its sha
    re-check 8 · the strict ground: ok

    gone: drop 45 · keep 3 · stale 7 · 55 line(s)
    guards: stands 6 · absent 2 · 8 line(s)
    divergences: code 75 · holds 6 · rule 14 · none 5 · 100 line(s)
    stale: none 3 · keep 2 · code 18 · 23 line(s)
    rekey: no fate · 0 line(s)
    relied: cut 12 · keep 45 · 57 line(s)
    moved: no fate · 0 line(s)

## The TX

`landed tx 803 · 4971 row(s) of 7 master(s) · 8098 node(s) · 3593 edge(s) · 2026-11-13T01-16-59-2026-11-13T01-15-10-landing.ndjson`

## The read-backs

- `rec.py check`: `check: clean`.
- `rec.py rows-form`: `masters: rows · landing 2026-11-13T01-15-10 uncommitted`, exit 3.
- `master_row` by master and sheet: 27 rows, each equal to the dry run's table (4971 rows of 7 masters).
- `rec.py tree doc:{doc}`: the seven masters read, exit 0 each — architecture 1062 · security-plan 637 · design-system 344 · layout-templates 226 · test-plan 1379 · obs-plan 1077 · a11y-plan 246.

## The leaves

- set: `total · whole · 15 group(s) · 27 leaves · 419656 B · unmapped 2`
- batch `leaf`: `VERDICT: 15 of 15 pass · failed: none`
- apply: `total · blocks 40 · applied 40` — 22 leaves written; 0 by hand · 0 declined; `cascade-dispositions.md` §Leaves
- outputs: `landed tx 826 · 82 record(s) of 15 payload(s)`

## No row

- no row · design-system · decision/two-hues-one-role-each · The second hue is the web page's handoff blue, and the web palette leaves the product in 0.2.0; amber keeps its own row.
- no row · security-plan · decision/budget-json-checked-read · The budget gate and its budget.json reading leave the product in 0.2.0 (census row F-08).
- no row · architecture · decision/percent-newtype · Percent's only users are the budget-gate thresholds, and the budget gate leaves the product in 0.2.0.
- no row · architecture · decision/no-env-config-channel · a duplicate of decision/no-environment-configuration
- no row · architecture · decision/budget-gate-reads-hook-written-file · The budget gate and its budget.json reading leave the product in 0.2.0 (census row F-08).

## Refused rows

refused rows: 2 — a row the fix round refused lands nothing, under any fate; its records are answered by its line alone where no landed row names them
- refused row · security-plan · decision/actions-pinned-by-sha · [ROW_FORM] taken — another row stands under this handle · fct-dj5p5k3a@1 named by decision/checkout-no-persisted-credentials · decision/actions-pinned-by-sha
- refused row · test-plan · decision/snapshot-replay-from-log · [ROW_FORM] taken — another row stands under this handle · fct-3svwp7h4@1 named by decision/links-survive-wrapper-restart · decision/replay-writes-no-file · decision/replayed-snapshot-has-no-endpoint · decision/collision-check-over-replay-reads-beat · decision/list-reads-live-through-replay · contract/event-log/start-wheel-record · decision/snapshot-replay-from-log

## twins:

- equal lead · design-system · decision · «no hint names viola release» · decision/no-hint-names-release · decision/no-hint-names-viola-release
- twin section · design-system · ## Motion (calibrated to the base expression level `0.2`) · motion-calibrated-to-the-base-expression-level-0-2 · motion
- twin section · design-system · ## Surface: cli · surface-cli · surface
- equal lead · a11y-plan · decision · «linear static cli output» · decision/linear-static-output · decision/linear-static-cli-output
- equal lead · a11y-plan · decision · «fixed cli strings pinned by literal» · decision/fixed-cli-strings-pinned · decision/fixed-cli-strings-pinned-by-literal
- equal lead · a11y-plan · contract · «cli exit codes · 2» · contract/cli-usage-error/exit-2 · contract/cli-exit-codes/2
- twin section · a11y-plan · ## 1. A11y Scope Summary · 1-a11y-scope-summary · a11y-scope-summary
- twin section · a11y-plan · ## 4. ARIA Patterns & Roles · 4-aria-patterns-roles · aria-patterns-roles
- twin section · a11y-plan · ## 6. Visual Design Verification · 6-visual-design-verification · visual-design-verification
- twin section · a11y-plan · ## 8. Cognitive Accessibility · 8-cognitive-accessibility · cognitive-accessibility
- equal lead · security-plan · decision · «mcp tool list is an affordance» · decision/mcp-tool-list-affordance · decision/mcp-tool-list-not-enforcement
- equal lead · security-plan · decision · «hook security fields in closed enums» · decision/hook-security-fields-closed · decision/hook-security-fields-closed-enums
- equal lead · security-plan · decision · «tracing-subscriber version floor» · decision/tracing-subscriber-floor · decision/tracing-subscriber-version-floor
- twin section · security-plan · ## Bootstrap phases (derive for route / setup-project) · bootstrap-phases-derive-for-route-setup-project · bootstrap-phases
- twin section · security-plan · ## Security Anti-Patterns (NEVER do these) · security-anti-patterns-never-do-these · security-anti-patterns
- equal lead · architecture · decision · «interprocess local sockets behind the channel seam» · decision/interprocess-local-socket-channel · decision/interprocess-local-socket-ipc
- equal lead · architecture · contract · «mcp tool results · refusal» · contract/mcp-tool-result/refusal · contract/mcp-tool-results/refusal
- equal lead · architecture · decision · «exec-form command hooks» · decision/exec-form-hooks · decision/exec-form-command-hooks
- equal lead · architecture · decision · «hooks and mcp never search path» · decision/no-path-lookup · decision/hooks-and-mcp-never-search-path
- equal lead · architecture · decision · «human text by default, --json for agents» · decision/cli-output-conventions · decision/cli-human-text-json-exit-codes
- equal lead · architecture · decision · «environment strip reads names only» · decision/env-strip-reads-names-only · decision/env-strip-names-only
- equal lead · architecture · decision · «nothing published to crates.io» · decision/no-crates-io-publish · decision/no-crates-io-publishing
- equal lead · architecture · contract · «viola verify process-exit detail · program-not-found» · contract/verify-process-exit/program-not-found · contract/verify-exit-detail/program-not-found
- equal lead · architecture · contract · «viola verify process-exit detail · pinned-hash-mismatch» · contract/verify-process-exit/pinned-hash-mismatch · contract/verify-exit-detail/pinned-hash-mismatch
- equal lead · architecture · contract · «viola verify process-exit detail · version-unreadable» · contract/verify-process-exit/version-unreadable · contract/verify-exit-detail/version-unreadable
- equal lead · architecture · contract · «viola verify process-exit detail · strict-modes-failed» · contract/verify-process-exit/strict-modes-failed · contract/verify-exit-detail/strict-modes-failed
- equal lead · architecture · contract · «viola verify process-exit detail · unclean-fixture» · contract/verify-process-exit/unclean-fixture · contract/verify-exit-detail/unclean-fixture
- equal lead · architecture · contract · «viola verify process-exit detail · interrupted» · contract/verify-process-exit/interrupted · contract/verify-exit-detail/interrupted
- equal lead · architecture · contract · «viola verify exit codes · 1» · contract/viola-verify-exit-codes/1 · contract/viola-verify-exit-codes/1-relied-row-failed
- equal lead · test-plan · decision · «no orphan modules» · decision/cargo-modules-orphans-gate · decision/no-orphan-modules
- equal lead · test-plan · decision · «each exit cause has its own hint» · decision/hint-names-its-cause · decision/each-exit-cause-own-hint
- equal lead · test-plan · decision · «mutation testing outside ci» · decision/mutation-not-a-ci-gate · decision/mutation-testing-off-ci
- equal lead · test-plan · decision · «perf verdict from the harness gate» · decision/perf-verdict-from-harness · decision/perf-verdict-from-harness-gate
- equal lead · test-plan · decision · «least workflow permissions» · decision/workflow-least-permissions · decision/least-workflow-permissions
- equal lead · test-plan · decision · «single writers of snapshot and stamps» · decision/single-writers · decision/single-writer-state-files
- equal lead · test-plan · decision · «stamps written only by verify» · decision/stamps-only-from-verify · decision/stamps-written-only-by-verify
- equal lead · test-plan · contract · «instance-unreachable causes · socket-dir-refused» · contract/instance-unreachable-cause/socket-dir-refused · contract/unreachable-cause/socket-dir-refused
- twin section · test-plan · ## 1. Test Scope Summary · 1-test-scope-summary · test-scope-summary
- twin section · test-plan · ## 2. Test Strategy · 2-test-strategy · test-strategy
- twin section · test-plan · ## 4. Unit Test Strategy · 4-unit-test-strategy · unit-test-strategy
- twin section · test-plan · ## 5. Integration Test Strategy · 5-integration-test-strategy · integration-test-strategy
- twin section · test-plan · ## 6. E2E Test Strategy · 6-e2e-test-strategy · e2e-test-strategy
- twin section · test-plan · ## 7. Test Data & Fixtures · 7-test-data-fixtures · test-data-fixtures
- twin section · test-plan · ## 8. Mocking & Stubbing Discipline · 8-mocking-stubbing-discipline · mocking-stubbing-discipline
- twin section · test-plan · ## 9. CI Integration · 9-ci-integration · ci-integration
- twin section · test-plan · ## 10. Quality Gates & Coverage Targets · 10-quality-gates-coverage-targets · quality-gates-coverage-targets
- equal lead · obs-plan · decision · «child exit from the process handle» · decision/child-exit-from-process-handle · decision/child-exit-from-handle-wait
- equal lead · obs-plan · decision · «no otel sdk exporter» · decision/no-otel-sdk-exporter · decision/no-otel-exporter
- equal lead · obs-plan · decision · «redaction cannot be disabled» · decision/redaction-not-disableable · decision/redaction-cannot-be-disabled
- equal lead · obs-plan · decision · «drift reports go to diagnostics only» · decision/drift-reports-to-diagnostics · decision/drift-reports-diagnostics-only
- equal lead · obs-plan · decision · «future reporter applies the never-log floor» · decision/future-reporter-never-log-floor · decision/reporter-never-log-floor
- equal lead · obs-plan · decision · «veil toggle feature banned» · decision/veil-toggle-feature-banned · decision/veil-toggle-banned
- equal lead · obs-plan · decision · «no telemetry from screen content» · decision/no-telemetry-from-screen-content · decision/no-telemetry-from-screen
- equal lead · obs-plan · decision · «hook log write failure swallowed» · decision/hook-log-failure-swallowed · decision/hook-log-write-failure-swallowed
- equal lead · obs-plan · decision · «mcp rejections carry codes only» · decision/mcp-results-codes-only · decision/mcp-rejections-codes-only
- equal lead · obs-plan · decision · «log level never from the environment» · decision/log-level-not-from-env · decision/log-level-never-from-env
- equal lead · obs-plan · decision · «no log rotation in v1» · decision/no-log-rotation-v1 · decision/no-log-rotation-in-v1
- equal lead · obs-plan · decision · «deterministic event emission» · decision/deterministic-event-emission · decision/deterministic-emission
- equal lead · obs-plan · decision · «dialog content never logged» · decision/no-dialog-content-in-logs · decision/dialog-content-never-logged
- twin section · obs-plan · ## 1. Obs Scope Summary · 1-obs-scope-summary · obs-scope-summary
- twin section · obs-plan · ## 2. Telemetry Strategy · 2-telemetry-strategy · telemetry-strategy
- twin section · obs-plan · ## 4. Span / Trace Coverage · 4-span-trace-coverage · span-trace-coverage
- twin section · obs-plan · ## 5. Metric Coverage · 5-metric-coverage · metric-coverage
- twin section · obs-plan · ## 6. Log Coverage · 6-log-coverage · log-coverage
- twin section · obs-plan · ## 7. Error Capture & Reporting · 7-error-capture-reporting · error-capture-reporting
- twin section · obs-plan · ## 8. PII Scrubbing & Compliance · 8-pii-scrubbing-compliance · pii-scrubbing-compliance
- twin section · obs-plan · ## 9. CI Integration · 9-ci-integration · ci-integration
- twin section · obs-plan · ## 10. SLO Invariants & Telemetry Budgets · 10-slo-invariants-telemetry-budgets · slo-invariants-telemetry-budgets
- twin section · obs-plan · ## 11. Obs Anti-Patterns (NEVER do these) · 11-obs-anti-patterns-never-do-these · obs-anti-patterns

## Route owed (`route-owed.md`)

layout-templates · decision/shared-captions-fixed · The planned fleet console (0.2.0, Epoch 6) reuses the six board captions, their words and slot order (src/human/board/rows.rs:42).
a11y-plan · decision/refusal-ends-with-hint · Every refusal reason prints a hint, unverified-cli and answer's other not-delivered details included (src/cmd/send.rs:85 · src/human/mirror.rs:65-67).
security-plan · decision/fixture-scrub-and-refusal · The email check runs on every kind of committed fixture and at the record, not on screens alone (crates/viola-agent-claude/src/ledger.rs:1327-1334).
security-plan · decision/exact-third-party-pins · The chunk that closes V182 settles rmcp's version (Cargo.toml:223, held to >=3.4.1, <3.5), and its wrap writes the rmcp exception in the form that proves right.
security-plan · decision/cli-names-through-name-check · run parses its name through the shared parse_name rather than its own copy of send's (src/cmd/run.rs:50).
security-plan · decision/strict-modes-check · The trusted-state checks refuse a symbolic link too, as the diagnostics and socket-directory checks do: check_instance, check_stamps, check_path and check_read stop following one (crates/viola-state/src/strict.rs:240).
architecture · decision/no-tmux-or-shell-dependency · The status-line shell-out leaves (crates/viola-agent-claude/src/statusline.rs:124): 0.2.0's entry «Status-line interception removed» (census F-09), already on the route.
test-plan · decision/pty-tested-on-real-openpty · The needless key at tests/tui_pty_seam.rs:64 goes; the size receipt follows a resize on its own.
test-plan · decision/fixture-hygiene-walk · The email check runs on the hook, agents and statusline fixture walks too, not only on the Mcp and Screen walks (tests/contract_fixture_hygiene.rs:72) — one work with security-plan · decision/fixture-scrub-and-refusal.
test-plan · decision/fd-test-reads-live-child-table · The self-dump fallback goes and the test always reads /proc/<pid>/fd, Viola being Linux-only (tests/tui_channel_fds.rs:25).
test-plan · decision/no-fd-count-assertion · The fds.len() <= 5 count assertion goes (tests/tui_channel_fds.rs:34-37); the /proc table assertions stay.
test-plan · decision/fixtures-walked-for-hygiene · Emails are walked for in every kind of committed fixture, not only MCP exchanges and screens (tests/support/hygiene.rs:10) — one work with security-plan · decision/fixture-scrub-and-refusal.
obs-plan · decision/payload-types-derive-redact · The hook and MCP payload types derive veil Redact too; today only the channel Request does (crates/viola-channel/src/frame.rs:51).
obs-plan · decision/private-diagnostics-files · The diagnostics open refuses a symlink and narrows the mode on the open file, closing the race after the earlier lstat (crates/viola-state/src/fs.rs:44-51).

## The reading's model

The proposal's reading batches ran on the session's own model (`proposal.json` holds `model: null` for them).
