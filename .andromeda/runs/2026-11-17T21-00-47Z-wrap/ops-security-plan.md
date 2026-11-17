@row_set n=1 row=decision/no-writes-under-claude-home cell=except seen=1 fact=F1 detector=none
value: viola verify writes a trust record for its own fresh temp folder into the Claude config its child reads and removes only that record after; a SIGKILL leaves it. Not a rule for run or revive.
basis: fct-yjvwqjma
because: The ruled widening lets verify write and remove its own trust record in the Claude config, the one bounded exception to this row, while the product's write for run and revive stays unruled.
@row_set n=2 row=decision/viola-writes-nothing-in-claude-dir cell=except seen=1 detector=none
value: viola verify writes a trust record for its own fresh temp folder into the Claude config its child reads and removes only that record after; a SIGKILL leaves it.
basis: fct-yjvwqjma
because: verify now writes and removes its own trust record in the Claude global config file, the bounded exception the ruling names against this row.
@row_set n=3 row=decision/nothing-written-under-claude-home cell=except seen=1 detector=none
value: viola verify writes a trust record for its own fresh temp folder into the Claude config its child reads and removes only that record after; a SIGKILL leaves it.
basis: fct-yjvwqjma
because: verify now writes its own trust record into the Claude config, which may stand under the kept Claude config directory, the bounded exception the ruling names.
@row_set n=4 row=decision/explicit-mode-before-write cell=except seen=1 detector=none
value: The trust record writer replaces Claude's config file keeping that file's own mode.
basis: fct-ytf3ugzg
because: The trust record writer replaces Claude's config with the file's own mode, a mode carried over from the replaced file, which the row otherwise forbids.
@row_set n=5 row=decision/r8-claude-env-strip cell=except seen=1 fact=F3 detector=D-security-auth
value: viola verify reads the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to choose the Claude config holding its trust record; else it uses the user HOME's .claude.json.
basis: fct-v6anf47h
because: verify now reads one CLAUDE value to place its trust record, so the row's names-only rule holds one bounded exception.
@row_set n=6 row=decision/credential-strip-by-name-only cell=except seen=1 detector=D-security-auth
value: viola verify reads the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to choose the Claude config holding its trust record.
basis: fct-v6anf47h
because: verify now reads the value of one kept CLAUDE name, so the claim that no variable's value is read holds one bounded exception.
@row_set n=7 row=decision/env-not-config-channel cell=except seen=1 detector=D-security-auth
value: viola verify reads CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps that name, to choose the Claude config holding its trust record.
basis: fct-v6anf47h
because: An environment value now chooses which config file verify writes its trust record into, an exception the row does not name.
@row_set n=8 row=decision/no-stored-secrets cell=except seen=1 detector=D-security-auth
value: the test-only seams compiled out of release builds, list's terminal colour facts, XDG_RUNTIME_DIR for the socket directory, and verify's CLAUDE_CONFIG_DIR value when claude_env_keep keeps it
basis: fct-v6anf47h
because: The row's list of environment reads lacks verify's new read of CLAUDE_CONFIG_DIR's value.
@row_add n=1 row=decision/trust-record-writer-guarded area=data-protection fact=F2 detector=D-security-input
name: Trust record writer guarded
rule: The trust record writer never creates an absent config, refuses a link, a non-regular file, a file of another owner, an oversize or malformed document and a present key, takes a lock beside the file only when free, tries three times, keeps the file's mode and touches no other key.
basis: fct-ytf3ugzg
because: A new writer into Claude's config file binds what any later write there, the product's included, must respect.
@row_add n=2 row=decision/trust-record-closed-words area=input-validation fact=F4 detector=D-security-input
name: Trust record outcomes are closed code words
rule: A trust record write or removal ends in one of 14 closed code words, and when nothing was written that word alone is stamped as the trust-record row's value, never a path, key or value.
basis: fct-ytf3ugzg
because: The trust record writer's outcome crosses into the stamps only as a closed word, a closed set future readers rely on.
@row_add n=3 row=decision/test-verify-under-test-home area=secret-management fact=F6 detector=none
name: Test-started verify under a test-owned home
rule: Every test-started viola verify runs under a test-owned user HOME, so the trust record it writes never reaches the user's own Claude config.
guard: every_test_verify_runs_under_a_test_home
except: A recording verify keeps the real HOME and points CLAUDE_CONFIG_DIR at the test-owned user home, so its record still lands there.
basis: fct-shari3xw
because: verify now writes into the config its child reads, so tests must keep that write off the user's own config, a rule enforced by a new census test.
@row_none n=1 fact=F5 type=9
what: The trust-record row's standing on the fake agent and on the real CLI, with the owed live re-read and the product's still unbuilt write, is the route's pending item.
