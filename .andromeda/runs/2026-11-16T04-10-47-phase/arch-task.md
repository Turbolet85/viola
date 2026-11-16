@task_prose part=chunk
Makes the driver's send confirmation tolerate exactly the whitespace the claude CLI trims from a submitted prompt: the prompt-submitted line is relabelled driver, the send confirmed ok, the wheel kept by the driver; a prompt differing in any other byte still claims nothing.
Keeps the driver's text validated as received (control characters refused, never stripped) and typed without removing the whitespace the CLI will trim; typed-text normalisation and the empty-text refusal stay as they are.
Closes the readiness-gate turn race: a turn starting while a send waits on the pre-send readiness gate never receives the send's text (refused turn-running, or typed only after that turn ends), with a test reproducing the race.
The rule naming what the CLI trims lives in the Claude agent crate and reaches the send path as a normalised predicate; tests stand on the fake agent and recorded payloads, no live claude start.
@task_file n=1 path=src/run/send.rs mark=edits
what: the wrapper's send path — the pre-send readiness gate, the turn check before the paste, the bracketed paste and the send outcome — edits: the readiness gate and the window between the turn check and the paste, the send's text validation and typing
@task_file n=2 path=src/run/send/slot.rs mark=edits
what: the in-flight send slot that matches a prompt-submitted hook event to the in-flight send and confirms it — edits: the slot's claim comparison, its feed from the appended hook event, the human wheel move on an unmatched prompt
@task_file n=3 path=src/run/send/relabel.rs mark=edits
what: the driver relabel of a prompt-submitted line whose text matches the in-flight send (origin driver instead of human)
@task_file n=4 path=src/run/send/classify.rs
what: the local-command list and its classification, which reads the exact typed text of a send
@task_file n=5 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the Claude hook payload shapes and the typed-text rule (typed_text) — edits: a new rule naming the whitespace the CLI trims, exposed as a normalised predicate
@task_file n=6 path=fixtures/fake-scripts/ mark=edits
what: the fake agent's scripts, driving viola-fake-agent to submit a trimmed prompt or start a turn inside the readiness gate's window
@task_item n=1 part=needs
The send-confirmation match: the exact prompt match after agent normalisation changes to tolerate exactly the whitespace the CLI trims and no more; any other differing byte still claims nothing.
@task_item n=2 part=needs
The prompt-submitted relabel to driver origin on a matched send (channel hook.event driver relabel, prompt-submitted origin and text event data) and the send-confirmed record and its order in the events log.
@task_item n=3 part=needs
The wheel on a matched trimmed send stays with the driver and a later driver send is not refused for the wheel; a human's own prompt still takes the wheel (human prompt confirms the wheel take, byte-source wheel claim, wheel decided in wrapper memory, the human always wins the wheel).
@task_item n=4 part=needs
Every send confirmed after the fact; unconfirmed delivery reported not-delivered — the trimmed send must now confirm ok instead of exiting not-delivered (no-prompt-submitted, unconfirmable).
@task_item n=5 part=needs
Paste text validation as received: control characters (C0 other than LF/CR/TAB, DEL, C1) refused not-delivered with the control-character detail, never stripped.
@task_item n=6 part=needs
Typed text normalisation unchanged: CR LF and CR become one LF, trailing CR and LF not typed, send never types a trailing newline; no other whitespace removed before typing; send text line-ending rules.
@task_item n=7 part=needs
An empty typed text stays the empty-text refusal (paste text empty, not-delivered empty-text detail); whether a text made only of CLI-trimmed whitespace needs its own decision is left to P3 and P4.
@task_item n=8 part=needs
Claude shapes isolated in one crate: the trim rule lives in the Claude agent crate, which does no file I/O, and the root bin consumes it only as a normalised predicate (claude shapes in one crate, root bin consumes only normalised events, claude-first with no adapter trait).
@task_item n=9 part=needs
The trim is an undocumented claude CLI behaviour: whether its tolerance stands on a capability-ledger row with a viola verify probe, on the recorded fixtures, or on a closed rule needing no row (capability ledger gate, per-behaviour CLI capability ledger, no probe for unrelied shapes) is read at P3 and decided at P4.
@task_item n=10 part=needs
The readiness-gate turn race: a turn that starts while a send waits on the vt100 pre-send readiness gate never receives the text; the send is refused turn-running or typed only once that turn ended (a send mid-turn is refused, never queued).
@task_item n=11 part=needs
The pre-send readiness gate itself: its settled-input-box predicate, compiled gate timings, the paste-hint wait, the send time bound, and the atomic bracketed-paste send the race window sits between.
@task_item n=12 part=needs
Send refusals keep a closed set of details and the first-refusal-only order; a turn-running refusal from the gate window uses the existing refusal reason and its send-refused event data.
@task_item n=13 part=needs
The wrapper's send records (send-issued, send-confirmed, send-refused appends) and the paste failing after send-issued stay ordered and log-only.
@task_item n=14 part=needs
The CLI send and MCP send outcomes change only as they already read the wrapper's reply (CLI exit codes 0 and 13, MCP send result and refusal).
@task_item n=15 part=needs
The fake agent (viola-fake-agent and its scripts) gains the means to submit a trimmed prompt and to start a turn inside the gate's window, through the fake-agent test seams, never read by the product.
@task_item n=16 part=needs
No live claude start: tests run against the fake agent and the recorded payloads; real CLI probes only locally, and a live start, if the trim's shape needs one, is named and asked for before the review card.
@task_item n=17 part=needs
A test that waits on a channel or a thread takes a bound and cannot hang past a failure (root test waits share one bound, nothing polls the channel).
@task_item n=18 part=needs
Upstream prompt text from the hook payload is untrusted content, parsed tolerantly; the match compares it without trusting or rendering it.
@task_item n=19 part=needs
The local-command classification keeps reading the exact typed text, not a trimmed one.
@task_standing n=1
convention: each new end-to-end or wrapper test runs in its own isolated viola home, against the fake agent
home: established-decisions
@task_standing n=2
convention: fake-agent test seam variables and harness knobs follow their prefixes and are no configuration channel
home: conventions · occupied-resources
@task_standing n=3
convention: test evidence survives a killed test, homes cleaned by owner record, waits share one bound
home: occupied-resources
@task_standing n=4
convention: new code keeps the coverage floors and the zero-panic-line rule of end-to-end runs
home: ci-cd-approach · project-directory-structure
@task_standing n=5
convention: OS-gated tests name their reason; no test gate names another OS
home: cross-cutting-patterns
@task_standing n=6
convention: new error variants stay one thiserror enum per crate with messages interpolating nothing
home: conventions · cross-cutting-patterns
@task_item n=1 part=not_needed
The dialog paths: dialog hold, pending dialog, answer, permission and plan answers — the scope leaves them untouched.
@task_item n=2 part=not_needed
wait's wake rule and the channel wait method contracts (wake kinds, start offset, timeout) — untouched.
@task_item n=3 part=not_needed
The MCP tool surface's shape: the tools list, tool parameters, list results, stdin line caps.
@task_item n=4 part=not_needed
Link and unlink, pause, release and their CLI-only rules; the link set and link events.
@task_item n=5 part=not_needed
Revive, run start refusals, the pinned exe copy, child argv and child environment strip.
@task_item n=6 part=not_needed
The board and list payload, agents listing, session liveness and heartbeat, instance snapshots.
@task_item n=7 part=not_needed
Hyprland seam, window features and the terminal fleet console.
@task_item n=8 part=not_needed
Plugin install, the plugin's .mcp.json and the marketplace tree.
@task_item n=9 part=not_needed
Release tooling, deploy model, dist release workflow and crates.io publishing.
@task_item n=10 part=not_needed
Mutation testing at the epoch boundary: no mutation run in this chunk.
@task_item n=11 part=not_needed
config.json keys and configuration precedence; colour environment variables.
@task_item n=12 part=not_needed
Hook tiers for StopFailure, SessionEnd and the API-error turn end, and the turn-end cause set.
