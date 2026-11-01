# Cascade compare — security-plan · phase · 2026-11-01-unix-endpoint-and-home-hardening
E items 185 · held 41 · lost at classify 1 · lost at strong 3 · beside a placed fact - · cascade only 10 · listed 3 · body 88 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### data-protection#8.1 · lost at strong · .andromeda/security-plan-detail/data-protection.md:64
- (.andromeda/security-plan-detail/data-protection.md:64) Strict modes on the diagnostics path must keep this per-path rule and its failed-open end, checking before anything narrows, and the chunk's check-then-refuse directory rule must not reshape it.
  > **Logs (the diagnostics opens):** `open_role_file` and `write_detail` run `viola_state::strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before `ensure_private_dir` narrows anything: a symlink is refused; a folder takes the read rule; a file takes the read rule on Unix and the write rule on Windows; an absent path passes. A refusal is a failed open: the writer ends at `std::io::sink` and the detail line is dropped.

### error-handling#1.18 · lost at classify · .andromeda/security-plan-detail/error-handling.md:25
- (.andromeda/security-plan-detail/error-handling.md:25) A hint this chunk adds to the exit-21 surface must follow the same form: last on stderr, plain ASCII, any `<name>` the parsed `ViolaName`.
  > The `unwrapped-name` and `endpoint-lost` hints come last on stderr in plain ASCII, their `<name>` the parsed `ViolaName`.

### input-validation#1.8 · lost at strong · .andromeda/security-plan-detail/input-validation.md:45
- (.andromeda/security-plan-detail/input-validation.md:45) The board's one rule must keep this consequence: a `list` the rule refuses reads no `config.json`, a loosened home included.
  > each rejection is logged through `obs::log_config_rejection`, under a `VIOLA_NAME` only, and a refused `list` reads no `config.json`

### security-anti-patterns#4.2 · lost at strong · .andromeda/security-plan-detail/security-anti-patterns.md:33
- (.andromeda/security-plan-detail/security-anti-patterns.md:33) The strict-modes order must survive this chunk's rule changes: a check runs before anything narrows the path it checks, as the scope's never-narrowed-in-place directory also requires.
  > NEVER run that check after a create, a mode set or a log open that narrows the path it checks (a widened `ledger/` narrowed to 0700 first would pass)

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### dependency-security#3.1 · listed (lost at classify) · .andromeda/security-plan-detail/dependency-security.md:20
- (.andromeda/security-plan-detail/dependency-security.md:20) The directory verification and the peer euid check on both sides, the Tokio client's included, must keep the sync crates' graph tokio-free and `viola-channel`'s normal graph free of any `tokio v` line, and `scripts/channel-tokio-free.sh` and `scripts/deny-probes.sh` must still read clean; a probe the chunk adds moves the `14/14` count, which must then be restated.
  > The arch bans stay as they are: C-building crates in `deny.toml`, and `tokio` in the sync crates' graph for three target triples in `deny-sync.toml`, run per sync crate as the sole root over the four crates of `scripts/sync-crates.txt` (arch §Build system). `viola-channel`, whose Tokio client (tokio `=1.53.3`, `io-util`) sits behind its `tokio` feature, left that list at chunk 2026-10-17-mcp-server-for-drivers and is covered instead by the `cargo tree` form `scripts/channel-tokio-free.sh` (its normal graph without the feature must print no `tokio v` line, its `--features tokio` control one; CI supply-chain step `Tokio ban per sync crate (sole root)`). Every ban, the tree form included (probe `channel-tokio-edge` and its channel control; `14/14 banned, control clean`, counted as the `probes=(` rows of `scripts/deny-probes.sh` plus the planted channel probe, as measured at viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md), is proven live by `scripts/deny-probes.sh`.

### dependency-security#8.1 · listed (lost at classify) · .andromeda/security-plan-detail/dependency-security.md:54
- (.andromeda/security-plan-detail/dependency-security.md:54) If the Linux and macOS parity work (`v1-53`) edits a workflow, every action it uses must match this SHA-pinned set, `actions/checkout` keeping `persist-credentials: false`.
  > Actions are pinned by full commit SHA (the set `ci.yml` uses; SHAs resolved 2026-09-24 via `gh api repos/{repo}/commits/{tag}`):
  >   - `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1` (with `persist-credentials: false`)
  >   - `Swatinem/rust-cache@6323deb102c322ba6fcbdcafc7e3dddab59af2b6 # v2.9.2`
  >   - `taiki-e/install-action@7623a79cdfecb99d681017af368ca353d9f49bb5 # v2.87.19`
  >   - `actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1`

### security-anti-patterns#3.1 · listed (lost at classify) · .andromeda/security-plan-detail/security-anti-patterns.md:23
- (.andromeda/security-plan-detail/security-anti-patterns.md:23) Any test or harness `cleanup` change from the leftover-socket measurement, and any test laying out a loosened socket directory, must remove only what the test side itself created.
  > NEVER let a test-side removal under `target/e2e-home` take anything but a dir the test side itself created: the root fixture chain's owner sweep and a test home's drop (by `owner.json` record, by its own `TempDir`) and harness `cleanup` (by the `viola-session-` name).

## Held

- authentication-authorization#1.4
- authentication-authorization#1.5
- authentication-authorization#1.6
- authentication-authorization#1.8
- authentication-authorization#1.9
- authentication-authorization#3.1
- authentication-authorization#3.2
- authentication-authorization#3.4
- authentication-authorization#3.5
- authentication-authorization#3.6
- authentication-authorization#3.8
- bootstrap-phases#1.1
- data-protection#1.1
- error-handling#1.1
- error-handling#1.15
- error-handling#1.16
- error-handling#1.17
- error-handling#1.2
- error-handling#1.20
- error-handling#1.21
- error-handling#1.24
- error-handling#1.27
- error-handling#1.28
- error-handling#1.29
- error-handling#1.3
- error-handling#1.5
- error-handling#1.6
- input-validation#1.13
- input-validation#1.4
- input-validation#1.7
- secret-management#1.1
- secret-management#2.1
- security-anti-patterns#1.1
- security-anti-patterns#5.1
- threat-model-summary#1.3
- threat-model-summary#13.1
- threat-model-summary#2.1
- threat-model-summary#5.1
- threat-model-summary#6.1
- threat-model-summary#8.1
- threat-model-summary#9.1

## Cascade only

- authentication-authorization#1.2
- authentication-authorization#1.3
- authentication-authorization#2.2
- authentication-authorization#3.9
- error-handling#1.11
- error-handling#1.12
- error-handling#1.22
- error-handling#1.30
- input-validation#1.3
- threat-model-summary#1.5
