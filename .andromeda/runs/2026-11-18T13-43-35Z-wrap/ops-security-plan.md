@row_add n=1 row=decision/start-writes-trust-record-flag-only area=data-protection fact=F1 detector=none
name: Start writes the trust record, flag only
rule: run and the start of revive write the claude CLI's trust record for an untrusted spawn folder on a verified CLI only, setting hasTrustDialogAccepted alone with every other key kept, or inserting a one-key entry where none stands; the record persists after the session.
why: So no trust dialog stands at a wrapped start and no key is typed; flag only, as ratified.
basis: fct-xro6zomc
because: The start now writes the CLI's trust record, a new write into the user's Claude config that future work must respect.
@row_add n=2 row=decision/start-trust-skips-trusted-folders area=data-protection fact=F2 detector=none
name: Start skips the home folder and trusted folders
rule: The start writes no trust record for the user home folder, nor for a folder the claude CLI's own walk reads as trusted by its own key or by a parent up to the git root.
basis: fct-dlm2lanp
because: The start's trust write is bounded by the home-folder rule and the CLI's own walk, a decision the row must carry.
@row_add n=3 row=decision/start-trust-words-closed area=input-validation fact=F4 detector=D-security-input
name: Start trust outcome in closed words
rule: The start's trust outcome is one of a closed set of 16 start words, logged as one trust-record line per start holding no path, key or value, and the outcome never refuses the start.
basis: fct-xro6zomc
because: A new closed set of 16 start words and a trust-record line arrived, and the start's outcome never refuses it.
@row_set n=1 row=decision/trust-record-writer-guarded cell=rule seen=1 fact=F3 detector=D-security-input
value: The one trust record writer, verify's and the start's, never creates an absent config, refuses a link, a non-regular file, another owner's file, an oversize or malformed document and, for verify, a present key, locks only when free, tries three times, keeps the mode and touches no other key.
basis: fct-osk2sbyj
because: The start's trust write now goes through the same guarded writer, where a present key is refused for verify's write only.
@row_set n=2 row=decision/revive-recorded-cwd-spawn-dir cell=rule seen=1 fact=F5 detector=none
value: viola revive spawns claude in the snapshot's recorded cwd, read only after the strict-modes check, required to exist, and used only as the spawn directory and as the key of the start's trust record; program resolution and the version gate never read it.
basis: fct-wxqaoxqd
because: The recorded cwd now also keys the revive's trust record, so «for nothing else» is false.
@row_set n=3 row=decision/revive-cwd-spawn-only cell=rule seen=1 detector=none
value: The recorded working directory of a revive is read after the strict-modes check and used only as the spawn directory and as the key of the start's trust record.
basis: fct-wxqaoxqd
because: The recorded cwd now has a second use as the revive's trust record key, so «used only as the spawn directory» is false.
@row_set n=4 row=decision/cwd-only-in-snapshot cell=rule seen=1 detector=none
value: The snapshot's recorded cwd, which can carry the OS username, stands only in the 0600 snapshot and, as the key of the start's trust record, in the Claude config, never on a log line, error body or stdout.
basis: fct-xro6zomc
because: The revive's start now writes the recorded cwd as its trust record key into the Claude config, so «only in the snapshot» is false.
@row_set n=5 row=decision/spawn-cwd-snapshot-only cell=rule seen=1 detector=none
value: The start's spawn directory lives only in the 0600 snapshot and, as the key of its trust record, in the Claude config, on no log line, error body or stdout, and is omitted from the snapshot when absent.
basis: fct-xro6zomc
because: The start now keys its trust record in the Claude config by the spawn directory, so «only in the snapshot» is false.
@row_set n=6 row=decision/nothing-written-under-claude-home cell=except seen=2 detector=none
value: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; run and revive's start set the trust flag of an untrusted spawn folder, which stays.
basis: fct-vwyqtjra
because: The rule «run and revive write no trust record» is retired, so the exception must name the start's write.
@row_set n=7 row=decision/no-writes-under-claude-home cell=except seen=2 detector=none
value: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; run and revive's start set the trust flag of an untrusted spawn folder, which stays.
basis: fct-vwyqtjra
because: The exception's «Not a rule for run or revive» is false now that the start writes the trust record.
@row_set n=8 row=decision/viola-writes-nothing-in-claude-dir cell=except seen=2 detector=none
value: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; run and revive's start set the trust flag of an untrusted spawn folder, which stays.
basis: fct-vwyqtjra
because: verify is no longer the only writer into the Claude config, so the exception must name the start's write.
@row_set n=9 row=decision/credential-strip-by-name-only cell=except seen=2 fact=F6 detector=D-security-auth
value: viola verify and the start of run and revive read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.
basis: fct-xro6zomc
because: CLAUDE_CONFIG_DIR's value is now read by run and revive's start too, so a verify-only exception is incomplete.
@row_set n=10 row=decision/env-not-config-channel cell=except seen=2 detector=D-security-auth
value: viola verify and the start of run and revive read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.
basis: fct-xro6zomc
because: CLAUDE_CONFIG_DIR's value is now read at the start too, so a verify-only exception is incomplete.
@row_set n=11 row=decision/no-stored-secrets cell=except seen=2 detector=D-security-auth
value: the test-only seams compiled out of release, list's terminal colour facts, XDG_RUNTIME_DIR for the socket directory, and CLAUDE_CONFIG_DIR's value for verify and run and revive's start when kept
basis: fct-xro6zomc
because: CLAUDE_CONFIG_DIR's value is now read at run and revive's start too, so a verify-only exception is incomplete.
@row_set n=12 row=decision/r8-claude-env-strip cell=except seen=2 detector=D-security-auth
value: verify and run and revive's start read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps the name, to choose the Claude config holding the trust record; else the user HOME's .claude.json.
basis: fct-xro6zomc
because: The start now resolves the child's config file through the same shared resolution as verify, so a verify-only exception is incomplete.
@row_set n=13 row=decision/tests-use-own-home cell=rule seen=1 fact=F7 detector=none
value: Development handles secrets as production does: tests use their own --home directory, and every test-side run and revive and every supervised harness wrapper gets a test-owned user home, never the runner's real one.
basis: fct-o4xnvz66
because: A start under the runner's real HOME would now lock, read and write the operator's real Claude config, so every test start gets a test-owned user home.
