@task_prose part=session
A live round on claude 2.1.325 that read the dialog hold at its 3600 s ceiling: the wrapper expired the dialog after 3600013 ms with the hook still waiting, and the founder ruled hold: stands, so the ceiling is no longer provisional.
The only product change is the doc comment of the dialog hold ceiling constant, which now says it was measured live at the ceiling; its value (3600 s), its references, every IPC method, event kind, flag, env var and config key are unchanged.
claude 2.1.325 was stamped 21 of 21 relied ledger rows pass on a test home only (the live home is not stamped for it), measuring ready settle 2339 ms, turn settle 603 ms, compact latency 7038 ms and 3 box-only guards.
Measured on the real CLI: the folder-trust dialog in a fresh cwd stands before any hook, and a human's trust keys passing through the wrapper write a wheel human-input line before session-start; the real CLI erases the input box on DEL; the question dialog stays drawn after the wrapper's dialog-expired.
The user's status row read present at the same place and hash wrapped and unwrapped, with settings untouched and no --settings at start.
Matrix v2-03's start-order clauses were disproved on such a start and it is not verified this round; nothing was retired.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: viola-core, the agent-neutral core crate holding the dialog hold's compiled ceiling (the 3600 s bound on a sync dialog hook's wait, DIALOG_HOLD_MAX) beside the other hold constants; only that ceiling's doc comment changed
@task_item n=1 part=touches
decision/dialog-hold: its provisional state (measured only to about 660 s) leaves; the per-instance hold's 3600 s ceiling is now measured live at the ceiling on claude 2.1.325, one reading on one CLI version, with the founder's hold: stands.
@task_item n=2 part=touches
decision/dialog-hold-bound: the bound a dialog hold places on a sync dialog hook is no longer provisional; the 3600 s value a cell holds stands, measured at 3600013 ms with the hook still waiting.
@task_item n=3 part=touches
contract/config-json/dialog-hold-s: the round ran dialog_hold_s 3600, the ceiling; any cell calling that ceiling provisional or measured only to about 660 s is now stale. No key added.
@task_item n=4 part=touches
decision/viola-ends-the-dialog-wait and contract/dialog-hold-end/null-reply: measured live, the wrapper (never Claude Code) ended the wait with a dialog-expired line sourced wrapper and a null reply; the question dialog stays drawn on the CLI's screen after that expiry.
@task_item n=5 part=touches
dialog-expired rows (contract/event-line/dialog-expired, contract/normalised-event-kinds/dialog-expired, contract/event-data/dialog-expired): read live once at the ceiling, source wrapper, one hour after the question line sourced hook; confirming, a row stating the ceiling's provisional reach is stale.
@task_item n=6 part=touches
decision/wheel-starts-with-driver, decision/byte-source-wheel-claim and contract/event-data/wheel: on a start in a fresh cwd, the human's folder-trust keys pass through the wrapper and write wheel human-input before session-start, and a later human key adds no wheel line since the wheel is already human.
@task_item n=7 part=touches
contract/events-ndjson/byte-offsets: a reused instance name keeps the earlier start's lines at the head of its events log (cursors are byte offsets, never rewritten), so a start's first lines are not line 1 of the log.
@task_item n=8 part=touches
Capability ledger rows (decision/cli-capability-ledger, decision/capability-ledger-gate, decision/version-verified-all-relied-rows, contract/ledger-stamps rows): claude 2.1.325 stamped 21 of 21 relied rows pass on a test home; a row holding the relied-row count or a newest stamped version is the one to check.
@task_item n=9 part=touches
decision/gate-timings-compiled and decision/partial-gate-on-unverified-cli: measured on 2.1.325 ready settle 2339 ms, turn settle 603 ms, compact latency 7038 ms, 3 box-only guards; a row whose cell holds a readiness timing or bound is the one touched. The live home is unstamped for 2.1.325, so a wrapped start there reads unverified.
@task_item n=10 part=touches
Input-box readiness rows (contract/not-delivered-details/input-not-ready, contract/send-results/input-not-ready, contract/human-hint-lines/input-not-ready): the real CLI erases its input box on DEL, after which release and a send read send-issued, prompt-submitted, send-confirmed with an empty box; input-not-ready is no residue.
@task_item n=11 part=touches
Fake-agent rows (decision/tests-against-fake-agent, decision/fake-agent-test-seams, decision/ci-against-fake-agent): the fake agent keeps a typed DEL in its prompt while the real CLI erases the box, a measured divergence left unchanged and carried.
@task_item n=12 part=touches
decision/global-statusline-untouched and absence/status-line-interception: read live, the user's status row stood one row below the input box's lower rule with the same hash wrapped and unwrapped, settings unchanged, no --settings at start; confirming.
@task_item n=13 part=touches
Test-home rows (decision/test-isolated-viola-home, decision/dev-host-e2e-home-on-tmpfs, decision/linked-e2e-home-base-strict, decision/end-to-end-homes-kept-after-each-job): a test home directory named home made the recorder's absolute-path scrub refuse a whole recording, and a rig that resolved the tmpfs-linked home path landed outside the root; the kept tmpfs home is the operator's to delete.
@task_fact n=1 slug=established-decisions
fact: On a start whose folder-trust dialog the human answers through the wrapper, the human's keys take the wheel before the session starts: a wheel human-input line precedes session-start, and a later human key adds no wheel line.
@task_fact n=2 slug=established-decisions
fact: When viola ends a dialog wait at the hold ceiling with a null reply, the CLI's question dialog stays drawn on the screen; the expiry ends the hook's wait, not the CLI's dialog.
@task_fact n=3 slug=established-decisions
fact: The real claude CLI erases its input box on DEL (measured on 2.1.325) while the fake agent keeps a typed DEL in its prompt, so a rehearsal against the fake agent cannot read the erase.
@task_item n=1 part=untouched
Dependencies, toolchain pins, licences, build tooling, CI workflows and deployment: no dependency, workflow, job or install path changed; the rigs are evidence scripts outside the product and the release build.
@task_item n=2 part=untouched
Channel methods, MCP tools, CLI verbs, exit codes and refusal codes: no method, tool, verb, code or document shape was added, moved or retired.
@task_item n=3 part=untouched
The names sheet: no binary, command, crate, env var or endpoint name was taken or freed; the round's rigs are no named binary.
@task_item n=4 part=untouched
decision/no-typing-into-trust-dialogs and the verify-never-types-into-dialog rows: viola itself typed into no trust dialog; the trust keys were a test rig's into its own pty, a rig being no viola.
@task_item n=5 part=untouched
contract/unclean-fixture-refusal/absolute-path, decision/recorded-fixtures-scrubbed and decision/verify-record-only-clean: the recorder's refusal behaved exactly as those rows state; the trip was a test-home naming slip, not a change to the scrub.
@task_item n=6 part=untouched
Board and agents-listing rows on unwrapped sessions: the status-row reading is a different fact from the board's unwrapped names and rows.
@task_item n=7 part=untouched
absence/usage-budget-gate and the other absences (daemon, database, network port, graphical surfaces, containers, secrets): nothing added back or removed.
@task_item n=8 part=untouched
Coverage floors, perf gate, zero-panics and fuzz rows: no new product surface, hot-path operation or UI element; CI read green 7 of 7 with no gate value moved.
@task_item n=9 part=untouched
Snapshot, replay, liveness, revive and link rows: the round exercised no change to snapshots, revive, liveness or links.
