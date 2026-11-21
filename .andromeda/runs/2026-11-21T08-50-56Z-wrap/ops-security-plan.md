@row_set n=1 row=decision/capture-arms-isolated cell=rule seen=2 fact=F3 detector=D-security-input
value: The hidden capture arm reads no VIOLA_* variable, opens no channel, adds no env var and always exits 0; on SessionStart alone it writes an owner-only file of its own process's env names, CLAUDE names kept and every other name only counted, never reading a value.
basis: fct-yw2wvymn, fct-pu7enr4k
because: The capture arm now also reads its own process's environment on SessionStart, names only, so the isolation row must say what that new read keeps and what it never touches.
@row_set n=2 row=decision/live-rig-kept-stamp-owner-only cell=rule seen=3 detector=none
value: The live rig keeps a stamp from its CS, WS, PQ, HF, HF-2 or HG scratch home owner-only and no-clobber, and seeds it only into its own CC or R3W scratch home; CC with no kept stamp refuses before its ledger line and before any scratch, R3W before its ledger line.
basis: fct-yw2wvymn, fct-wqdvpir3
because: The rig's new HF, HF-2 and HG labels keep owner-only stamps, and the row's closed list of labels that keep a stamp lacked them.
@row_set n=3 row=contract/artifact-canary-scan-classes/claude-stripped cell=promise seen=1 detector=none
value: The scan fails on any of the sixteen critical CLAUDE* and env canary needles, the kept, other, notes and launched env canaries among them.
basis: fct-wu5hyrfe, fct-v4enyx24
because: The critical canary list grew from twelve to sixteen with the kept, other, notes and launched env canaries, and the cell holds that count.
@row_retire n=1 row=decision/identity-floor-eleven-names seen=1 heir=decision/non-identity-hook-names detector=D-security-auth
why: The identity floor no longer stands at eleven names, which the row's key and rule are made from.
basis: fct-yw2wvymn, fct-wu5hyrfe
because: The identity floor grew from eleven to thirteen names, so a row keyed on eleven is false and its non-identity half passes to an heir.
@row_add n=1 row=decision/identity-floor-ledger-row area=secret-management fact=F1 detector=D-security-auth
name: Identity floor checked by a relied ledger row
rule: The identity-floor relied ledger row reads pass only when every CLAUDE name the CLI adds to a hook is an identity-floor name or one of the four non-identity hook names; an unknown name fails the row, and names kept by claude_env_keep are dropped before the check, the stamp and the record.
guard: identity_floor_reads_fail_on_an_unknown_name
basis: fct-yw2wvymn, fct-d6aupwop
because: A new relied ledger row now holds the identity floor against what the CLI adds to a hook, a decision no row of the section states.
@row_add n=2 row=decision/non-identity-hook-names area=secret-management fact=F2 detector=D-security-auth
name: Four hook names are not identity names
rule: CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE and CLAUDE_PLUGIN_DATA, names the CLI adds to a hook, are known and not identity names: they stand off the identity floor and are read only by verify's env reading.
why: Ratified.
basis: fct-yw2wvymn, fct-up2g4cpp
because: The retired eleven-name row's non-identity half now names a fourth name, CLAUDE_PLUGIN_DATA, and the one reader of the list.
@row_add n=3 row=decision/env-fixture-names-only area=data-protection fact=F4 detector=none
name: Recorded env fixture holds names only
rule: The recorded env fixture holds only its version and the CLAUDE names the CLI adds to a hook, never a value; it passes the record scrub and the env-name check, an env name carrying the user word refusing the recording as username with nothing recorded, and the hygiene walk checks it as its own class.
guard: env_fixtures_pass_hygiene; record_refuses_an_env_name_carrying_the_user_word
basis: fct-y2ah5a4y, fct-yw2wvymn, fct-cyougjyd
because: A new recorded fixture class of env names lands in the tree, and no data-protection row says it holds names only or how an unclean name is refused.
@row_add n=4 row=decision/env-probe-names-and-counts-only area=secret-management fact=F5 detector=D-security-auth
name: Verify stamp's env probe holds names and counts only
rule: The verify stamp's env probe holds names and counts only (read, added, unknown, other count, unshaped), never a value, and names kept by claude_env_keep are removed before it is written.
guard: merge_stamp_writes_the_env_block; kept_name_reaches_no_stamp_or_record
basis: fct-yw2wvymn, fct-y2ah5a4y, fct-wi3sokcx
because: The stamp gains an env_probe block over the hook's environment, and no secret-management row bounds what of the environment the ledger may hold.
@row_add n=5 row=decision/launched-by-session-ids-on-floor area=secret-management fact=F6 detector=D-security-auth
name: Launched-by name on the floor from bundle strings
rule: CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS stands on the identity floor on a reading of the CLI bundle's strings alone, never met live, and the identity-floor row rests on one live recording on one CLI version under a scratch home.
why: A provisional floor accepted as such.
state: provisional
basis: fct-wi3sokcx
because: The floor now holds a name no live run has met, accepted provisionally, which future work must know before it relies on the floor as measured.
