# Cascade compare — security-plan · phase · 2026-10-27-pinned-copy-race-and-reproducible-builds
E items 101 · held 15 · lost at classify 1 · lost at strong 1 · beside a placed fact - · cascade only 6 · listed 4 · body 53 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### dependency-security#4.1 · lost at strong · .andromeda/security-plan-detail/dependency-security.md:26
- (.andromeda/security-plan-detail/dependency-security.md:26) If P3 changes `pin_companions`' wait-out on Windows, the ConPTY pins and their one textual home must stay untouched; the chunk moves no version.
  > The vendored ConPTY package is pinned exactly by four pins with one textual home, `src/conpty.rs`: the package version (`1.24.260710001`), the nupkg SHA-256 (`175640566a3b59c4b132070ee96c2c77e5ab7edd2e92732a5eb3610bbf63d90e`), `conpty.dll`'s SHA-256 (`39fba2713e2495117b1591ae8c32a3b904bea7aa66069cf7815e2844c76d75d8`) and `OpenConsole.exe`'s (`b7fd936c2668b87b9ecf7b3366dc6568afc1c6f981874cba3e955a1c35cf8160`); `scripts/conpty-vendor.sh` parses them from that text.

### threat-model-summary#9.1 · lost at classify · .andromeda/security-plan-detail/threat-model-summary.md:140
- (.andromeda/security-plan-detail/threat-model-summary.md:140) The pinned copy's folder is the child's first PATH entry, so a missing or half-written copy during a concurrent start is reachable by the child's own lookups too, not only the hook spawn.
  > `run` puts the pinned copy's folder first on the child's PATH.

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### dependency-security#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/dependency-security.md:14
- (.andromeda/security-plan-detail/dependency-security.md:14) The race fix lands in `viola-state` (a sync crate) and so must pull no `tokio` and no C-building crate; the bans and their live probes stay as they are.
  > The arch bans stay as they are: C-building crates in `deny.toml`, and `tokio` in the sync crates' graph for three target triples in `deny-sync.toml`, run per sync crate as the sole root over the four crates of `scripts/sync-crates.txt` (arch §Build system).

### dependency-security#7.1 · listed (lost at classify) · .andromeda/security-plan-detail/dependency-security.md:48
- (.andromeda/security-plan-detail/dependency-security.md:48) Any CI step P4 adds to measure the builds on CI's three OSes must use only the SHA-pinned action set.
  > Actions are pinned by full commit SHA (the set `ci.yml` uses; SHAs resolved 2026-09-24 via `gh api repos/{repo}/commits/{tag}`):

### secret-management#1.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:8
- (.andromeda/security-plan-detail/secret-management.md:8) The race fix, and any narrower concurrent-pin control that widens the race window, must add no environment read: only the two listed test seams and the three colour facts are read outside `VIOLA_*` plumbing, so a pin delay or lock-timeout knob read from the environment is outside the plan.
  > environment variables are not a configuration channel — the variables outside `VIOLA_*` plumbing are the test-only seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, compiled out of release builds, and the terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM`, read in every build by `viola list`'s human mode only to decide whether and how deep SGR is written (they configure nothing, disable no control and widen no redaction; the founder's own word, chunk 2026-10-17-cli-output-discipline; §Input Validation)

### secret-management#1.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:9
- (.andromeda/security-plan-detail/secret-management.md:9) The two-instance boot witness and the agent's hook spawn of the pinned path must keep the R8 strip as it is. Any reading of the failed spawn's kernel error (the `ETXTBSY` hypothesis), whether in diagnostics or in evidence, must carry no inherited `CLAUDE*` value, and names only where the environment is recorded at all.
  > Inherited credentials (`CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and the other `CLAUDE*` identity variables) stay only in the `viola run` wrapper's environment. They are removed from the child by the R8 rule and are never serialised into events, snapshots, diagnostics or the ledger.

### secret-management#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:15
- (.andromeda/security-plan-detail/secret-management.md:15) The race reproduction (red before, clean after) and its single-instance control must each run on a fresh home of the test's own, never on the user's default home.
  > - **Development:** Same as production. Tests use their own `--home` directory.

### secret-management#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:16
- (.andromeda/security-plan-detail/secret-management.md:16) If the two clean release builds, or any reproducibility change measured on the Linux dev host (a path remap, a linker flag, a profile key), go through the pre-push launcher, they must add no assignment beyond `HOME` and `PATH`. A value taken from the host, such as a remap prefix holding the home or repository path, is a boundary widening that needs its own Decisions Log entry, and so is a fork for the founder.
  > No other assignment is made: every launcher call carries exactly `HOME` and `PATH`, every PATH component a constant under the home or a system dir.
  > `<home>` is the passwd entry's field 6, read by two probes that carry only `PATH=/usr/bin:/bin` (`id -u`, then `getent passwd <uid>`); the harness's own `$HOME` is never read for it (that would carry a host value across `env -i`), and an unreadable entry stops at `tools` with `tool-missing` `passwd-home`. Any assignment whose value comes from the host is a boundary widening and needs its own Decisions Log entry;

## Held

- authentication-authorization#1.8
- authentication-authorization#1.9
- bootstrap-phases#1.1
- data-protection#1.1
- error-handling#1.10
- error-handling#1.9
- input-validation#1.20
- input-validation#1.6
- security-anti-patterns#3.1
- security-anti-patterns#4.2
- security-anti-patterns#5.1
- threat-model-summary#1.3
- threat-model-summary#10.1
- threat-model-summary#11.1
- threat-model-summary#8.1

## Cascade only

- error-handling#1.1
- input-validation#1.12
- security-anti-patterns#4.1
- threat-model-summary#1.5
- threat-model-summary#5.1
- threat-model-summary#7.1
