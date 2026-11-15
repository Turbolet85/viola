@history_item n=1 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** (schema default-deny keyword) — set default-deny in `schemas/diag-line.v1.json` as a top-level `unevaluatedProperties: false` over per-event field lists (row decision/schema-top-level-unevaluated-properties). Why it matters: this chunk removes the `statusline-shell` subject (and the shell_exit_status field if nothing else emits it) from that schema. The per-event lists and the enum have to be cut together, or a line that stays will fail the schema (contract_diag_schema).

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the uninstrumented `hook --capture` arm) — made the hidden capture arm uninstrumented by design (no VIOLA_*, no obs init, no lines; §10 bounded exemption 5, restated in §2), ratified by the founder. Why it matters: this is the origin of rows decision/hidden-capture-arms-uninstrumented and decision/hook-capture-arms-unlogged. The `hook <event> --capture` arm stays; only the statusline capture arm leaves. Exemption 5 and §2 have to go back to naming one capture arm.

@history_item n=3 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — added `verify-probe` to the closed `subject` enum, which already held `statusline-shell` in the spawn set (`version-probe|verify-probe|agents-probe|statusline-shell`). It also set that CI verify runs only against the fake agent and the real-CLI verify runs only locally (rows decision/ci-verify-fake-agent-only, decision/real-cli-verify-never-in-ci). Why it matters: removing `statusline-shell` reverses this subject-set entry. CI keeps proving the removal on the fake agent alone.

@history_item n=4 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — kept the capture arm uninstrumented but let `--answers` print one product-built decision body on a dialog event (row contract/hook-stdout/capture-arms, «except --answers' probe decision body»). Why it matters: the `hook <event> --capture` and `--answers` forms stay in this chunk (scope «What stays»). The capture-arms row must lose its «two hidden --capture arms» count but keep the --answers exception.

@history_item n=5 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — the origin of the arm this chunk removes. It added the statusline arm's log lines: parse-rejected hook-stdin, the `statusline-shell` process-start/exit pair, hook-decision with deadline_hit, and `statusline.shell_out` closing at `STATUSLINE_DEADLINE` (5 s, provisional). It also made the instance check run before obs init and declared the start's statusline source read and settings override write unspanned by design. It set hook stdout to carry the user's status-line output and excluded the arm from the `max < 1.0 s` budget and `run --perf`, and added the statusline reader to E5. Why it matters: these became the rows contract/statusline-log-fields/*, contract/statusline-decision-detail/*, decision/statusline-deadline, decision/statusline-command-deadline, decision/statusline-settings-steps-unlogged, decision/hook-latency-budget's except clause, contract/hook-stdout/statusline and contract/stdout-by-role/hook. All of them leave or get rewritten here.

@history_item n=6 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (a refused instance check writes no line, on both arms) — set `viola hook statusline` to write only three fail-open codes (oversize-stdin, malformed-json, deadline). A refused check now opens no role file and writes no line. It also added §10 exemption 7 for a refused check on either arm. Why it matters: these are the source of rows contract/hook-decision-detail/statusline-codes and decision/statusline-silent-without-instance. Exemption 7 and the Vector 4/7 triggers name the statusline arm and must drop it.

@history_item n=7 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — made the statusline arm's no-line refused check the founder's own answer, no longer the operator's (§1, §4, §6, §10 exemption 7). Why it matters: removing the arm removes one of the founder-ruled no-line refusal sites. The Vector 7 list of strict-modes refusals that leave no line («`hook statusline`'s check») must drop it, and the founder's ruling must not be restated as if it still applied.

@history_item n=8 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — added the second hidden capture arm `hook statusline --capture <DIR> [--shell <WORD>]` (sole caller: Run B's probe settings), uninstrumented by design and filing `StatusLine.<k>.json` with a `bin-sh`/`other` verdict. It widened exemption 5 and the §2 exemption list to «both capture arms» and set verify's `[NN/MM]` step lines per relied row. Run B's verify-pty-probe pair carries one probe `--settings`. Why it matters: this is the exact entry this chunk reverses. The capture arm, the probe `--settings` and the statusline ledger rows leave, exemption 5 and §2 go back to one arm, and MM drops to 21 (rows contract/verify-stdout/step-line, contract/verify-stdout/summary-line).

@history_item n=9 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (the reserved web vocabulary leaves) — removed a whole surface's vocabulary (process role, events, OFF targets, schema words) together with its code, and recounted §1. Why it matters: it is the precedent for a whole-surface removal from obs-plan. The `statusline` span area, the `statusline-shell` subject, the `contract-hook-statusline` tag and the statusline hook_event leave the same way, with their counts restated.

@history_item n=10 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — moved verify to eight child spawn pairs (Run B's agents read reuses `agents-probe`) and set MM to 23 on Unix, with the `subject` enum unchanged. Why it matters: row contract/verify-process-exit/child-exit-status-and-duration counts eight verify pairs. Run B keeps its agents read and only its status-line probe `--settings` leaves (scope «What stays»). P3 must check that the pair count holds while MM moves.

@history_item n=11 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — raised verify's MM to 24 on Unix with the `local-command-compact` row and added Run B's sixth paste with no new spawn pair or subject. Why it matters: this is the MM count history that this chunk takes from 23 (after the budget removal) to 21. The amended §4 `verify` edge flow and the step-line rows have to state the new relied count.

@history_item n=12 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — made the four no-line strict-modes refusals the founder's ruling as built, `viola hook statusline`'s check among them. It also cleared the provisional marks on the statusline capture arm's owed-rows entry. Why it matters: once the arm is gone, the §1/§6/§10 lists of founder-ratified no-line refusals lose the statusline site. The ratification of the remaining sites stands.

@history_item n=13 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (E5: the other snapshot readers, five to three) — set the E5 snapshot-reader count by the rule `read_snapshot(` under `src/`, and listed `src/cmd/hook/statusline.rs` as a reader. It also set the trap that a count carried from a plan is re-measured at the wrap, never copied. Why it matters: removing `src/cmd/hook/statusline.rs` takes a reader out of E5, and the count must be re-measured.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the snapshot readers over a replayed snapshot) — stated that a replayed snapshot carries no `endpoint`, `pid`, `started_at` or `statusline_command`, so `hook statusline` runs nothing and prints nothing over it (founder's answer to P4 fork Q1). Why it matters: this is the source of rows contract/replayed-snapshot/live-fields and decision/no-endpoint-over-replayed-snapshot («the statusline runs no command»). Both name `statusline_command` and the statusline arm, which leave with the snapshot field.

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (E5: the replay's product readers two to six) — listed six product readers of `read_snapshot_or_replay`, statusline `serve` among them, each writing one `state-recovered` line per call. Why it matters: rows contract/state-recovered/snapshot-replayed and contract/state-recovered-detail/snapshot-replayed rest on this list. With statusline `serve` gone, E5's reader count drops and must be re-measured.

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook event and dialog paths' instance check before obs init) — extended the instance check to every `viola hook` arm except the two `--capture` arms. Before this, `hook statusline` was the one arm with a check. Why it matters: rows decision/hook-instance-check-before-obs-init and decision/instance-check-before-log-init name «the two --capture arms» as the exception. With the statusline capture arm gone, one remains.

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`socket-dir-refused` in the log catalogue) — raised the hook-decision fail-open codes from 7 to 8, with `viola hook statusline` writing «three of the eight, the same three». Why it matters: this is the current wording of rows contract/hook-decision-detail/statusline-codes and contract/hook-decision-detail/fail-open-codes. `deadline` is written only by the statusline arm, so P3 has to check whether that code still has an emitter.

@history_item n=18 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows host arms of the panic capture, the env strip, verify and the statusline retired) — set the relied count to all 24 rows of `ALL` after folding `statusline::WRAPPED_HERE`, dropped the Unix-only qualifiers on the probe `--settings`, the statusline capture arm and the settings override write, and made `shell_argv` return `[&str; 3]`. The statusline-shell pair now needs only a recorded `statusline_command`. Why it matters: it is the last amendment to the surfaces that leave here (`shell_argv`, the probe settings, the settings override and the statusline-shell pair).

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the closed words that left `diag-line.v1.json` with their emitter) — added the rule that a closed word no build can emit leaves the schema with its emitter (row decision/unemittable-schema-words-removed), and recounted the enum by its members. Why it matters: `statusline-shell` (subject), the `statusline` hook_event and any detail only the arm writes (`deadline`) must leave `schemas/diag-line.v1.json` in this chunk. A row the store refuses to edit becomes a CARRY, as this entry's process-start row did.

@history_item n=20 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0) — did not apply census C-301 (the §1 instrumentability rule for a user-written shell command, the status line's `/bin/sh -c` shell-out) and deferred it explicitly to the route entry «Status-line interception removed». Why it matters: this chunk is that entry. C-301 and its rows decision/statusline-command-duration-exit-only and decision/statusline-shell-out-instrumentation are owed here.

@history_item n=21 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (6. Log Coverage) — added `hyprctl-probe` to the subject list of contract/diag-line-fields/process-start, next to `statusline-shell`. Why it matters: it is the latest edit of the row whose `statusline-shell` subject leaves here. The whole-row edit must keep `hyprctl-probe`, and contract/diag-subject/statusline-shell is its sibling row that goes.

@history_item n=22 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (2. Telemetry Strategy) — added `hyprland` to decision/span-naming's closed area list, which still names `statusline`. Why it matters: the `statusline` area (C-309) leaves that list in this chunk while `hyprland` stays.

@history_item n=23 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4. Span / Trace Coverage, 1 of 2) — added contract/statusline-spans/span-chain (hook.statusline › statusline.shell_out) as the successor of the retired budget spans, and dropped `budget_written` from contract/statusline-log-fields/hook-decision. Why it matters: the row this created leaves with the arm. The budget chunk is also the immediate precedent for removing statusline-adjacent rows by `−` with a successor or `→ none`.

@history_item n=24 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Obs Scope Summary: decision/statusline-command-confined) — rewrote the rule to «The statusline_command string never appears outside diagnostics/», on the grounds that the status-line pass-through stays. Why it matters: that premise ends with this chunk. This row and its siblings (statusline-command-string-diagnostics-only, statusline-command-never-in-home-logs, statusline-command-only-in-diagnostics, statusline-command-out-of-home-logs, statusline-command-never-logged, no-statusline-settings-in-telemetry) lose their subject.

@history_item n=25 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (decision/diag-schema-additive-only, decision/log-fields-additive-only, decision/log-schema-additive-only, decision/obs-aligns-to-tests-log-format) — narrowed the never-remove rules to «a field the product still emits» on the operator's Q2 answer «Сузить правило». Why it matters: these narrowed rules allow dropping `statusline-shell`, `shell_exit_status` and the statusline hook_event without a new exception. The own-format `v` exception from that chunk was the founder's ruling for the budget removal only (directive item 7), so its reuse here is his fork.

@history_item n=26 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (6. Log Coverage) — dropped `budget_written` from contract/diag-line-fields/hook-events, keeping «one of ten closed hook names incl. statusline». Why it matters: this row must lose the statusline hook name, nine names after this chunk. The same row-edit pattern applies to contract/diag-line-fields/process-exit's `shell_exit_status`.
