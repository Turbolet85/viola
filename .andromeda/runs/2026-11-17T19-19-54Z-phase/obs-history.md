@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — (fake agent's print-ban exemption) the fake agent is a `[[bin]]` of the root package, so it inherits the root `[lints]` and is exempt only through the crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]` in `src/bin/viola-fake-agent.rs`. This chunk makes the fake agent read the trust record from the config file. That code lives under this exemption and needs no lint table of its own.

@history_item n=2 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — (obs_event! field set, schema default-deny keyword) `obs_event!` attaches `event`, `process` and `instance`, and the diag schema enforces default-deny through a top-level `unevaluatedProperties: false`. Any new field or closed word for the sixth run or for the record writer's outcome must be added to `schemas/diag-line.v1.json`, or the field is a build failure (rows decision/schema-top-level-unevaluated-properties, decision/default-deny-log-fields).

@history_item n=3 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — (raw-tracing ban) `clippy.toml` bans the `tracing` level macros by path, and a fail-closed grep in `scripts/lint-probes.sh` catches a raw `event!`. Any line the writer or the new verify run logs must go through `obs_event!`.

@history_item n=4 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — (env_kept, D-34 supersedes D-14) `process-start{claude-child}` logs every stripped and kept `CLAUDE*` name, names only, and never reads a value. S0 runs verify with `CLAUDE_CONFIG_DIR` kept through `claude_env_keep`, so the kept name shows on that line while its value, the config path, must never appear (rows contract/claude-child-process-start/env-*, decision/env-names-only).

@history_item n=5 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — (E2's floor sentence) the R8 strip always removes the 11 `IDENTITY_FLOOR` names, whatever the persistent set says. Trap: whether `CLAUDE_CONFIG_DIR` can reach verify's child through `claude_env_keep` decides which config file the child reads, and so where the record has to be written (row decision/claude-env-strip).

@history_item n=6 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (verify's outcome on stdout, the uninstrumented `hook --capture` arm) This entry set verify's outcome: exit 0 or 1, one stdout step line per row, and a last line `stamped <version>  N pass  N fail`. Refusals print the `unable:` + `hint:` pair, and a fault prints `error: internal error`. The hidden capture arm is uninstrumented (§10 exemption 5). The sixth run and the 23rd row extend this stdout contract (rows contract/verify-stdout/step-line, contract/verify-stdout/summary-line).

@history_item n=7 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (panic backtrace raw frames; the `cli` role's internal-error line) a `cli` panic under a resolved instance writes `process-exit{self, 1, internal-error}`, puts the chain only in `detail-cli.ndjson` and writes exactly `error: internal error\n`. This holds for verify's new run and for the writer's errors (row contract/panic-outcome-by-role/cli).

@history_item n=8 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — (verify-probe joins the spawn subjects) verify logs each child spawn as a `process-start`/`process-exit` pair at its call site, with `child_exit_status` and `duration_ms`. In CI, verify runs only against the fake agent; the real-`claude` verify runs only through `run --local-live`, which refuses under CI. A new closed value needs a Decisions Log entry (rows decision/ci-verify-fake-agent-only, decision/real-cli-verify-never-in-ci).

@history_item n=9 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — (strict-modes-failed on the stamps read) `parse-rejected{parser:"ledger-stamps"}` gained `strict-modes-failed`. When `run`'s version gate refuses the stamps file, it reads unverified (`cli_verified:false`). This is the line by which a stale 22-row stamp shows once the 23rd row is relied (rows contract/parse-rejected-detail/strict-modes-failed, decision/unreadable-ledger-is-unverified).

@history_item n=10 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — (verify-pty-probe: four verify spawn pairs) this entry added the subject `verify-pty-probe`, one pair per interactive PTY run. CI's fake-agent runs pass `--trusted-root`, the argv trust signal that the fake agent alone reads today. This chunk makes the fake agent read the config record as well (row contract/verify-spawn-subject/verify-pty-probe).

@history_item n=11 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — (six verify spawn pairs) Runs C and D raised the spawn pairs from four to six with the subject enum unchanged, and CI's fake-agent flags added `--dialogs`. This is the precedent for adding an interactive run: the count moves, the subject stays `verify-pty-probe`.

@history_item n=12 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — (the fake-agent verify flag list gains --framing) CI's fake-agent verify flag list is bound to test-plan §3's boot step 4 and moves with it. A new fake-agent flag or reading for the trust record moves both masters together.

@history_item n=13 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — (`detail:"interrupted"` on verify's process-exit) This entry added `interrupted`. A stopped verify starts no further spawn and closes every pair it opened. The probe dir is removed on a pass, a fail and a stop, but SIGKILL or a crash leaves it. This is the direct precedent for removing verify's own trust record on every exit path it reaches and naming what a SIGKILLed verify leaves (rows contract/verify-process-exit/interrupted, contract/verify-stop-stderr/unable-hint-pair).

@history_item n=14 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — (verify's relied step lines) the step lines' `MM` is the relied row count, not the ledger row count, and the summary sums to it. The six spawn pairs stay six, and no line carries the probe command, a payload or a path. The 23rd relied row moves `MM` again.

@history_item n=15 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — (seven verify spawn pairs; a killed PTY child's exit status) Run E made five interactive runs and seven pairs, and `MM` moved to 22. A child of an interactive PTY run killed at a stop carries `child_exit_status` and `duration_ms`. A sixth interactive run follows the same form (row contract/verify-process-exit/child-exit-status-and-duration).

@history_item n=16 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (session-live, the agents-probe callers and verify's eight spawn pairs) verify logs eight child spawns: version-probe, verify-probe, five `verify-pty-probe` and one `agents-probe`. A sixth interactive run makes nine and moves the rows that state the counts: contract/diag-subject/verify-pty-probe («five interactive PTY runs, Run A to Run E»), contract/verify-spawn-subject/verify-pty-probe («five»), and contract/verify-process-exit/child-exit-status-and-duration («eight verify spawn pairs»).

@history_item n=17 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — (CL-1's third close condition) `verify`'s `MM` moved 23 to 24 with the 24th row, and Run B's pastes added no spawn pair. This is the last precedent for a relied-row step-count sweep across masters. A count carried from a plan is re-measured, never copied.

@history_item n=18 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`viola verify`'s home-tree check and the catch site's narrowed reach) verify runs `check_home_tree` before its `cli` log opens, and a refusal writes no line. `update_stamps` runs `check_stamps` before and under its lock, and a refusal maps to the home pair with exit 1 through verify's own `process-exit`. The record writer's refusals (absent, link, oversize) follow this place-and-line discipline (rows contract/viola-verify/home-tree-refused, decision/home-tree-check-before-log-init).

@history_item n=19 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the refusal-telemetry record: two strict-modes lines, every other refusal none) a strict-modes failure has a line only on the `ledger-stamps` read and verify's refused `update_stamps`. Every other refused check leaves none. A writer refusal on the config file that is not a strict-modes check must not be logged as one (rows decision/strict-modes-failure-lines, decision/strict-modes-refusal-writes-no-line).

@history_item n=20 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the pre-init window and the panic exemptions gain the chunk's checks) verify's home-tree check is in the pre-step-4 window and in exemption 8. A refused check writes no line, so a panic there writes none (rows decision/panic-one-line-record, decision/zero-unlogged-panics).

@history_item n=21 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — (`run.pin_copy` under a concurrent first start) the no-clobber placement goes through `viola_state::fs::place_private_new`: a temp file beside the target, its mode set first, then persisted. It opens no span and writes no line of its own. This is the precedent for an atomic-replace write helper on the sync state side, which is where the record writer's lock, replace and read-back may live (rows decision/state-sync-instrumentation, decision/os-apis-at-call-sites).

@history_item n=22 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the nine thiserror derives, CoreError retired) the product's `thiserror::Error` derive census reads 9 named types, `StampsError` among them. A new writer error type moves the census (rows decision/typed-errors-fixed-display, decision/error-messages-interpolate-nothing).

@history_item n=23 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`Refused::NotADirectory`'s fixed message) a new variant on an existing derive keeps a fixed, path-free message and adds nothing to the 9-derive census. This is the cheaper path for the writer's refusals (a link, a non-regular file, oversize).

@history_item n=24 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the thiserror derive census on the product sources) the census rule now reads «a grep of `thiserror::Error` over the product sources, `src` and `crates` outside `crates/viola-e2e`». Re-derive it with that rule if the writer adds a derive.

@history_item n=25 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (`verify` writes its refusal's code) under a parsed `VIOLA_NAME`, verify's `process-exit` carries `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture` or `interrupted`. A failing row carries no code, and verify takes no `--json`. A writer fault on the sixth run is a row reading, never pass, not a new exit code unless a new closed word is added to the enum (row contract/process-exit-detail/verify-refusal).

@history_item n=26 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the `process-exit` `detail` enum's count) the count stands by its rule, the members of `schemas/diag-line.v1.json`'s enum. A new verify refusal word moves it and needs a Decisions Log record (row decision/distinct-detail-code-per-exit-cause).

@history_item n=27 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the `cli` catch site's document under `--json`) under `--json` the `cli` catch site prints `{"v":1,"error":"internal-error","detail":null}`; otherwise it prints `error: internal error` (rows contract/cli-internal-error/*, contract/panic-outcome-by-role/cli-json).

@history_item n=28 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the Windows host arms of the panic capture, the env strip, verify and the statusline retired) A verify stop is now `SIGINT`/`SIGTERM` only. The relied count is all rows of `ALL` with no OS split. SIGKILL and a crash leave the probe dir. The 23rd row's count and its stop and leftover wording are stated for Linux alone.

@history_item n=29 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the `.cmd`/`.bat` start refusal retired; the refusal writer's site counts) `batch-script-child` left verify's codes. The `human::refuse(` census reads `run` 7 and `verify` 5, re-derived by `grep -rcE 'human::refuse\(' src`. A new verify refusal through `human::refuse` moves it.

@history_item n=30 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the closed words that left `diag-line.v1.json` with their emitter) this added the rule that a closed word no build can emit leaves the schema with its emitter, and that a promise row's edit must be a whole-fact edit.

@history_item n=31 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** — (Cookie clause dropped) This amended decision/no-secrets-in-log-lines, decision/secrets-never-logged, decision/content-only-in-instance-detail-file and decision/claude-env-and-cookie-never-logged, and retired decision/cookie-header-never-logged. The never-log floor now stands as `CLAUDE*` values and content only. The writer must keep the config path and the record's key, the project path, out of every home-level line.

@history_item n=32 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (6. Log Coverage: 5 row(s)) `~ contract/diag-line-fields/process-start` gained `hyprctl-probe`, `+ contract/diag-subject/hyprctl-probe` was added, and `parse-rejected` gained a parser. This is the precedent for widening a closed subject or parser list: each new word has its own row and edits the field-list row with it.

@history_item n=33 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (1. Obs Scope Summary: 2 row(s) and 3 row(s)) `~ decision/os-apis-at-call-sites` and `~ decision/instrument-call-sites-not-os-internals` added the hyprctl spawn. `~ decision/parse-failures-unknown-counter`, `~ decision/parse-failures-surface-as-unknown` and `~ decision/per-parser-parse-failure-counts` took the hyprctl exception. A config-file read and parse by the writer meets these rows: name its call site, and give a parse failure a closed word rather than `unknown`.

@history_item n=34 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1. Obs Scope Summary / 3. / 6. additive-only rules narrowed) `~ decision/diag-schema-additive-only`, `~ decision/log-fields-additive-only`, `~ decision/log-schema-additive-only` and `~ decision/obs-aligns-to-tests-log-format` now read «must not rename or remove a field the product still emits». This was resolved by the operator (Q2), «Сузить правило». Any field this chunk changes is held to that rule.

@history_item n=35 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1. Obs Scope Summary: 6 row(s)) `~ decision/exit-code-log-field` now reads exit codes 0, 1, 2, 10, 12–14, 20, 21, and 11 is retired and never reused. verify's new outcomes stay inside 0 and 1.

@history_item n=36 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (4. Span / Trace Coverage: 1 row(s), and 6. Log Coverage: 6 row(s)) `~ decision/verify-lines-carry-no-content` dropped «a status line payload»: Run B's status-line probe retired. `~ contract/diag-line-fields/process-start` dropped `statusline-shell`. The capture arms went from two to one. The sixth run's lines must carry no row text, prompt, session id, probe command or path, and so no config path and no record key.

@history_item n=37 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — (1. Obs Scope Summary and 4. Span / Trace Coverage) `~ contract/run-start-events/order` and `~ decision/product-order-from-events-log` added `wheel{human-input}` before `session-start` where folder-trust keys pass through the wrapper. This is the trust dialog's current obs footprint. It changes only when the product's trust write (the next entry, «Session folder trusted by viola») lands, not in this chunk.

@history_item n=38 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (1. Obs Scope Summary: 2 row(s)) `~ decision/claude-internals-not-a-contract` gained its one exception, the transcript stdout-tag count. `~ decision/refusal-telemetry` gained its exception: the echo read refuses a symlinked, non-regular or foreign transcript with no line of its own. Reading and writing `.claude.json` is a second crossing of the claude-internals boundary, so it needs its own named exception on these rows. The `O_NOFOLLOW`, regular-file and owner pattern is the precedent for the writer's link and non-regular refusals.

@history_item n=39 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (11. Obs Anti-Patterns: 1 row(s); 1. and 10. deterministic-emission rows) `~ decision/transcripts-not-a-contract` and `~ decision/deterministic-emission` / `~ decision/deterministic-event-emission` gained exceptions. If the sixth run's verdict keys on anything but screen gate outcomes and hook events, the same rows must say so.

@history_item n=40 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — (the delegate's confirm ratified by the founder) the founder ratified the 2026-11-16-local-command-delivery exceptions on decision/claude-internals-not-a-contract, decision/refusal-telemetry, decision/deterministic-emission and decision/transcripts-not-a-contract. These are the standing exceptions that a new claude-config exception sits beside.

@history_item n=41 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** — (3. Observability Harness Contract: 1 row(s)) chrono is kept under a cargo-deny advisory ignore. If the writer brings a file-lock crate, it must pass cargo deny, stay Tokio-free and build no C (rows decision/obs-deps-tokio-free, decision/cargo-deny-failure-fails).
