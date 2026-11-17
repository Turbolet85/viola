@row_set n=1 row=decision/verify-child-set cell=rule seen=1 detector=none
value: viola verify makes one print-mode probe, six interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts.
basis: fct-skwar3iy, fct-yjvwqjma
because: Run F, the trust-record run, makes verify's interactive runs six, a ruled widening of the child set, so the row's count of five is false.
@row_set n=2 row=decision/permission-request-question-stays-null cell=why seen=3 detector=none
value: A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a 24th row, its viola verify probe and a re-stamp of every stamped version.
basis: fct-skwar3iy
because: The relied ledger rows moved from 22 to 23, so the row a new body would need is the 24th, not the 22nd.
@row_set n=3 row=decision/verify-run-e-resumes-run-b-first-session cell=why seen=1 detector=none
value: The resume ledger row checks Run E's first SessionStart for source resume and that same id, and the six runs execute in order A to F.
basis: fct-skwar3iy, fct-ytf3ugzg
because: Run F now runs last after Run E, so the runs are six in order A to F.
@row_set n=4 row=decision/verify-removes-only-own-dirs cell=rule seen=1 detector=D-arch-decisions
value: viola verify removes its probe folders whole, and its own trust record from the Claude config file, when it passes, fails or is stopped by Ctrl-C or TERM, only what that run created, never by a pattern or a listing.
basis: fct-ytf3ugzg, fct-2t56x2kc
because: Run F's record guard removes verify's own trust record after a pass, a fail and a TERM stop, so the cleanup decision now covers that record too.
@row_set n=5 row=decision/claude-shapes-in-one-crate cell=except seen=2 detector=D-arch-decisions
value: Test-only: the live rig reads the key names and source word of its own captured UserPromptSubmit payload, never a value; the fake agent and the rig read a Claude config file's trust record.
basis: fct-ytf3ugzg
because: The test-only fake agent and live rig now read the trust record of a Claude config file outside viola-agent-claude, so the named exception must include them.
@row_set n=6 row=decision/claude-first-no-adapter-trait cell=except seen=2 detector=D-arch-decisions
value: Test-only: the live rig reads the key names and source word of its own captured UserPromptSubmit payload, never a value; the fake agent and the rig read a Claude config file's trust record.
basis: fct-ytf3ugzg
because: The test-only fake agent and live rig now read the trust record of a Claude config file outside viola-agent-claude, so the named exception must include them.
@row_set n=7 row=decision/claude-shapes-isolated cell=except seen=1 detector=D-arch-decisions
value: Test-only: the live rig reads its own captured UserPromptSubmit payload's key names and source word; the fake agent and the rig read a Claude config file's trust record.
basis: fct-ytf3ugzg
because: Test-only code outside viola-agent-claude now reads the Claude config's trust-record shape, which the row's unqualified «only» contradicts.
@row_set n=8 row=decision/env-strip-names-only cell=except seen=1 fact=F4 detector=D-arch-resources
value: viola verify reads the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
basis: fct-v6anf47h, fct-ytf3ugzg
because: verify now reads one environment value beside the names-only strip, so the names-only rule needs that one exception named.
@row_set n=9 row=decision/env-strip-reads-names-only cell=except seen=1 detector=D-arch-resources
value: viola verify reads the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
basis: fct-v6anf47h, fct-ytf3ugzg
because: verify now reads one environment value beside the names-only strip, so the names-only rule needs that one exception named.
@row_set n=10 row=decision/env-var-viola-prefix cell=except seen=1 detector=D-arch-resources
value: Ratified: test seams FAKE_AGENT_PUMP_DELAY_MS, FAKE_AGENT_HOOK_PANIC; NO_COLOR, TERM, COLORTERM for SGR only; PATH, set for the child; XDG_RUNTIME_DIR; CLAUDE_CONFIG_DIR, read by verify when kept.
basis: fct-v6anf47h
because: verify now reads CLAUDE_CONFIG_DIR's value, an environment variable without the VIOLA_ prefix, so the exception list must name it.
@row_set n=11 row=decision/no-environment-configuration cell=except seen=1 detector=D-arch-resources
value: NO_COLOR, TERM, COLORTERM for list's colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent-only test seams; CLAUDE_CONFIG_DIR's value, read by verify only when kept.
basis: fct-v6anf47h
because: verify's choice of the Claude config file now follows CLAUDE_CONFIG_DIR's value when claude_env_keep keeps it, an environment read the exception list must name.
@row_set n=12 row=contract/config-json/claude-env-keep cell=promise seen=1 detector=D-arch-resources
value: The R8 pass-list of CLAUDE* names, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect. Keeping CLAUDE_CONFIG_DIR makes viola verify read its value to locate the Claude config file.
basis: fct-v6anf47h
because: claude_env_keep now also decides whether verify reads CLAUDE_CONFIG_DIR's value, a promise of the key outward.
@row_set n=13 row=contract/ledger-stamps/rows cell=promise seen=1 detector=D-arch-resources
value: Maps each ledger row id to "pass" or "fail"; trust-record holds the trust writer's code word instead when no record was written.
basis: fct-v6anf47h, fct-ytf3ugzg
because: A stamp's trust-record value may now be the writer's word, so «pass or fail» alone is false.
@row_set n=14 row=contract/ledger-stamps/content cell=promise seen=1 detector=D-arch-resources
value: Per CLI version, the envelope holds each ledger row's `pass` or `fail`, or for trust-record the trust writer's word when no record was written, and the probe's measured values, of which `run` reads none.
basis: fct-v6anf47h, fct-ytf3ugzg
because: A stamp's trust-record value may now be the writer's word, so the envelope's content promise must say so.
@row_set n=15 row=contract/ledger-stamps/measured cell=promise seen=2 detector=D-arch-resources
value: Probe readings (largest hook payload per spine event, typed_probe timings and box_only_guards, the count of typed-run guards that passed on the input box's own rows with no footer literal on any row, dialog_probe, trust_probe write and remove); additions need no v bump, and run reads no field of it.
basis: fct-v6anf47h
because: measured.trust_probe {write, remove} was added to the stamps, additively, so the list of readings must hold it.
@row_add n=1 row=decision/verify-own-trust-record area=established-decisions fact=F1 detector=D-arch-decisions
name: Verify writes and removes its own trust record
rule: viola verify writes a trust record for its own temp folder only into the Claude config file its child reads, never creating an absent file, and removes that one record after; run and revive write no trust record.
why: Ratified as the one bounded exception to viola writing nothing into Claude's config.
basis: fct-yjvwqjma, fct-ytf3ugzg, fct-2t56x2kc
because: verify now writes into the Claude config file, a ruled exception no row states, and the planner of run's and revive's owed write must see its bound.
@row_add n=2 row=decision/trust-record-ledger-row area=established-decisions fact=F2 detector=none
name: Trust-record ledger row and its probe
rule: The trust-record relied ledger row, a trust record written for an untrusted folder opens it with no modal, is probed by verify's Run F after Run E and passes only when the record was written, Run A showed the trust modal, and Run F's start shows no modal literal and the input box.
why: Ratified as a widening of verify's child set.
basis: fct-ytf3ugzg, fct-yjvwqjma, fct-vw5u2c4x
because: A 23rd relied ledger row now stands, a CLI behaviour viola relies on, with its own probe and check that no row states.
@row_add n=3 row=decision/trust-writer-takes-config-lock area=occupied-resources fact=F5 detector=D-arch-resources
name: Trust writer holds the Claude config's lock
rule: The trust-record writer holds the Claude config file's own lock, its name plus .lock, as a 0700 directory taken only when free, and reads lost after three attempts; a held lock writes nothing.
basis: fct-ytf3ugzg, fct-d4gxu5ji
because: The trust writer now occupies the Claude CLI's lock name beside its config file, a resource no row registers.
@row_add n=4 row=contract/trust-writer-words/written area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Trust writer code words
item: written
promise: The trust record was written into the Claude config file.
basis: fct-ytf3ugzg
because: The trust writer answers in a closed set of code words that reach the stamps, a new closed contract no row states.
@row_add n=5 row=contract/trust-writer-words/removed area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Trust writer code words
item: removed
promise: The written trust record was removed again, every other key of the config left as found.
basis: fct-ytf3ugzg, fct-2t56x2kc
because: The trust writer answers in a closed set of code words that reach the stamps, a new closed contract no row states.
@row_add n=6 row=contract/trust-writer-words/config-absent area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Trust writer code words
item: config-absent
promise: The Claude config file does not exist; it is never created, nothing is written and Run F starts no child.
basis: fct-ytf3ugzg, fct-2t56x2kc
because: The trust writer answers in a closed set of code words that reach the stamps, a new closed contract no row states.
@row_add n=7 row=contract/trust-writer-words/config-unreadable area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Trust writer code words
item: config-unreadable
promise: The Claude config file could not be read; nothing is written.
basis: fct-ytf3ugzg
because: The trust writer answers in a closed set of code words that reach the stamps, a new closed contract no row states.
@row_add n=8 row=contract/trust-writer-words/config-link area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Trust writer code words
item: config-link
promise: The Claude config path is a symbolic link; it is refused and left untouched.
basis: fct-ytf3ugzg, fct-oyqnxj2g
because: The trust writer answers in a closed set of code words that reach the stamps, a new closed contract no row states.
@row_add n=9 row=contract/trust-writer-words/config-not-regular area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Trust writer code words
item: config-not-regular
promise: The Claude config path is not a regular file; it is refused and left untouched.
basis: fct-ytf3ugzg, fct-oyqnxj2g
because: The trust writer answers in a closed set of code words that reach the stamps, a new closed contract no row states.
@row_add n=10 row=contract/trust-writer-words/config-foreign area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Trust writer code words
item: config-foreign
promise: The Claude config file belongs to another owner; nothing is written.
basis: fct-ytf3ugzg, fct-oyqnxj2g
because: The trust writer answers in a closed set of code words that reach the stamps, a new closed contract no row states.
@row_add n=11 row=contract/trust-writer-words/config-oversize area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Trust writer code words
item: config-oversize
promise: The Claude config file is over the frame cap; nothing is written.
basis: fct-ytf3ugzg, fct-oyqnxj2g
because: The trust writer answers in a closed set of code words that reach the stamps, a new closed contract no row states.
@row_add n=12 row=contract/trust-writer-words/config-malformed area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Trust writer code words
item: config-malformed
promise: The Claude config file is no JSON document of the expected shape; nothing is written.
basis: fct-ytf3ugzg
because: The trust writer answers in a closed set of code words that reach the stamps, a new closed contract no row states.
@row_add n=13 row=contract/trust-writer-words/config-dir-unusable area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Trust writer code words
item: config-dir-unusable
promise: The kept CLAUDE_CONFIG_DIR gives no usable folder for the Claude config file; nothing is written.
basis: fct-ytf3ugzg
because: The trust writer answers in a closed set of code words that reach the stamps, a new closed contract no row states.
@row_add n=14 row=contract/trust-writer-words/home-unknown area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Trust writer code words
item: home-unknown
promise: No user home is known to locate the Claude config file; nothing is written.
basis: fct-ytf3ugzg
because: The trust writer answers in a closed set of code words that reach the stamps, a new closed contract no row states.
@row_add n=15 row=contract/trust-writer-words/key-present area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Trust writer code words
item: key-present
promise: The config already holds a record under the key; it is left as found and nothing is written.
basis: fct-ytf3ugzg
because: The trust writer answers in a closed set of code words that reach the stamps, a new closed contract no row states.
@row_add n=16 row=contract/trust-writer-words/lock-held area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Trust writer code words
item: lock-held
promise: The config file's lock is held or cannot be made; nothing is written.
basis: fct-ytf3ugzg, fct-d4gxu5ji
because: The trust writer answers in a closed set of code words that reach the stamps, a new closed contract no row states.
@row_add n=17 row=contract/trust-writer-words/lost area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Trust writer code words
item: lost
promise: Three attempts each failed to replace the config file or to read the change back.
basis: fct-ytf3ugzg, fct-d4gxu5ji
because: The trust writer answers in a closed set of code words that reach the stamps, a new closed contract no row states.
@row_none n=1 fact=F6 type=4
relied: The fake agent and the live rig find verify's record under the same key, the nearest ancestor holding a git entry, else the canonical folder.
