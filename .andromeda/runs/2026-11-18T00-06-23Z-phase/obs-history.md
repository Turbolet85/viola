@history_item n=1 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (4. Span / Trace Coverage, 1 row) — `+ decision/trust-record-write-uninstrumented`: verify's trust-record write and removal emit no line and no span; the outcome rides only the stamps. This is the row this chunk re-reads live. The first-screen words and the `--record` witness must ride the stamp's `measured` or the rig, never a log line, and must carry no path, key or value of the claude config.

@history_item n=2 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (4. Span / Trace Coverage, 1 row) — `~ contract/verify-spawn-subject/verify-pty-probe`: five to six interactive PTY runs (Run F is the trust-record run). The six first-screen words map one-to-one onto these six `verify-pty-probe` pairs. A new run, or a change to a run, would move this count again.

@history_item n=3 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (6. Log Coverage, 2 rows) — `~ contract/diag-subject/verify-pty-probe` (Run A to Run F) and `~ contract/verify-process-exit/child-exit-status-and-duration` (eight to nine spawn pairs, each `process-exit` with `child_exit_status` and `duration_ms`, a killed PTY child included). These pairs are the existing per-run logged fact ("did each child exit, or was it cut?"). The chunk's closed facts per run should read from them or the stamp, with no new subject or field.

@history_item n=4 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (1. Obs Scope Summary, 2 rows) — `~ decision/claude-internals-not-a-contract` except (verify writes and removes its own temp folder's trust record in the claude config) and `~ decision/refusal-telemetry` except (the trust-record write refuses a symlinked, non-regular or foreign file with no line; the outcome shows only on the stamps). The `--record` witness of where the record went must stay inside this bounded exception, and any reading of the claude config beyond it is a widening. PROVISIONAL (delegate viola-overseer).

@history_item n=5 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (1. Obs Scope Summary, 1 row) — `~ decision/r8-strip-logged-as-names-only`: no env value is read, bar the kept `CLAUDE_CONFIG_DIR` that verify reads, and no value is logged. Verify's interactive children receive that kept value. The P3 env-difference list (credentials' reach, `HOME`, `TERM`) may name variables only, never values.

@history_item n=6 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (3. Observability Harness Contract, 1 row) — `~ decision/env-vars-not-configuration`: the one `CLAUDE_CONFIG_DIR` value verify reads locates the child's claude config, is never logged and configures nothing of viola. A rig `V` arm seeding a private config, or a git-initialised cwd, must not make env a configuration channel for viola.

@history_item n=7 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (1. Obs Scope Summary, 2 rows) — first added the `claude-internals-not-a-contract` and `refusal-telemetry` excepts (the transcript echo read). The trust-record chunk later rewrote both excepts. This entry is the precedent for a bounded, no-byte-kept reading of claude's files as a named exception.

@history_item n=8 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified the 2026-11-16-local-command-delivery excepts (claude-internals-not-a-contract, refusal-telemetry, deterministic-event-emission, deterministic-emission, transcripts-not-a-contract). The trust-record rewrites of those excepts still stand PROVISIONAL on the delegate's word, which is a ratification the founder still owes.

@history_item n=9 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (10. SLO Invariants & Telemetry Budgets, 1 row) — `~ decision/deterministic-emission` except: the readiness gate reads the vt100 screen before a paste. Together with the `no-telemetry-from-screen` rows, this bounds the new first-screen word: it is a closed word from compiled signatures in the stamp, never telemetry derived from screen content or screen text.

@history_item n=10 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ contract/run-start-events/order` and `~ decision/product-order-from-events-log`. On 2.1.325, folder-trust keys passing through the wrapper write `wheel{human-input}` before `session-start`, which is a live trust-screen reading on this CLI. This is the precedent for the `trust` first-screen word. Note that `viola run` stays unchanged in this chunk.

@history_item n=11 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (verify's outcome on stdout, the uninstrumented `hook --capture` arm) — verify's agent-readable outcome is the exit code plus one stdout step line per row and the `stamped <version>  N pass  N fail` summary. The hidden capture arm is uninstrumented by design (the founder's ratification). The `V` arm's printed six words must come from the stamp, not from new stdout lines or capture-arm logging.

@history_item n=12 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the `cli` role's internal-error line) — a `cli` fault writes `process-exit{detail:"internal-error"}` when an instance resolves, then exactly `error: internal error`. This is the shape behind `contract/cli-internal-error/stderr-line` for any new verify fault path.

@history_item n=13 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — added `verify-probe` to the subject set; verify logs its spawns at the call site and `run_bounded` stays unlogged. In CI, verify runs only against the fake agent. The real-CLI verify runs only through the local `run --local-live`, refused under `CI`. This is the source of `ci-verify-fake-agent-only` and `real-cli-verify-never-in-ci`: S0′ is local only, and the fake-agent work lands first.

@history_item n=14 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — `verify-pty-probe` joined the subject enum with verify's interactive PTY runs, each logged at the call site. CI drives the fake agent with `--screens --turn-stop --trusted-root`. A change to the fake agent's screens (the first-screen signatures) follows this flag binding.

@history_item n=15 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — moved from four to six spawn pairs (Runs C and D). The capture arm's `--answers` body is printed to stdout and the arm stays uninstrumented. CI adds `--dialogs`. This is a precedent for adding runs without a new subject.

@history_item n=16 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — CI's fake-agent verify flag list gained `--framing`, bound to test-plan §3 boot step 4. Any new fake-agent flag this chunk adds to the screens moves this list and that bind together.

@history_item n=17 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `process-exit` `detail:"interrupted"` for a stopped verify. A stopped run starts no further spawn. The stop gives exit 1 with the fixed stderr pair, empty stdout and no stamp. The probe dir is removed on pass, fail and stop. Relevant for the rig's standing stop (Ctrl-C or group kill) on S0′ and for the `verify_window_` kill: a stopped verify writes no stamp, and so no `measured` words.

@history_item n=18 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify's stdout gives one `[NN/MM]` step line per relied row, MM being the relied count. No line carries the probe command, a payload or a path. These are the step-line and summary contracts the rig's `V` arm reads around.

@history_item n=19 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — a child of an interactive PTY run killed at a stop carries `child_exit_status` and `duration_ms`, as measured. A print-mode or `--version` child killed at a stop is unmeasured. This bounds what the per-run "exited before the read or cut" fact can read from the log.

@history_item n=20 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added an `agents-probe` pair around Run B's agents read (verify's eighth pair, now nine with Run F). The subject enum was unchanged. This is the precedent that a new verify read reuses an existing subject rather than adding one.

@history_item n=21 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — MM moved to 24 with `local-command-compact`, and Run B's six pastes added no spawn pair. These are count registers (`contract/verify-stdout/step-line`, `summary-line`). The chunk adds no row, so MM stays at the relied count.

@history_item n=22 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`viola verify`'s home-tree check and the catch site's narrowed reach) — verify runs `check_home_tree` before its log opens, and a refusal writes no line. `update_stamps` runs `check_stamps` before and under its lock, and a refusal maps to the home pair through verify's own `process-exit`. This is the origin of `contract/viola-verify/home-tree-refused` and `contract/viola-verify-refusal/*`, which bind any new `measured` write.

@history_item n=23 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the pre-init window and the panic exemptions gain the chunk's checks) — verify's home-tree check sits in the pre-init window, and a refusal is exemption 8 with no line. This is the order trap: the check precedes the first open.

@history_item n=24 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the refusal-telemetry record) — this entry set the record of which strict-modes refusals leave a line (the `ledger-stamps` read and verify's own `process-exit` on a refused `update_stamps`) and named every other refusal as leaving none. It is the base of `decision/strict-modes-failure-lines` and `strict-modes-refusal-writes-no-line`.

@history_item n=25 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — `parse-rejected{parser:"ledger-stamps"}` gained `strict-modes-failed`: run's version gate reads a refused stamps file as unverified. This is the source of `contract/parse-rejected-detail/strict-modes-failed` and `unreadable-ledger-is-unverified`. A stamp whose `measured` holds new words must still parse in run's gate.

@history_item n=26 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`verify` writes its refusal's code) — verify's `process-exit` under a parsed `VIOLA_NAME` carries `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture` or `interrupted`. A failing row carries no code, and verify takes no `--json`. This is the origin of `contract/process-exit-detail/verify-refusal`. A trust-record fail is a failing row, not a refusal code.

@history_item n=27 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's document under `--json`) — the `cli` catch site's `internal-error` document versus the stderr line. Verify has no `--json`, so its fault stays `error: internal error`.

@history_item n=28 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the ConPTY sideload and the Windows kill fallback retired) — a strict-modes failure has a line in two places: the `ledger-stamps` read and verify's refused stamp write. The PTY backend is openpty alone. This is the current form of `decision/strict-modes-failure-lines`.

@history_item n=29 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows host arms of the panic capture, the env strip, verify and the statusline retired) — the strip matches `CLAUDE*` names exactly, with the persistent set from `claude_env_keep`. A verify stop reads `SIGINT`/`SIGTERM`. SIGKILL or a crash leaves the probe dir. The relied count is all rows of `ALL`. These bear on the `V` arm's group-kill stop and on the env differences between verify's children and the rig's start.

@history_item n=30 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the closed words that left `diag-line.v1.json` with their emitter) — the rule is that a closed word no build emits leaves the schema with its emitter, and `os` is closed to `linux`. If the chunk adds any closed log word, the schema and its emitter move together.

@history_item n=31 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — the E2 strip always removes the 11-name identity floor, whatever the persistent set says. A differing Linux reading on the dev host is recorded in architecture. This is the source of `decision/identity-floor-always-stripped`, a candidate difference between verify's children and the rig's own wrapped start.

@history_item n=32 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — `process-start{subject:"claude-child"}` gained `env_kept`, and D-34 makes every stripped and kept name logged as names only, never values. This is the source of the `env_kept` / `env_stripped_*` contract rows, the names-only reading for the env candidate list.

@history_item n=33 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (4. Span / Trace Coverage, 9 rows) — `− decision/no-statusline-settings-in-telemetry`, pointing to `decision/child-settings-never-logged`: run no longer passes `--settings`. Verify's probe argv (model flag, `--settings`, plugin dir) is a P3 candidate. Its `--settings` stays out of every line and span.

@history_item n=34 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (4. Span / Trace Coverage, 7 rows) — `~ decision/hidden-capture-arms-uninstrumented` and `~ decision/hook-capture-arms-unlogged`: one hidden capture arm (`hook <event> --capture`) stays, uninstrumented, after the Run B status-line probe was retired.

@history_item n=35 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (4. Span / Trace Coverage, 1 row) — `~ decision/verify-lines-carry-no-content`: no verify line carries row text, a prompt, a session id, the probe command or a path. The first-screen words and closed facts must keep to this, holding no screen text.

@history_item n=36 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (6. Log Coverage, 6 rows) — `~ contract/diag-line-fields/process-start` (subject set without `statusline-shell`) and `~ contract/diag-line-fields/process-exit` (`shell_exit_status` dropped). These are the current field catalogues verify's pairs write against.

@history_item n=37 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (6. Log Coverage, 1 row) — `~ contract/child-spawn-log/subject`: child spawns only, with no shell spawn.

@history_item n=38 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (6. Log Coverage, 5 rows) — `~ contract/diag-line-fields/process-start` added `hyprctl-probe`, and `+ contract/diag-subject/hyprctl-probe`. This is the precedent that a new subject word joins the closed enum with its emitter, each word its own row.

@history_item n=39 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Obs Scope Summary, 3 rows) — `~ decision/instrument-call-sites-not-os-internals` (adds the hyprctl spawn), `~ decision/per-parser-parse-failure-counts` and `~ decision/parse-failures-surface-as-unknown` (the hyprctl except).

@history_item n=40 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Obs Scope Summary, 2 rows) — `~ decision/os-apis-at-call-sites` (adds the hyprctl spawn) and `~ decision/parse-failures-unknown-counter` (the hyprctl except).

@history_item n=41 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (1. Obs Scope Summary, 1 row) — `~ decision/parse-failures-unknown-counter` except: a StopFailure word outside the CLI's set reads `api-error` with one `parse-rejected` line.

@history_item n=42 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (1. Obs Scope Summary, 1 row) — `~ decision/parse-failures-surface-as-unknown`, the same except as its twin row.

@history_item n=43 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (the StopFailure no-ledger-row answer ratified) — StopFailure stands with no ledger row and no verify probe, and its live reading is owned by «The 2.1.325 recorded fixture set». That is the same entry this chunk's scope names as the owner of re-stamping the live home. Do not fold that work here.

@history_item n=44 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Obs Scope Summary, 6 rows) — `~ decision/exit-code-log-field`: exit 11 is retired and never reused. Verify's exits stay 0 and 1.

@history_item n=45 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Obs Scope Summary, 1 row) — `~ decision/diag-schema-additive-only`, narrowed to "a field the product still emits" by the operator's Q2. This binds any field the chunk adds or moves.

@history_item n=46 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Obs Scope Summary, 1 row) — `~ decision/log-fields-additive-only`, narrowed the same way.

@history_item n=47 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (6. Log Coverage, 1 row) — `~ decision/log-schema-additive-only`: rename nothing, and remove only a field no build emits.

@history_item n=48 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (3. Observability Harness Contract, 1 row) — `~ decision/obs-aligns-to-tests-log-format`, narrowed the same way.

@history_item n=49 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (8. PII Scrubbing & Compliance, 2 rows) — `~ decision/secret-scan-before-upload` (classes: `CLAUDE*` values, the content canary, 0600, an unreadable file) and `~ decision/secrets-never-logged`. These are the secret-scan classes any kept rig or verify output faces.

@history_item n=50 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (1. Obs Scope Summary, 1 row) — `~ decision/no-secrets-in-log-lines`: no stripped `CLAUDE*` value in any line, with the Cookie clause dropped.

@history_item n=51 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (3. Observability Harness Contract, 1 row) — `~ decision/content-only-in-instance-detail-file`: home-level files are codes-only.

@history_item n=52 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (11. Obs Anti-Patterns, 1 row) — `~ decision/claude-env-and-cookie-never-logged` (rule): `CLAUDE*` values are never logged, detail files included.

@history_item n=53 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (11. Obs Anti-Patterns, 1 row) — `~ decision/claude-env-and-cookie-never-logged` (name): "CLAUDE* values never logged".

@history_item n=54 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (process-start's fields on Linux only) — `pty_backend` is `openpty` alone and `sideload_fallback` is gone. This is the current `process-start` field set verify's pairs write.

@history_item n=55 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (pinned-exe hash-mismatch refusal telemetry) — the pinned-exe hash mismatch adds no line, field or code through a lost placement. This is the refusal-telemetry row's pinned-exe half, and verify's `pinned-hash-mismatch` refusal code shares it.

@history_item n=56 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (the `ui` process and four events leave) — the process enum is closed to `run|hook|mcp|cli` and the role files to `run-<name>`, `hook-<name>`, `mcp`, `cli-<name>`. Verify logs as `cli` into `cli-<name>.ndjson` only under a parsed `VIOLA_NAME`.

@history_item n=57 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — plugin install writes its self pair into the caller's `cli-<name>.ndjson` only under a parsed `VIOLA_NAME`, after its check passes, and a refused check writes no line. This is the shared shape of `decision/verify-logs-only-with-instance` and `refused-home-tree-writes-no-line`.

@history_item n=58 marker=2026-09-24-log-redaction-and-never-log-floor
**2026-09-24-log-redaction-and-never-log-floor** — anyhow's scope is the root-bin dispatch edge and its catch-site reporter, and the chain goes only to `detail-<process>.ndjson` when an instance resolves. This is the source of `anyhow-error-capture-boundary` and `error-chain-in-detail-log-only` for any new verify error path.

@history_item n=59 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — the registry migration lifted D-34 (the names-only strip and the 11-name `IDENTITY_FLOOR`) and D-35 (the closed `subject` enum; verify's spawn exits carry `child_exit_status` and `duration_ms`) into the body. These are the in-force origins of the env and spawn-pair rows above.
