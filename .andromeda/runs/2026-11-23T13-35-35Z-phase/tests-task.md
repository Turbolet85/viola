@task_prose part=chunk
Adds `monitors`, a new query, to the Hyprland seam's closed request set: its closed argv builder, its parse keeping only what a closed per-request keep-list names, a read-only record by the test-only hyprctl tap, and a replay by the fake agent standing as hyprctl.
Adds ONE closed move-to-workspace dispatch to the same closed set, taking only a validated window address and an integer workspace, never an exec; its exact argv, recorded fixture and replay case come from the one live reading R1.
Settles the hyprctl read deadline at 5 s and adds a test that times a real read against it; gives the hyprctl reply bounded-reader row a witness that drives the read's own stdout drain past MAX_FRAME.
Adds the tap's R1 window verb (open one foot window with no claude in it, move it once, kill the pid it started, leave only fixture files), run only at R1; implement builds and greens all that needs no live window and stops once before R1.
Claims no capability and gives the seam no product caller: it stays a root dev-dependency only.
@task_file n=1 path=crates/viola-hyprland/src/lib.rs mark=edits
what: the Hyprland compositor seam — its closed request set, its closed words list and its argv builders — edits: the Request set (a `monitors` query and one move-to-workspace dispatch), WORDS, the argv builders
@task_file n=2 path=crates/viola-hyprland/src/reply.rs mark=edits
what: the seam's hyprctl reply parses, dropping reply text save a closed per-request keep-list — edits: the `monitors` parse and its keep-list
@task_file n=3 path=crates/viola-hyprland/src/read.rs mark=edits
what: the seam's bounded hyprctl read and its read deadline — edits: HYPRCTL_DEADLINE (settled at 5 s), the read's stdout drain
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs mark=edits
what: the test-only hyprctl tap that records compositor replies into fixtures — edits: a `monitors` record, the R1 window verb
@task_file n=5 path=src/bin/viola-fake-agent/hyprctl.rs mark=edits
what: the fake agent standing as hyprctl, replaying recorded fixtures by exact argv in its recorded, oversize, malformed and absent modes — edits: the `monitors` replay case, the move-dispatch replay case
@task_file n=6 path=fixtures/hyprland/0.56.2/Monitors.recorded-1.json
what: a recorded hyprctl fixture of the seam for Hyprland 0.56.2 — the `monitors` reply recorded read-only by the tap
@task_file n=7 path=tests/contract_bounded_readers.rs mark=edits
what: the bounded-readers contract table, one Boundary row per external reader with its witness — edits: the `hyprctl reply` Boundary row's witness
@task_item n=1 part=needs
The seam's closed request set grows by two: a `monitors` query and one move-to-workspace dispatch; each closed request, its closed argv and its closed words change only by a recorded decision, and nothing outside the two enters.
@task_item n=2 part=needs
The move dispatch's argv takes only a validated window address and an integer workspace; no free text, no exec form reaches it.
@task_item n=3 part=needs
The `monitors` parse drops all reply text save what its closed keep-list names, and is held as a parser surface by property tests over its input.
@task_item n=4 part=needs
Every external reader is bounded and keeps the frame cap: the hyprctl read's own stdout drain is driven past MAX_FRAME by a bounded remove-the-guard witness, not by a parse test.
@task_item n=5 part=needs
The read deadline is one product constant, settled at 5 s; the test that times a read against it takes the constant from the product, never a copy, and waits without sleep synchronisation or retries.
@task_item n=6 part=needs
The fake agent serves the hyprctl boundary as a real stub executable on a test-scoped PATH, replaying recorded bytes by exact argv; the new requests get replay cases in its recorded, oversize, malformed and absent modes, and the fake stays held to the recorded fixtures byte for byte.
@task_item n=7 part=needs
New hyprctl fixtures are recorded by the test-only tap from a real run, per compositor version, never hand-written, scrubbed of home paths and usernames, and pass the fixture hygiene walk and schema check.
@task_item n=8 part=needs
The tap is a test-only binary in the one test-only workspace member; no test-only binary reaches a release build.
@task_item n=9 part=needs
The R1 window verb is a live reading: run only locally at the operator's host, never in CI and never from a test; what it gives is recorded as a reading, never as verification.
@task_item n=10 part=needs
The tap ends only the foot window it started: the pid and start time it spawned are verified before the kill, and it is told apart from standing windows; it leaves only fixture files.
@task_item n=11 part=needs
No test or tap reads the operator's real claude sessions or home; every test runs in its own fresh home.
@task_item n=12 part=needs
Mocks only at product seams: the Hyprland seam is tested against the real stub process and real files, with no mock library over hyprctl.
@task_item n=13 part=needs
Seam errors stay in the closed error word set and carry no path or upstream reply text.
@task_standing n=1
convention: each new test function is named by the test naming rule
home: 2-test-strategy
@task_standing n=2
convention: each new table of cases is written as an rstest case table with literal oracles
home: 2-test-strategy · 11-test-anti-patterns-never-do-these
@task_standing n=3
convention: each new test waits on events or bounded reads, never on sleeps, and is never retried
home: 2-test-strategy · 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: each new fixture file carries synthetic, scrubbed content only and passes the hygiene walk
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=5
convention: each new parser surface gets proptest coverage with committed seeds
home: unit-test-strategy · test-data-bootstrap
@task_item n=1 part=not_needed
MCP surfaces — tools/list, tool arguments, tool results, reply screens: the chunk adds no MCP tool and touches no MCP reply.
@task_item n=2 part=not_needed
The CLI verbs and their outputs — send, answer, wait, last, release, list, revive, run, verify, their exit codes, JSON documents and human lines: no verb is added or changed; viola open belongs to a later chunk.
@task_item n=3 part=not_needed
The wrapper channel, JSON-RPC errors, link and unlink methods and the instance-unreachable causes: the seam has no product caller and speaks no channel.
@task_item n=4 part=not_needed
Hook dialogs, hook decision bodies, hook exec entry points and their performance budgets: nothing on the hook path is touched.
@task_item n=5 part=not_needed
The PTY passthrough, wheel, human input, send confirmation and screen signatures: no terminal spawn or PTY behaviour enters the product.
@task_item n=6 part=not_needed
The instance snapshot, event log records, heartbeat liveness and snapshot replay: the chunk records no window in product state and adds no event kind.
@task_item n=7 part=not_needed
The harness contract commands — boot, cleanup, status, supervise, session records and their readiness codes: the chunk adds no harness command or document.
@task_item n=8 part=not_needed
Revive and start refusals, socket directory and strict-modes refusals: no start path is reached.
@task_item n=9 part=not_needed
Claude CLI version fixtures, verify stamps and the ledger probes: the new fixtures are compositor replies, not claude recordings.
