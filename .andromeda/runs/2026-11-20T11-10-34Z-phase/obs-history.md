@history_item n=1 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — fixed default-deny in `diag-line.v1.json` as a top-level `unevaluatedProperties: false`, and made `corr` a field the caller supplies. Why it matters: any new field on the PermissionRequest-question answer path (for example a decision-kind word) is a build failure unless the schema and the obs catalog name it (decision/schema-top-level-unevaluated-properties, decision/default-deny-log-fields).

@history_item n=2 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — made `corr` required on `dialog-raised`/`dialog-answered`, and on `hook-decision` for `pre-tool-use`/`permission-request` without `detail`. A dialog `hook-invoked` and a `hook-decision` that carries `detail` may be null. Why it matters: the new `allow` + `updatedInput` decision for a permission-request question must still carry `corr`=dialog_id on the hook-decision (contract/process-log-corr/hook-decision, contract/corr-by-event/hook).

@history_item n=3 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (G2's seam exemption, perf rows built) — G2's `src/cmd/hook/seam.rs` panic exemption, and the hook perf rows judged by `gate --require perf`. Why it matters: a heavier decision body on the PermissionRequest path stays inside the timed hook rows under SPINE_DEADLINE (decision/hook-perf-budgets-timed, decision/spine-hook-perf-gate).

@history_item n=4 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (verify's stdout, the uninstrumented `hook --capture` arm) — verify's outcome is exit 0/1 plus one stdout step line per row and the `stamped <version>  N pass  N fail` summary. The hidden `hook --capture` arm is uninstrumented by founder ratification. Why it matters: the new ledger row's verify probe uses this stdout shape and the capture arm; it must add no obs init or hook lines (contract/verify-stdout/*, decision/hidden-capture-arms-uninstrumented).

@history_item n=5 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — added `verify-probe` to the closed `subject` enum, logged verify's spawns at the call site, and ran CI verify only against the fake agent. Why it matters: the new probe must reuse an existing subject or take a Decisions Log entry for a new closed value. Its CI form drives the fake agent's PermissionRequest-question script (decision/ci-verify-fake-agent-only, contract/verify-spawn-subject/*).

@history_item n=6 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** (Decisions Log migration) — moved D-07 into §4's dialog scenario: a dialog hook's `invoked_at` is its true start, and pre-`dialog_id` paths write `hook-invoked{corr:null}` at once. Why it matters: the PermissionRequest question path keeps this hook-invoked-after-response ordering (decision/dialog-hook-invoked-after-response).

@history_item n=7 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — added `strict-modes-failed` to `parse-rejected{ledger-stamps}`, which reads as unverified. Perf grew to five rows with `pre-tool-use` timed. Why it matters: the dialog decision stays gated on the verify stamp, and an unreadable or refused stamps file means `cli_verified:false` (contract/run-log-parse-rejected/ledger-stamps, decision/unreadable-ledger-means-unverified).

@history_item n=8 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added verify's interactive PTY runs, each logged as one `verify-pty-probe` pair with no row text, prompt or path. Why it matters: if the new probe adds an interactive run, it is one more `verify-pty-probe` pair (contract/verify-spawn-subject/verify-pty-probe).

@history_item n=9 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — added verify Runs C and D (the dialog rows) and the capture arm's `--answers` mode, which prints one product-built decision body to stdout for a dialog event. Direct precedent: the new PermissionRequest-question probe extends this answers arm and its decision body. The arm stays uninstrumented (contract/hook-stdout/capture-arms).

@history_item n=10 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — CI's fake-agent verify flag list gained `--framing`, bound to test-plan §3's boot step 4. Precedent: if the new probe needs a fake-agent switch, the obs CI bullet and boot step 4 move together.

@history_item n=11 marker=2026-10-10-session-links
**2026-10-10-session-links** — a taken `answer` whose `from` names another instance records the implicit link before `dialog-answered`. A failed record is `-32603`: the answer is not taken, the dialog stays held and no `dialog-answered` line is written. Why it matters: the answers-carrying answer keeps this ordering (decision/implicit-link-on-send-and-answer, contract/answer-link-record/failure).

@history_item n=12 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a stopped verify starts no further spawn, closes every pair it opened and ends `process-exit{detail:"interrupted"}`. Why it matters: a new probe spawn has to honour the stop rule (contract/verify-process-exit/interrupted).

@history_item n=13 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify prints one `[NN/MM]` step line per relied row, MM being the relied count, with the summary summing to it. Why it matters: the new relied row moves MM and the counts (contract/verify-stdout/step-line, summary-line).

@history_item n=14 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E raised the spawn pairs to seven and moved MM. A killed PTY child still carries `child_exit_status`. Precedent for a new ledger row adding a PTY run: the obs counts move, and the subject enum stays unchanged.

@history_item n=15 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify's eighth spawn pair (`agents-probe` reused) moved MM to 23, and the closed subject enum was unchanged. Precedent: a probe reuses an existing subject instead of adding one.

@history_item n=16 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the 24th row (`local-command-compact`) moved MM, and Run B's sixth paste added no spawn pair or subject. Precedent: a probe folded into an existing run adds no pair. Only MM moves.

@history_item n=17 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — expiry belongs to the wrapper: one `dialog-expired` product line plus `dialog-answered{deadline_hit:true}` with no `from`. The hook, given `null`, writes `hook-decision{decision_emitted:false}`, and its own `deadline_hit:true` is only the 3605 s backstop. Why it matters: the scope keeps the expired-hold fallback to the human (contract/events-log/dialog-expired, contract/hook-decision/deadline-hit).

@history_item n=18 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — `dialog-answered`'s `from`/`from_trust` appear only on an answer that carried a `from` (measured live). Why it matters: the answers-carrying answer path keeps this field rule (contract/dialog-log-fields/dialog-answered).

@history_item n=19 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — placed the pre-frame server check inside `hook.handle` (before `hook.dialog`) and `answer.client`. A failed check on the dialog path writes `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`, and MCP `answer` writes no line. Why it matters: the PermissionRequest question rides this dialog hook path (contract/hook-decision/server-verify-failed, decision/server-refusal-inside-root-span).

@history_item n=20 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — every hook arm but `--capture` runs its instance check before obs init, and a refused check exits 0 silently. `update_stamps` refuses before writing, and verify's refused stamp write is its own `process-exit` `strict-modes-failed`. Also set: the refusal-telemetry record and the hook's instance check inside the timed path. Why it matters: these bind the dialog hook and the re-stamp (decision/hook-refused-instance-silent, contract/viola-verify-refusal/*, decision/strict-modes-failure-lines).

@history_item n=21 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the mcp list tool's lines, invalid-params) — the codes-only set gained `invalid-params`, whose `detail` is a closed field word. Why it matters: if the MCP `answer` shape changes to carry answers, a bad field must still read a closed word such as `response` (contract/mcp-invalid-params/detail).

@history_item n=22 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (D-05 `mcp-stdin`) — added the `mcp-stdin` parser to the per-parser parse-failure counts. A new closed `parser` value takes a Decisions Log record. Why it matters: an answers payload on the MCP or hook stdin stays bounded at MAX_FRAME (decision/per-parser-parse-failure-counts).

@history_item n=23 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `channel-response` `detail` is closed to three `-32602` words, and the answer's response line carries no `detail` (`unknown-dialog` belongs to the refusal). Answer and send exit-2 paths write the fixed unusable line with no document. Clients screen peer replies with `sanitised_fault`/`is_code_word`. `-32603` is proved at the unit layer. Why it matters: these bind any change to the answer shape (contract/channel-response/detail, contract/answer-send-exit-2/stderr-line, decision/clients-screen-peer-replies-to-code-words).

@history_item n=24 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — exit 21's document names its `UnreachableCause`, with `strict-modes-failed` on a refused check. verify writes its refusal code on `process-exit`, and a refused `update_stamps` carries `strict-modes-failed`. Founder Direction 4 states the distinct-code rule and its carve-outs. Why it matters: these bind the CLI `answer` and the re-stamp (contract/cli-json-error/strict-modes-failed, contract/process-exit-detail/verify-refusal, decision/distinct-detail-code-per-exit-cause).

@history_item n=25 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — a refused socket directory at `viola hook` writes one `hook-decision{detail:"socket-dir-refused"}` at WARN. On the dialog path it follows `hook-invoked{corr:null}` with `decision_emitted:false`. The fail-open codes became eight, and CLI `answer` exits 21 `socket-dir-refused` with `during:"connect"`. Why it matters: the dialog hook's fail-open set is unchanged by an answer body (contract/hook-decision-detail/socket-dir-refused, fail-open-codes).

@history_item n=26 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (Windows host arms retired) — the relied count reads all rows of `ALL` on the one Linux target (was 24 Unix / 21 Windows). The strict-modes failure line sits in two places (the ledger-stamps read and verify's refused stamp write). Why it matters: MM is the whole relied set, and the new row adds one on every host (decision/strict-modes-failure-lines).

@history_item n=27 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows, each verified against the code. Why it matters: every row picked for this chunk derives from this landing, and amendments since then name rows by handle (`+`/`~`/`−`).

@history_item n=28 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** — dropped the Cookie clauses from decision/no-secrets-in-log-lines and decision/content-only-in-instance-detail-file. Why it matters: these picked rows now name only CLAUDE* values and user content. Answer text stays out of home-level files.

@history_item n=29 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `decision/payload-types-derive-redact` and `decision/veil-redact-payload-types`: a payload type that carries content derives veil Redact, and one with no content field derives none (veil 0.3.0 refuses). Why it matters: an answers-carrying decision body or `updatedInput` type is content, so it must derive Redact.

@history_item n=30 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — `~ decision/os-apis-at-call-sites`, `~ decision/parse-failures-unknown-counter` / `decision/parse-failures-surface-as-unknown` (except: hyprctl), and `~ decision/per-parser-parse-failure-counts`. Why it matters: these picked rows' current wording and exceptions come from this entry.

@history_item n=31 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/diag-schema-additive-only`, `~ decision/log-fields-additive-only` and `~ decision/obs-aligns-to-tests-log-format`, narrowed to "a field the product still emits" by the operator's Q2. Also `~ decision/exit-code-log-field`: 11 retired. Why it matters: these are the rules a new dialog field or word must satisfy.

@history_item n=32 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — the hook keeps one capture arm: `~ decision/hidden-capture-arms-uninstrumented`, `~ decision/hook-capture-arms-unlogged`, `~ contract/hook-stdout/capture-arms`, `~ decision/hook-instance-check-before-obs-init` and `~ decision/instance-check-before-log-init`. Also `~ decision/hook-stdout-decision-body-only` and hook_event nine. Why it matters: the new probe's capture runs through that one `hook <event> --capture` arm, whose stdout is only the decision body.

@history_item n=33 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — hook_event rose to ten (stop-failure), with the StopFailure "no ledger row" answer. The ratification is 2026-11-16T06-05-33-wrap. Contrast precedent: StopFailure stands with no ledger row and no verify probe, whereas this chunk adds a relied row with its probe (contract/diag-line-fields/hook-events).

@history_item n=34 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (the StopFailure no-ledger-row answer ratified) — the founder ratified that StopFailure stands registered with no ledger row and no verify probe. Why it matters: it is the ruling this chunk's new relied row is set against; whether a hook behaviour earns a ledger row is the founder's word.

@history_item n=35 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `~ decision/claude-internals-not-a-contract` (except: transcript echo), `~ decision/refusal-telemetry` (except) and `~ decision/deterministic-event-emission`. Founder ratification came at 2026-11-17T05-21-35Z-wrap. Why it matters: these picked rows' exceptions are closed lists. Reading the CLI's question-tool input shape to build `updatedInput` must not become a new claude-internals read unless an exception names it.

@history_item n=36 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — `~ decision/parse-failures-unknown-counter` / `decision/parse-failures-surface-as-unknown` (except: an outside StopFailure word reads api-error), plus `+ contract/parse-rejected/hook-stdin-unknown-value`. Why it matters: these give the current wording of picked rows, and set a precedent for an unknown hook-stdin value on the PermissionRequest path.

@history_item n=37 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — the latest ledger-row precedent. verify went to six PTY runs (A–F) and nine spawn pairs (`~ contract/verify-spawn-subject/verify-pty-probe`, `~ contract/diag-subject/verify-pty-probe`, `~ contract/verify-process-exit/child-exit-status-and-duration`). It also set `~ decision/claude-internals-not-a-contract`, `~ decision/refusal-telemetry`, `~ decision/r8-strip-logged-as-names-only` and `~ decision/env-vars-not-configuration`, and added `+ decision/trust-record-write-uninstrumented`. Why it matters: a new probe run moves these counts in the same way.

@history_item n=38 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ decision/r8-strip-logged-as-names-only`, `~ decision/claude-boundary-only-instrumentation` (the start's one trust-record line), `~ decision/env-vars-not-configuration`, and `~ decision/claude-internals-not-a-contract` / `decision/refusal-telemetry` (except). Why it matters: these give the current wording of picked rows that close the claude boundary list.

@history_item n=39 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/verify-stdout/step-line` (`pass|absent|fail`), `~ contract/verify-stdout/summary-line` and `~ contract/verify-exit-codes/0` (an absent browser-tools row counts). Why it matters: the new relied row's step line and the stamp's "every relied row counts" gate follow this shape.

@history_item n=40 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — `~ decision/browser-tools-prompt-reading-uninstrumented`: verify's fixed stdout line is the prompt's only record. Why it matters: this is the re-stamp chunk whose carry (cry-3qv2r3iy) this chunk holds. Re-stamping the CLI versions against the grown relied set follows it.

@history_item n=41 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — `+ decision/wheel-typed-mark-uninstrumented`: the wheel's typed mark and held wakes are in-memory state with no instrumentation. Why it matters: "the human always wins" (the human answers the dialog at the keyboard) touches the wheel, and typed keys stay unlogged (decision/typed-keys-never-logged).
