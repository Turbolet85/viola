
## 2026-09-29-sideloaded-conpty — the vendored binaries' own audit, pins and CI step
**Section:** Dependency Security → Audit tool, Pinning, CI integration; Bootstrap phases → dep-audit-tooling-install, dep-security-ci-gate
**Change:**
- Audit tool: the vendored ConPTY binaries sit outside every cargo and npm graph; their gate is `scripts/conpty-vendor.sh` — `--verify` (fetch outside the tree, nupkg SHA-256 before extraction, byte compare, Authenticode signer; `conpty-vendor: verified <version>`) and `--probe` (four refusals and a control; `conpty-vendor probe: 4/4 refused, control clean`). The signer is checked there and in CI; at run time the SHA-256 pin carries it.
- Pinning: four exact pins (package version, nupkg SHA-256, two file SHA-256s) with one textual home, `src/conpty.rs`; committed binary; a version move is a re-vendor through the script.
- CI integration and both bootstrap phases: the `test` job's `windows-2025` step `ConPTY vendor verification` and the script join the lists.
**Why:** a new non-crate dependency needs its own audit like the npm graph and the fuzz lockfile before it (an own audit, never an exemption).
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/
