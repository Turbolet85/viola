@task_prose part=chunk
Answers whether real claude on Linux loses a key typed right after a resize: first what can be told with no live start (where the claim comes from, what viola does on the resize path today, what the installed CLI bundles read as strings).
Prices in one P3 block the cheapest live start that would measure it: PTY sizes before and after, the key bytes and their delays, the empty input box typed into, a closed word telling a lost key from a kept one, repetitions, home and config.
Prices the fix options (a settle wait after a resize, a read-back and one re-type, a refusal) with the rows each moves; the choice is made only after the reading, and this phase holds at the end of P3 with no live start.
Re-reads two carries: the fake agent's DEL already erasing its prompt, and the test-module comment over the child wait bound naming the old mutants kill.
@task_file n=1 path=crates/viola-pty/src/pump.rs
what: the PTY seam's pump — the resize sent to the child's PTY, the pump's writes after it, and the paste handle's one blocking write
@task_file n=2 path=crates/viola-pty/src/lib.rs mark=edits
what: the PTY seam crate root and its test module — edits: the test-module comment over the child wait bound, reworded from the 10 s to the 15 s mutants kill only if this chunk edits the PTY seam
@task_file n=3 path=src/run/
what: the viola run pump and wheel — the resize path, what is typed after it, and every wait or settle bound on that path
@task_file n=4 path=src/bin/viola-fake-agent/input.rs mark=re-reads
what: the fake agent's keystroke input check that a typed DEL erases the prompt's last character, as the live CLI's input box does
@task_file n=5 path=src/bin/viola-fake-agent/
what: the fake agent's resize and input behaviour, reached only if a fix or a rehearsal needs it
@task_file n=6 path=crates/viola-e2e/
what: the one standing committed live rig, reached only if a measuring arm is added
@task_file n=7 path=crates/viola-agent-claude/
what: the claude capability ledger and its relied rows, reached only if the answer becomes a relied row
@task_item n=1 part=needs
The key typed after a resize: the rule that a key after a resize waits for the new size, and the pump's resize baseline being the spawn size — the behaviour this chunk reads today and may change by a settle wait, a read-back and re-type, or a refusal.
@task_item n=2 part=needs
A resize, like focus and mouse reports, never takes the wheel; any fix on the resize path keeps the wheel where it is.
@task_item n=3 part=needs
driver send's gate and its confirmation around a resize: the typed text, the input-not-ready refusal, the human key during the gate wait, and the prompt-submitted confirmation and its window — what a lost key would break and a fix must keep.
@task_item n=4 part=needs
A lost key told from a kept one by a closed word from logged events or a receipt, never by parsing the child's rendered screen; the screen is read only for compiled signatures and no screen literal is asserted beyond them.
@task_item n=5 part=needs
A live reading of the real claude CLI is recorded as a reading, never as verification; a fake-agent run is no proof of real-CLI behaviour; the real CLI is driven only locally, never in CI.
@task_item n=6 part=needs
The installed claude bundles are read as strings only — a reading, never a measurement — and nothing of the host's real claude home or sessions is read by any test.
@task_item n=7 part=needs
Any measuring arm stands in the one standing committed live rig, its start tied to a test-owned user home, and the rig rehearsal reads no key typed from its own keys word.
@task_item n=8 part=needs
The fake agent's resize and keystroke input contracts and its receipt (key, size, prompt), held to the recorded fixtures byte for byte — reached if a fix or rehearsal changes how the fake agent takes a key after a resize.
@task_item n=9 part=needs
Every wait a new test takes is bounded under the 20 s kill line and the WITHIN bound, keys on logged events, and never sleeps or retries to synchronise.
@task_item n=10 part=needs
Every test start — run, revive or verify — runs under a test-owned user home, never the operator's.
@task_item n=11 part=needs
If the answer becomes a relied ledger row: the row carries a verify probe, joins the ledger probe contract suite, and the version gate then needs it among every relied row.
@task_item n=12 part=needs
The paste's missing time bound — the one blocking write in the paste handle — taken only if this chunk's change opens that write path, and then only as a fork of the P3 block.
@task_item n=13 part=needs
The intermittent rig rehearsal red after X and DEL: at its next red read the printed rig summary and kept frame and name the cause before mending the wait; no retry rule, no loosened entry.
@task_item n=14 part=needs
PTY behaviour proved on a real openpty spawn, the child's exit read on the process handle, never on EOF.
@task_item n=15 part=needs
The pinned product build is untouched and no mutation run is made in this chunk.
@task_standing n=1
convention: every new test creates and owns its home, and every start it makes runs under a test-owned user home
home: test-data-bootstrap
@task_standing n=2
convention: every new test waits on logged events, never sleeps or retries, and stays inside the WITHIN bound below the per-test kill
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=3
convention: every new test function follows the test naming rule and runs in parallel with no shared mutable state
home: 2-test-strategy · 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: every new test oracle is a literal from the contract, never a Debug string or a fixture internal
home: 11-test-anti-patterns-never-do-these
@task_item n=1 part=not_needed
MCP tool surfaces, tools/list, tool results and the MCP instance-unreachable details: the chunk adds no tool and changes no reply.
@task_item n=2 part=not_needed
Hook exec entry points, dialog hook decision bodies, dialog holds and answer mapping: a resize reaches no hook or dialog.
@task_item n=3 part=not_needed
The revive verb, its refusal codes and exit causes: the chunk reaches no revive path.
@task_item n=4 part=not_needed
The list board and list --json rows, liveness and skipped counts: nothing on the board changes.
@task_item n=5 part=not_needed
The wrapper channel's link and unlink methods and their event records: the chunk adds no channel method.
@task_item n=6 part=not_needed
Harness boot, cleanup, status, session record and pre-push contracts: the chunk changes no harness command.
@task_item n=7 part=not_needed
CI artifacts, workflow permissions, SHA pinning and the weekly advisory scan: no workflow is changed.
@task_item n=8 part=not_needed
Hook performance budgets and hyperfine invocation: no hook timing is reached.
@task_item n=9 part=not_needed
Socket directory, strict-modes and server identity checks before any frame: the resize path opens no endpoint.
@task_item n=10 part=not_needed
MCP fixture scrub refusals and the MCP exchange tap: no MCP exchange is recorded.
