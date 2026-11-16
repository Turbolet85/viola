@row_set n=1 row=contract/driver-send/local-command cell=promise seen=1 detector=none
value: A ledger-listed local command is classified by its name and confirmed only by its post-condition, else reported unconfirmable, never presumed delivered.
basis: fct-ywi7ddhz
because: Classification moved from exact text to by name (a name, or a name, one space and a non-empty rest), so the promise's exact-equality match is now false.
@row_set n=2 row=decision/local-command-post-condition cell=rule seen=1 fact=F2 detector=none
value: On a verified CLI /clear is confirmed only by a session-start of cause clear with a new session id, /compact only by one of cause compact with the unchanged id, and /mcp only by a new stdout tag in the transcript past its pre-paste end; none by a prompt-submitted, a missed one is not-delivered.
basis: fct-ywi7ddhz
because: /mcp joins the listed local commands with the transcript's stdout tag count as its post-condition, so the rule naming only /clear and /compact is incomplete.
@row_set n=3 row=decision/local-command-unconfirmable cell=rule seen=1 fact=F3 detector=none
value: A local command with no measured post-condition (/cost, /usage, /remote-control), or sent to an unverified CLI, or an echo-confirmed one whose flight remembers no transcript or whose transcript end is unreadable, gives unconfirmable with no confirmation window.
basis: fct-ywi7ddhz
because: An echo-confirmed command without a remembered or readable transcript now reads unconfirmable, and /cost, /usage and /remote-control carry no post-condition, so the rule must name both.
@row_set n=4 row=contract/send-result/unconfirmable cell=promise seen=1 detector=none
value: A listed local command with no measured post-condition, any listed command on an unverified CLI, or an echo-confirmed command with no remembered transcript or no readable transcript end, yields ok with confirmed false, detail unconfirmable and the cursor at once, with no confirmation window.
basis: fct-ywi7ddhz
because: The send's echo start records no transcript end when the flight has none or it is unreadable, and that send's confirms becomes unconfirmable, a case the promise does not yet hold.
@row_set n=5 row=decision/fake-agent-replays-recorded-bytes cell=rule seen=1 detector=none
value: The fake agent offers each hook the recorded fixture bytes and builds no payload, save the fields it sets: the typed prompt of a default UserPromptSubmit, the launch SessionStart's source and session_id under --resume, its transcript_path under --transcript, and the compact replay's session_id.
basis: fct-m2rgle3l
because: Under --transcript the fake agent now sets transcript_path on the launch SessionStart alone, a field the rule's closed list of set fields does not hold.
@row_set n=6 row=contract/fake-agent-hooks/stdin-payload cell=promise seen=1 detector=none
value: The payload on a hook's stdin is the recorded fixture's bytes, save the fields a prompt, a resume, a compact replay or the launch SessionStart under --transcript sets.
basis: fct-m2rgle3l
because: The launch SessionStart under --transcript carries a transcript_path the recorded bytes lack, so the promise's list of set fields is incomplete.
@row_retire n=1 row=decision/local-command-exact-match seen=1 heir=decision/local-command-by-name detector=none
why: Local commands are no longer recognised by exact equality of the typed text but by their name, which the row's key names wrongly.
basis: fct-vjdxki2s
because: LOCAL_COMMANDS moved from 3 to 6 names and classification from exact text to by name, so the exact-match row is false and its key cannot carry the new rule.
@row_add n=1 row=decision/local-command-by-name area=test-scope-summary fact=F1 detector=none
name: Local commands classified by name
rule: Send recognises a local command by its name: the typed text is exactly a listed name, or a listed name, one space and a non-empty rest not starting with a space; the compiled list holds /clear, /remote-control, /compact, /mcp, /cost and /usage.
basis: fct-ywi7ddhz
because: The classification by name and the six-name list replace the retired exact-match rule, and the planner must read them to add or test a local command.
@row_add n=2 row=decision/transcript-path-shape-check area=unit-test-strategy fact=F4 detector=none
name: Transcript path accepted only in its own shape
rule: A SessionStart transcript path is accepted only when absolute, with no .. component, its file name exactly the session id plus .jsonl for a valid session id of its own SessionStart; it is checked hook-side and again wrapper-side, and any other transcript param is dropped, the line still taken.
guard: session_start_transcript_is_kept_only_for_its_own_shape, run_hook_event_transcript_is_re_validated
basis: fct-ywi7ddhz
because: The transcript path now crosses from the hook to the wrapper and is held to a shape check on both sides, a control no row of the plan holds.
@row_add n=3 row=contract/hook-event-method/transcript area=unit-test-strategy fact=F5 detector=none
boundary: the wrapper channel hook.event method
item: transcript
promise: An optional string param beside event, never inside it, sent only for a SessionStart holding a valid transcript path; the wrapper re-validates it for a session-start line alone against that line's own session id.
basis: fct-ywi7ddhz
because: hook.event gains a sibling param, a change of a channel method's promise the plan has no row for.
@row_add n=4 row=decision/transcript-read-bounded-count-only area=6-e2e-test-strategy fact=F6 detector=none
name: Transcript read bounded, a tag count alone
rule: The session transcript is read with a no-follow open, only as a regular file owned by the effective uid, through MAX_FRAME, and only a stdout tag count is kept; a tag split across the start offset is not counted.
why: The transcript is no contract; its stdout tag count is the one exception, with no content crossing.
guard: the echo_ unit cases of the send's transcript reader
basis: fct-ywi7ddhz
because: A new reader of an external file joins the send path with its own controls, which the plan must hold for the tests that guard them.
@row_add n=5 row=decision/local-command-echo-row-check area=unit-test-strategy fact=F7 detector=none
name: local-command-echo relied row check
rule: The local-command-echo relied ledger row passes only when no UserPromptSubmit carries the probe command and the measured echo latency lies within the fallback confirmation window.
why: It follows the compact row's pattern, against the send's 10 s window.
guard: check_local_command_echo_needs_a_tag_within_the_window_and_no_prompt
basis: fct-lel7lpiz
because: A new relied ledger row arrives with a check the plan's ledger rows do not name.
@row_add n=6 row=decision/verify-echo-probe-paste area=integration-test-strategy fact=F8 detector=none
name: Run B's echo probe paste
rule: viola verify's Run B pastes /mcp disable viola-probe-absent seventh, after the newline paste and before /compact, waiting up to the probe deadline for a stdout tag or a prompt; with no readable, shape-checked transcript the paste is skipped and the local-command-echo row fails.
why: Ratified as the probe of the 22nd relied row; without a transcript nothing could be measured.
basis: fct-kdqc42gr
because: Verify's Run B gains the probe of the new relied row and its skip rule, which the verify test cases rely on.
@row_add n=7 row=contract/fake-agent-cli/transcript area=7-test-data-fixtures fact=F10 detector=none
boundary: viola-fake-agent command line
item: --transcript
promise: The argv-only --transcript creates <exe dir>/transcripts/ at 0700 at start and gives the launch SessionStart a transcript_path to a seeded 0600 file named by its session id; replayed framing variants are not rewritten.
basis: fct-ywi7ddhz
because: The fake agent gains an option the tests and verify drive, beside the other command-line option rows.
@row_add n=8 row=contract/fake-agent-input/echo-command area=7-test-data-fixtures fact=F11 detector=none
boundary: viola-fake-agent input
item: echo-confirmed local command
promise: Under --local-command-mode a submitted local command whose post-condition is the stdout tag, and under --framing the exact verify echo probe command, append one stdout tag line to the last started session's transcript and fire no hook.
basis: fct-ywi7ddhz
because: The fake agent gains an input behaviour the echo send cases and Run B's probe rely on, beside the /clear and /compact input rows.
@row_add n=9 row=decision/stamping-verify-passes-transcript area=test-data-bootstrap fact=F12 detector=none
name: Stamping verifies give the fake agent a transcript
rule: Every stamping viola verify against the fake agent, in the harness boot and the stamped-home helpers, passes the fake agent --transcript beside --framing.
why: Without a transcript Run B's echo probe is skipped and the local-command-echo row fails.
except: The framing-free verify helper passes no --transcript.
basis: fct-rxehbcl7
because: The harness boot and the stamped-home helpers now pass --transcript, a convention every new stamping helper must keep for its stamp to hold the 22nd row.
@row_none n=1 fact=F9 type=4
relied: the local-command-echo row's check reads the stamp's echo_latency_ms.
@row_none n=2 fact=F13 type=9
what: open residuals of the echo probe, unmeasured and carried by the route (the real-config write, the input-history line, /context's echo shape).
