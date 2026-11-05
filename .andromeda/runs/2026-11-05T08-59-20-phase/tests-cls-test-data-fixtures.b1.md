@answer n=1 entity=test-data-fixtures#1.1 v=yes
why: the Windows x64 seed_conpty exception and the windows-2025 strict-modes and SDDL text name seeding the chunk removes (scope items 7 and 8)
@answer n=2 entity=test-data-fixtures#1.2 v=yes
why: a widened seeded home propagated through seed_conpty hard links is a Windows-only mechanism retired with seed_conpty and the icacls grants (scope items 7 and 8)
@answer n=3 entity=test-data-fixtures#1.3 v=no
why: ledger/stamps.json written by viola verify carries no Windows or macOS arm and the chunk does not reach it
@answer n=4 entity=test-data-fixtures#1.4 v=no
why: budget.json generated statusline stdin carries no OS text and no arm in the chunk's scope
@answer n=5 entity=test-data-fixtures#1.5 v=yes
why: Wrapper::boot_as_user's Windows USERPROFILE branch is a cfg(windows) arm the chunk removes, while the Linux HOME rule it names stays
@answer n=6 entity=test-data-fixtures#1.6 v=yes
why: SOCKET_ROOT_VAR's macOS TMPDIR arm is a cfg(target_os = "macos") arm the chunk removes
@answer n=7 entity=test-data-fixtures#1.7 v=yes
why: the icacls set and read-back path of widen, grant_read, unwiden and still_widened in tests/support/home.rs is the Windows DACL code scope item 8 removes
@answer n=8 entity=test-data-fixtures#1.8 v=no
why: the socket-directory Form sets a 0755 or symlinked directory on Linux with no Windows or macOS text, and retiring the file's macOS run leaves it unchanged
@answer n=9 entity=test-data-fixtures#1.9 v=no
why: the claude agents --json fake output and its PATH placement name no OS arm and the chunk does not change them
@answer n=10 entity=test-data-fixtures#1.10 v=yes
why: the StatusLine.default-1 and StatusLine.rate-limits-1 fixtures recorded on Unix are the statusline pass-through gate scope item 12 makes unconditional
@answer n=11 entity=test-data-fixtures#1.11 v=no
why: the recorded MCP exchange capture names no Windows or macOS arm and no gate in the chunk's scope
@answer n=12 entity=test-data-fixtures#1.12 v=yes
why: the CAUSES exit-cause table with its .cmd human pair and the HELP literal with its OS words are in the chunk's scope (items 10 and 14)
@answer n=13 entity=test-data-fixtures#1.13 v=no
why: insta decision bodies per dialog kind are Path 4 work marked not needed for this chunk
@answer n=14 entity=test-data-fixtures#1.14 v=no
why: fake-agent hook transcripts compared byte for byte to recorded payloads carry no OS text and no arm in scope
@answer n=15 entity=test-data-fixtures#1.15 v=no
why: proptest strategies and their committed regressions name no OS arm and the chunk does not touch them
@answer n=16 entity=test-data-fixtures#1.16 v=no standing=S3
why: time-sensitive data injected per test through MockClock and no timer in viola-mcp is the injected-values convention S3 states
@answer n=17 entity=test-data-fixtures#1.17 v=no
why: the channel error-body witness's planted value through the PTY spawn names no OS arm and is outside the chunk's scope
@answer n=18 entity=test-data-fixtures#2.1 v=yes
why: the Linux and Windows run of tests/security_negatives_server_identity.rs and the Windows impostor cases are the file's OS gate that scope item 11 retires
@answer n=19 entity=test-data-fixtures#2.2 v=yes
why: record_live's unserved-row rule rests on a Windows named-pipe Serving with no Drop, which the named-pipe removal of scope item 9 retires
@answer n=20 entity=test-data-fixtures#2.3 v=yes
why: the per-OS oracle for a home planted under a regular file, its Windows NotFound branch, is a cfg(windows) arm the chunk removes
@answer n=21 entity=test-data-fixtures#2.4 v=no
why: endpoint-directory creation through create_private_dir holds on Linux with no Windows or macOS text, and the removed gate of its file does not change it
@answer n=22 entity=test-data-fixtures#3.1 v=no
why: planting several fake-agent copies on the tmpfs target/e2e-home concerns ETXTBSY and has no OS arm or free-space rule in the chunk's scope
@answer n=23 entity=test-data-fixtures#4.1 v=yes
why: the statusline replay under --framing, including the $0 word handed to /bin/sh, is the statusline pass-through Unix gate scope item 12 makes unconditional
@answer n=24 entity=test-data-fixtures#5.1 v=no
why: the fake agent's receipt kinds carry no gate that scope item 12 names and Linux receipt behaviour is unchanged
@answer n=25 entity=test-data-fixtures#6.1 v=yes
why: the --exit-no-eof holder spawning in its own process group on Unix is the fake agent's Unix-only process-group arm scope item 12 makes unconditional
