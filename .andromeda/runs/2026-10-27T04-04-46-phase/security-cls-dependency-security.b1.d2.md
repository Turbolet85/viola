# Classified — security · dependency-security.b1

## Answers
dependency-security#1.1 · no · the vendored ConPTY binaries and their conpty-vendor audit are out of scope (Not needed: no input boundary this chunk bounds), and no vendor file is reached
dependency-security#2.1 · no · a re-check every change owes: the chunk adds no crate, feature or manifest edit (no Cargo.toml, Cargo.lock or deny config is reached), so the arch bans stand as they are
dependency-security#3.1 · no · standing S4 · a new fuzz or property target stays in the separate `fuzz/` workspace, never linked into `viola` and never in the root `[workspace]`, its lockfile audited on its own
dependency-security#4.1 · no · the four ConPTY pins sit in src/conpty.rs (not reached) and belong to the vendored audit, out of scope (Not needed); viola's own pinned key is a different pin
dependency-security#5.1 · no · standing S6 · a new CI step pins actions by full SHA, keeps `permissions: {}` / `contents: read`, and takes no `github.event` value through `${{ }}` in `run:`
dependency-security#6.1 · no · a re-check every change owes: the chunk adds no G2 exemption and reaches no seam file (src/cmd/hook/seam.rs), so the one exempt location stays as it is
dependency-security#7.1 · no · standing S6 · a new CI step pins actions by full SHA, keeps `permissions: {}` / `contents: read`, and takes no `github.event` value through `${{ }}` in `run:`
