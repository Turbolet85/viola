@row_set n=1 row=decision/live-round-rig-keys-own-pty cell=rule seen=1 fact=F1 detector=none
value: On the founder's own word anew for each chunk, the live rig alone types into a live claude pty, by a closed argv grant: at most two down-arrows and Enter only on a read trust row, one x and one DEL in a ready box; any other key writes nothing; Ctrl-C is its stop's alone; viola types into no dialog.
basis: fct-onwod2ys
because: The rig is now one standing committed rig whose key grant, read from argv and naming Enter only on a read trust row and Ctrl-C as the stop's alone, stands as the founder granted it for this chunk, so the row's per-round grant is stale.
@row_set n=2 row=decision/real-cli-local-only cell=rule seen=1 fact=F4 detector=none
value: The real claude CLI runs only locally: in CI viola verify and the live rig run only against the fake agent and the rig's live arm refuses with the word ci; live rig starts run by hand under a per-chunk start cap with one spare, each ledgered before it is made.
basis: fct-sjq6de27
because: The standing live rig's rehearsals now run in CI against the fake agent while its live arm refuses under CI and its live starts run by hand under a cap, so the row that keeps the real CLI local must name it.
@row_add n=1 row=decision/live-rig-credentials-copy area=secret-management fact=F2 detector=D-security-auth
name: Live rig credentials copy transient and unrecorded
rule: The live rig copies the claude OAuth credentials file owner-only into its private scratch, reads only its expiry, refuses a start with under 20 minutes of token, removes the copy on every exit path, and writes no credential, host path or config value into any ledger, frame or summary.
guard: rig_leaves_no_credentials_copy_on_any_exit_path; rig_keeps_no_credential_or_host_path_in_any_record
basis: fct-nwy7zwl7
because: The chunk adds a test-only site that handles the claude OAuth credentials, and no row of Secret Management bounds how it is held.
@row_add n=2 row=decision/live-rig-isolated-child area=secret-management fact=F3 detector=D-security-auth
name: Live rig child isolated from the user's config
rule: The live rig runs its child under a private CLAUDE_CONFIG_DIR in its own scratch, with an environment built from a closed kept list of host names plus its own, and reads the user's ~/.claude.json only as a sha256 prefix and mtime before and after each start.
guard: rig_reads_the_user_config_before_and_after
basis: fct-yubkirys
because: The rig spawns the real claude CLI with a private config dir and a kept environment list, a child-environment boundary no standing row covers.
@row_add n=3 row=decision/live-rig-no-key-into-imports-modal area=threat-model-summary fact=F5 detector=none
name: Live rig types no key into the imports modal
rule: The live rig reads the external-imports modal and types no key into it; a key into that modal needs its own grant.
basis: fct-seoizoiu
because: The rig now reads the external-imports modal, and the boundary that no key is typed there until granted is a decision no row holds.
@row_add n=4 row=decision/live-rig-kills-only-its-own area=data-protection fact=F7 detector=none
name: Live rig kills only its own processes
rule: The live rig's census and kill reach a process only by pid and start time, and only one carrying the rig's own run marker or standing in its scratch cwd; a dead rig's leftovers are killed only by an explicit census --kill.
guard: rig_ends_what_it_starts_on_every_exit_path
basis: fct-nwy7zwl7
because: The rig adds a census and kill over /proc, and the bound that it reaches only its own processes is a decision no row holds.
@row_add n=5 row=name/binary/viola-live-rig area=new fact=F8 detector=none
section: Occupied Names
class: binary
name: viola-live-rig
basis: fct-yubkirys
because: The chunk occupies the test-only binary name viola-live-rig.
@row_add n=6 row=name/other/agent-run-rig area=new fact=F8 detector=none
section: Occupied Names
class: other
name: AGENT_RUN_RIG
basis: fct-yubkirys
because: The chunk occupies the env name AGENT_RUN_RIG, set on the rig's child only as its run marker.
@row_none n=1 fact=F6 type=4
what: How the rig's send-reply, events and screen reads are built; the standing MAX_FRAME, closed code word and vt100 catch_unwind rows already hold the rules it follows, and the code holds the rest.
