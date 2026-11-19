@row_set n=1 row=decision/local-command-post-condition cell=rule seen=2 fact=F5 detector=none
value: On a verified CLI /clear is confirmed only by a session-start of cause clear with a new session id, /compact only by one of cause compact with the same id, and /mcp and /context only by a new stdout tag past the transcript's pre-paste end; none by a prompt-submitted, a missed one is not-delivered.
basis: fct-z3derqaa
because: /context joins the listed local commands confirmed by a new stdout tag with no prompt submitted, so the rule naming each command's post-condition must name it beside /mcp.
@row_set n=2 row=contract/fake-agent-input/echo-command cell=promise seen=1 detector=none
value: Under --local-command-mode a submitted local command whose post-condition is the stdout tag, and under --framing the exact verify echo probe command or the exact /context probe command, append one stdout tag line to the last started session's transcript and fire no hook.
basis: fct-z3derqaa
because: The fake agent's --framing now echoes the /context probe literal as it does the echo probe, so the promise naming only the echo probe command is stale.
@row_add n=1 row=decision/live-rig-kept-stamp area=6-e2e-test-strategy fact=F1 detector=none
name: Live rig context stamp kept and seeded
rule: The standing live rig's CS label stamps one verify into its scratch home and keeps that stamp; its CC label seeds the kept stamp into its own scratch home before one wrapped start and refuses before its ledger line when no kept stamp exists.
guard: rig_rehearsal_cc_without_a_kept_stamp_refuses_before_its_ledger_line
basis: fct-z3derqaa
because: The rig gained the CS and CC labels whose stamp is taken once in a scratch home and reused by the wrapped start, a rule later live rounds must keep.
@row_add n=2 row=decision/live-rig-config-top-reading area=6-e2e-test-strategy fact=F2 detector=none
name: Live rig config-top reading keeps names and counts only
rule: The live rig's config-top reading reports only the top-level plain names of its private config and their regular file counts, a name outside letters, digits, dot, underscore and dash or over 40 characters reading other; no byte of a file is kept.
guard: config_top_reads_plain_names_and_their_file_counts_only
basis: fct-hqoszkd6
because: The rig gained a reading of its private config whose bound on what it keeps is a privacy rule future readings must respect.
@row_add n=3 row=decision/live-rig-hint-cell-reading area=6-e2e-test-strategy fact=F3 detector=none
name: Live rig hint-cell reading recorded as a reading
rule: The live rig's hint-cell reading records the attribute words of every drawn hint cell past the cursor cell after the second send, as a reading and never as verification; no drawn cell reads unread (no-cell).
basis: fct-xi25rmsu
because: The founder widened the rig's screen reading to the hint cells past the cursor in exactly that width, so the widening and its standing as a reading must be kept.
@row_add n=4 row=decision/private-fake-for-no-transcript area=test-data-fixtures fact=F4 detector=none
name: No-transcript tests run a private fake-agent copy
rule: A test that must have no fake-agent transcript removes none from the shared build; it runs its own private copy of the fake agent in a test-owned directory.
why: Transcripts sit beside the fake's exe, a path every concurrently running test of the shared build shares, so removing one would race them.
basis: fct-7bwwjoem
because: The chunk set the way a test gets a transcript-free fake, a convention every later test of that need must follow.
