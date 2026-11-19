@task_prose part=chunk
Lands a new recorded fixture set for claude 2.1.328, recorded by a clean live viola verify --record under the rig's private config plus the viola-mcp-tap re-record, committed whole as a plain copy of the recorders' scrubbed output, none hand-edited.
Names the new set in the ledger probes' STAMPED list, so the drift suites and the recorded-exchange suites read over it.
Re-runs the drift suites, the recorded-screen suites and the fixture hygiene walk over the landed set; a drift suite red on it stops the chunk.
Disposes the entry's three carries by name; the /context post-condition (half unread on 2.1.328) is read from a readable transcript beside the recording, or named as leaving with its reason.
Any code change to the rig or the fake agent for 2.1.328 (the trust dialog's decline text, a moved literal) is decided at P3; if none, the chunk is the two live starts, the landing and the gates.
@task_file n=1 path=fixtures/claude/2.1.328/
what: the recorded Claude fixtures for one CLI version, recorded by live verify and the MCP tap: hook payloads named by hook event, screen fixtures, the agents listing and the MCP tools/list exchange
@task_file n=2 path=tests/contract_ledger_probes.rs mark=edits
what: the ledger probe contract suite, its drift replay of every stamped CLI-version fixture set against the fake agent — edits: the STAMPED list of stamped fixture versions
@task_item n=1 part=needs
A new per-CLI-version fixture set (fixtures per CLI version) recorded by live verify, landed whole for claude 2.1.328.
@task_item n=2 part=needs
Fixtures are recorded only when nothing fails: a stamping verify on 2.1.328 with zero failing ledger rows, every ledger row with its verify probe.
@task_item n=3 part=needs
The recorder's scrub and hygiene bind every recorded file: home paths and usernames scrubbed, a dirty payload refuses the recording, the MCP tap refuses a dirty exchange whole with its scrub refusal codes; a refused payload is a red, never a hand scrub.
@task_item n=4 part=needs
Fixtures free of real data and never from real prompts: synthetic fixture text only, no production data, nothing hand-written into a recorded fixture or product state file.
@task_item n=5 part=needs
The fixture hygiene run-time walk and the fixture schema check (recorded fixtures checked against the tolerant schema) pass over the new set.
@task_item n=6 part=needs
Every stamped CLI-version fixture set is replayed: the ledger probes' stamped list names 2.1.328, and the drift suites compare the fake agent's output to the recording byte for byte.
@task_item n=7 part=needs
The fake agent is held to the recorded verify fixtures and replays recorded bytes; what in the fake is keyed to a CLI version (its --cli-version, its screens such as the trust dialog) must match the 2.1.328 recording, and the default CLI version the harness and the fake share.
@task_item n=8 part=needs
The recorded-screen suites over the new set: screen fixtures checked scrub-as-detector, box rows as literals, the screen read only for compiled signatures, the rendered child screen never parsed.
@task_item n=9 part=needs
Recorded hook payloads walked at run time by one test and replayed in CI; send-after-turn cases copy the recorded fixtures; the MCP tools/list exchange recorded by the test-only tap.
@task_item n=10 part=needs
Real claude CLI only locally: the live recording is a local-only start, never in CI, and CI's fake agent keeps replaying recorded sets; a fake-agent run is no proof of real-CLI behaviour.
@task_item n=11 part=needs
Live starts under the rig's private config: the one standing committed live rig and its labels, every verify start tied to a test-owned user home, nothing read under the operator's real home beyond the rig's credentials read.
@task_item n=12 part=needs
A live reading is recorded as read, never as verification: the /context local-command post-condition (local commands confirmed by post-condition only, unconfirmable without one) stays unverified on 2.1.328 unless read from a readable transcript.
@task_item n=13 part=needs
The unverified CLI version gate and stamps: stamps written only by verify, the version gate needing every relied ledger row, the stamped line of viola verify.
@task_item n=14 part=needs
Waits in any new or touched test key on logged events, take a bound under the 20 s per-test kill line for verify-driven tests, no sleeps, no retries; no scheduled wake or timer.
@task_item n=15 part=needs
No mutation run in this chunk; every process a test starts is ended (tied spawns, drop guards).
@task_standing n=1
convention: each recorded fixture set is recorded by live verify and the tap, scrubbed by the recorder, hygiene-walked and never hand-edited
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=2
convention: each stamped fixture set is replayed by the fake agent and drift-compared byte for byte
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: each test starts under a test-owned home, waits on logged events within its bound, and ends every process it starts
home: test-data-bootstrap
@task_item n=1 part=not_needed
The MCP tool and wrapper channel error contracts (JSON-RPC codes, invalid-params fields, reply screens): the chunk changes no channel or MCP surface.
@task_item n=2 part=not_needed
The send, answer, release and wait verb contracts and their exit codes: no verb behaviour changes.
@task_item n=3 part=not_needed
The revive, run start refusal and start-exit contracts: no start or revive path changes.
@task_item n=4 part=not_needed
Harness session, status, boot readiness, cleanup and supervisor contracts: the chunk adds no harness command.
@task_item n=5 part=not_needed
Performance budgets and the hyperfine gate: no hook path changes and nothing is timed.
@task_item n=6 part=not_needed
Fuzzing, property-test generators and their corpora: no parser changes.
@task_item n=7 part=not_needed
Mutation verdict contracts and the mutation gate cadence: the scope runs no mutation.
@task_item n=8 part=not_needed
The process log line fields and log event corr contracts: the chunk adds no log line.
@task_item n=9 part=not_needed
Windows and macOS support, a browser suite, a database, outbound HTTP and a deploy stage: absent and untouched.
@task_item n=10 part=not_needed
Peer-credential, socket directory and strict-modes refusal contracts: no IPC or filesystem permission code changes.
