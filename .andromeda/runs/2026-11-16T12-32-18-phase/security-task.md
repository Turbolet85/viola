@task_prose part=chunk
A key the human presses to answer or dismiss the claude CLI's session-feedback survey edits no prompt, so it leaves the wheel with its holder; every other key keeps taking the wheel.
The survey's grammar (which keys answer or dismiss it, and the screen state in which they do) is measured live on the installed CLI before it joins the stdin classifier's closed non-editing list; an unmeasured prompt shape still takes the wheel.
The measured grammar becomes a capability-ledger row in the Claude adapter with a viola verify probe; the screen signature comes only from a compiled ledger row.
At P3 the chunk re-measures the non-editing list, the classification and what is known of the survey; live claude starts wait on the founder's word, every other step runs on the fake agent and recorded payloads.
@task_file n=1 path=src/run/wheel.rs mark=edits
what: the wrapper's stdin wheel classifier, which decides whether a stdin read holds an editing key against the closed non-editing list — edits: the Classifier and Observed path
@task_file n=2 path=crates/viola-agent-claude mark=edits
what: the Claude adapter's capability ledger, its compiled screen signatures and viola verify's probes — edits: a new ledger row with its screen signature, and its verify probe
@task_file n=3 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent, the test-only stand-in for claude used in CI — edits: a survey screen it can draw
@task_file n=4 path=fixtures/claude mark=edits
what: recorded claude screen fixtures per CLI version — edits: any recorded survey screen
@task_item n=1 part=needs
The stdin classifier's closed non-editing list gains the survey's keys, only in the measured survey screen state; the list holds measured shapes only, and an unmeasured shape keeps taking the wheel.
@task_item n=2 part=needs
Human typing takes the wheel: every key other than a measured survey answer or dismissal still takes it, and the wheel event's human-input cause and the send refusal for human typing stay as they are for those keys.
@task_item n=3 part=needs
The wheel classifier only observes: the human's survey key reaches the child unchanged, and viola never answers the survey nor types a survey key itself in the product.
@task_item n=4 part=needs
The survey's screen state reaches the wheel decision from the pump's vt100 screen model, read only against compiled literals from a ledger row; the screen feed stays bounded, isolated from passthrough and under its panic catch.
@task_item n=5 part=needs
Classification sources are compiled only: the survey signature is a compiled ledger row, never a value read from config, env or the screen text itself; upstream screen text is content, never a command.
@task_item n=6 part=needs
No env var, flag or config key widens the non-editing list or switches the classification off; configuration cannot disable the control.
@task_item n=7 part=needs
The new viola verify probe reads the PTY by signatures only, never answers a dialog and never writes a dialog byte; its PTY output is bounded and caught, its probe directory owner-only and removed, its prompts synthetic.
@task_item n=8 part=needs
The real claude CLI runs only locally, never in CI: CI proves the survey path on the fake agent's survey screen and recorded payloads; live starts, their readings, how the survey is made to appear and any key a rig types wait on the founder's word at P3's end.
@task_item n=9 part=needs
A live-round rig that types a survey key types only into its own pty, in its own isolated home, writing nothing under the Claude home.
@task_item n=10 part=needs
A recorded survey screen fixture is scrubbed or refused whole, keeps only signature rows, stays local where it is a live session screen, and is reviewed before commit.
@task_item n=11 part=needs
The fake agent's survey screen is a test seam available only under the fake-agent feature and compiled out of the release build.
@task_item n=12 part=needs
Focus reports, mouse reports, terminal replies and resizes on the closed non-editing list stay as they are.
@task_standing n=1
convention: each new or recorded claude screen fixture is scrubbed, keeps only signature rows and is reviewed before commit
home: data-protection
@task_standing n=2
convention: each new test seam lives only under the fake-agent feature and is compiled out of release
home: security-anti-patterns · input-validation
@task_standing n=3
convention: each new test uses its own home and removes only directories it created
home: secret-management · security-anti-patterns-never-do-these
@task_item n=1 part=not_needed
Channel socket, peer-euid and server-verification checks, and the wrapper channel methods and their errors: the chunk adds no channel method or frame.
@task_item n=2 part=not_needed
MCP tools, MCP stdin caps and MCP error shapes: the chunk adds or changes no MCP tool.
@task_item n=3 part=not_needed
Paste text and dialog free-text character sets and bracketed-paste send: the chunk changes no send or answer text path.
@task_item n=4 part=not_needed
Hook payload parsing, hook fail-open and hook refusals: no hook event is added or read differently.
@task_item n=5 part=not_needed
Revive, snapshot, link and list rows, with their JSON codes and refusals: none of those surfaces change.
@task_item n=6 part=not_needed
Pinned binary placement and re-hash, plugin file rewrites and the marketplace tree: no install or pin behaviour changes.
@task_item n=7 part=not_needed
Release, signing, attestation and secret-manager rows, and the absences of hosting, network and accounts: the chunk adds no release or network surface.
