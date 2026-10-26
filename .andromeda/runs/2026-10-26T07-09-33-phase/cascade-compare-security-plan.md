# Cascade compare — security-plan · phase · 2026-10-26-server-verification-before-any-frame
E items 165 · held 22 · lost at classify 1 · lost at strong 0 · beside a placed fact - · cascade only 7 · listed 3 · body 95 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### secret-management#1.1 · lost at classify · .andromeda/security-plan-detail/secret-management.md:8
- (.andromeda/security-plan-detail/secret-management.md:8) The server verification must be switched or faked by no environment variable: a test seam that forces a mismatch, or any env read that skips the check, is outside the closed set of non-`VIOLA_*` reads and needs its own Decisions Log entry.
  > environment variables are not a configuration channel — the variables outside `VIOLA_*` plumbing are the test-only seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, compiled out of release builds, and the terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM`

### secret-management#1.1 · lost at classify · .andromeda/security-plan-detail/secret-management.md:9
- (.andromeda/security-plan-detail/secret-management.md:9) Every line the new check writes on a mismatch or a check it cannot make (diagnostics, events, a refusal) must carry no inherited `CLAUDE*` value.
  > Inherited credentials (`CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and the other `CLAUDE*` identity variables) stay only in the `viola run` wrapper's environment. They are removed from the child by the R8 rule and are never serialised into events, snapshots, diagnostics or the ledger.

### secret-management#1.1 · lost at classify · .andromeda/security-plan-detail/secret-management.md:9
- (.andromeda/security-plan-detail/secret-management.md:9) The hook client (`hook.event`, `hook.dialog`) gains its verification inside the environment Claude Code hands a hook, which holds both messaging variables, so the hook's fail-open mismatch path must read and record names only, never a value.
  > the environment `claude` 2.1.287 hands a hook, read in a child started with every inherited `CLAUDE*` name removed, holds 12 names, both messaging variables among them

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### bootstrap-phases#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/bootstrap-phases.md:24
- (.andromeda/security-plan-detail/bootstrap-phases.md:24) Any new diagnostics this chunk writes on a mismatch must pass the CI artifact canary scan over test-home diagnostics, which gates every scan-gated upload.
  > The CI `test` job does run obs-plan §9's artifact canary scan: `viola-harness secret-scan` over test-home diagnostics, the harness capture except `run --mutants`' `target/agent-run/chunk.diff`, and the nextest JUnit report, checked against this plan's NEVER-log floor.

### secret-management#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:15
- (.andromeda/security-plan-detail/secret-management.md:15) The mismatch and unverifiable-server tests must run against their own `--home`, never the user's `~/.viola`.
  > - **Development:** Same as production. Tests use their own `--home` directory.

### secret-management#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:16
- (.andromeda/security-plan-detail/secret-management.md:16) The chunk's gates run under the pre-push launcher with exactly `HOME` and `PATH`; a test of the verification must not need a host value crossing into a child, and any such assignment is a boundary widening held for the founder's word.
  > launches every child natively as `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>` from the repository root, so no host environment value — the inherited `CLAUDE*` credentials included — crosses into a child. No other assignment is made: every launcher call carries exactly `HOME` and `PATH`, every PATH component a constant under the home or a system dir.

### secret-management#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:16
- (.andromeda/security-plan-detail/secret-management.md:16) A host-valued assignment added for a verification test is a widening, never an implementation choice.
  > Any assignment whose value comes from the host is a boundary widening and needs its own Decisions Log entry

### security-anti-patterns#3.1 · listed (lost at classify) · .andromeda/security-plan-detail/security-anti-patterns.md:22
- (.andromeda/security-plan-detail/security-anti-patterns.md:22) The mismatch and impersonation tests this chunk adds create test homes under `target/e2e-home`; their removal must take only dirs the test side created.
  > NEVER let a test-side removal under `target/e2e-home` take anything but a dir the test side itself created: the root fixture chain's owner sweep and a test home's drop (by `owner.json` record, by its own `TempDir`) and harness `cleanup` (by the `viola-session-` name).

## Held

- authentication-authorization#1.4
- authentication-authorization#1.6
- authentication-authorization#1.8
- bootstrap-phases#1.1
- data-protection#1.1
- dependency-security#2.1
- error-handling#1.1
- input-validation#1.12
- input-validation#1.4
- input-validation#1.5
- input-validation#1.6
- security-anti-patterns#1.1
- security-anti-patterns#5.1
- threat-model-summary#1.2
- threat-model-summary#1.3
- threat-model-summary#1.5
- threat-model-summary#10.1
- threat-model-summary#2.1
- threat-model-summary#5.1
- threat-model-summary#6.1
- threat-model-summary#7.1
- threat-model-summary#8.1

## Cascade only

- authentication-authorization#1.7
- input-validation#1.1
- input-validation#1.19
- input-validation#1.2
- input-validation#1.3
- input-validation#2.0
- threat-model-summary#1.1
