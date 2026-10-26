# Cascade compare — security-plan · phase · 2026-10-26-home-and-code-bearing-file-integrity
E items 170 · held 27 · lost at classify 1 · lost at strong 4 · beside a placed fact - · cascade only 2 · listed 4 · body 62 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### authentication-authorization#3.6 · lost at classify · .andromeda/security-plan-detail/authentication-authorization.md:75
- (.andromeda/security-plan-detail/authentication-authorization.md:75) Boundary: the macOS cases and the client euid check belong to "Unix endpoint and home hardening", not to this chunk.
  > The macOS server-verification cases, the per-user socket directory loosened to 0755 giving exit 21 and the client euid check, are a CARRY owed to the route entry "Unix endpoint and home hardening".

### authentication-authorization#3.7 · lost at strong · .andromeda/security-plan-detail/authentication-authorization.md:78
- (.andromeda/security-plan-detail/authentication-authorization.md:78) The witness for each newly checked verb must have this shape: a widened instance refused before any log, its mode as found.
  > `link_verbs_refuse_a_widened_instance_before_any_log` (Unix only) reads a group-writable driven instance refused before any log, its mode as found;

### secret-management#1.1 · lost at strong · .andromeda/security-plan-detail/secret-management.md:8
- (.andromeda/security-plan-detail/secret-management.md:8) The strict-modes checks this chunk adds (the hook's read through `VIOLA_DIR`, every CLI / MCP entry point) must take no new environment read and no env-var switch, and a test-side way around the check for a seeded home (CARRYs 6, 13) must not be an environment seam.
  > environment variables are not a configuration channel — the variables outside `VIOLA_*` plumbing are the test-only seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, compiled out of release builds, and the terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM`, read in every build by `viola list`'s human mode only to decide whether and how deep SGR is written

### secret-management#1.1 · lost at strong · .andromeda/security-plan-detail/secret-management.md:9
- (.andromeda/security-plan-detail/secret-management.md:9) Every snapshot, diagnostics file and ledger stamp this chunk writes, checks or newly guards (the DACL check before `diagnostics/detail-*.ndjson`, the single snapshot/stamp writers, any refusal record) must carry no inherited `CLAUDE*` credential.
  > Inherited credentials (`CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and the other `CLAUDE*` identity variables) stay only in the `viola run` wrapper's environment. They are removed from the child by the R8 rule and are never serialised into events, snapshots, diagnostics or the ledger.

### threat-model-summary#1.5 · lost at strong · .andromeda/security-plan-detail/threat-model-summary.md:32
- (.andromeda/security-plan-detail/threat-model-summary.md:32) The snapshot's `cwd` lives only in the 0600 snapshot, so a new strict-modes refusal at any entry point must print no path from it.
  > an absolute host path that can carry the OS username; it lives in the 0600 snapshot only, on no log line, error body or stdout, and is omitted when absent

### threat-model-summary#9.1 · lost at strong · .andromeda/security-plan-detail/threat-model-summary.md:137
- (.andromeda/security-plan-detail/threat-model-summary.md:137) The child's PATH puts the pinned copy's folder first, so that copy must be re-hashed before reuse and must not be half-written by a concurrent start (CARRY 16).
  > `run` puts the pinned copy's folder first on the child's PATH.

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### bootstrap-phases#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/bootstrap-phases.md:24
- (.andromeda/security-plan-detail/bootstrap-phases.md:24) Every new or changed test home's diagnostics the chunk produces in CI must pass the artifact canary scan against the NEVER-log floor.
  > The CI `test` job does run obs-plan §9's artifact canary scan: `viola-harness secret-scan` over test-home diagnostics, the harness capture except `run --mutants`' `target/agent-run/chunk.diff`, and the nextest JUnit report, checked against this plan's NEVER-log floor.

### bootstrap-phases#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/bootstrap-phases.md:25
- (.andromeda/security-plan-detail/bootstrap-phases.md:25) Any test home the chunk adds or reseeds (the `seed_conpty` question, widened-mode negatives) must stay out of git.
  > `.gitignore` excludes any local `--home` test directories so recorded state never lands in git.

### dependency-security#2.1 · listed (lost at strong) · .andromeda/security-plan-detail/dependency-security.md:14
- (.andromeda/security-plan-detail/dependency-security.md:14) Any dependency or feature the chunk adds for the strict-modes, DACL or re-hash work in a sync crate (`viola-state` among them) must keep the C-building-crate ban and the per-sync-crate `tokio` ban green.
  > The arch bans stay as they are: C-building crates in `deny.toml`, and `tokio` in the sync crates' graph for three target triples in `deny-sync.toml`, run per sync crate as the sole root over the four crates of `scripts/sync-crates.txt` (arch §Build system).

### dependency-security#2.1 · listed (lost at strong) · .andromeda/security-plan-detail/dependency-security.md:14
- (.andromeda/security-plan-detail/dependency-security.md:14) The live proof of every ban must still read green after any dependency change the chunk makes.
  > Every ban, the tree form included (probe `channel-tokio-edge` and its channel control, `13/13 banned, control clean`), is proven live by `scripts/deny-probes.sh`.

### secret-management#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:15
- (.andromeda/security-plan-detail/secret-management.md:15) The chunk's negative cases (a 0770 `--home`, a widened instance DACL, a seeded Windows home) must each run in a test's own `--home`, never the user's real home.
  > Same as production. Tests use their own `--home` directory.

### secret-management#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:16
- (.andromeda/security-plan-detail/secret-management.md:16) Any gate this chunk runs through the local pre-push launcher (its stamped-home gate, CARRY 11's two clean release builds if run there) must launch under `env -i` with exactly `HOME` and `PATH`; an added host-valued assignment is a boundary widening that halts for the founder's word.
  > launches every child natively as `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>` from the repository root, so no host environment value — the inherited `CLAUDE*` credentials included — crosses into a child. No other assignment is made: every launcher call carries exactly `HOME` and `PATH`, every PATH component a constant under the home or a system dir.

### security-anti-patterns#3.1 · listed (lost at classify) · .andromeda/security-plan-detail/security-anti-patterns.md:23
- (.andromeda/security-plan-detail/security-anti-patterns.md:23) Test homes under `target/e2e-home`, which every newly checked verb will now read, must stay on owner-only bases created 0700 through the directory builder, never a chmod after — the seeded-home question (CARRYs 6, 13) is answered by moving the seed, never by loosening the check.
  > They create the target with mode 0700 through the directory builder (never a chmod after, never a parent) and remove nothing.

## Held

- authentication-authorization#1.6
- authentication-authorization#1.8
- authentication-authorization#3.2
- authentication-authorization#3.3
- authentication-authorization#3.4
- bootstrap-phases#1.1
- data-protection#1.1
- dependency-security#1.1
- dependency-security#4.1
- error-handling#1.1
- error-handling#1.2
- error-handling#1.3
- input-validation#1.12
- input-validation#1.19
- input-validation#1.20
- input-validation#1.6
- input-validation#2.0
- security-anti-patterns#1.1
- security-anti-patterns#2.1
- security-anti-patterns#4.1
- security-anti-patterns#5.1
- threat-model-summary#1.3
- threat-model-summary#11.1
- threat-model-summary#2.1
- threat-model-summary#5.1
- threat-model-summary#7.1
- threat-model-summary#8.1

## Cascade only

- input-validation#1.18
- input-validation#1.4
