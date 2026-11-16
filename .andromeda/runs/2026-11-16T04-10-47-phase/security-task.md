@task_prose part=chunk
Makes the driver's send confirmation tolerate exactly the whitespace the claude CLI trims from a submitted prompt: a trimmed prompt is matched to the in-flight send, relabelled driver, confirmed ok, and the wheel stays with the driver.
Keeps the driver's text validated as received: a refused control character is still refused whole, never stripped, and viola types the text without removing the whitespace the CLI will trim (line-ending normalisation unchanged, empty typed text still refused).
Closes the readiness-gate turn race: a turn that starts while a send waits on the settled-input-box predicate never receives the text; the send is refused turn-running or typed only after that turn ends, and a test reproduces the race.
Re-measures from the code and the recorded payloads what the product compares and what the CLI's submitted prompt shows of the trim, on the fake agent and recorded fixtures, with no live claude start planned.
@task_file n=1 path=src/run/send.rs mark=edits
what: the wrapper's send path: the pre-send readiness gate on the settled input box, the in-flight send and the turn check before the bracketed paste — edits: the readiness-gate wait, the turn check before the paste
@task_file n=2 path=src/run/send/slot.rs mark=edits
what: the in-flight send slot that matches a hook's submitted prompt to the driver's send for delivery confirmation — edits: SendSlot::claim, the prompt comparison fed by append_hook_event
@task_file n=3 path=src/run/send/relabel.rs mark=edits
what: the relabel of a prompt-submitted event line to origin driver and the move of the wheel to the human on an unmatched prompt
@task_file n=4 path=src/run/send/classify.rs
what: the stdin classifier's closed local-command list, read against the exact typed text
@task_file n=5 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the Claude adapter's hook payload reading and typed-text rule (CR LF and CR as one LF, trailing CR and LF not typed) — edits: typed_text, a new rule naming the whitespace the CLI trims, exposed as a normalised predicate
@task_file n=6 path=fixtures/fake-scripts/ mark=edits
what: the fake agent's scripts used by tests in place of the real claude CLI, where it must submit a trimmed prompt or start a turn inside the readiness gate's window
@task_item n=1 part=needs
Delivery confirmation stops being exact equality: the hook's submitted prompt matches the in-flight send when the two differ only by the whitespace the CLI trims, no more; a prompt differing in any other byte claims nothing.
@task_item n=2 part=needs
A matched trimmed prompt: the hook.event prompt-submitted line is relabelled driver, the send exits ok, the wheel stays with the driver, and a later driver send is not refused human-typing.
@task_item n=3 part=needs
Wheel rule the new match must stay inside: a prompt that does not match the in-flight send is still the human's own, takes the wheel and emits the wheel event with cause human-input.
@task_item n=4 part=needs
The send text's control-character check over the paste text character set: a C0 other than LF, CR or TAB, DEL or C1 is refused not-delivered / control-character, the text refused whole and never stripped, with line breaks and tabs allowed.
@task_item n=5 part=needs
Validation on both sides through the shared core validators: the client checks the text before any frame and the wrapper re-checks it authoritatively on the send params, both on the text as received.
@task_item n=6 part=needs
Typed text normalisation stays as it is (CR LF and CR as one LF, trailing CR and LF not typed) and nothing else is taken off; an empty typed text is still refused empty-text.
@task_item n=7 part=needs
The send is still typed as one bracketed paste, untrimmed, with no extra keystroke added to reach the CLI's trimmed form.
@task_item n=8 part=needs
Readiness-gate race: the pre-send gate reads input-box readiness from compiled screen literals; a turn starting during that wait makes the send refuse turn-running or wait for the turn's end, never paste into the running turn.
@task_item n=9 part=needs
The turn-running refusal and the confirmation outcome reach the CLI send and MCP send only through the wrapper's existing reply: channel result refusal codes, CLI exit codes and MCP isError structuredContent refusal, closed code words read by the clients.
@task_item n=10 part=needs
Claude shapes stay in the Claude adapter crate: the rule naming what the CLI trims lives there and the send path consumes it as a normalised predicate, with no Claude literal in the core send path.
@task_item n=11 part=needs
The hook's prompt field is untrusted content: parsed tolerantly from the hook payload, compared only, never interpreted as a command, and the hook.event frame revalidated by the wrapper.
@task_item n=12 part=needs
The submitted prompt and the send text stay out of error messages, refusals and stderr: error messages interpolate nothing, user content goes only to diagnostics.
@task_item n=13 part=needs
The trim is undocumented claude CLI behaviour: the plan stands on the fake agent and the recorded fixtures; the real CLI runs only locally and never in CI, and any live start is asked for before the review card.
@task_item n=14 part=needs
Fake-agent test seams to submit a trimmed prompt or start a turn inside the gate's window (as the pump delay seam does): reachable only under the fake-agent feature and compiled out of the release build.
@task_item n=15 part=needs
Test proof: no mutation run in this chunk (mutation testing stays at the epoch audit); every guard control fires with a timeout of 120 s or less on an entry filtered to its own cases; a test waiting on a channel or thread takes a bound and cannot hang past a failure.
@task_standing n=1
convention: each new test runs under its own viola home, never the user's real home
home: secret-management
@task_standing n=2
convention: each new test seam lives only under the fake-agent feature and is compiled out of release
home: security-anti-patterns
@task_standing n=3
convention: each new or re-recorded fixture is scrubbed of paths, usernames and emails and reviewed before commit
home: data-protection
@task_standing n=4
convention: each new seam or environment read needs a log entry
home: security-anti-patterns-never-do-these
@task_item n=1 part=not_needed
Socket directory, socket mode, peer-euid and server-verification rows: the chunk opens no channel and changes no bind or connect path.
@task_item n=2 part=not_needed
Strict-modes refusal contracts of the home tree, stamps and snapshot across the verbs: the chunk adds no trusted state read.
@task_item n=3 part=not_needed
revive arguments, its --json codes and session listing: the chunk does not touch revive.
@task_item n=4 part=not_needed
link and unlink methods, their replies and the event log's link lines: untouched by the send match.
@task_item n=5 part=not_needed
Pinned binary placement and re-hash, plugin file rewrites and the marketplace tree: no install or pin path changes.
@task_item n=6 part=not_needed
rmcp feature set and the MCP stdin line caps, and the MCP tool list's shape: the MCP tool surface's shape is untouched.
@task_item n=7 part=not_needed
Dialog answer paths, dialog free text and the dialog hold setting: the scope leaves the dialog paths untouched.
@task_item n=8 part=not_needed
wait's wake rule and the wait and last params: untouched by this chunk.
@task_item n=9 part=not_needed
Hyprland recorder, hyprctl deadline and terminal colour facts: no window-manager or list output change.
@task_item n=10 part=not_needed
Release workflow artefact attestations, self-update signatures and the release cache rules: no release stage change.
@task_item n=11 part=not_needed
Event log rotation, growth risk and log file modes: no new log file or rotation change.
@task_item n=12 part=not_needed
The release method's driver-origin guard and pause and release taking from from VIOLA_NAME: the release verb is not changed.
@task_item n=13 part=not_needed
config.json keys such as claude_env_keep, diagnostics_level and dialog_hold_s: the chunk adds no configuration.
