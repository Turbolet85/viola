@answer n=1 entity=occupied-resources#26.1 v=yes
why: check_home_tree is the strict-modes home-tree rule (read and write rules, per OS) that the parity suite reads per OS.
@answer n=2 entity=occupied-resources#27.1 v=yes
why: entry points run the instance strict-modes check first and refuse leaving modes as found, which the per-OS refusal tests read.
@answer n=3 entity=occupied-resources#28.1 v=yes
why: a regular file above a checked path is NotFound on Windows but a strict-modes refusal on Unix, so a parity test planting one needs the per-OS oracle.
@answer n=4 entity=occupied-resources#29.1 v=yes
why: viola list refuses a 0750 home on every OS under the read rule, a strict-modes rule the parity suite reads on list.
@answer n=5 entity=occupied-resources#30.1 v=yes
why: names the home write rule that mcp start and plugin install keep, a strict-modes rule the parity suite reads per OS.
@answer n=6 entity=occupied-resources#31.1 v=no
why: the budget.json read is the budget governor, which nothing in the scope reaches, and its strict check is not a named parity rule.
@answer n=7 entity=occupied-resources#32.1 v=no
why: ledger stamps belong to viola verify and run's version gate, which the chunk does not reach.
@answer n=8 entity=occupied-resources#33.1 v=no
why: verify's ledger probe dirs are verify's own, excluded by the not-needed verify and probe-dir item.
@answer n=9 entity=occupied-resources#34.1 v=no
why: verify's interactive probe dirs and their stop handling are verify's own, excluded by the not-needed item on viola verify.
@answer n=10 entity=occupied-resources#35.1 v=yes
why: the scope names the root Watch WITHIN 7 s and its viola-root-watch evidence report, which the kill-and-revive support uses.
@answer n=11 entity=occupied-resources#36.1 v=yes
why: holder_gone is the Unix reading Wrapper::kill uses for a killed wrapper, and harness cleanup's endpoint_gone shares it.
@answer n=12 entity=occupied-resources#37.1 v=no
why: the error-body helper checks existing error bodies, and the scope neither names it nor adds error-body tests.
@answer n=13 entity=occupied-resources#37.2 v=no
why: the exit-cause table checks hints for existing codes, and the scope keeps the typed exit-code set unchanged.
@answer n=14 entity=occupied-resources#38.1 v=no
why: the dev-host tmpfs link is outside the CI runners the chunk's jobs and gates run on, and no CI runner has it.
@answer n=15 entity=occupied-resources#39.1 v=yes
why: the test-owned socket roots and the per-OS SOCKET_ROOT_VAR (TMPDIR on macOS) are what the socket-dir parity tests use under the scope's socket-dir item.
@answer n=16 entity=occupied-resources#40.1 v=no
why: fixture recording is not needed (not-needed item on fixtures/claude) and the chunk changes no fixture or replay.
@answer n=17 entity=occupied-resources#41.1 v=yes
why: the perf job's perf-<hook>.json exports under target/agent-run/artifacts are what gate --require perf reads on the two Unix runners.
@answer n=18 entity=occupied-resources#42.1 v=yes
why: the scope names the test homes under target/e2e-home kept in CI that the obs gates G1 to G4 read, with the owner-record sweep that keeps them.
@answer n=19 entity=occupied-resources#43.1 v=yes
why: ci.yml's AGENT_RUN_KEEP_HOMES and always() keep the test homes for the gates, with the viola-chaos carve-out the scope names.
@answer n=20 entity=occupied-resources#44.1 v=no
why: the icacls inheritance into seed_conpty is Windows-only, and Windows is unmoved.
@answer n=21 entity=occupied-resources#45.1 v=no
why: the profraw census is a dev-host witness no CI job runs, excluded by the not-needed item on the profraw census.
@answer n=22 entity=occupied-resources#46.1 v=no
why: the Windows mutation scratch and the dev-host mutants TMPDIR are outside the chunk, excluded by the not-needed mutation-arm item.
@answer n=23 entity=occupied-resources#47.1 v=no
why: the Windows DACL SDDL on windows-2025 is Windows-only, and Windows is unmoved.
@answer n=24 entity=occupied-resources#47.2 v=no
why: the Windows runner temp-dir ACL variance is Windows-only, and Windows is unmoved.
