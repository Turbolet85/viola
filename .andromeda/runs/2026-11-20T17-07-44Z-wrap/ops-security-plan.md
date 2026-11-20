@row_set n=1 row=decision/start-writes-trust-record-flag-only cell=rule seen=1 fact=F7 detector=none
value: On a CLI verified by the final verdict, after the start's own verify, run and revive's start write the claude CLI's trust record for an untrusted spawn folder into its child's config, setting hasTrustDialogAccepted alone, other keys kept, or a one-key entry; unlike verify's, it persists.
basis: fct-zfntgo5a
because: The start now writes the trust record after its automatic verify, on the final verdict rather than the gate's first reading, so the row's condition must name which verdict decides.
@row_set n=2 row=decision/config-cannot-disable-controls cell=rule seen=2 detector=none
value: No configuration value makes the dialog hold zero or unbounded or lifts the unverified CLI gate, and an identity-floor name in claude_env_keep has no effect.
basis: fct-gfs4rlxl
because: The new config key verify_new_cli turns on a start's verify but is held never to lift the unverified gate, a control the row must now name.
@row_add n=1 row=contract/config-json/verify-new-cli area=input-validation fact=F1 detector=D-security-input
boundary: config.json
item: verify_new_cli
promise: A JSON boolean, default false; any other value reads false with one parse-rejected config-json malformed line, and the key is read only from a version 1 document.
basis: fct-xgc7qjiw
because: The chunk adds a new config.json key, an external-input surface whose validation the input-validation contracts must carry beside the other keys.
@row_add n=2 row=decision/start-verifies-new-cli-once area=authentication-authorization fact=F2 detector=none
name: Start verifies a new claude version once
rule: With config.json verify_new_cli true, viola run and revive's start run viola verify once for a claude version with no stamp entry; verify stays the only stamps writer, and an entry already attempted and red runs no verify again.
basis: fct-zfntgo5a
because: A start can now cause the stamp that switches on dialog answering, so the decision of when it does and who writes the stamp must stand beside the gate.
@row_add n=3 row=decision/verify-attempt-lock area=input-validation fact=F3 detector=none
name: Verify attempt lock owner-only and try-locked
rule: The start's verify attempt lock ledger/verify.lock is owner-only and taken by try-lock only after the stamps strict-modes check, never the stamps writer's lock; a held lock means that start runs no verify and reads the gate unverified.
basis: fct-zfntgo5a
because: A new owner-only file under the home is taken after the strict-modes check, and its held reading leaves the gate unverified, a rule future starts must keep.
@row_add n=4 row=decision/start-verify-child-isolated area=threat-model-summary fact=F4 detector=none
name: Start's verify child spawned directly and tied to the start
rule: The start's verify child is a direct spawn with a closed argv (the pinned copy, the canonical home, the resolved program), stdin null, the start's strip plan applied and VIOLA_NAME, VIOLA_DIR and VIOLA_BIN removed, tied to the start by a parent-death SIGTERM.
basis: fct-zfntgo5a
because: The start gains a new child process, and its isolation controls are a threat-model decision no existing row, written for verify's own claude children, states.
@row_add n=5 row=decision/start-verify-stop-leaves-nothing area=data-protection fact=F5 detector=none
name: A stop during the start's verify leaves nothing
rule: A SIGINT or SIGTERM during the start's verify TERMs the verify child, KILLs it after a 30 s grace and ends the start by the signal, leaving no stamp, no snapshot and no session receipt.
basis: fct-zfntgo5a
because: The start's verify window gains its own stop, whose grace and what it leaves on disk are a data-protection decision.
@row_add n=6 row=decision/start-verify-lines-closed-words area=error-handling fact=F6 detector=none
name: Start's verify lines hold closed words only
rule: The start's verify reports on stderr in five fixed ASCII lines, one write each, no SGR, holding only the parsed version and a closed word (spawn-failed, unstamped, lock-refused); none is written under revive --json.
basis: fct-zfntgo5a
because: A new stderr surface is added on the start, and what it may hold is an error-output rule under the never-log floor.
