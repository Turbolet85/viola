@answer n=1 entity=dependency-security#1.1 v=yes mode=contradict
why: the session deleted the vendored ConPTY binaries and their conpty-vendor.sh gate, which this row describes as live
@answer n=2 entity=dependency-security#2.1 v=no
why: the session's deny.toml edits move only the targets and the licence exceptions, so the serde_json bans.features entry still holds
@answer n=3 entity=dependency-security#3.1 v=yes mode=stale
why: deny-sync.toml's targets dropped from three triples to x86_64-unknown-linux-gnu alone, so the "three target triples" count moved
@answer n=4 entity=dependency-security#4.1 v=no
why: the session changed no supply-chain step (its removals sit in the test job) and the measured supply-chain run is red only on RUSTSEC-2026-0441
@answer n=5 entity=dependency-security#5.1 v=yes mode=contradict
why: the four ConPTY pins in src/conpty.rs and the vendor binary rule in .gitattributes are removed with the vendored package
@answer n=6 entity=dependency-security#6.1 v=yes mode=contradict
why: windows-mutants.yml is deleted, so the dispatch-only third workflow with its mutants job no longer exists
@answer n=7 entity=dependency-security#7.1 v=no
why: the session leaves the FAKE_AGENT_HOOK_PANIC seam and the G2 gate in the test and perf jobs, and the measured run shows those jobs green
@answer n=8 entity=dependency-security#8.1 v=yes mode=stale
why: ci.yml lost its windows-2025 test steps and junit artifacts, so the action SHA set it uses may have moved and the report does not show it holds
