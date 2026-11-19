@row_set n=1 row=decision/live-rig-prompt-capture-closed cell=rule seen=1 detector=D-security-input
value: The live rig's hook capture reads a payload under MAX_FRAME at default JSON depth and appends one owner-only line: key names matching [a-z_]{1,40}, others counted, and a UserPromptSubmit source word or a StopFailure error word of the CLI's 13 else other with two text lengths, never a text.
basis: fct-usi3wfxj
because: The ratified widening extends the rig's capture to StopFailure payloads in exactly that width (key names, the error word of the 13 else other, two text lengths), so the row naming only the UserPromptSubmit capture is incomplete.
@row_set n=2 row=decision/unverified-cli-gate cell=except seen=5 detector=none
value: StopFailure is registered with no capability ledger row and no verify probe, the exception settled on a contained live StopFailure reading of claude 2.1.328 that classified the word server_error.
basis: fct-vj6tsu22
because: The exception is no longer owned by a pending entry: it is kept on the founder's word on the green live reading, outcome A not opened, so the except must cite the reading instead of the spent entry.
@row_add n=1 row=decision/live-rig-sf-arm-network-closed area=secret-management fact=F1 detector=none
name: Live rig's SF arm network-closed
rule: The live rig's SF arm runs the real claude inside a network-closed bwrap sandbox with its own network namespace; an absent or failing sandbox, or one sharing the rig's namespace, refuses the arm with refused: sandbox, exit 3, uncounted, before any credentials read or ledger line.
guard: rig_rehearsal_sf_refuses_contain_without_the_sandbox_before_its_ledger_line
basis: fct-wlojdipp
because: The chunk adds a contained live arm whose sandbox refusal precedes any credentials read, a containment decision no standing row of the rig's isolation states.
@row_add n=2 row=decision/live-rig-sandbox-runtime-dir area=secret-management fact=F2 detector=none
name: Contained rig start needs an outside runtime dir
rule: A contained live-rig start is refused as no-runtime-dir unless XDG_RUNTIME_DIR is absolute and outside the temp root.
why: So viola's socket directory stays reachable outside the sandbox's private temp root.
basis: fct-zrtvvb6q
because: The operator pass added the no-runtime-dir precondition to the sandbox probe with a stated reason, a rule future contained arms must respect.
@row_none n=1 fact=F3 type=4
relied: A live-rig kept record set for a CLI version already kept is refused as exists with nothing written, so a standing version's set is never overwritten.
@row_none n=2 fact=F4 type=4
relied: The fake agent's --turn-fail and --settings-hooks options read no environment, so no new environment test seam stands.
