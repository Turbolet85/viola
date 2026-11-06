@answer n=1 entity=test-data-fixtures#1.1 v=no standing=S3
why: on-disk product state is produced by real verbs and never hand-written, which is the new-test convention that seeds no state by hand (S3)
@answer n=2 entity=test-data-fixtures#1.2 v=no
why: ledger/stamps.json comes from viola verify against the fake agent, a path the chunk does not reach or change
@answer n=3 entity=test-data-fixtures#1.3 v=no
why: budget.json stdin for viola hook statusline is not reached and the chunk changes no budget path
@answer n=4 entity=test-data-fixtures#1.4 v=no
why: the planted statusline source in tests/support/home.rs is not reached and the chunk changes no statusline path
@answer n=5 entity=test-data-fixtures#1.5 v=no
why: the test-owned socket root under /tmp is not reached by the chunk
@answer n=6 entity=test-data-fixtures#1.6 v=no
why: widened test homes set by the test on Unix modes are not reached and the chunk changes no mode check
@answer n=7 entity=test-data-fixtures#1.7 v=no
why: the loosened socket-directory negatives in tests/security_negatives_socket_dir.rs are not reached and the chunk moves no socket-directory check
@answer n=8 entity=test-data-fixtures#1.8 v=no
why: the fake agent placed as claude in tests/support/fake.rs is not a reached unit, and the EXE_SUFFIX joins are empty on Linux so removing them leaves its paths unchanged
@answer n=9 entity=test-data-fixtures#1.9 v=no
why: the chunk records and changes no hook payload fixture, and the CI replay of them is a re-check every change owes
@answer n=10 entity=test-data-fixtures#1.10 v=no
why: the recorded MCP exchange and Mcp.tools-list-1.json are not reached and the chunk records no exchange
@answer n=11 entity=test-data-fixtures#1.11 v=yes
why: the chunk re-measures the missing tests/cmd trycmd home and edits the viola --help HELP literal's file tests/cli_output_plain.rs at lines 104 and 116
@answer n=12 entity=test-data-fixtures#1.12 v=no
why: insta decision-body snapshots are the not_needed decision-body scope and the chunk changes no decision body
@answer n=13 entity=test-data-fixtures#1.13 v=no
why: fake-agent transcripts compared through stdin_hex are not changed by the chunk and no scoped check reads them
@answer n=14 entity=test-data-fixtures#1.14 v=no
why: proptest strategies and their persisted regressions are not reached and are the not_needed viola-core and mcp unit scope
@answer n=15 entity=test-data-fixtures#1.15 v=no
why: mock_instant and tokio test-util mocking discipline is the not_needed time scope, and the portable-pty move does not alter the tokio manifest pins the tokio-free checks read
@answer n=16 entity=test-data-fixtures#1.16 v=yes
why: the planted value reaches the child through the PTY spawn's env_set, the spawn path the portable-pty move under viola-pty reaches and must keep working
@answer n=17 entity=test-data-fixtures#2.1 v=no
why: the server-identity impostor cases in tests/security_negatives_server_identity.rs and tests/hook_fail_open.rs are not reached and the chunk changes no server-identity check
@answer n=18 entity=test-data-fixtures#2.2 v=no
why: record_live in crates/viola-mcp/tests/handlers.rs is not reached and the chunk changes no unserved-row check
@answer n=19 entity=test-data-fixtures#2.3 v=no standing=S3
why: a home a test makes before viola through viola_state::fs::create_private_dir is the new-test home convention (S3)
@answer n=20 entity=test-data-fixtures#2.4 v=no standing=S3
why: a test's endpoint directory made through viola_state::fs::create_private_dir is the new-test isolation convention (S3)
@answer n=21 entity=test-data-fixtures#3.1 v=no
why: the ETXTBSY plant-before-thread rule for multiple fake-agent copies is not reached and the chunk plants no multiple copies
@answer n=22 entity=test-data-fixtures#4.1 v=no
why: the fake agent's hook-command runner over hooks.json is not reached and no chunk step runs or changes hook commands
@answer n=23 entity=test-data-fixtures#5.1 v=yes
why: the chunk measures the no-leak property through the fake agent's fds receipt kind and its size and key kinds back the Module-to-PTY resize and key-after-resize boundary
@answer n=24 entity=test-data-fixtures#6.1 v=yes
why: the fake agent's Unix holder spawns in its own process group and holds the PTY slave open, which the integration boundary and the fake-agent Unix behaviour the live run confirms both rely on
@answer n=25 entity=test-data-fixtures#7.1 v=no
why: the fixture hygiene walks over the committed fixture sets are not reached, the chunk changes no fixture, and a green test gate is a re-check every change owes
