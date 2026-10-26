# Cascade compare — security-plan · phase · 2026-10-26-explicit-session-links
E items 117 · held 17 · lost at classify 3 · lost at strong 2 · beside a placed fact - · cascade only 2 · listed 5 · body 60 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### authentication-authorization#1.2 · lost at strong · .andromeda/security-plan-detail/authentication-authorization.md:12
- (.andromeda/security-plan-detail/authentication-authorization.md:12) The served `link` and `unlink` methods must join the one all-or-nothing surface: every caller that passes the OS check reaches them, and no per-method role or check is added for them.
  > Capability-based, all or nothing. Any caller that passes the OS check (IPC) gets the full surface. No roles, because the only principal is the local OS user (Threat Model: Auth model).

### authentication-authorization#1.4 · lost at strong · .andromeda/security-plan-detail/authentication-authorization.md:18
- (.andromeda/security-plan-detail/authentication-authorization.md:18) The `link` / `unlink` verbs must go through the Unix socket-directory check before they connect, and report `instance-unreachable` (exit 21) when it fails.
  > **Directory check:** before bind, and again before any client connect, `lstat` the directory and require owner == `geteuid()`, `mode & 0o077 == 0` and not a symlink. On failure `run` refuses to start (exit 1), CLI/MCP clients report `instance-unreachable` (exit 21), and `hook` fails open (exit 0, no body, a note in `diagnostics/`) without writing any frame.

### authentication-authorization#1.5 · lost at classify · .andromeda/security-plan-detail/authentication-authorization.md:21
- (.andromeda/security-plan-detail/authentication-authorization.md:21) The wrapper must dispatch the new `link` / `unlink` methods only on a connection that passed the server-side euid peer check.
  > On every accept, `stream.peer_creds()`; if `euid() != geteuid()`, drop the connection before reading a frame.

### dependency-security#2.1 · lost at classify · .andromeda/security-plan-detail/dependency-security.md:14
- (.andromeda/security-plan-detail/dependency-security.md:14) The verbs write their frames through `viola-channel`'s sync client and the methods are served on the sync wrapper path, so the sync crates' graph stays Tokio-free and `viola-channel`'s normal graph (without its `tokio` feature) still prints no `tokio v` line.
  > The arch bans stay as they are: C-building crates in `deny.toml`, and `tokio` in the sync crates' graph for three target triples in `deny-sync.toml`, run per sync crate as the sole root over the four crates of `scripts/sync-crates.txt` (arch §Build system). `viola-channel`, whose Tokio client (tokio `=1.53.3`, `io-util`) sits behind its `tokio` feature, left that list at chunk 2026-10-17-mcp-server-for-drivers and is covered instead by the `cargo tree` form `scripts/channel-tokio-free.sh` (its normal graph without the feature must print no `tokio v` line, its `--features tokio` control one; CI supply-chain step `Tokio ban per sync crate (sole root)`).

### dependency-security#6.1 · lost at classify · .andromeda/security-plan-detail/dependency-security.md:42
- (.andromeda/security-plan-detail/dependency-security.md:42) The verbs' and methods' test runs pass CI's G2 zero-panics gate with its one seam exemption only; the chunk adds no G2 exemption.
  > CI's G2 zero-panics gate (`scripts/g2-zero-panics.sh`, in the `test` and `perf` jobs) has exactly one exemption. It does not count a role-file `panic` line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>` (the `FAKE_AGENT_HOOK_PANIC` seam's forced panic). The location is compared as a whole string, never as a prefix, suffix, regex or home path. The perf arm's own zero-panic check exempts nothing. G2's `--probe` runs before every G2 check. It proves that each of these reads red: another file, a look-alike path (`x/…/seam.rs`, `seam.rs.bak`, `seam.rsx`), the seam path without a line number, a missing location and an empty scope. Any further G2 exemption needs its own Decisions Log entry (§Security Anti-Patterns → Universal).

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### bootstrap-phases#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/bootstrap-phases.md:24
- (.andromeda/security-plan-detail/bootstrap-phases.md:24) The chunk's new test homes and harness capture must pass CI's artifact canary scan before any scan-gated upload.
  > The CI `test` job does run obs-plan §9's artifact canary scan: `viola-harness secret-scan` over test-home diagnostics, the harness capture except `run --mutants`' `target/agent-run/chunk.diff`, and the nextest JUnit report, checked against this plan's NEVER-log floor.

### data-protection#1.1 · listed (lost at strong) · .andromeda/security-plan-detail/data-protection.md:8
- (.andromeda/security-plan-detail/data-protection.md:8) The `link` / `unlink` lines the wrapper appends and the snapshot `links` it rewrites stay plaintext state under the home, protected by OS permissions only, and the verbs' read of `snapshot.json` (crossing (c)) answers to the strict-modes check this rule names, whichever of the founder's forms is chosen at P4.
  > **Files (`~/.viola/`):** Plaintext by design (Established Decisions: Database / State Store). Protected by OS permissions only:
  >   - Unix: 0700 directories and 0600 files, except the pinned `bin/<version>-<hash>/viola`, which is 0700.
  >   - Windows: the inherited `%USERPROFILE%` DACL (user + SYSTEM + Administrators), or an explicit protected user + SYSTEM DACL for a `--home` outside `%USERPROFILE%`.
  >   - The strict-modes check runs at startup (Authentication & Authorization, `~/.viola/` access control).

### secret-management#1.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:8
- (.andromeda/security-plan-detail/secret-management.md:8) The `link` / `unlink` verbs, their `--json` arm and the wrapper's two served methods must read no environment variable outside `VIOLA_*` plumbing as configuration: no new env read, seam or credential source enters with them.
  > environment variables are not a configuration channel — the variables outside `VIOLA_*` plumbing are the test-only seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, compiled out of release builds, and the terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM`, read in every build by `viola list`'s human mode only to decide whether and how deep SGR is written (they configure nothing, disable no control and widen no redaction;

### secret-management#1.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:9
- (.andromeda/security-plan-detail/secret-management.md:9) The `link` and `unlink` lines appended to `events.ndjson`, the snapshot's `links` items and any diagnostic the served methods write must carry no inherited `CLAUDE*` value; a link record is built from viola's own parameters only.
  > Inherited credentials (`CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and the other `CLAUDE*` identity variables) stay only in the `viola run` wrapper's environment. They are removed from the child by the R8 rule and are never serialised into events, snapshots, diagnostics or the ledger.

### secret-management#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:15
- (.andromeda/security-plan-detail/secret-management.md:15) The chunk's root cases (`tests/channel_links.rs`, Scenario E3's steps, the raw-channel name negative) must run against their own `--home`, never the user's home.
  > Same as production. Tests use their own `--home` directory.

### secret-management#2.1 · listed (lost at classify) · .andromeda/security-plan-detail/secret-management.md:16
- (.andromeda/security-plan-detail/secret-management.md:16) The chunk's tests and gates run inside the local pre-push gate, so they must pass with every child launched under `env -i` with exactly `HOME` and `PATH`; no test or harness change may add a host-valued assignment to the launcher without its own Decisions Log entry.
  > The local pre-push gate (`viola-harness pre-push`) runs only on a Linux host (`pre-push-linux-only`, exit 2, elsewhere) and launches every child natively as `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>` from the repository root, so no host environment value — the inherited `CLAUDE*` credentials included — crosses into a child. No other assignment is made: every launcher call carries exactly `HOME` and `PATH`, every PATH component a constant under the home or a system dir. `<home>` is the passwd entry's field 6, read by two probes that carry only `PATH=/usr/bin:/bin` (`id -u`, then `getent passwd <uid>`); the harness's own `$HOME` is never read for it (that would carry a host value across `env -i`), and an unreadable entry stops at `tools` with `tool-missing` `passwd-home`. Any assignment whose value comes from the host is a boundary widening and needs its own Decisions Log entry;

### security-anti-patterns#5.1 · listed (lost at strong) · .andromeda/security-plan-detail/security-anti-patterns.md:37
- (.andromeda/security-plan-detail/security-anti-patterns.md:37) No flag of the new verbs (`--json` included), `VIOLA_*` variable or `config.json` key may switch off the server verification, the strict-modes check or any other control on the link path.
  > - NEVER let `config.json`, a `VIOLA_*` environment variable or a CLI flag switch off any control in this plan: the IPC DACL or socket-directory check, the peer or server verification, the home strict-modes check or `validate_paste_text`.

## Held

- authentication-authorization#1.6
- authentication-authorization#1.7
- authentication-authorization#1.8
- authentication-authorization#3.2
- authentication-authorization#3.3
- authentication-authorization#3.5
- bootstrap-phases#1.1
- input-validation#1.12
- input-validation#1.3
- input-validation#1.6
- security-anti-patterns#1.1
- threat-model-summary#1.3
- threat-model-summary#1.5
- threat-model-summary#2.1
- threat-model-summary#6.1
- threat-model-summary#7.1
- threat-model-summary#8.1

## Cascade only

- authentication-authorization#3.1
- threat-model-summary#1.1
