@history_item n=1 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** (obs_event! field set, schema default-deny keyword) — obs_event! attaches only event/process/instance, and default-deny is a top-level unevaluatedProperties false in diag-line.v1.json. This explains rows decision/schema-top-level-unevaluated-properties and decision/default-deny-log-fields: adding veil Redact derives (item 10) must add no logged field or schema word.

@history_item n=2 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** (raw-tracing ban) — the level macros are banned by path in clippy.toml and raw event! by the fail-closed grep in scripts/lint-probes.sh. This explains decision/raw-tracing-ban and decision/obs-event-macro-only. Any line this chunk adds in src/obs.rs or the payload types goes through obs_event! only.

@history_item n=3 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** (D-28 half carried) — the concurrent-append check landed for the hook files (8 processes). Its over-4 KiB half was carried. This is the guard behind decision/shared-log-files-concurrent-append and decision/one-write-per-log-line, and item 11's rework of open_private_append must keep it green.

@history_item n=4 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (G2's seam exemption, the D-28 over-4 KiB half) — the over-4 KiB half of D-28 landed (8 concurrent forced panics, hook_panics_append_whole_lines_over_4_kib_side_by_side), as did the hook perf rows. Item 11 changes the diagnostics open on the hook's timed path, so these guards and the SPINE_DEADLINE rows must stay green.

@history_item n=5 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — every start refusal of run is two fixed stderr lines through src/human.rs refuse → write_refusal, in one write_all, with no print macro and no #[allow]. Item 1 adds hints in src/human/ (write_unable), so it must keep this one-writer, no-print-macro rule (rows decision/stderr-fixed-strings, contract/cli-stderr/refusal-pair).

@history_item n=6 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the uninstrumented hook --capture arm) — the hidden capture arm is uninstrumented by design: no obs init, no role or detail line. It is the "two --capture arms run ahead of the check" exception in decision/hook-instance-check-before-obs-init and decision/instance-check-before-log-init. Item 2's email check at the --record / capture write must not add obs lines there.

@history_item n=7 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — parse-rejected{parser:"ledger-stamps"} gained detail strict-modes-failed (run's version gate refusing the stamps file through read_stamps_strict, read as unverified). This is the origin of contract/run-log-parse-rejected/ledger-stamps and contract/parse-rejected-detail/strict-modes-failed. If item 6 makes check_stamps refuse a symlink, a symlinked stamps file lands on this line, and the closed detail words stay as they are.

@history_item n=8 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — E5: read_snapshot_or_replay writes state-recovered{snapshot-replayed} at WARN with no corr, one line per call, none for a present or absent snapshot and none on a failed replay. This explains contract/state-recovered/snapshot-replayed and contract/state-recovered-detail/snapshot-replayed. Item 12's replay guard reads this path.

@history_item n=9 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — read_snapshot_or_replay gained its first product caller (revive's start), so a product run writes state-recovered{snapshot-replayed}. This is the replay surface item 12's guard protects.

@history_item n=10 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — measured trap: the role-file open narrows an existing home to 0700, so any check made after it passes a widened home, and the instance check moved ahead of obs init. Item 11 narrows the open file, so the same order trap applies: no narrowing may run before or in place of the strict check (rows decision/diagnostics-checked-at-open, decision/strict-modes-before-diagnostics).

@history_item n=11 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (a refused instance check writes no line, on both arms) — revive's and the statusline arm's instance check moved ahead of any log, and a refusal writes no line. This is the start of the pre-log, no-line refusal family in decision/pre-log-strict-refusals-write-no-line and decision/strict-modes-refusal-writes-no-line, which item 6's symlink refusals join.

@history_item n=12 marker=2026-10-10-session-links
**2026-10-10-session-links** — the replay's one pass also derives the link set through apply_link_line, with no span and no line. A link's trace is the channel pair, the product line and the snapshot's links. This is the replay-of-links behaviour that item 12's guard e5_the_replay_reads_the_link_the_wrapper_wrote asserts.

@history_item n=13 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — board::read is the replay's second caller, and a scan skipping a symlinked or malformed entry writes one parse-rejected{state-entry, symlink-ignored|malformed, count}, naming no entry. This explains decision/symlink-skip-logged-as-count and decision/skipped-symlink-never-named-in-logs. Item 6 (trusted-state checks refusing a symlink) must not double this count or name a link target.

@history_item n=14 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — state-recovered gained scanned on the snapshot details (the instance name, never a path), and revive --list logs as cli. This explains contract/diag-line-fields/state-recovered and contract/cli-log-file/revive-list, which bind item 12's replay guard.

@history_item n=15 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the mcp sink is always mcp.ndjson) — viola mcp always writes diagnostics/mcp.ndjson once the home and diagnostics/ pass the check. This explains decision/mcp-log-sink and decision/mcp-log-file-sink. Item 11's diagnostics open is mcp's role-file open too, and item 10 names the MCP payload types that gain Redact.

@history_item n=16 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the server-identity negatives' no-number scan) — tests/security_negatives_server_identity.rs scans every output with digit_runs, which digit_runs_reads_whole_numbers_only tests. This is the first copy of the helper that item 13 consolidates into tests/support/ (rows decision/server-verify-refusal-names-nothing, decision/detail-codes-no-paths).

@history_item n=17 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (E5: the other snapshot readers, five to three) — the reader count follows the rule `read_snapshot(` under src/, measured at the wrap and never copied from a plan. This is a precedent for item 12: re-measure the replay guard census rather than carry it.

@history_item n=18 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link verbs' instance check before their log) — link/unlink run check_instance on the driven instance before client::start. A refusal writes no line, and link_verbs_refuse_a_widened_instance_before_any_log holds the order trap. This explains decision/link-verbs-log-to-driven-instance and contract/viola-link-unlink/strict-modes-failed. Item 6 changes check_instance itself.

@history_item n=19 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (E3: the explicit methods served; the link pair names in the PII table) — link appends one link line {driver, driven} and unlink one unlink line. Their trace is the channel pair, the product line and the snapshot's links, asserted in tests/cli_links.rs. That file is item 12's guard site.

@history_item n=20 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the diagnostics/ opens behind check_diagnostics) — the role and detail files open only after check_diagnostics (src/obs.rs, open_role_file and write_detail), which runs before ensure_private_dir narrows anything. A symlink is refused, and a refusal is exactly a failed open. This is item 11's direct precedent (decision/private-diagnostics-files, decision/content-sink-behind-strict-check, decision/refused-diagnostics-path-is-a-failed-open). The lstat → open → path-chmod race is the gap left there.

@history_item n=21 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (home-tree check of run and revive's start; verify's home-tree check and update_stamps) — checked_exe / check_home_tree run before the wrapper log, and check_stamps runs before create_private_dir(ledger/) and again under the lock. A refused update_stamps exits 1 through verify's own process-exit. Item 6's symlink refusal in check_stamps / check_path lands on these rows (contract/viola-verify-refusal/*, decision/home-tree-check-before-log-init).

@history_item n=22 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the pre-init window and panic exemptions gain the chunk's checks; the hook paths' instance check before obs init) — every hook arm except the two --capture arms checks the instance before read_diagnostics_level, and a refusal exits 0 silently. This explains decision/hook-refused-instance-silent, decision/hook-silent-without-trusted-instance and decision/instance-check-before-log-init. Item 6's stricter checks inherit the no-line shape.

@history_item n=23 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the snapshot readers over a replayed snapshot; E5's product readers two to six) — a replayed snapshot carries no endpoint, pid, started_at or statusline_command. Hook connect fails open as channel-unreachable, and run.collision_check reads the heartbeat alone. This explains contract/replayed-snapshot/live-fields, decision/replayed-snapshot-unreachable and decision/no-raw-snapshot-reads, the row family item 12's guard backs.

@history_item n=24 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the refusal-telemetry record; the six frame verbs through client::refuse_untrusted) — a strict-modes failure has a line only on the ledger-stamps read and verify's stamp write. Every other refused check writes none. tests/security_negatives_strict_modes.rs reads every output with digit_runs, the third copy item 13 consolidates. This explains decision/strict-modes-failure-lines.

@history_item n=25 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the MCP frame tools' instance check before the liveness read) — frame runs check_instance before live_endpoint. A refusal is instance-unreachable with no frame and no line. This explains contract/mcp-frame-tool-refusal/strict-modes-failed and contract/mcp-instance-unreachable/strict-modes-failed. Item 6's symlink refusal reaches MCP through the same check_instance.

@history_item n=26 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the nine thiserror derives) — the thiserror census (Refused, StampsError, StampsReadError …) and fixed, non-interpolating messages are guarded by thiserror_messages_interpolate_nothing. A new Refused variant for a symlink (item 6) must carry a fixed, pathless message and moves no derive count (rows decision/typed-errors-fixed-display, decision/error-messages-interpolate-nothing).

@history_item n=27 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the exit-2 paths of answer and send; the error-body census) — answer/send exit 2 writes the fixed UNUSABLE_RESPONSE / UNUSABLE_TEXT line with no document. tests/support/events.rs holds error_body_hit / assert_error_body_clean. Item 1 touches src/cmd/{send,answer}.rs refusals, and item 13 adds a shared helper in tests/support/ beside these.

@history_item n=28 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (a peer's reply screened by sanitised_fault and is_code_word) — an unknown code word stays visible and exits 14 (unknown). This explains decision/peer-replies-screened-as-code-words and contract/refusal-reason/unknown-fallback. Item 1's hint_cause maps Unknown to None today, so a hint for unknown must add no line or field.

@history_item n=29 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document names its cause; MCP's body names its cause; verify writes its refusal's code) — refused instance checks read detail strict-modes-failed under --json, still with no line. A refused update_stamps logs process-exit strict-modes-failed. This explains contract/exit-21-document/detail, contract/cli-json-error/strict-modes-failed and contract/process-exit-detail/verify-refusal. Item 6's symlink refusals map onto these existing codes, with no new code (Boundaries).

@history_item n=30 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's logged causes and their during) — without_endpoint reads a no-endpoint instance by symlink_metadata (a directory is instance-dead; absent, a file or a symlink is unwrapped-name), and the instance check stays first. This explains contract/no-endpoint-instance/classification and is a precedent for the symlink-aware stat that item 6 extends to check_instance and its siblings.

@history_item n=31 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (Refused::NotADirectory's fixed message) — the new Refused variant reads the fixed "a trusted state path is not a directory", names no path and adds no derive. This explains decision/not-a-directory-refusal-message-fixed and is the template for any symlink refusal variant item 6 adds to crates/viola-state/src/strict.rs.

@history_item n=32 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory is no strict-modes failure; the OS call sites) — check_socket_dir (viola's one symlink_metadata) takes the strict read verdict but writes under socket-dir-refused. This explains contract/process-exit-detail/socket-dir-refused and decision/socket-dir-check-changes-no-mode. Item 6 makes other checks symlink-aware: the socket-dir code family must not shift.

@history_item n=33 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory refusal's PII reading) — tests/security_negatives_socket_dir.rs reads outputs with assert_names_nothing and digit_runs (the second copy and its second digit_runs_reads_whole_numbers_only test). Item 13 folds these into one shared copy.

@history_item n=34 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory refusal's human pair; run's start refusals and the human::refuse census) — the pair is plain ASCII with hint: last and no SGR, naming only the parsed ViolaName, and the census rule is `human::refuse(` under src/. Item 1 adds hints, and item 5 edits src/cmd/run.rs: re-derive the census if call sites move (contract/refusal-stderr-pair/socket-dir-refused).

@history_item n=35 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows home owner/DACL strict-modes arm retired) — check_diagnostics holds folder and file to the read rule (no group or other bit), and init step 4 checks owner, mode and not-a-symlink by lstat. strict.rs keeps only the Unix arm. This is item 6's and item 11's ground: Linux-only, so no OS split is needed (decision/diagnostics-owner-only-permissions, decision/linux-only-os-target).

@history_item n=36 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS target, Linux only; the refusal writer's site counts) — append atomicity is POSIX O_APPEND alone, and the concurrent-append check runs on ubuntu-latest. The human::refuse census reads run 7 and verify 5. This explains decision/log-append-atomicity. Item 11's fd-based narrowing must keep the append-mode single-write line.

@history_item n=37 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the closed words that left diag-line.v1.json with their emitter) — a closed schema word no build can emit leaves with its emitter, and os is closed to linux. This explains decision/unemittable-schema-words-removed and contract/diag-line-schema/os. The chunk adds no word (Boundaries).

@history_item n=38 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the thiserror derive census on the product sources) — the census rule is a grep of thiserror::Error over src and crates outside crates/viola-e2e, reading 9. A new strict refusal type (item 6) would move it, but a variant would not.

@history_item n=39 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (1077), each verified against the code. The picked rows, including decision/payload-types-derive-redact and decision/private-diagnostics-files with their divergences "as written, owed by the route", come from this migration. That landing's route-owed lines are this chunk's items 1–12.

@history_item n=40 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (8. PII Scrubbing & Compliance: 2 rows) — ~ decision/secret-scan-before-upload and ~ decision/secrets-never-logged: the cookie class was retired, and an unreadable-file class gained a planted case. This is the latest form precedent (`~ handle · rule: was / now`) for row amendments in §8. That chunk also left the duplicated digit_runs that item 13 closes.

@history_item n=41 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (1. Obs Scope Summary: 1 row) — ~ decision/no-secrets-in-log-lines drops its Cookie clause. This is an adjacent §1 row precedent for the same cleanup.

@history_item n=42 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (3. Observability Harness Contract: 1 row) — ~ decision/content-only-in-instance-detail-file loses "cookie". This row sits beside the picked decision/codes-only-process-logs and decision/content-only-in-detail-file that item 11's detail-file open serves.

@history_item n=43 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (8. PII Scrubbing & Compliance: 1 row) — − decision/cookie-header-never-logged → none. This is the removal form precedent in §8, where item 10's decision/payload-types-derive-redact sits.

@history_item n=44 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (11. Obs Anti-Patterns: rule and name) — ~ decision/claude-env-and-cookie-never-logged changes its rule and then its name to follow it. This is the §11 precedent beside the picked decision/veil-toggle-banned and decision/scrub-before-bytes-leave-process.
