@task_prose part=chunk
A key the human presses to answer or dismiss the claude CLI's session-feedback survey edits no prompt, so it leaves the wheel with its holder; every other key keeps taking it.
The survey's grammar (which keys answer or dismiss it, and the screen state in which they do) is measured live on the installed CLI before it joins the closed non-editing list; an unmeasured prompt shape still takes the wheel.
The measured grammar becomes a capability-ledger row with a screen signature and a viola verify probe; the fake agent learns to draw a survey screen for CI.
At P3 the chunk re-measures the closed non-editing list, the classification of stdin reads, and what is known of the survey; live claude starts wait on the founder's word.
@task_file n=1 path=src/run/wheel.rs mark=edits
what: the wheel classifier that decides whether a stdin read holds an editing key, against the closed non-editing list — edits: the Classifier / Observed path
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the capability ledger of undocumented claude behaviours, each row with its screen signature and its viola verify probe — edits: a new ledger row for the survey grammar and its screen signature
@task_file n=3 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent used in CI in place of claude — edits: a survey screen it can draw
@task_file n=4 path=fixtures/claude/
what: recorded claude screen payloads per CLI version — a recorded survey screen joins them
@task_item n=1 part=needs
The wheel decision: a stdin read holding a survey key, while the survey screen is shown, leaves the wheel with its holder; any other key, and any prompt shape not measured, still takes the wheel.
@task_item n=2 part=needs
viola verify gains a probe for the survey grammar ledger row; its run reports through the viola verify output (step line, failing row, summary, exit 0 / exit 1, plain ASCII) like every other probe.
@task_item n=3 part=needs
viola verify --record and its refusals (CLI not found, CLI version unreadable, recorded fixture not clean, verify stopped) stand over the new probe and any recorded survey fixture the chunk adds.
@task_item n=4 part=needs
The wheel state the human sees: after a survey key the WHEEL holder shown by viola list and the wheel verbs stays the previous holder; the builder's next send is not routed to the human.
@task_item n=5 part=needs
Mechanism only: viola never answers the survey and never types a survey key itself; the human's key reaches the child unchanged — the classifier only observes, so no new verb, hint, output line or MCP tool appears for the survey.
@task_item n=6 part=needs
The closed non-editing list stays closed: no env var, flag or config key widens it, and the existing entries (focus reports, mouse reports, terminal replies, resizes) stay as they are.
@task_item n=7 part=needs
Screen signatures come only from compiled ledger rows; the survey's screen state must reach the wheel decision through such a signature, never a free-form match.
@task_item n=8 part=needs
New tests are proved on the fake agent and the recorded payloads; live claude starts, their readings and any key a rig types are named at P3's end and wait on the founder's word.
@task_standing n=1
convention: each new viola verify probe reports as one plain-ASCII step line and counts in the summary and exit code like every other probe
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page, and this chunk adds none.
@task_item n=2 part=not_needed
The viola list board layout — column widths, truncation, colour depth, stale rows, DIALOG word, empty board — is untouched; only who holds the wheel is at stake, not how the board draws it.
@task_item n=3 part=not_needed
viola send output, the readback mirror and send refusals — the chunk changes no send outcome, issue line or read back.
@task_item n=4 part=not_needed
viola revive and viola run start output, their refusals and --json error codes — no start path changes.
@task_item n=5 part=not_needed
viola link / unlink output, viola plugin install output, and viola last output — none of these verbs is reached.
@task_item n=6 part=not_needed
The MCP tool list and tool results (answer, send, wait, list, last) — no MCP tool is added or changed; viola never answers the survey.
@task_item n=7 part=not_needed
CLI token rendering (colour, spacing, radius, motion, roles) and the exit-21 frame verb hints and details — no rendering or unreachable-instance path changes.
@task_item n=8 part=not_needed
viola wait output and viola answer output for dialogs — the survey is not a dialog viola answers or waits on.
