@task_prose part=session
Epoch 5 cleanup mends the code audit's ranked defects and makes the root crate's mutation baseline green; the three file splits are left for their own entry.
The shared start of viola run, viola revive and viola adopt now goes through one start prologue in the run verb. It runs the instance trust check, then the home-tree strict-modes check (under --json the caller's strict-modes refusal, else the tree refusal), then the wrapper log open, in the same order as before.
viola up now uses viola open's refusal writer instead of its own byte-identical copy. Its four refusal sites, codes and exits are unchanged.
The PTY pump's exit drain bound is now a seam that tests can set; the product keeps its 500 ms drain. The liveness parent-chain walk was split out so it can be tested; the 8-pid limit stays with the callers. New killing tests cover the pump, the board scan and the replay ended-after-start reading.
The nextest verify class (20 s × 4 kill in the ci and mutants profiles) now names six more test functions: the path4 dialog-rows test and five cli_version_gate run tests. The root crate's in-diff mutation run read 326 caught, 0 missed, 0 timeout, 46 unviable, score 100.0, recorded as an addendum to the code audit.
The operator-fired R2 window script now refuses with a new verdict line, r2: red precondition=scratch|locked, before it starts anything. No IPC method, endpoint, socket, env var, detail code, log field, span, dependency or config key was added or changed.
@task_file n=1 path=.config/nextest.toml
what: the test runner's profiles: the ci and mutants verify-class kill override (its filter line), the mutants default kill and the other overrides
@task_file n=2 path=tests/contract_lints.rs
what: the contract lint probes, a test that pins the verify class's filter line in the runner profiles
@task_file n=3 path=crates/viola-pty/src/pump.rs
what: the PTY pump of viola run: the child output copy to the host terminal, the respawn of the claude child and the exit drain of output in flight, with its seams and inline tests
@task_file n=4 path=crates/viola-state/src/liveness.rs
what: the state crate's liveness reading: the process ancestor chain walk over the process table, used by focus and open
@task_file n=5 path=crates/viola-state/src/board.rs
what: the state crate's board: the instance scan listing the checked instances in name order (tests added only)
@task_file n=6 path=crates/viola-state/src/replay.rs
what: the state crate's snapshot replay from the events log, including the ended-after-last-start reading (tests added only)
@task_file n=7 path=scripts/r2-open.sh
what: the operator-fired, test-only live R2 window-placement script, its verdict lines, its preconditions and its exit cleanup
@task_file n=8 path=src/cmd/run.rs
what: the viola run verb and the start shared by run, revive and adopt: the instance trust check, the start prologue (instance check, home-tree strict-modes check, wrapper log open) and the start itself
@task_file n=9 path=src/cmd/revive.rs
what: the viola revive verb: its --list arm and its start arm, which logs as run and now goes through the shared start prologue
@task_file n=10 path=src/cmd/adopt.rs
what: the viola adopt verb: its start arm, which logs as run and now goes through the shared start prologue
@task_file n=11 path=src/cmd/open.rs
what: the viola open verb, which now holds the one refusal writer (unable and hint pair, --json document) shared with viola up
@task_file n=12 path=src/cmd/up.rs
what: the viola up verb, its terminal spawn and window recording; its refusals are now written by viola open's refusal writer
@task_file n=13 path=tests/contract_error_bodies.rs
what: the clean-error-bodies contract test: per-verb refusal rows with their codes, exits and witness needles
@task_item n=1 part=touches
decision/spine-hook-budget: the report's sweep says the rule text still reads «provisionally 1.0 s» and calls the wording stale. obs-plan owns this row, and the wrap directive names it for this reconcile.
@task_item n=2 part=touches
decision/instance-check-before-log-init and decision/instance-check-before-role-file: the trust check that comes before any role file is opened for run, revive and adopt now lives in one shared start prologue. The order is kept.
@task_item n=3 part=touches
decision/home-tree-check-before-start-log, decision/home-tree-check-before-log-init and decision/strict-modes-before-diagnostics: in the shared prologue the home-tree strict-modes check still runs before the wrapper log opens, now through one code path for all three starts.
@task_item n=4 part=touches
decision/pre-open-strict-modes-refusal-silent, decision/strict-modes-refusal-writes-no-line, decision/pre-log-strict-refusals-write-no-line and decision/refused-home-tree-writes-no-line: a strict-modes refusal at a revive or adopt start is now written by the caller's writer passed into the shared prologue. It still writes no process log line.
@task_item n=5 part=touches
decision/revive-start-logs-as-run and decision/adopt-start-logs-as-run, plus contract/cli-log-file/revive-start: the revive and adopt start arms now open the run wrapper log through the shared prologue.
@task_item n=6 part=touches
contract/viola-revive/strict-modes-failed, contract/strict-modes-refusal-exit/1 and contract/strict-modes-refusal-exit/21: the revive and adopt strict-modes refusals now come through the shared prologue's --json branch. Their codes and exits are unchanged.
@task_item n=7 part=touches
decision/revive-preflight-order: the revive start's prologue steps (trust, home tree, log open) moved into the shared start, so the preflight order is now defined in one place.
@task_item n=8 part=touches
contract/process-exit-detail/exit-1-up, contract/cli-log-file/up and contract/cli-log-file/open: viola up's refusals are now written by viola open's single refusal writer. The detail codes, exits and the four up call sites are unchanged.
@task_item n=9 part=touches
contract/cli-stderr/refusal-pair and contract/refusal-stderr-pair/socket-dir-refused: the unable and hint pair for open and up now comes from one writer.
@task_item n=10 part=touches
decision/error-bodies-clean: the clean-error-bodies test's four up rows now point at open's needle. The codes, exits and witnesses are unchanged, and the up constant is retired.
@task_item n=11 part=touches
decision/obs-code-mutation-tested and decision/surviving-obs-mutant-fails: the root crate's mutation reading moved from baseline-test-failure to 100.0 (0 missed, 0 timeout), and new killing tests were added in the pump, liveness, board and replay.
@task_item n=12 part=touches
decision/mutants-out-never-uploaded and decision/mutation-diff-never-uploaded: this session made a mutation run, an in-diff epoch diff and a mutants output. They were kept in a scratch directory outside the repository and never uploaded.
@task_item n=13 part=touches
decision/pty-byte-stream-never-logged, contract/pty-seam-log/spawn and contract/pty-seam-log/kill-fallback: the PTY pump's respawn and exit-drain paths changed, by a drain bound passed through its seams. No log line was added and the product drain stays 500 ms.
@task_item n=14 part=touches
decision/no-hot-path-instrumentation and decision/no-info-in-hot-loops: these rules cover the pump's copy loop, which now has a tested flush after each copy. No instrumentation was added.
@task_item n=15 part=touches
contract/process-exit-detail/exit-1-focus: the process ancestor walk used by focus and open was split into a tested walk that stops at pid 1. Its signature, its callers and the 8-pid limit are unchanged.
@task_item n=16 part=touches
decision/real-cli-verify-never-in-ci and decision/ci-verify-fake-agent-only: five cli_version_gate run tests and one path4 dialog-rows test now run under the verify-class kill bound by name, in the ci and mutants profiles.
@task_item n=17 part=touches
decision/no-raw-snapshot-reads and decision/replayed-snapshot-unreachable: replay and board gained tests only (the ended-after-start reading, the instance scan order). Their behaviour is unchanged.
@task_item n=18 part=touches
decision/window-line-untraced and decision/window-view-restore-uninstrumented: the operator-fired R2 window script, which proves the window step, now refuses on a locked desktop or a scratch home that is not fresh before anything starts. It prints a closed precondition verdict word and skips its cleanup.
@task_item n=1 part=untouched
The channel and MCP wire: JSON-RPC errors, channel lines and params, MCP tools and frames, conn and corr propagation. No IPC method, endpoint, socket or env var was added or changed.
@task_item n=2 part=untouched
The process log line schema, the event enum, the detail-code and exit-code contracts (exit 1, exit 20, exit 21, parse-rejected, send-refused, state-recovered): no field, word or code was added, moved or retired.
@task_item n=3 part=untouched
Span and trace coverage, span attributes and span chains (send, wait, dialog, wheel, hook, mcp): no span was declared or changed.
@task_item n=4 part=untouched
PII scrubbing and redaction (veil, the allow-list, the detail-file line), the secret scan and the CI artifact uploads: none changed, and no CI step was changed.
@task_item n=5 part=untouched
Panic capture and error reporting (panic hook, catch sites, the panic line, the reporter absences): unchanged.
@task_item n=6 part=untouched
Obs dependencies, logging stack, subscriber, log levels, config.json keys and service identity: no dependency, config key or schema was changed.
@task_item n=7 part=untouched
The hook role's telemetry, its fail-open, its stdout and its session-end perf gate: no hook code was changed. Only the spine hook budget's wording is flagged as stale.
