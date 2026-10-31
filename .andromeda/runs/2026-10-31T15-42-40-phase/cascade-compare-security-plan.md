# Cascade compare — security-plan · phase · 2026-10-31-exit-cause-code-catalogue
E items 176 · held 44 · lost at classify 2 · lost at strong 4 · beside a placed fact - · cascade only 16 · listed 5 · body 82 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### data-protection#3.1 · lost at strong · .andromeda/security-plan-detail/data-protection.md:34
- (.andromeda/security-plan-detail/data-protection.md:34) A placement failure other than a present name must be counted when P3 closes the exit-1 list against the code: it either holds a code of its own or is named under `internal-error`, and either way carries no path.
  > `place_private_new` never replaces a present name: `AlreadyExists` is `Placed::Present`, the target untouched and the temp file removed; any other failure is an error that leaves the target as it was, with no retry.

### data-protection#8.1 · lost at strong · .andromeda/security-plan-detail/data-protection.md:64
- (.andromeda/security-plan-detail/data-protection.md:64) A cause's new log form must tolerate this dropped line and must not narrow or skip the check to keep it, and the table must not assert a process-log line for a home whose diagnostics open is refused.
  > `open_role_file` and `write_detail` run `viola_state::strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before `ensure_private_dir` narrows anything: a symlink is refused; a folder takes the read rule; a file takes the read rule on Unix and the write rule on Windows; an absent path passes. A refusal is a failed open: the writer ends at `std::io::sink` and the detail line is dropped.

### data-protection#9.1 · lost at classify · .andromeda/security-plan-detail/data-protection.md:70
- (.andromeda/security-plan-detail/data-protection.md:70) `viola verify --record`'s fixture refusal is an exit-1 cause that P3 must place in the catalogue or name outside it by a reason, keeping the founder-ratified stderr form, its closed codes and no content, and any machine code for it must leave out the `<file>` name.
  > refuses the whole recording (exit 1, nothing written) when a drive path at string start, `/home/`, `/Users/`, `\Users\`, the user word or an email-shaped token survives. A screen fixture (`Screen.<phase>.json`) keeps only the rows that hold a compiled signature literal (the rest `""`), and a kept row the scrub would rewrite, or a username split across a row seam, refuses it too. The refusal names the fixture file and a closed check code, never the content: `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen, with codes `home-path` · `absolute-path` · `username` · `email`.

### dependency-security#3.1 · lost at strong · .andromeda/security-plan-detail/dependency-security.md:20
- (.andromeda/security-plan-detail/dependency-security.md:20) The chunk's edits to `crates/viola-channel/src/client.rs` for the mid-call reading, and any code it places in a sync crate, must add no `tokio` edge: the channel's normal graph must keep printing no `tokio v` line and the sync crates must keep passing `deny-sync.toml`.
  > The arch bans stay as they are: C-building crates in `deny.toml`, and `tokio` in the sync crates' graph for three target triples in `deny-sync.toml`, run per sync crate as the sole root over the four crates of `scripts/sync-crates.txt` (arch §Build system). `viola-channel`, whose Tokio client (tokio `=1.53.3`, `io-util`) sits behind its `tokio` feature, left that list at chunk 2026-10-17-mcp-server-for-drivers and is covered instead by the `cargo tree` form `scripts/channel-tokio-free.sh` (its normal graph without the feature must print no `tokio v` line, its `--features tokio` control one; CI supply-chain step `Tokio ban per sync crate (sole root)`).

### input-validation#1.21 · lost at classify · .andromeda/security-plan-detail/input-validation.md:87
- (.andromeda/security-plan-detail/input-validation.md:87) A ConPTY companion's strict-modes refusal must stay out of the catalogue: its `strict-modes-failed` is a recorded fallback with the exit unchanged, never an exit-1 cause.
  > a refusal is `PinError::StrictModes`, nothing written or loaded, recorded as `strict-modes-failed`, and the child runs on the inbox ConPTY with its exit unchanged.

### secret-management#1.1 · lost at strong · .andromeda/security-plan-detail/secret-management.md:9
- (.andromeda/security-plan-detail/secret-management.md:9) Every new or moved cause record (a `process-exit` `detail`, a cause's new log form, a diagnostics line, a code word) must hold no inherited credential value; a code or log form may be added only as a closed word, never built from the environment.
  > Inherited credentials (`CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and the other `CLAUDE*` identity variables) stay only in the `viola run` wrapper's environment. They are removed from the child by the R8 rule and are never serialised into events, snapshots, diagnostics or the ledger.

### secret-management#1.1 · lost at strong · .andromeda/security-plan-detail/secret-management.md:8
- (.andromeda/security-plan-detail/secret-management.md:8) The exit-cause table must reach each cause (endpoint lost mid-call, internal error, a `claude` not on `PATH`) without a new environment read or test seam: the seams named here are the set, and a cause no existing means reaches is named as such, not given a new variable.
  > environment variables are not a configuration channel — the variables outside `VIOLA_*` plumbing are the test-only seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, compiled out of release builds

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### bootstrap-phases#2.1 · listed (lost at strong) · .andromeda/security-plan-detail/bootstrap-phases.md:24
- (.andromeda/security-plan-detail/bootstrap-phases.md:24) Any new `process-exit` log form and the new table's test homes and JUnit output must pass this CI canary scan against the NEVER-log floor.
  > The CI `test` job does run obs-plan §9's artifact canary scan: `viola-harness secret-scan` over test-home diagnostics, the harness capture except `run --mutants`' `target/agent-run/chunk.diff`, and the nextest JUnit report, checked against this plan's NEVER-log floor.

### dependency-security#7.1 · listed (lost at classify) · .andromeda/security-plan-detail/dependency-security.md:48
- (.andromeda/security-plan-detail/dependency-security.md:48) The table's `internal-error` exit-1 case must be reached without a panic G2 would count, because a forced panic outside the `FAKE_AGENT_HOOK_PANIC` seam reads red and a second exemption needs its own Decisions Log entry.
  > CI's G2 zero-panics gate (`scripts/g2-zero-panics.sh`, in the `test` and `perf` jobs) has exactly one exemption. It does not count a role-file `panic` line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>` (the `FAKE_AGENT_HOOK_PANIC` seam's forced panic). The location is compared as a whole string, never as a prefix, suffix, regex or home path. The perf arm's own zero-panic check exempts nothing. G2's `--probe` runs before every G2 check. It proves that each of these reads red: another file, a look-alike path (`x/…/seam.rs`, `seam.rs.bak`, `seam.rsx`), the seam path without a line number, a missing location and an empty scope. Any further G2 exemption needs its own Decisions Log entry (§Security Anti-Patterns → Universal).

### secret-management#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:15
- (.andromeda/security-plan-detail/secret-management.md:15) `tests/cross_exit_causes.rs` and every case moved in the existing suites must run under a test-owned `--home`, never the operator's real home.
  > **Development:** Same as production. Tests use their own `--home` directory.

### security-anti-patterns#5.1 · listed (lost at strong) · .andromeda/security-plan-detail/security-anti-patterns.md:39
- (.andromeda/security-plan-detail/security-anti-patterns.md:39) The table's negative cases must reach each cause without a new test seam or environment read unless that seam has its own Decisions Log entry, and no colour fact may reach a code, hint, log line or refusal of `viola list`.
  > NEVER add another such seam or environment read, let these reach a release build, or let a colour fact change a control or reach a log line, error or refusal, without its own Decisions Log entry.

### security-anti-patterns#3.1 · listed (lost at classify) · .andromeda/security-plan-detail/security-anti-patterns.md:23
- (.andromeda/security-plan-detail/security-anti-patterns.md:23) The homes `tests/cross_exit_causes.rs` lays must come from the existing test-side creators, so that its removals take only a dir the test side itself created.
  > NEVER let a test-side removal under `target/e2e-home` take anything but a dir the test side itself created: the root fixture chain's owner sweep and a test home's drop (by `owner.json` record, by its own `TempDir`) and harness `cleanup` (by the `viola-session-` name).

## Held

- authentication-authorization#1.3
- authentication-authorization#1.4
- authentication-authorization#1.6
- authentication-authorization#1.8
- authentication-authorization#1.9
- authentication-authorization#3.2
- authentication-authorization#3.3
- authentication-authorization#3.4
- authentication-authorization#3.6
- authentication-authorization#3.7
- bootstrap-phases#1.1
- data-protection#1.1
- data-protection#4.1
- data-protection#7.1
- error-handling#1.1
- error-handling#1.11
- error-handling#1.12
- error-handling#1.14
- error-handling#1.15
- error-handling#1.17
- error-handling#1.19
- error-handling#1.21
- error-handling#1.22
- error-handling#1.23
- error-handling#1.5
- error-handling#1.6
- error-handling#1.7
- input-validation#1.13
- input-validation#1.15
- input-validation#1.17
- input-validation#1.18
- input-validation#1.3
- input-validation#1.5
- input-validation#1.6
- input-validation#1.7
- security-anti-patterns#1.1
- security-anti-patterns#4.2
- threat-model-summary#1.5
- threat-model-summary#11.1
- threat-model-summary#2.1
- threat-model-summary#5.1
- threat-model-summary#6.1
- threat-model-summary#8.1
- threat-model-summary#9.1

## Cascade only

- authentication-authorization#1.2
- authentication-authorization#1.7
- authentication-authorization#3.1
- error-handling#1.10
- error-handling#1.16
- error-handling#1.18
- error-handling#1.2
- error-handling#1.24
- error-handling#1.3
- error-handling#1.4
- error-handling#1.8
- error-handling#1.9
- input-validation#1.1
- input-validation#1.2
- input-validation#1.4
- input-validation#1.8
