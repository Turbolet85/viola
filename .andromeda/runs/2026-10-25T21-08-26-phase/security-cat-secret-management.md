# security · Secret Management — category read

## Carried
- (.andromeda/security-plan-detail/secret-management.md:8) No environment variable may become a switch for the Windows listener's admission controls (DACL, remote-client rejection, first-instance, non-inheritable handles) or for a test of them; the chunk adds no environment read outside `VIOLA_*` plumbing.
  > No stored secrets in v1 (Cross-cutting Patterns: Config management, "There are no secrets in v1"; environment variables are not a configuration channel — the variables outside `VIOLA_*` plumbing are the test-only seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, compiled out of release builds
- (.andromeda/security-plan-detail/secret-management.md:9) Any log line, diagnostic or event the chunk adds (an admission refusal, a squatted-name start refusal, the budget test's log capture it repairs) must carry no inherited `CLAUDE*` value.
  > Inherited credentials (`CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and the other `CLAUDE*` identity variables) stay only in the `viola run` wrapper's environment. They are removed from the child by the R8 rule and are never serialised into events, snapshots, diagnostics or the ledger.
- (.andromeda/security-plan-detail/secret-management.md:15) The chunk's new listener and squat tests, and the plain-`cargo test` re-runs of the budget case, use their own `--home`, never the user's home.
  > Tests use their own `--home` directory.
- (.andromeda/security-plan-detail/secret-management.md:16) The local gate the chunk must pass (`viola-harness pre-push`) launches every child, the three plain-`cargo test` runs of the CARRY's budget case included if routed through it, under `env -i` with exactly `HOME` and `PATH`.
  > launches every child natively as `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>` from the repository root, so no host environment value — the inherited `CLAUDE*` credentials included — crosses into a child. No other assignment is made: every launcher call carries exactly `HOME` and `PATH`, every PATH component a constant under the home or a system dir.
- (.andromeda/security-plan-detail/secret-management.md:16) Fixing the budget case's log capture must not add a host-valued assignment to a launcher (e.g. a thread-count or capture variable taken from the host); such an assignment is a widening that halts for the founder's live word.
  > Any assignment whose value comes from the host is a boundary widening and needs its own Decisions Log entry;
- (.andromeda/security-plan.md:377) The chunk's Windows CI leg must keep the artifact canary scan green over whatever its new tests write under `target/agent-run/`.
  > What CI does run is the obs-owned artifact canary scan (`viola-harness secret-scan`, `id: secret-scan`, `if: always()`, obs-plan §9 step 3).
- (.andromeda/security-plan.md:383) No upload the chunk adds or touches may bypass the scan's gate.
  > It never prints or writes the matched bytes, and the test-home uploads run only when it succeeds.

## Bindings
- **Env vars not a configuration channel ↔ body §Security Anti-Patterns → Universal**: no `config.json` key, `VIOLA_*` var or flag may switch off the IPC DACL; the category's rule (.andromeda/security-plan-detail/secret-management.md:8) and the ban (.andromeda/security-plan.md:516) bind together on the Windows listener.
- **R8 floor ↔ body §Bootstrap phases `logging-redaction-wire`**: the stripped `CLAUDE*` values reach no log, diagnostic or fixture (.andromeda/security-plan-detail/secret-management.md:9), the consolidated floor at (.andromeda/security-plan.md:341); obs owns any new admission log line.
- **Pre-push `env -i` launcher ↔ body §Dependency Security (pre-push `tools` stage)**: the launcher carries exactly `HOME` and `PATH` (.andromeda/security-plan-detail/secret-management.md:16) and the gate installs nothing and launches nothing as uid 0 (.andromeda/security-plan.md:269).
- **Artifact canary scan ↔ obs-plan §9 / tests CI integration**: the scan's classes and upload gate (.andromeda/security-plan.md:377), owned by obs-plan §9 step 3, gate the Windows CI leg's new test homes (.andromeda/security-plan.md:383).
