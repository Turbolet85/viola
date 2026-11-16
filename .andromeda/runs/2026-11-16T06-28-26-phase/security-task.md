@task_prose part=chunk
Makes a driver's send of a local slash command that the CLI runs read as delivered (ok), confirmed by an echo of that command the product can observe, never presumed; a local command the CLI did not run is never confirmed by it.
Today an unlisted local command is classified as a prompt send, waits for a prompt-submitted the CLI never fires and ends not-delivered / no-prompt-submitted; a listed command with no measured post-condition, or on an unverified CLI, reads ok with confirmed false, detail unconfirmable.
The echo is measured, not assumed: P3 re-measures from the code and the recorded claude fixtures what echo the CLI gives (a hook event, a screen line the screen readiness model reads, or another signal) and which commands it covers; an echo relied on becomes a capability-ledger row with a viola verify probe, measured and stamped.
Re-reads at take-up the three readings a local-command send gives today (listed post-conditions, unconfirmable, no-prompt-submitted) across the send path, its classifier, the CLI send human mirror and the MCP send result.
The plan stands on the fake agent's local-command mode and recorded payloads; no live claude start without asking first, no mutation run.
@task_file n=1 path=src/run/send.rs mark=edits
what: the wrapper's send path — the send method's delivery confirmation and its outcome (ok, confirmed, detail, not-delivered refusal)
@task_file n=2 path=src/run/send/classify.rs mark=edits
what: the send classifier deciding what confirms a send — edits: the classification of a local slash command (today a prompt send outside the compiled list)
@task_file n=3 path=src/run/send/slot.rs mark=edits
what: the send's confirmation slot, where the wrapper waits for the confirming signal within the send's window
@task_file n=4 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the Claude adapter's capability ledger — edits: the local-command list (LOCAL_COMMANDS and its post-conditions), a new ledger row if the echo is relied on
@task_file n=5 path=src/cmd/verify/ mark=edits
what: viola verify, the probe runner that measures and stamps capability-ledger rows — edits: the probe for the local-command echo
@task_file n=6 path=src/run/version_gate.rs mark=edits
what: viola run's version gate, reading the verify stamps to decide a verified or unverified CLI
@task_file n=7 path=src/human/mirror.rs mark=edits
what: the CLI human-mode output mirror of a send's outcome — edits: the send outcome's lines, unconfirmable's fixed note
@task_file n=8 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent test binary standing in for claude — edits: its --local-command-mode
@task_file n=9 path=fixtures/fake-scripts/ mark=edits
what: the fake agent's scripts driving its scripted sessions
@task_file n=10 path=fixtures/claude/ mark=re-reads
what: the recorded claude fixtures read back to measure what echo the CLI gives for a local command
@task_item n=1 part=needs
The send outcome for a local slash command: a run command reads ok, confirmed by its own observed echo; a command the CLI did not run is never confirmed; it no longer ends not-delivered / no-prompt-submitted after its window. Carried on the wrapper channel's send reply and read as-is by the CLI send and the MCP send tool results (their codes, detail and refusal values).
@task_item n=2 part=needs
The honest unconfirmable reading stays where the echo cannot be confirmed: a listed command with no measured post-condition, or any local command on an unverified CLI, reads ok with confirmed false and detail unconfirmable, its human note fixed text.
@task_item n=3 part=needs
Every dependence on an undocumented claude behaviour is a capability-ledger row with a viola verify probe: an echo the product relies on is a ledger row, measured and stamped, and the unverified CLI gate governs what the send may claim.
@task_item n=4 part=needs
Claude shapes stay inside the Claude adapter crate: what counts as a local command's echo is read there, and the send path consumes a normalised fact, not raw Claude output.
@task_item n=5 part=needs
Upstream text (a local command's output) is content, never a viola command or config, and crosses no log, event or error; error and refusal messages stay fixed and interpolate no upstream text, paths or ids.
@task_item n=6 part=needs
The local-command list and any echo signature are compiled classification sources: the echo is matched only against compiled literals, never against config, environment or upstream-supplied patterns.
@task_item n=7 part=needs
If the echo is a screen line: it is read through the vt100 screen model the input-box readiness already uses — compiled literals only, feed bounded and isolated from passthrough, inside a panic catch.
@task_item n=8 part=needs
If the echo is a hook event: the hook payload is content parsed tolerantly, an unknown shape fails to unknown, security-relevant fields are closed enums, invalid hook frames are dropped and the wrapper re-validates hook.event frames.
@task_item n=9 part=needs
The new verify probe: verify reads PTY output by signatures only, never answers or types into a dialog, keeps its effects and captures in its own owner-only probe directory removed by drop guards, uses synthetic probe prompts, and only viola verify writes the stamps file.
@task_item n=10 part=needs
The version gate reads the stamps through the strict-modes check before trusting them; a missing or failing stamp leaves the CLI unverified.
@task_item n=11 part=needs
The local command is still typed as one bracketed paste through the send text checks (control characters refused whole, empty text refused), validated on both client and wrapper side.
@task_item n=12 part=needs
The human mirror escapes C0/C1 control characters in any line it prints for the send outcome.
@task_item n=13 part=needs
The fake agent's local-command mode is a test seam: only under the fake-agent feature, compiled out of release, refused by the release build check.
@task_item n=14 part=needs
Any live claude reading is local only, never in CI, asked for before the review card with its exact count of starts; it runs under a private CLAUDE_CONFIG_DIR inside a started session's turn, viola writing nothing under the Claude home and verify running only in a prepared trusted folder.
@task_item n=15 part=needs
New tests that wait on a channel or a thread take a bound and cannot hang past a failure; a test socket goes under the short /tmp root, never at the temp root itself; a stamped home comes from the shared stamp's copy.
@task_item n=16 part=needs
The send confirmation's wait stays bounded by the send's window; no reader of the echo signal is unbounded.
@task_item n=17 part=needs
Each control's --timeout S is named in the plan, on an entry filtered to the guard's own cases, a control id within 16 characters; no mutation run in the chunk, acceptance thresholds checked at P4 against the class's ci and mutants kills.
@task_standing n=1
convention: each new test runs in its own home, never the real one, removes only the directories it created and follows only a safe linked e2e base; no test home enters git
home: secret-management · security-anti-patterns
@task_standing n=2
convention: each new or re-recorded fixture is scrubbed of paths, usernames and emails or refused whole, keeps only signature rows for screens, and is reviewed before commit
home: data-protection
@task_standing n=3
convention: each new test seam lives only under the fake-agent feature and is compiled out of release
home: security-anti-patterns · input-validation
@task_standing n=4
convention: a new seam or environment read needs its log entry
home: security-anti-patterns-never-do-these
@task_standing n=5
convention: CI holds no secrets and runs no real claude CLI; the real CLI is driven only locally
home: threat-model-summary
@task_item n=1 part=not_needed
The socket, peer-euid and server-verification checks of the authentication area: the chunk adds no listener, channel method or client seam.
@task_item n=2 part=not_needed
The dialog paths, answer behaviour and dialog free-text rules: untouched by this chunk.
@task_item n=3 part=not_needed
The wheel and human-typing-takes-the-wheel rules, and the wait wake rule: untouched.
@task_item n=4 part=not_needed
The MCP tool input schemas, tool list and stdin line cap: the MCP tool surface's shape is unchanged.
@task_item n=5 part=not_needed
link, unlink, revive and the board list (their codes, cwd handling, colour facts, Hyprland reads): not reached.
@task_item n=6 part=not_needed
Pinned binary and plugin install, marketplace and code-bearing file rewrites: not reached.
@task_item n=7 part=not_needed
The CLAUDE credential strip and the child's environment handling: not touched.
@task_item n=8 part=not_needed
Release, signing, self-update and the fuzz workspace: no release stage in this chunk.
@task_item n=9 part=not_needed
config.json keys: the chunk adds no configuration value.
@task_item n=10 part=not_needed
Event log rotation, growth and link lines: no change to event storage.
