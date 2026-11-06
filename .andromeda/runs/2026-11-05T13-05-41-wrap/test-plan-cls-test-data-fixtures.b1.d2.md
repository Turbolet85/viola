@answer n=1 entity=test-data-fixtures#1.1 v=yes mode=contradict
why: the four-exceptions list names seed_conpty on Windows x64 and the windows-2025 hook_statusline cases, both retired with the ConPTY sideload and the Windows legs
@answer n=2 entity=test-data-fixtures#1.2 v=yes mode=contradict
why: the widened seeded home on Windows x64 depends on seed_conpty hard links and icacls grants, both removed
@answer n=3 entity=test-data-fixtures#1.3 v=no
why: ledger/stamps.json comes from viola verify against the fake agent, a path with no OS claim that the session does not reach
@answer n=4 entity=test-data-fixtures#1.4 v=no
why: budget.json is piped into viola hook statusline with no OS claim, and budget.rs is not among the files the session reached
@answer n=5 entity=test-data-fixtures#1.5 v=yes mode=stale
why: tests/support/home.rs is reached whole and its Windows USERPROFILE qualifier on boot_as_user is a Windows reading the Linux-only session retires
@answer n=6 entity=test-data-fixtures#1.6 v=yes mode=contradict
why: SOCKET_ROOT_VAR now reads XDG_RUNTIME_DIR alone, so the TMPDIR-on-macOS arm this entity names is gone
@answer n=7 entity=test-data-fixtures#1.7 v=yes mode=contradict
why: the icacls arms of widen, grant_read, unwiden and still_widened in tests/support/home.rs are removed
@answer n=8 entity=test-data-fixtures#1.8 v=no
why: the Form cases and their Unix 0755 and symlink claims are unchanged by the session, and the socket-dir forms it moved are not named here
@answer n=9 entity=test-data-fixtures#1.9 v=no
why: the fake claude placement under a test-scoped PATH still holds after the bare-name resolution change, and tests/support/fake.rs is not among the files the session reached
@answer n=10 entity=test-data-fixtures#1.10 v=yes mode=stale
why: the on-Unix limit on the recorded StatusLine probe variants is stale because the statusline pass-through now runs on every build
@answer n=11 entity=test-data-fixtures#1.11 v=no
why: the recorded MCP exchange, its scrub codes and its schema are not touched by any change the session made
@answer n=12 entity=test-data-fixtures#1.12 v=yes mode=stale
why: the refusal code counts pinned in tests/cross_exit_causes.rs moved from revive 11 to 10 and verify 6 to 5, and the .cmd human pair it names is retired
@answer n=13 entity=test-data-fixtures#1.13 v=no
why: the insta decision-body snapshots make no OS claim and the session does not change the decision bodies they hold
@answer n=14 entity=test-data-fixtures#1.14 v=no
why: the byte-for-byte hook transcript comparison through stdin_hex makes no OS claim and the contract suite files it names are not changed
@answer n=15 entity=test-data-fixtures#1.15 v=no
why: the proptest strategies and their committed proptest-regressions are untouched by the session
@answer n=16 entity=test-data-fixtures#1.16 v=no
why: the injected Clock seam, mock_instant absence and tokio test-util note are not changed by the session
@answer n=17 entity=test-data-fixtures#1.17 v=no
why: the planted error-body value reaches its child through the PTY spawn env_set, which the R8 strip change does not touch
@answer n=18 entity=test-data-fixtures#2.1 v=yes mode=stale
why: the server-identity impostor cases are named on Linux and Windows, and the Windows leg of the server-identity negatives is retired
@answer n=19 entity=test-data-fixtures#2.2 v=yes mode=contradict
why: the Windows Serving with no Drop and its pipe in the accept thread belong to the retired Windows named pipe in viola-channel
@answer n=20 entity=test-data-fixtures#2.3 v=yes mode=contradict
why: the Windows NotFound oracle for a home planted under a regular file belongs to the removed Windows strict-modes arm
@answer n=21 entity=test-data-fixtures#2.4 v=no
why: the endpoint directory is still made through viola_state::fs::create_private_dir in the named files, and the session moved the socket-dir forms and the gates but not that call
@answer n=22 entity=test-data-fixtures#3.1 v=no
why: the etxtbsy drift note and its tmpfs hard-link reading name no OS claim the session changed, and contract_fake_agent_drift.rs is not among the files it reached
@answer n=23 entity=test-data-fixtures#4.1 v=no
why: the fake agent's hooks.json and statusline replay carry no Windows arm or OS limit the session removed, and the fake agent source is not among the files it reached
@answer n=24 entity=test-data-fixtures#5.1 v=no
why: the receipt kinds, including the Unix-only fds kind, are the fake agent's own output, which the session does not change
@answer n=25 entity=test-data-fixtures#6.1 v=no
why: the fake agent's argv options and mode words are unchanged by the session, and its recorded vt100 panic bytes are not a Windows reading
