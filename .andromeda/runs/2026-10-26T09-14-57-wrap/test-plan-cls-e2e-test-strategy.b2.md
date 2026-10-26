# Classified — test-plan · e2e-test-strategy.b2

## Answers
e2e-test-strategy#6.4 · no · its Path 5 cleanup (Ctrl-C into the outer PTY, child.wait, TempDir drop) is harness-side, in no reached file, and names no exit or document the server check changes
e2e-test-strategy#7.1 · no · its steps run send and release with unchanged arguments against the recorded wrapper, where the new pre-frame check passes, and feed the statusline arm in src/cmd/hook/statusline.rs, which is not reached
e2e-test-strategy#7.2 · no · its budget signals (send exit 11, the release document, the statusline marker, budget.json) are the wrapper's replies and the unreached statusline arm, and the change alters only the pre-frame check and its mismatch path, which this run never takes
e2e-test-strategy#7.3 · no · the MCP budget-paused refusal and its CLI-equal document are the recorded wrapper's reply behind the pre-frame check in tools.rs, which passes there, and the open-gate documents of wait, last, answer and pause are unchanged on that path
e2e-test-strategy#7.4 · no · its cleanup (harness cleanup) is in no reached file
e2e-test-strategy#8.1 · no · its steps run send, answer and the fake agent's PreToolUse dialog against the recorded wrapper with unchanged arguments, where the new pre-frame checks in send, answer and the hook pass
e2e-test-strategy#8.2 · no · the send, the answer exit 12 unverified-cli, the MCP refused text and the hook's empty stdout are the recorded wrapper's replies behind pre-frame checks that pass there, and the change alters only the mismatch path
e2e-test-strategy#8.3 · no · its cleanup (harness cleanup and the TempDir drop for H3) is in no reached file
e2e-test-strategy#9.1 · no · the R8 strip runs in viola run and the fake agent, neither of which is a reached file
e2e-test-strategy#9.2 · no · its env-strip, fd, process-start and secret-scan signals sit in run and the fake agent, neither a reached file, so no reached file carries them
e2e-test-strategy#9.3 · no · its cleanup (TempDir drop) is in no reached file
e2e-test-strategy#10.1 · no · the implicit link is the wrapper's record (source wrapper) on the recorded wrapper, where the pre-frame checks in send and answer pass, and the explicit link and unlink verbs stay unbuilt
e2e-test-strategy#11.1 · yes · contradict · the entry it names closes without the snapshot readers, which the session re-pins to "Home and code-bearing file integrity", and the measured readers are three (src/cmd/hook.rs:204, src/cmd/hook/statusline.rs:142, src/cmd/run.rs:379), not five
e2e-test-strategy#12.1 · yes · add-to · F3: the impostor rows (case::impostor, question_impostor, permission_impostor) join the hook fail-open matrix in hook_fail_open.rs, so its cause list gains the server-verification mismatch, and the 11-case count it states moves with it
e2e-test-strategy#13.1 · yes · add-to · the new negatives assert by digit runs that no pid, start time, path or endpoint reaches any output or line, a check this helper list does not yet name
e2e-test-strategy#14.1 · yes · add-to · F1, F4 and F5: the server-verification case is F1's binary in tests/security_negatives_server_identity.rs, not a cross_exit_causes.rs row, exit 21 gains its own server-verify hint and log detail (F4), and the macOS 0755 case is owed to "Unix endpoint and home hardening" (F5)
e2e-test-strategy#15.1 · yes · add-to · F1 and F2: tests/security_negatives_server_identity.rs gains the server-identity negative and its reaches-the-recorded-wrapper control, compiled out on macOS
e2e-test-strategy#16.1 · no · its list rows are read from list.rs, the board and the agents parse, none of them in the reached files, and no list case was added or moved
e2e-test-strategy#17.1 · no · the stale and gone readings are the liveness classifier's, which the change only extends (server_verdict, verify_server), and live_endpoint's changed return has no caller outside the reached files, so the board and run readings hold; no chaos case moved
e2e-test-strategy#18.1 · no · the property cases are unchanged, and the reached channel files add only the connected peer pid, not the framing or any parser these cases cover
e2e-test-strategy#19.1 · yes · stale · contract_fake_agent_drift.rs is reached, and its agents-replay test now plants every set's binary sequentially before its threads start (a test-only ETXTBSY fix), so the set walk and drift comparator this suite names are re-read on that file
