@task_prose part=chunk
Bounds the wrapper's paste write into the child's PTY by a compiled time bound, so send never waits without an end on a child that stops reading.
At the bound the send ends with a closed word the driver reads on CLI send and on the MCP send tool, never upstream text, and the wheel is not moved.
First measures what the code does today with a fake agent that stops reading its PTY (its buffer full): whether the paste blocks send past the readiness gate's and the confirm window's bounds.
Reads the bound's value against send's own compiled total, the readiness gate bound, the confirm window and the MCP call deadline, the arithmetic shown.
Disposes what a cut paste leaves in the child's input box (text with no Enter) for the next send, the human and the confirmation, and what the human's keys do while a paste is blocked and after it is cut.
@task_file n=1 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY seam's pump, its write contract into the child's PTY — edits: the paste write (one blocking write with no time bound)
@task_file n=2 path=crates/viola-pty/src/lib.rs mark=edits
what: the PTY crate's root and its test module with the CHILD_WITHIN wait bound — edits: the test-module comment over CHILD_WITHIN naming the mutants profile kill
@task_file n=3 path=src/run/ mark=edits
what: the run pump and the wheel; the wrapper's send path, its flight lock and its send confirmation
@task_file n=4 path=crates/viola-core mark=edits
what: the closed refusal, detail and cause sets, if a new closed word is added
@task_file n=5 path=crates/viola-mcp mark=edits
what: the MCP server's send tool and its tool result at the bound
@task_file n=6 path=fake-agent mark=edits
what: the fake agent (viola-fake-agent), the boundary standing in for claude — edits: a behaviour that stops reading its PTY
@task_item n=1 part=needs
The paste into the child's PTY ends within a compiled time bound; send never waits on the child's reading without an end.
@task_item n=2 part=needs
At the bound the send ends with a closed word: the CLI send result (its --json document, its human line and its exit code) and the MCP send tool result carry that word, never upstream or free text, identical over CLI and MCP.
@task_item n=3 part=needs
At the bound the wheel is not moved: a cut paste neither hands the wheel to the human nor records a wheel change.
@task_item n=4 part=needs
A new closed word joins the closed refusal, detail or cause sets only by decision, and it passes the MCP reply screen and the CLI human-line screen as a known word, its error body holding no path or upstream text.
@task_item n=5 part=needs
The bound's value is read against send's compiled total, the readiness gate's GATE_MAX_WAIT, the send confirmation window and the MCP call deadline (55 s, its expiry call-deadline), so the paste bound plus the rest of send stays under the MCP call deadline and the driver never reads call-deadline for a cut paste.
@task_item n=6 part=needs
A cut paste is never presumed delivered: the send's outcome at the bound is not-delivered or unconfirmable as the plan disposes, never confirmed, and the send records and send-issued line say what was written.
@task_item n=7 part=needs
What a cut paste leaves in the child's input box (text with no Enter) and what the next send, the human-typing refusal and the send confirmation then meet is disposed in the plan, its choice among priced answers left to the founder's fork.
@task_item n=8 part=needs
The send flight lock is released when the paste is cut, so the next send is not held behind a dead paste.
@task_item n=9 part=needs
The human is never blocked: the human's keys while a paste is blocked and after it is cut reach the child or are held only as the plan allows, and human input takes the wheel as it does today.
@task_item n=10 part=needs
The PTY pump's write contract, its callers and the pump's resize and reader paths keep their behaviour apart from the bounded paste; child exit stays read on the process handle.
@task_item n=11 part=needs
The fake agent gains a behaviour, set by argv alone, that stops reading its PTY so its buffer fills; it is a fake-agent command-line option and its receipt lines stay in their format.
@task_item n=12 part=needs
The measuring tests run on a real openpty spawn with the fake agent as claude, never a real claude start and no live rig start; a fake-agent run is no proof of real-CLI behaviour.
@task_item n=13 part=needs
A test that waits on the bound takes a bound under the 20 s kill line and the root wait bound, below the per-test kills; the CHILD_WITHIN bound stays below the mutants profile kill (now 15 s) and its comment says so.
@task_item n=14 part=needs
Where the bound is driven in a unit test, wrapper deadlines go through the injected clock seam; integration tests synchronise on logged events, never sleeps.
@task_item n=15 part=needs
The send-refused or send-issued process log line at the bound carries its corr and a code only, no sent text, no secret.
@task_item n=16 part=needs
Every test start of viola or a wrapper runs under a test-owned user home with a fresh per-test home, tied in a drop guard.
@task_item n=17 part=needs
No mutation run in this chunk; send-window tests keep their mutants kill classes as they stand.
@task_standing n=1
convention: each new test function is named by the test naming rule and sits in its layer by binary prefix
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: each new test creates its own fresh home and runs viola under a test-owned user home
home: test-data-bootstrap
@task_standing n=3
convention: no sleep synchronisation, no retries, literal oracles, cleared environments keep LLVM_PROFILE_FILE
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: each new root wait is bounded below the kills and every spawned wrapper is held in a drop guard
home: 5-command-implementation · integration-test-strategy
@task_standing n=5
convention: each new process log line is codes-only JSON per line with its corr
home: log-format
@task_item n=1 part=not_needed
viola revive: its preflight, refusal codes, exit codes and documents.
@task_item n=2 part=not_needed
Dialog hook decision bodies, viola answer, permission, plan and question answers, dialog hold and expiry.
@task_item n=3 part=not_needed
The list board and list --json rows, liveness classification and heartbeat staleness.
@task_item n=4 part=not_needed
The served link and unlink methods and their event records.
@task_item n=5 part=not_needed
viola wait and viola last, their wake kinds and timeouts.
@task_item n=6 part=not_needed
Run and revive start refusals, strict-modes and socket-directory checks at start, plugin install and child environment strip.
@task_item n=7 part=not_needed
Hook performance budgets, hyperfine invocation and the perf CI job.
@task_item n=8 part=not_needed
Harness boot, cleanup, status document and session record contracts, agent-run status fields.
@task_item n=9 part=not_needed
Verify stamps, recorded hook fixtures, fixture recording and scrub refusals, ledger probe rows.
@task_item n=10 part=not_needed
Fuzz corpora and nightly fuzzing, property tests on parser surfaces.
@task_item n=11 part=not_needed
The mutation run verdict, thresholds and cadence at the epoch boundary.
@task_item n=12 part=not_needed
The weekly advisory scan and the advisory ignore list review, the operator's at the epoch boundary.
