@task_prose part=session
Epoch 5 cleanup: the code audit's ranked defects mended, and the root crate's mutation baseline made green under the mutants profile; the three file splits were not built and leave for their own entry.
The verify class's per-test kill filter, identical in the ci and mutants nextest profiles, now takes six more tests by name beside its two terms (the verify_window_ prefix and the two drift replay tests): path4_a_stamp_failing_the_dialog_rows_leaves_dialogs_to_the_human and five cli_version_gate run_* tests; kill bounds unchanged (class 20 s x 4, mutants default 5 s x 3).
Measured: the root crate's in-diff mutation run read 372 mutants, baseline Success, 326 caught, 0 missed, 0 timeout, 46 unviable, score 100.0 (was baseline-test-failure), landed as an addendum to the code audit's record, the sealed audit run and ledger untouched.
Added: five pump cases killing the respawn-path survivors (the drain bound now injected through the pump's seams, the product drain still 500 ms), five viola-state cases (the ancestors parent-chain walk split into a pure walk with a pid-1 stop, the instance scan order, ended-after-last-start), and an R2 live-script precondition refusal on a locked desktop or a non-fresh scratch.
Retired: the duplicate trust check and the duplicate start prologue of revive and adopt (one trusted and one start_prologue in the shared run start), the duplicate refusal writer of viola up (one refused, in viola open), and the error-body table's viola up needle (its four rows re-pointed to viola open's).
@task_file n=1 path=.config/nextest.toml
what: the nextest profiles' per-test overrides — the verify class's kill-line filterset in the ci and mutants profiles, and the mutants profile's default kill
@task_file n=2 path=tests/contract_lints.rs
what: the root contract-lint test binary, holding the pin on the verify class's filter line
@task_file n=3 path=crates/viola-pty/src/pump.rs
what: the PTY pump — copies the child's output to the host output, drains in-flight output at the child's exit within a bound, respawns the child; its seams and inline unit cases
@task_file n=4 path=crates/viola-state/src/liveness.rs
what: instance liveness — heartbeat staleness and the parent-chain ancestors walk (pid-1 stop, at most max pids)
@task_file n=5 path=crates/viola-state/src/board.rs
what: the board — the instance scan listing checked instances in name order (inline unit cases only changed)
@task_file n=6 path=crates/viola-state/src/replay.rs
what: snapshot replay from the event log — read-or-replay and the ended-after-last-start reading (inline unit cases only changed)
@task_file n=7 path=scripts/r2-open.sh
what: the operator-fired R2 live reading script for real window opening and placement, its verdict lines and exit-trap cleanup
@task_file n=8 path=src/cmd/run.rs
what: viola run start, the start shared by run, revive and adopt — trust check, instance check, pinned-exe check with its strict-modes refusal, then the wrapper log open
@task_file n=9 path=src/cmd/revive.rs
what: the viola revive verb — preflight, --list, and its start through the shared start prologue
@task_file n=10 path=src/cmd/adopt.rs
what: the viola adopt verb — its start through the shared start prologue
@task_file n=11 path=src/cmd/open.rs
what: the viola open verb and its refusal writer, now the one refusal writer viola up also uses
@task_file n=12 path=src/cmd/up.rs
what: the viola up verb — fleet bring-up, its refusals written by viola open's refusal writer
@task_file n=13 path=tests/contract_error_bodies.rs
what: the error-body surface witnesses — the literal contract table of each surface's error bodies, codes, exits and needles
@task_item n=1 part=touches
decision/verify-window-80s-kill is stale: the verify class it bounds now holds, beside the verify_window_ prefix and the two drift replay tests, six tests taken by name (the path4_ dialog-rows test and five cli_version_gate run_* tests).
@task_item n=2 part=touches
decision/verify-driven-tests-20s-kill is stale on the same membership: the class at 20 s x 4 in both the ci and mutants profiles now names the six tests; its bound and retries unchanged.
@task_item n=3 part=touches
decision/mutants-kill-below-cargo-mutants-timeout and decision/mutants-fail-fast-wait rule over the mutants profile overrides the session edited; their values (class 20 s x 4, default 5 s x 3) unchanged — check only.
@task_item n=4 part=touches
decision/test-layer-filtersets and decision/nextest-is-the-runner rule over the nextest filterset file the session edited (one filter line grew, in two profiles).
@task_item n=5 part=touches
Mutation rows on the root crate: decision/mutation-gate-at-epoch-boundary, decision/mutation-gate-thresholds, decision/mutation-run-verdict, decision/mutation-verdict-by-counts and contract/mutants-verdict/unviable-exceeds-caught — the root reading moved from baseline-test-failure to 100.0 (326 caught, 0 missed, 46 unviable, unviable not above caught); where a row holds a threshold or a reading, name it.
@task_item n=6 part=touches
decision/equivalent-survivors-known-limit and decision/peer-admitted-survivor-known-limit: the session killed the pump's respawn-path survivors and viola-state's three survivors (liveness pid-1 stop, instance scan list, ended-after-last-start kind); check no named known-limit survivor is among them.
@task_item n=7 part=touches
decision/window-placement-proven-live and decision/window-reopen-proven-live, decision/standing-live-rig and decision/live-rig-clean-start rule over the R2 live script, which now refuses before anything is made or started on a locked desktop or a scratch that is not fresh, with a verdict line r2: red precondition=scratch or locked.
@task_item n=8 part=touches
decision/error-bodies-held-by-surface-witnesses, decision/error-writers-each-named and decision/contract-tables-are-literal: the viola up refusal writer is retired into viola open's one writer, and the error-body table's four viola up rows re-point to viola open's needle; codes, exits and witnesses unchanged.
@task_item n=9 part=touches
decision/strict-modes-before-any-log, decision/run-checks-before-child-spawn and decision/run-start-verify-and-trust-after-bind rule over the start of run, revive and adopt, now one start prologue keeping the order trust check, instance check, pinned-exe check (strict-modes refusal under --json), then the wrapper log open.
@task_item n=10 part=touches
decision/revive-preflight-order and the revive and start refusal contracts (contract/revive-refusal-code/*, contract/start-exit-1/*, contract/run-refusal/*, contract/strict-modes-refusal/revive and /run) rule over the revive and adopt starts, now through the shared prologue; codes and order unchanged — check only.
@task_item n=11 part=touches
contract/viola-run-start/home-tree-check, contract/viola-run-start/tampered-pinned-exe and contract/viola-run/start-refusal-exit-1 rule over the pinned-exe check and its refusal, moved into the shared start prologue unchanged.
@task_item n=12 part=touches
decision/child-exit-read-on-the-process-handle, decision/pty-exit-from-wait-not-eof, decision/exit-detected-on-handle and decision/outer-pty-reader-drained rule over the pump's exit and drain path, whose drain bound now comes through the pump's seams; the product's drain stays 500 ms.
@task_item n=13 part=touches
decision/mocks-only-at-product-seams, decision/time-faked-through-seams and decision/wrapper-deadline-injected-clock: the pump's seams gained a drain bound the unit cases inject; check the seam rules cover it.
@task_item n=14 part=touches
decision/heartbeat-staleness-pure-classifier, decision/heartbeat-staleness-pure-function, decision/pure-functions-tested-directly and contract/instance-liveness/*: liveness now holds a pure parent-chain walk (pid-1 stop, at most max pids) tested directly over an injected parent table; the ancestors signature and its 8-pid limit at the callers unchanged.
@task_item n=15 part=touches
decision/snapshot-read-through-replay, decision/snapshot-replay-from-log and decision/board-on-the-read-rule rule over the board's instance scan and replay readings that gained unit cases; no behaviour changed — check only.
@task_item n=16 part=touches
decision/unit-tests-inline and decision/rstest-case-tables: the session added inline unit cases in the pump, liveness, board and replay modules, with new private test helpers in the pump module.
@task_item n=17 part=touches
decision/no-manual-smoke-step and contract/test-harness/boot, /status, /cleanup: the boot-path change was smoked by the harness boot, status and cleanup verbs (green, ready, cleaned); no harness shape changed — check only.
@task_fact n=1 slug=e2e-test-strategy
fact: The R2 live reading refuses before anything is made or started, printing r2: red precondition=scratch when its scratch home or its runtime-dir entry already exists, or r2: red precondition=locked when the desktop's locked reading is not exactly false, and exits 1 closing no window.
@task_fact n=2 slug=3-test-harness-contract
fact: The product's start paths are declared once: run, revive and adopt start through one shared start prologue (trust check, instance check, pinned-exe check with the strict-modes refusal under --json, then the wrapper log), so a start-order test over one verb holds for all three.
@task_fact n=3 slug=unit-test-strategy
fact: A pump case asserting that the exit drain was skipped must have the run loop consume the output-ended signal before the child exits, else the exit-path drain reads it and the mutant passes too; the case's child exits only after the reader has ended.
@task_item n=1 part=untouched
The MCP server, MCP tools and their result and invalid-params contracts: no MCP surface changed.
@task_item n=2 part=untouched
The wrapper channel, JSON-RPC error codes, link and unlink methods and the event-log line contracts: no IPC method, event kind or log field added or changed.
@task_item n=3 part=untouched
The agent-run harness verbs, their documents, exit codes, readiness codes, the gate breaches and pre-push stages: no harness verb, status shape or CI step changed.
@task_item n=4 part=untouched
CI integration — workflows, artifacts, actions pinning, supply-chain, cargo-deny and zizmor gates, toolchain pins: untouched.
@task_item n=5 part=untouched
Coverage floors and exclusions, performance budgets and the hyperfine gate, fuzzing and corpora: untouched.
@task_item n=6 part=untouched
The fake agent, its command line, receipts, recorded fixtures and fixture hygiene: untouched.
@task_item n=7 part=untouched
viola send, wait, last, answer, release and list contracts and the dialog hook decision bodies: untouched.
@task_item n=8 part=untouched
Log format and process log line contracts, and the pid-file and status-endpoint shape areas: untouched.
