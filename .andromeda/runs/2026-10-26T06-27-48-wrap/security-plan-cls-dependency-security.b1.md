# Classified — security-plan · dependency-security.b1

## Answers
dependency-security#1.1 · no · the report's Untouched section says the vendored ConPTY binaries still sit outside every lockfile and did not move, and the lock delta is the tokio entry alone
dependency-security#2.1 · yes · stale · the tokio pin this line names (=1.53.1, viola-channel's Tokio client) moves to =1.53.3 with the workspace pin
dependency-security#3.1 · no · the report shows the fuzz/ exemption still holds on the changed root Cargo.toml and lock, its exception group untouched and the fuzz lockfile audit reading green
dependency-security#4.1 · no · the report's Untouched section says the four ConPTY pins in src/conpty.rs and scripts/conpty-vendor.sh did not move, and no reached file holds them
dependency-security#5.1 · no · the windows-mutants workflow is not a reached file, and the report's CI integration controls say no workflow changed and its SHA pins and permissions hold
dependency-security#6.1 · no · the report's security-anti-patterns line says the G2 exemption was not added, loosened or exercised differently, and no seam or G2 script is in a reached file
dependency-security#7.1 · no · the report's CI integration controls say no workflow changed and the actions stay SHA-pinned, and no reached file holds those pins
dependency-security#8.1 · yes · contradict · the bump to tokio =1.53.3 closed RUSTSEC-2026-0386 and CI supply-chain is green, so the open-advisory line no longer holds
