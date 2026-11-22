@task_prose part=chunk
Adds a compiled time bound to the wrapper's paste write into the child's PTY, so the wrapper's send never waits without end on a child that stops reading.
At the bound the send ends with a closed word the driver reads through the CLI send and the MCP send tool, never upstream text, and the wheel is not moved.
Re-measures first, with a fake agent that stops reading its PTY, whether today's unbounded paste write holds send past the readiness gate's and the confirm window's bounds and past the MCP call deadline.
Disposes what a cut paste leaves in the child's input box (text with no Enter) for the next send, the human and the confirmation, and what the human's keys do while a paste is blocked and after it is cut.
@task_file n=1 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY seam's pump, its write contract into the child's PTY and its callers — edits: the paste write, today one blocking write with no time bound
@task_file n=2 path=crates/viola-pty/src/lib.rs mark=edits
what: the PTY crate's test module and its child-wait bound — edits: the test-module comment over the child-wait bound naming the mutants profile's kill
@task_item n=1 part=needs
The PTY pump's write contract: the wrapper's paste into the child's PTY gains a compiled time bound, so no write into a child that stops reading blocks without end.
@task_item n=2 part=needs
The send path's time bounds and who waits: the paste bound plus the rest of send stays under send's own compiled total, the readiness gate's and the confirm window's bounds, and the MCP call deadline of 55 s with its expiry cause call-deadline, the arithmetic shown.
@task_item n=3 part=needs
The send outcome at the bound: a closed word the driver reads, never upstream text; if a new word is added it joins the closed refusal, detail or cause sets in the core crate and is read by clients as a code word only.
@task_item n=4 part=needs
The MCP send tool's result at the bound: its isError structuredContent refusal, detail or error code, and the MCP frame tool result, carrying closed codes and no upstream text.
@task_item n=5 part=needs
The CLI send's answer at the bound: its exit code and its --json and human lines, fixed messages that interpolate nothing and carry no path or upstream text.
@task_item n=6 part=needs
The wrapper channel's send method reply and its result refusal or detail at the bound, and the channel-response log line's detail for it.
@task_item n=7 part=needs
The wheel: a send cut at the bound moves no wheel and writes no wheel event; the human typing takes the wheel rule and the human-input wheel event stay as they are.
@task_item n=8 part=needs
What a human key may be held behind: the human's keys while a paste is blocked and after it is cut — security refusals and bounds never block the human.
@task_item n=9 part=needs
Delivery confirmation and never presumed delivered: a cut paste leaves text with no Enter in the child's input box; the send is not reported delivered, and what the next send and the confirmation meet is disposed.
@task_item n=10 part=needs
Send text is one bracketed paste: a bound that cuts the write mid-paste leaves the bracketed paste unfinished; the cut's handling must keep the one-paste rule's meaning for the next send.
@task_item n=11 part=needs
The send flight lock: a send cut at the bound releases its flight so the next send is not held behind it.
@task_item n=12 part=needs
The bound is compiled: no config key, environment variable or flag sets or switches it off.
@task_item n=13 part=needs
The PTY crate stays a sync crate free of tokio: the write bound is built without bringing tokio into it.
@task_item n=14 part=needs
The fake agent gains a behaviour that stops reading its PTY, a test seam under the fake-agent feature only, compiled out of the release build.
@task_item n=15 part=needs
No live start and no rig start of a real claude: the measure runs on the fake agent alone; the real claude CLI runs only locally and never in CI.
@task_item n=16 part=needs
Tests that wait take a bound under the 20 s lint and each test start runs under a test-owned user home; no mutation run in this chunk, mutation testing staying at the epoch audit with its profile's kill above the child-wait bound.
@task_item n=17 part=needs
Full detail of a cut paste goes only to diagnostics under the never-log floor: no typed text or upstream text on stderr or in external output.
@task_item n=18 part=needs
The pinned product drives the operator's session and is untouched: the pinned viola is never replaced; the change reaches the driver only at the re-pin.
@task_standing n=1
convention: each new test runs under its own test-created home and removes only the directories it created
home: secret-management
@task_standing n=2
convention: each new test seam lives only under the fake-agent feature and is compiled out of the release build
home: security-anti-patterns · security-anti-patterns-never-do-these
@task_standing n=3
convention: each new seam or environment read gets its log entry
home: security-anti-patterns-never-do-these
@task_standing n=4
convention: each new test or CI job runs with no real claude CLI and no secrets
home: threat-model-summary
@task_standing n=5
convention: each new log or error line carries no user content or upstream text outside diagnostics and interpolates nothing
home: bootstrap-phases-derive-for-route-setup-project
@task_item n=1 part=not_needed
Peer euid checks, socket-directory checks and socket modes: the chunk adds no seam, listener or socket and changes no client check order.
@task_item n=2 part=not_needed
Revive, list, link and unlink, the hook and its dialog surfaces: none of them is on the send path the chunk bounds.
@task_item n=3 part=not_needed
Live rig arms, verify probes and their residuals, and recorded fixture hygiene: no live or rig start and no fixture is recorded.
@task_item n=4 part=not_needed
Paste and dialog text character sets and the control-character refusals: the text checked before the paste is unchanged.
@task_item n=5 part=not_needed
Credential and environment strips of the claude child: the chunk spawns no new child and changes no child environment.
@task_item n=6 part=not_needed
Pinned binary placement and re-hash, plugin files rewritten at start, and the trust record writer: none is touched.
@task_item n=7 part=not_needed
rmcp feature sets, action pinning, workflow permissions and release attestations: the chunk changes no workflow or release stage.
@task_item n=8 part=not_needed
config.json keys such as the dialog hold and the env keep list: the chunk adds no key.
