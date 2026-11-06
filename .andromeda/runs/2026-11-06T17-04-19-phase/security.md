@extract_section name=relevance
partial
@extract_section name=constraints
- security-plan §Dependency Security requires the RUSTSEC-2026-0441 remedy (scope item 12) to move `portable-pty` off `=0.8.1` to a release whose graph drops `pin-utils`, with no ignore widened: the `[advisories]` ignore list keeps its one entry, and the RUSTSEC-2017-0008 ignore (its reason naming `portable-pty =0.8.1`) is re-checked whenever the pin moves. Whether a candidate release (or arch's named swap path, portable-pty-psmux 0.9.7) actually drops `pin-utils`/`nix 0.25.1` and `serial` is research's question.
- security-plan §Dependency Security requires the new `portable-pty` version to stay exactly pinned in `[workspace.dependencies]` with `Cargo.lock` committed, crates.io as the only source, and the licence `allow` list unchanged: any new transitive crate under a licence outside it takes a per-crate `[[licenses.exceptions]]` with its own Decisions Log entry, never an `allow` addition.
- security-plan §Data Protection (In transit, Handle inheritance) requires the channel-handle no-leak property to be measured on Linux (scope item 15) before fct-p7rzex2i is re-worded off «recorded, not established», and requires any portable-pty move or swap to keep that property; whether the code's channel descriptors are already close-on-exec is research's question.
- security-plan §Error Handling requires the `Display` impls of the product error enums (the list naming `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError`) to use fixed messages with path/payload fields excluded; any fold or ratification under scope item 6 or the `thiserror` census of item 16 must keep that and keep the list in step with the code.
- security-plan §Bootstrap phases (`logging-redaction-wire`) requires that evidence from the chunk's live sessions (v1-53's live `viola run`, the overseer-drives-builder session) carry no R8-stripped `CLAUDE*` value, no absolute path or username and no user content outside `diagnostics/`; security-plan §Data Protection (Repository fixtures) requires any recorded fixture to be synthetic-prompt and scrubbed before commit.
- security-plan §Dependency Security (CI integration) requires CI's jobs to stay as specified while the gates are driven green: `cargo deny check` over all four families plus the separate `fuzz/Cargo.lock` audit, zizmor on the workflows, actions pinned by full SHA, `permissions: {}` with job-level `contents: read`, and the `release` job's `scripts/release-check.sh` refusing test-only features.
- security-plan §Security Anti-Patterns (Universal) requires that the Windows-remnant removals (scope item 17) and any test edits add no environment read, test seam or listener without its own Decisions Log entry; the two test-only seams and terminal colour facts stay the only carve-outs.
@extract_section name=patterns
- `scripts/deny-probes.sh` proves every `deny.toml` / `deny-sync.toml` ban live, beside `scripts/channel-tokio-free.sh` for `viola-channel` (per security-plan §Dependency Security); the portable-pty move is re-checked through them.
- `nightly.yml` runs the weekly `cargo deny check advisories` on both lockfiles, the scheduled advisory signal behind the manual update policy (per security-plan §Dependency Security).
- `viola-harness secret-scan` (obs-owned artifact canary scan) gates every test-home upload, never printing matched bytes (per security-plan §Secret Management).
- The critical CVE SLA's named path is bump, or swap portable-pty → portable-pty-psmux 0.9.7 (per security-plan §Dependency Security).
@extract_section name=anti_patterns
- NEVER make channel handles inheritable (per security-plan §Security Anti-Patterns, Data Protection) — binds the portable-pty move and the item 15 measurement.
- NEVER reference a GitHub Action by a mutable ref; full SHA with a version comment only (per security-plan §Security Anti-Patterns, Code Patterns).
- NEVER add another seam or environment read, nor let any config/env/flag switch off a control, without its own Decisions Log entry (per security-plan §Security Anti-Patterns, Universal).
@extract_section name=bindings
- security ↔ arch: the portable-pty pin and its swap path are arch's dependency decision (security-plan §Dependency Security); the no-leak property travels with any swap (security-plan §Data Protection).
- security ↔ tests: the `supply-chain` job, deny probes and release check are CI security gates inside tests' one workflow; the item 15 `/proc/<child pid>/fd` measurement runs under the fake agent.
- security ↔ obs: live-session evidence and uploads pass obs-plan §9's secret-scan against this plan's NEVER-log floor (security-plan §Bootstrap phases); the item 6/16 error-enum folds touch obs error-capture alongside §Error Handling.
@extract_section name=acceptance
- `cargo deny check` (advisories, bans, licenses, sources) and CI's `supply-chain` job read green, `pin-utils` absent from the root graph, the `[advisories]` ignore list holding no new entry (per security-plan §Dependency Security).
- Under the fake agent, the wrapped child's `/proc/<child pid>/fd` holds no channel socket or listener, and only then is fct-p7rzex2i re-worded (per security-plan §Data Protection).
- `scripts/deny-probes.sh`, `scripts/release-check.sh --probe` and zizmor on `.github/workflows/` pass after the dependency move (per security-plan §Dependency Security).
- The `secret-scan` step passes on the chunk's CI run and on any live-session evidence committed (per security-plan §Secret Management).
