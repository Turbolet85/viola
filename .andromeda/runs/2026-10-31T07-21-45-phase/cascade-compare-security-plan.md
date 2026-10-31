# Cascade compare — security-plan · phase · 2026-10-31-sanitised-error-surfaces
E items 179 · held 43 · lost at classify 6 · lost at strong 1 · beside a placed fact - · cascade only 9 · listed 3 · body 95 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### authentication-authorization#1.2 · lost at classify · .andromeda/security-plan-detail/authentication-authorization.md:12
- (.andromeda/security-plan-detail/authentication-authorization.md:12) The channel half of the error-body enumeration must cover this whole served surface, every method and the notification, not a sample of it.
  > the nine served methods (`send`, `wait`, `last`, `hook.dialog`, `answer`, `pause`, `release`, `link`, `unlink`) and the `hook.event` notification, each behind the IPC rows below and its own params check (§ Input Validation, Channel frames)

### data-protection#9.1 · lost at classify · .andromeda/security-plan-detail/data-protection.md:70
- (.andromeda/security-plan-detail/data-protection.md:70) Wherever this refusal can reach a surface in scope it must keep the closed code set and hold no content; its `<file>` slot is a ratified named form, so the enumeration must name it, and folding it to a code alone is a change to the founder's ratified form, not the chunk's to make silently.
  > The refusal names the fixture file and a closed check code, never the content: `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen, with codes `home-path` · `absolute-path` · `username` · `email`. The founder ratified this named form live at this chunk's wrap (2026-10-05, through the overseer's AskUserQuestion, the closed code set and no-content rule shown).

### dependency-security#3.1 · lost at classify · .andromeda/security-plan-detail/dependency-security.md:20
- (.andromeda/security-plan-detail/dependency-security.md:20) Changes to `viola-channel` (the `ProtocolError` causes, the client's `fault_of`) and to the sync crates' error types (the `Refusal` / `AgentError` question) must keep the tokio bans green: `viola-channel`'s normal graph prints no `tokio v` line without its feature, and the probe verdict is re-counted if the chunk moves a row.
  > The arch bans stay as they are: C-building crates in `deny.toml`, and `tokio` in the sync crates' graph for three target triples in `deny-sync.toml`, run per sync crate as the sole root over the four crates of `scripts/sync-crates.txt` (arch §Build system). `viola-channel`, whose Tokio client (tokio `=1.53.3`, `io-util`) sits behind its `tokio` feature, left that list at chunk 2026-10-17-mcp-server-for-drivers and is covered instead by the `cargo tree` form `scripts/channel-tokio-free.sh` (its normal graph without the feature must print no `tokio v` line, its `--features tokio` control one; CI supply-chain step `Tokio ban per sync crate (sole root)`). Every ban, the tree form included (probe `channel-tokio-edge` and its channel control; `14/14 banned, control clean`, counted as the `probes=(` rows of `scripts/deny-probes.sh` plus the planted channel probe, as measured at viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md), is proven live by `scripts/deny-probes.sh`.

### dependency-security#2.1 · lost at classify · .andromeda/security-plan-detail/dependency-security.md:14
- (.andromeda/security-plan-detail/dependency-security.md:14) The serde error mapping and the folded cap-driving cases on the `config.json` readers must keep every external parse at serde_json's default recursion limit; the ban, its probe row and its test stay in force.
  > `deny.toml`'s `[[bans.features]]` also bans serde_json's `unbounded_depth` feature, its reason that every parse of external bytes keeps serde_json's default recursion limit; the `scripts/deny-probes.sh` row `serde_json-unbounded-depth` proves the ban fires, and the test `serde_json_unbounded_depth_is_banned_in_deny_toml` reads it in `deny.toml`.

### security-anti-patterns#4.1 · lost at classify · .andromeda/security-plan-detail/security-anti-patterns.md:32
- (.andromeda/security-plan-detail/security-anti-patterns.md:32) The enumeration of CLI error bodies must keep the code and exit of this refusal unchanged, and the hook's side of it must stay silent.
  > the hook prints nothing and `answer` exits 12 `unverified-cli`.

### security-anti-patterns#4.2 · lost at classify · .andromeda/security-plan-detail/security-anti-patterns.md:33
- (.andromeda/security-plan-detail/security-anti-patterns.md:33) A fixed message given to this refusal must not change what it does: it still reads and writes nothing.
  > a refusal (`StampsError::StrictModes`) reads and writes nothing.

### threat-model-summary#13.1 · lost at strong · .andromeda/security-plan-detail/threat-model-summary.md:178
- (.andromeda/security-plan-detail/threat-model-summary.md:178) The witnesses and owed cases that run in CI must drive the fake agent, not the real CLI.
  > The real `claude` CLI runs only locally; `viola verify` runs in CI only against the fake agent, and its real-CLI probe and `--record` only locally.

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### dependency-security#7.1 · listed (lost at classify) · .andromeda/security-plan-detail/dependency-security.md:48
- (.andromeda/security-plan-detail/dependency-security.md:48) A witness that drives a panicked handler to its `"internal error"` / `data: null` body must not read G2 red, and it must gain no new G2 exemption without its own Decisions Log entry: the one exempt location stays the seam's, compared as a whole string, and the perf arm exempts nothing.
  > CI's G2 zero-panics gate (`scripts/g2-zero-panics.sh`, in the `test` and `perf` jobs) has exactly one exemption. It does not count a role-file `panic` line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>` (the `FAKE_AGENT_HOOK_PANIC` seam's forced panic). The location is compared as a whole string, never as a prefix, suffix, regex or home path. The perf arm's own zero-panic check exempts nothing. G2's `--probe` runs before every G2 check. It proves that each of these reads red: another file, a look-alike path (`x/…/seam.rs`, `seam.rs.bak`, `seam.rsx`), the seam path without a line number, a missing location and an empty scope. Any further G2 exemption needs its own Decisions Log entry (§Security Anti-Patterns → Universal).

### secret-management#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:15
- (.andromeda/security-plan-detail/secret-management.md:15) Each surface's witness must run against its own `--home`, never a host home.
  > Tests use their own `--home` directory.

### secret-management#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:16
- (.andromeda/security-plan-detail/secret-management.md:16) The planted value must be set by the witness itself on the child it drives, because a credential on the host never crosses the launcher and so proves nothing under the local gate.
  > launches every child natively as `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>` from the repository root, so no host environment value — the inherited `CLAUDE*` credentials included — crosses into a child.

### secret-management#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:16
- (.andromeda/security-plan-detail/secret-management.md:16) The planted value must be a test constant, since carrying a real host value into a gate child is a boundary widening that needs its own Decisions Log entry and the founder's live word.
  > Any assignment whose value comes from the host is a boundary widening and needs its own Decisions Log entry;

### security-anti-patterns#3.1 · listed (lost at classify) · .andromeda/security-plan-detail/security-anti-patterns.md:23
- (.andromeda/security-plan-detail/security-anti-patterns.md:23) Any new witness or cap-driving case that lays a test home must take its removal through the test side's own creations, never by a wider sweep.
  > NEVER let a test-side removal under `target/e2e-home` take anything but a dir the test side itself created: the root fixture chain's owner sweep and a test home's drop (by `owner.json` record, by its own `TempDir`) and harness `cleanup` (by the `viola-session-` name).

## Held

- authentication-authorization#1.6
- authentication-authorization#1.7
- authentication-authorization#1.8
- authentication-authorization#1.9
- authentication-authorization#3.4
- authentication-authorization#3.7
- bootstrap-phases#1.1
- bootstrap-phases#2.1
- data-protection#1.1
- data-protection#7.1
- data-protection#8.1
- error-handling#1.1
- error-handling#1.10
- error-handling#1.11
- error-handling#1.5
- error-handling#1.6
- error-handling#1.7
- error-handling#1.8
- error-handling#1.9
- input-validation#1.1
- input-validation#1.12
- input-validation#1.15
- input-validation#1.17
- input-validation#1.18
- input-validation#1.2
- input-validation#1.20
- input-validation#1.3
- input-validation#1.5
- input-validation#1.6
- input-validation#1.7
- input-validation#1.8
- input-validation#2.0
- secret-management#1.1
- security-anti-patterns#5.1
- threat-model-summary#1.2
- threat-model-summary#1.4
- threat-model-summary#1.5
- threat-model-summary#11.1
- threat-model-summary#2.1
- threat-model-summary#5.1
- threat-model-summary#6.1
- threat-model-summary#7.1
- threat-model-summary#8.1

## Cascade only

- error-handling#1.4
- input-validation#1.13
- input-validation#1.19
- input-validation#1.4
- input-validation#1.9
- threat-model-summary#1.1
- threat-model-summary#1.3
- threat-model-summary#3.1
- threat-model-summary#9.1
