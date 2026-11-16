@task_prose part=chunk
A key the human presses to answer or dismiss the claude CLI's session-feedback survey edits no prompt, so it leaves the wheel with its holder; every other key keeps taking the wheel.
The survey's grammar (which keys answer or dismiss it, and the screen state in which they do) is measured live on the installed CLI before it joins the closed non-editing list; an unmeasured prompt shape still takes the wheel.
The measured grammar becomes a capability-ledger row in the Claude agent crate, with its compiled screen signature and a viola verify probe; the fake agent learns to draw a survey screen for CI.
P3 re-measures the closed list, today's wheel classification and what is known of the survey; live claude starts wait on the founder's word at P3's end.
@task_file n=1 path=src/run/wheel.rs mark=edits
what: the wheel classifier in the wrapper, which decides whether a human stdin read holds an editing key and so moves the wheel by byte source — edits: the Classifier and Observed path, the closed non-editing list it applies
@task_file n=2 path=crates/viola-agent-claude mark=edits
what: the Claude agent crate holding the per-behaviour CLI capability ledger, its compiled screen-signature literals and the viola verify probes — edits: a new ledger row for the survey grammar, its screen signature, its verify probe
@task_file n=3 path=fixtures/claude mark=edits
what: recorded claude CLI fixtures per CLI version, written only from a clean verify — edits: a recorded survey screen
@task_item n=1 part=needs
The closed list of non-editing bytes gains the survey's answer and dismiss keys only while the measured survey screen state holds; it stays closed, every key outside it still claims the wheel by byte source, and focus reports, mouse reports, terminal replies and resizes keep their place on it.
@task_item n=2 part=needs
The human always wins the wheel: a shape not measured and compiled reads as an editing key and moves the wheel to the human, and the wheel keeps being decided in wrapper memory and returning only through an explicit release.
@task_item n=3 part=needs
The wheel classifier only observes: the human's survey key reaches the child unchanged and unheld, never delayed behind a resize or a screen-model read; passthrough never waits on the screen model.
@task_item n=4 part=needs
Mechanism, not policy: viola never answers the survey and never types a survey key in the product; no typing into a CLI dialog by the product or by verify.
@task_item n=5 part=needs
The survey's screen state must reach the wheel decision from the pump's vt100 screen model, read only through a compiled screen-signature literal that comes from a compiled ledger row; the rules on which roles may read screen content (run never reading screen content, screen rows read only by verify) bound how that state is obtained.
@task_item n=6 part=needs
Every dependence on an undocumented claude behaviour is a capability-ledger row with a viola verify probe; the survey grammar is one such row, and the ledger stays the single gate for CLI behaviour.
@task_item n=7 part=needs
A CLI version is verified only when every relied row passes; a new relied row joins that set, its probe's step and summary lines and its failure verdict follow the verify output and exit contracts, and stamps are written only by verify.
@task_item n=8 part=needs
On an unverified CLI or with the survey row failing, the survey exemption does not apply: the degrade keeps the human-takes-the-wheel behaviour.
@task_item n=9 part=needs
Claude-specific shapes stay isolated in the agent crate, which does no file I/O; the root bin consumes the signature and the ledger verdict, not raw Claude shapes; no adapter trait is added.
@task_item n=10 part=needs
No environment variable, CLI flag or config key widens the non-editing list: environment variables are no configuration channel and test seams are not configuration.
@task_item n=11 part=needs
The fake agent gains a survey screen it can draw for CI, reached through the fake-agent test seam variables and features; CI runs against the fake agent and never starts a live claude.
@task_item n=12 part=needs
Real claude CLI probes run only locally, bounded to verify's child set; live starts for this chunk are capped and named at P3's end, every other step planned on the fake agent and recorded payloads.
@task_item n=13 part=needs
A recorded survey screen fixture is written only from a clean verify, scrubbed and hygiene-walked, keeping only signature rows, with the unclean-fixture refusal guarding it.
@task_item n=14 part=needs
The wheel event and its event data are appended only when the wheel truly moves; a survey key now appends no wheel event and no human take.
@task_item n=15 part=needs
Upstream survey text on the screen is untrusted content: it is never logged in codes-only process logs nor rendered into human lines.
@task_standing n=1
convention: each new test runs in its own isolated viola home, cleaned by its owner
home: established-decisions
@task_standing n=2
convention: each new fake-agent behaviour is reached through the named fake-agent test seam variables, never a product configuration channel
home: conventions
@task_standing n=3
convention: each new recorded fixture is scrubbed, hygiene-walked and keeps only signature rows
home: occupied-resources
@task_standing n=4
convention: new code and tests stay within the coverage floors enforced by the gate
home: ci-cd-approach
@task_standing n=5
convention: new insta snapshots run in check mode only, and end-to-end runs show zero panic lines
home: project-directory-structure
@task_standing n=6
convention: OS-gated tests name their reason and no test gate names another OS
home: cross-cutting-patterns
@task_item n=1 part=not_needed
MCP server rows (rmcp, MCP tools, MCP stdin caps, MCP tool results): the chunk adds no tool and changes no MCP output.
@task_item n=2 part=not_needed
Revive rows (revive refusals, revive JSON documents and exit codes): nothing in the chunk resumes a session.
@task_item n=3 part=not_needed
Link and unlink rows (link recording, link events, link exit codes): the survey key touches no driver-to-driven link.
@task_item n=4 part=not_needed
Board and list rows (list payload, agents listing, session liveness): the chunk changes no listing.
@task_item n=5 part=not_needed
Hyprland seam and window rows: no compositor work.
@task_item n=6 part=not_needed
Hook-to-kind map, hook tiers and hook exit rows: the survey is a screen, not a hook event, and no hook changes.
@task_item n=7 part=not_needed
Release, dist, licence and crates.io publishing rows: no release work.
@task_item n=8 part=not_needed
Snapshot replay and instance snapshot field rows: no snapshot field is added or read anew.
@task_item n=9 part=not_needed
Channel protocol fault codes and frame envelope rows: no channel method or frame changes.
