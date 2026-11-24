@row_add n=1 row=decision/adopt-signals-no-foreign-process area=authentication-authorization detector=none
name: Adopt acts on no process it did not start
rule: viola adopt resumes only a session no live claude holds, spawns its own child in its own terminal, and never ends or signals a process viola did not start.
why: A plain session still running elsewhere is refused rather than taken over.
state: provisional
basis: fct-zdl3zlbi, fct-k2nhjvap
because: The plan's W1 arm 1 is built — adopt refuses a held session and acts on no foreign process — and no row states it, so it stands as its own provisional row on the delegate's word.
@row_set n=1 row=decision/start-checks-home-tree-before-log-opens cell=rule seen=1 detector=none
value: The start of run, revive and adopt checks the whole home tree before the wrapper log opens: the home first, then the instance, ledger, bin and plugin folders and the pinned copy's own. A refusal exits 1 with no line or child.
basis: fct-zdl3zlbi
because: Adopt's start runs the pinned-copy home-tree check before its wrapper log opens, so the starts the rule names gain adopt's.
@row_set n=2 row=decision/start-checks-socket-dir-before-bind cell=rule seen=1 detector=none
value: A start, run, revive or adopt alike, checks the socket directory before any bind, first snapshot or child, makes it only when absent, and checks again so the loser of a race to make it reads the same verdict.
basis: fct-zdl3zlbi
because: Adopt launches through run's own start, which holds this socket-directory check, so the starts the rule names gain adopt's.
@row_set n=3 row=decision/start-writes-trust-record-flag-only cell=rule seen=1 detector=none
value: On a verified CLI only, the start of run, revive and adopt writes the claude CLI's trust record for an untrusted spawn folder into the config its child reads, setting hasTrustDialogAccepted alone with every other key kept, or a one-key entry where none stands; unlike verify's, the record persists.
basis: fct-zdl3zlbi
because: Adopt's start runs run's own start, whose trust step writes this record for the caller's current folder, so the starts the rule names gain adopt's.
@row_set n=4 row=decision/nothing-written-under-claude-home cell=except seen=3 detector=none
value: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; the start of run, revive and adopt sets the trust flag of an untrusted spawn folder, which stays.
basis: fct-zdl3zlbi
because: Adopt's start runs run's trust step, so the exception naming the starts that set the trust flag gains adopt's.
@row_set n=5 row=decision/no-writes-under-claude-home cell=except seen=3 detector=none
value: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; the start of run, revive and adopt sets the trust flag of an untrusted spawn folder, which stays.
basis: fct-zdl3zlbi
because: Adopt's start runs run's trust step, so the exception naming the starts that set the trust flag gains adopt's.
@row_set n=6 row=decision/viola-writes-nothing-in-claude-dir cell=except seen=3 detector=none
value: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; the start of run, revive and adopt sets the trust flag of an untrusted spawn folder, which stays.
basis: fct-zdl3zlbi
because: Adopt's start runs run's trust step, so the exception naming the starts that set the trust flag gains adopt's.
@row_set n=7 row=decision/credential-strip-by-name-only cell=except seen=3 detector=none
value: viola verify and the start of run, revive and adopt read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.
basis: fct-zdl3zlbi
because: Adopt's start runs run's trust step, which reads that value to find the config, so the starts the exception names gain adopt's.
@row_set n=8 row=decision/env-not-config-channel cell=except seen=3 detector=none
value: viola verify and the start of run, revive and adopt read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.
basis: fct-zdl3zlbi
because: Adopt's start runs run's trust step, which reads that value to find the config, so the starts the exception names gain adopt's.
@row_set n=9 row=decision/no-stored-secrets cell=except seen=3 detector=none
value: the test-only seams compiled out of release, list's colour facts, XDG_RUNTIME_DIR for the socket directory, and CLAUDE_CONFIG_DIR's value for verify and the start of run, revive and adopt when kept
basis: fct-zdl3zlbi
because: Adopt's start runs run's trust step, which reads that value to find the config, so the starts the exception names gain adopt's.
@row_set n=10 row=decision/r8-claude-env-strip cell=except seen=3 detector=none
value: verify and the start of run, revive and adopt read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps it, to choose the Claude config of the trust record; else the user HOME's .claude.json.
basis: fct-zdl3zlbi
because: Adopt's start runs run's trust step, which reads that value to find the config, so the starts the exception names gain adopt's.
@row_set n=11 row=decision/session-id-argv-closed-shape cell=except seen=1 detector=none
value: viola adopt places an operator-typed id there too, not a logged one, in the same closed shape.
basis: fct-zdl3zlbi, fct-jtbdy5p2
because: Adopt hands the operator-typed id to the child as the one argv element after --resume through the same builder, so the row that holds the crossing to a logged value must name the typed one.
