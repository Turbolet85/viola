@task_prose part=chunk
Builds one standing, committed live rig that every later live round reuses: it starts the real claude CLI (or viola wrapping it) in a pty it owns, keeps a terminal screen model of what the CLI draws, keeps the screens it reads as evidence, and reads the CLI's trust dialog and modal dialogs by their drawn literals and selector rows.
The rig owns its isolation from the operator's real Claude config and login and its process hygiene, each property proven once by a named check on the fake agent with no live start.
It carries live readings to take on the rig: which whitespace the CLI trims at submit, the hook-latency window before a send's paste, and whether a fired scheduled wake's prompt-submit payload carries a source field and makes turn-ended scheduled_wakes read 1.
It re-measures, from the earlier rounds' evidence scripts and reports, what each earlier rig built for itself and got wrong; the rig's home (a test-only binary of the test-only crate, a script, or both) and its screen model are settled at P3.
No product behaviour changes; the product binary gains no verb and no surface.
@task_item n=1 part=needs
The live rig drives the real claude CLI locally only: a live start is never part of CI, is not a test layer, and a fake-agent run is not proof of real-CLI behaviour, so each live reading is taken on the rig itself.
@task_item n=2 part=needs
The rig's screen model reads the CLI's trust dialog and modal dialogs only by their compiled drawn literals and selector rows (box rows recorded as their literals alone); the product's own rule that the rendered child screen is never parsed for verdicts stays as it is.
@task_item n=3 part=needs
The fake agent's fidelity rides the rig: its modal fidelity (the trust dialog's literal and selector rows as the live CLI draws them) and its DEL fidelity on a key after a resize are held against what the rig reads live; the fake agent stays the contract its recorded fixtures pin.
@task_item n=4 part=needs
Where the rig and its checks may live and what they may expose: test-only code in the one test-only workspace member or a script, never a verb or surface of the product binary, never in a release build; tokio-using test code confined to the E2E crate.
@task_item n=5 part=needs
Isolation the rig owns: a scratch root outside every repository, a private config dir, a credentials copy guarded by the token-window read and removed on every exit path, and no write to the operator's real ~/.claude or ~/.claude.json left unexplained; the rig never reads the host's claude sessions.
@task_item n=6 part=needs
A live start is refused under 20 minutes of token left, read from the token window; that value is never counted or written in any record.
@task_item n=7 part=needs
No host path, nothing of the operator's status line or config, and no credential reaches a record, a kept screen, a log line or an evidence file the rig writes; kept screens are scrubbed and checked for leaks.
@task_item n=8 part=needs
Each isolation property is proven once by its own named check run on the fake agent, with no live start.
@task_item n=9 part=needs
Process hygiene: the rig and any booted test it stands for end every process they start on every exit path, the rig's own kill deadline included, with kills verified by pid and start time and children in their own process group; proven by a census of processes under the rig's scratch root at exit or a stronger means.
@task_item n=10 part=needs
A viola the rig starts gets an environment cleared of the inherited session and agent-identity variables, as a test-started viola does.
@task_item n=11 part=needs
Record-home rule, the live rig's half: the home a live viola verify --record takes comes from a helper like the tests' record_home() and holds no path component named home.
@task_item n=12 part=needs
Live reading of which whitespace the CLI trims at submit on the interactive input box (leading, trailing, inside a paste); the send confirmation's trailing-only trim tolerance (ASCII space, TAB, CR, LF) stays unchanged until that reading lands.
@task_item n=13 part=needs
Live reading of the hook-latency window: a turn the CLI starts whose UserPromptSubmit has not reached the wrapper before a send's paste, which the flight lock closes only for a turn the wrapper has learned of.
@task_item n=14 part=needs
Live reading of a fired scheduled wake: whether its prompt-submit payload carries the optional source field (schedule_wakeup, loop_wakeup) on which CLI versions, the raw payload shape captured as closed words, and whether turn-ended scheduled_wakes reads 1 on a real wake (one start, two sends, the wrapper stopped before the wake is due); the classification of a harness prompt stays as it is.
@task_item n=15 part=needs
Every step that needs no live start is built and proven on the fake agent: a test waiting on a channel or a thread takes a bound, a test socket lives under the short /tmp root, and a stamped home is taken from the shared stamp's copy.
@task_item n=16 part=needs
No mutation run in this chunk.
@task_item n=17 part=needs
The rig's own steps and files, whether rig plus readings fit one implement window, a split if not, and each live start by its shape, count and typed keys are stated in one block at the end of P3, before P4.
@task_standing n=1
convention: each new test synchronises by waiting on logged events or file state under a bound, never by sleep or retry
home: 11-test-anti-patterns-never-do-these
@task_standing n=2
convention: each new test creates its own private home and seeds its own data, never hand-written product state
home: 7-test-data-fixtures
@task_standing n=3
convention: each new test function is named by the test-function naming rule and lives in its layer's place
home: 2-test-strategy
@task_standing n=4
convention: each new test input or kept fixture is synthetic, scrubbed of real data, and carries the canary where the plan asks it
home: test-data-fixtures
@task_item n=1 part=not_needed
MCP server and tool surfaces (tools/list, tool results, invalid-params fields, reply screens): the rig drives the CLI in a pty and changes no MCP behaviour.
@task_item n=2 part=not_needed
Outputs, documents and exit codes of the list, last, answer, release, wait and revive verbs, and the revive and start refusal codes: the product gains no verb and no surface and these behaviours are untouched.
@task_item n=3 part=not_needed
Wrapper channel JSON-RPC error bodies, link and unlink methods and their log lines: the chunk changes nothing the channel serves.
@task_item n=4 part=not_needed
Hook dialog decision bodies, the dialog answer mapping and dialog hold expiry: the rig reads the CLI's modal dialogs on screen and changes no hook decision.
@task_item n=5 part=not_needed
Performance budgets and the hyperfine perf measurement of hook deadlines: the rig times no hook path.
@task_item n=6 part=not_needed
Heartbeat staleness classification, snapshot replay and torn-append healing: no state file's writing or reading changes.
@task_item n=7 part=not_needed
Fuzz targets and their corpora: the chunk adds no product parser surface.
