# Codebase Research — 2026-09-25-pty-wrapper-on-windows

## Scope
- **Depth:** deep · **Reads:** 19 (files/sections) · **Globs/Greps:** 24 · **Spikes:** 4 scratch measurements (portable-pty/ConPTY, cargo-modules orphans, curl exit-35 retry, cargo-deny on the portable-pty graph), all in the session scratchpad and none in the tree
- **Harness rules consulted:** `.claude/rules/verification-harness.md`, read in full (1 Session Addition: `--in-diff` never regenerates older mutants). `.claude/rules/testing.md`, read in full (11 Session Additions applied, including remove-the-guard runs with the rustfmt re-Read, inline `#[cfg(test)]`, the `cfg(unix)`/`cfg(windows)` mutant union, `try_wait` bounded below 20 s, and "no unobservable body").
- **Platform issues consulted:**
  - `wezterm/wezterm#6783` (fetched): "portable-pty 0.9.0 doesn't work on windows — pty.read is returning garbage … starting with version 0.9.0"; 0.8.1 works. It is **open**, with no fix or release named.
  - crates.io API `/api/v1/crates/portable-pty` (fetched): `max_version` 0.9.0 (2025-02-11), so no newer line exists. The 0.8.1 pin stands on current evidence.
  - `hendrikmuhs/ccache-action#287` (fetched, opened 2025-02-08, still open): the same `CRYPT_E_REVOCATION_OFFLINE` from Git-for-Windows' Schannel curl on GitHub Windows runners, described as "a high rate of failures … transient Windows infra errors". The options it proposes are `--ssl-revoke-best-effort` / `--ssl-no-revoke`, and it chose neither.
  - A web search (runner-images + the error text) found no runner-images tracker entry naming a fix; the other hits (mamba#3346, AppVeyor, MS Q&A) describe the same Schannel behaviour.

## Files inspected
- `src/run/mod.rs` (full, 66 lines). `spawn_child` (l.64-66) is `Command::new(program).args(args).spawn()`: no PTY, full inherited env, no cwd set. The `log_child_start` / `log_child_exit(exit_source="handle-wait")` / `log_self_exit` helpers already exist (l.22-62).
- `src/cmd/run.rs` (full). `run()` (l.24-49) runs obs init, `log_self_start`, `spawn_child`, then `child.wait()` / `log_child_exit` / `log_self_exit(0)`. Any spawn `Err` becomes `log_self_exit(1, "internal-error")` with **no stderr line**. There is no stdin/stdout pump: the child inherits viola's stdio.
- `src/main.rs` (l.1-80). The panic hook is the first statement. `cmd::dispatch` errors go to `obs::report_internal_error`. **No human output site exists in the root bin** (grep `print|stderr()|write_all` over `src/` minus `src/bin/`: 3 hits, all role/detail-file writes).
- `crates/viola-e2e/src/harness/supervise.rs` (full, 187 lines). `Wrapper{child, stdin: Option<ChildStdin>}`. `spawn_wrapper` (l.24-47) has stdin piped and stdout/stderr null. `stop` (l.85-98) writes `\x03` to the stdin pipe, then `wait_or_kill` (10 s, `try_wait` + `kill`). 4 inline tests.
- `tests/support/home.rs` (full, 220 lines). `Wrapper::boot` (l.119-149) pipes stdin, `send` writes to the pipe, `stop` writes `\x03` and waits, and `Drop` kills. `wait_ready` is bounded at 10 s and exit-aware.
- `tests/run_cli.rs` (l.1-150). `run_viola` pipes stdin, sets `CLAUDE_CODE_MESSAGING_TOKEN=canary-token-value-7f3a`, writes `\x03` and waits. The role-file shape assertions are `[start self, start claude-child, exit claude-child, exit self]` and `exit_source == "handle-wait"`.
- `src/bin/viola-fake-agent.rs` (l.1-100, 310-500 of 650).
  - `start_receipts` writes an `env {names}` receipt (sorted, names only) and, on Unix, `fds`.
  - `read_stdin` reads std stdin byte by byte, and `Input::plain` maps `0x03` to Exit.
  - `--exit-no-eof` re-execs a grandchild that holds stdout.
  - The fake agent does **not** touch the console/tty mode.
- `crates/viola-e2e/src/harness/boot.rs` (grep). The fake agent is copied to `<session>/bin/claude[.exe]`, and `supervise` passes that **absolute path** after `--` (`supervise.rs:33`), so shim resolution never sees it.
- `Cargo.toml` (root, full).
  - Members are `crates/*`.
  - `[workspace.dependencies]` has one pin per crate with a rationale comment on the non-obvious ones.
  - windows-sys `=0.61.2` has features `Win32_Foundation, Win32_Security, Win32_Storage_FileSystem, Win32_System_Pipes, Win32_System_Threading` and no `Win32_System_Console`.
  - The root has windows-sys only as a `cfg(windows)` **dev**-dependency.
  - The fake agent is a `[[bin]]` of the root package behind `fake-agent`.
- `crates/viola-e2e/Cargo.toml` (full). Deps are viola-core, clap, jsonschema, serde, serde_json, sysinfo, tempfile, thiserror. It has its own `[lints]`.
- `tests/contract_lints.rs` (grep). Every member except `viola-e2e` must carry exactly `[lints] workspace = true`, and the test iterates the members it finds, so a new crate is checked automatically.
- `schemas/diag-line.v1.json` (python walk).
  - `process-start` already admits `pty_backend` (string), `cli_version`, `cli_verified`, `env_stripped_count` (count) and `env_stripped_known` (**string**).
  - `process-exit` admits `exit_source ∈ {handle-wait, kill-fallback}` and a `detail` enum that includes `batch-script-child` and `internal-error`.
  - No schema change is needed.
- `deny.toml` (grep). The RUSTSEC-2017-0008 ignore (`serial` via portable-pty `=0.8.1`) is **already present** (l.11-12).
- `scripts/sync-crates.txt`. It lists `viola-core` only. CI `lint` runs `cargo check -p <each>`, and `supply-chain` runs the sole-root `deny-sync.toml` per line (`ci.yml:283-286, 383-386`).
- `scripts/orphans-check.sh` (l.1-80). It runs per lib/bin target from `cargo metadata`, uses `cargo modules orphans -p … --deny` with the host's cfg only, and has a `--probe` (planted stray + control).
- `scripts/install-ripgrep.sh` (full, 73 lines). It downloads with `curl -fsSL --retry 3 -o …` (l.55-56), checks sha256 (l.57-61) and fails closed on `tool-missing: <tool>`.
- `.github/workflows/ci.yml` (l.1-345 outline, l.266-320 full). `lint` (3 OSes):
  - sync check → fmt → clippy → `Install ripgrep` (l.293-297) → `rg --pcre2-version` → G1/G3 → lint probes (Linux) → cargo-modules → `orphans-check.sh --probe && orphans-check.sh`.
  - `test` (3 OSes) runs `run --coverage` plus the harness lifecycle (`boot`/`cleanup`) through both shims.
  - `mutants` runs on ubuntu + windows.
- The portable-pty 0.8.1 source (cargo registry):
  - `src/win/psuedocon.rs:110-160` (`spawn_command`)
  - `src/cmdbuilder.rs:209-340, 395-430, 528-600`
  - `src/win/mod.rs:40-100`
- `.andromeda/input.md` §4.1 S6, `refs/viola-brief.md` §4.1 (l.160-222), `.andromeda/architecture.md` §Occupied Resources (env vars, l.355-366), `viola-0.1.0/intent.md` F-25/F-27, `requirements.md` v1-25, `design-system.md:772, 809`, `obs-plan.md:857-868, 1057, 1176, 1556-1557`.

## Graph impact
- **`spawn_child`**: 1 caller, `cmd/run/run()` at `src/cmd/run.rs:36` (trace `tree-query-2026-09-25-pty-wrapper-on-windows.json`, rust plane). Its signature changes, and `run()` is the only threading site.
- **`supervise` / `Wrapper` (harness)**: `supervise()` is defined at `crates/viola-e2e/src/harness/supervise.rs:49` and `Wrapper` at `:18`. They are called from the harness dispatch (`viola-harness supervise`), which is a runtime subcommand, so graph callers are not dead-code evidence.
- **`Wrapper` (root tests)**: `tests/support/home.rs:110` and `booted_wrapper` at `:218`. They are consumed by root integration tests (tests are not all indexed, so the companion sweep below is by name).
- **Crate edges** (`SELECT from_crate,to_crate FROM crate_edges`): `viola → viola-core`, `viola-e2e → viola-core`. Both new crates are additive: `viola-pty` has zero viola deps, `viola-agent-claude` has `→ viola-core`, and the `viola`/`viola-e2e` → `viola-pty` edges are new.
- Companion sweep for `Wrapper|booted_wrapper|stdin.*\x03|Stdio::piped` over `src/ crates/ tests/` (by name): the hits are `supervise.rs` (Wrapper/spawn_wrapper/stop + 4 tests), `tests/support/home.rs`, `tests/run_cli.rs::run_viola`, and the fake agent's own `Stdio::piped` for hook commands (no change). Each hit is on the modify list or carries `no change`.

## Measured facts (spikes; the load-bearing equalities)
1. **ConPTY + the unmodified fake agent.** The fake agent was spawned through portable-pty 0.8.1 `openpty` + `spawn_command`, then sent `ab\r` and `\x03`.
   - It received `61 62 0d` followed by `0a`. ConPTY's cooked console turned CR into CR LF, delivered it only after Enter and echoed `ab\r\n` onto the master.
   - `\x03` never arrived and the child did **not** exit within 5 s (killed).
   - **So the fake agent as it stands cannot be driven through a PTY on Windows.**
2. **ConPTY + a child that sets raw VT input.** The child called `SetConsoleMode(stdin, mode & ~(ECHO|LINE|PROCESSED) | ENABLE_VIRTUAL_TERMINAL_INPUT)`, going from `0x1f7` to `0x3f0`.
   - It received exactly `61 62 0d 03`, with no echo, and exited 0 on `0x03` (`EXIT success=true code=0`).
   - The cooked control repeated fact 1.
   - This is what a TUI like `claude` does itself (brief §4.1 S1: it switches bracketed paste on by itself).
3. **Nested: outer ConPTY → relay → inner ConPTY → raw child.** The relay is shaped like viola: it puts its own console into raw VT input plus VT output processing, pumps both ways, and exits on the child's `wait()`.
   - With the relay's raw mode on, the child got `61 62 0d 03` and the relay exited 0 via the handle.
   - With the relay's raw mode **off** (control), the relay's console line-buffered (`61 62 0d 0a`), `\x03` never reached the child, and nothing exited in 5 s.
   - **So on Windows the wrapper must put its own console stdin into raw VT input (and stdout into VT processing) when stdin/stdout are consoles, and restore the original modes on exit.** Otherwise a human keystroke is delayed until Enter and Ctrl-C is swallowed, a Critical Warning ("never … delay a human keystroke").
4. **ConPTY writes bytes the child never wrote.** On every spawn the master stream began `ESC[?9001h ESC[?1004h ESC[?25l ESC[2J ESC[m ESC[H` and carried `ESC]0;<child command line>BEL` and `ESC[?25h`.
   - The OSC 0 title holds the spawned exe path, and through two layers both titles appear.
   - Nested, the outer stream is the outer ConPTY's **re-render** of the inner output (`ESC[K\r\n` rows, cursor moves).
   - **So on Windows neither byte identity nor "no cursor control the child did not emit" is achievable.** The only sound zero-own-bytes oracle there is the absence of viola's own literals, as a11y-plan §3 already states for ConPTY. On Unix openpty does not re-render, so a byte comparison is possible there (not measurable on this host).
5. **portable-pty 0.8.1 spawn facts (source):**
   - `CreateProcessW(..., bInheritHandles = 0, EXTENDED_STARTUPINFO_PRESENT | CREATE_UNICODE_ENVIRONMENT, ...)`, with stdio set to `INVALID_HANDLE_VALUE` under `STARTF_USESTDHANDLES` (`psuedocon.rs:119-142`). Non-inheritable is satisfied.
   - With no `cwd` set, the child's cwd defaults to `%USERPROFILE%` (`cmdbuilder.rs:560-567`). **viola must pass `cwd` = its own current dir** (a bare `claude` without the wrapper starts in the caller's dir).
   - The Windows PATH search tries the **exact name first**, then PATHEXT (`cmdbuilder.rs:532-556`). On this host `<npm prefix>/claude` (an extensionless sh shim) exists beside `claude.cmd`, so `CommandBuilder::new("claude")` resolves to the sh script, which `CreateProcessW` cannot run. **viola resolves the program itself and hands portable-pty an absolute path.**
   - A spawn failure is an `anyhow` error whose message embeds the command line and cwd (`psuedocon.rs:152-157`), which is content. It also goes to `log::error!`, which reaches no sink because viola installs no `log` bridge.
   - `kill()` is `TerminateProcess(handle, 1)` (`win/mod.rs:72`), `wait()` is `WaitForSingleObject(INFINITE)` and `try_wait()` exists.
   - `CommandBuilder` starts from the parent's full env, and `env_remove` removes names (measured: `CLAUDECODE` and `CLAUDE_CODE_MESSAGING_TOKEN` set then removed were absent from the child's `env` receipt).
6. **npm shim layout (this host, CLI 2.x).** `<npm prefix>/` holds `claude` (sh: `exec "$basedir/node_modules/@anthropic-ai/claude-code/bin/claude.exe"`), `claude.cmd` (`"%dp0%\node_modules\@anthropic-ai\claude-code\bin\claude.exe" %*`) and `claude.ps1`. The real binary is `<npm prefix>/node_modules/@anthropic-ai/claude-code/bin/claude.exe`, which exists. This matches brief §4.1 "Windows details". A resolution that **never parses the shim** (upstream text is content): the resolved `claude.cmd`'s sibling `node_modules/@anthropic-ai/claude-code/bin/claude.exe`, when it is a file.
7. **Inherited `CLAUDE*` names on this host.** The shell this session runs in holds 11 names (read as names only, values never read): `CLAUDECODE, CLAUDE_CODE_BRIDGE_SESSION_ID, CLAUDE_CODE_CHILD_SESSION, CLAUDE_CODE_ENTRYPOINT, CLAUDE_CODE_EXECPATH, CLAUDE_CODE_MESSAGING_SOCKET, CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_SESSION_ATTENDED, CLAUDE_CODE_SESSION_ID, CLAUDE_EFFORT, CLAUDE_PID`. The spike child additionally saw `CLAUDE_CODE_DISABLE_MOUSE_CLICKS`. S6 says the spike's parent held **14** and names only five, and the brief says "the wrapper removed them", i.e. all 14. **No artifact enumerates the 14.** The test-plan's "literal 14-name S6 list" cannot be copied from S6 as written.
8. **cargo-modules 0.27.0 and cfg-gated files** (scratch crates, Windows host):
   - `#[cfg(unix)] mod gated;` + `src/gated.rs` gives exit 1, `orphaned module 'gated' at src/gated.rs`.
   - `#[cfg(windows)]` gives exit 0.
   - The same `cfg(unix)` crate with `--target x86_64-unknown-linux-gnu` gives exit 0.
   - An inline `#[cfg(unix)] mod gated { … }` gives exit 0.
   - **The CARRY's hypothesis is verified, and `--target <own triple>` per leg does not fix it.** A `cfg(windows)` file is an orphan on the Linux/macOS legs, and only analysis for a triple that compiles it clears it.
9. **curl exit 35 and retries.** Git-for-Windows curl 8.18.0 (Schannel) is the curl the runner's `shell: bash` step resolves (`C:\Program Files\Git\mingw64\bin\curl.exe`). It was pointed at a local TCP server that answers with non-TLS bytes, which gives handshake failure exit 35, the class of `CRYPT_E_REVOCATION_OFFLINE`.
   - `--retry 3 --retry-delay 1`: exit 35 after **1** connection. The existing flag does not retry this class, which is why attempt 1 failed at once.
   - Adding `--retry-all-errors`: **4** connections.
   - `--ssl-revoke-best-effort --retry 2 --retry-all-errors`: 3 connections, and the flag is accepted.
   - `--ssl-revoke-best-effort` turned out to work on this host's real HTTPS fetch of crates.io (exit 0).
   - Non-Schannel curls (ubuntu/macOS runners) were not measurable here. curl documents the flag as Schannel-scoped.
10. **cargo-deny 0.20.2 with the project's `deny.toml` over portable-pty `=0.8.1` + windows-sys `=0.61.2`** (scratch crate, `--all-features`): `advisories ok, bans ok, licenses ok, sources ok`, exit 0. The graph adds anyhow, bitflags 1, downcast-rs, filedescriptor, lazy_static, libc, log, nix 0.25, serial 0.4, shared_library, shell-words, winapi 0.3, winreg 0.10 and thiserror 1. It has no C build, and no new licence or exception is needed.

## CI verdict (runner-only; closed against the run)
- **Run 36141312209 · sha `02535078faca200f099920313eb52dbeec424a88` · job `lint (windows-2025)` · step `Install ripgrep (PCRE2, checksum-pinned)`.**
  - Attempt 1 (13:31Z): `curl: (35) schannel: next InitializeSecurityContext failed: CRYPT_E_REVOCATION_OFFLINE (0x80092013)` and `Process completed with exit code 35`.
  - Attempt 2 (13:34Z): `install-ripgrep: ripgrep 15.2.0 installed at …` in 0.9 s, and the job succeeded, so the run is 15/15.
  - The failure reproduced on the runner and cleared on re-run: a transient, not a code defect. Non-reproduction on this host is expected.
  - The mechanism that turned a transient into a red is **measured locally** (fact 9): `--retry 3` does not retry exit 35.

## Patterns detected
- **Seam wrapping** (`crates/viola-core` validators; arch §Crate dependency direction): a viola-owned API over a third-party crate, with one thiserror enum per crate and fixed-message `Display` (per the `cmd::Failure` / `obs::report_internal_error` split, `src/main.rs:45-50`).
- **Probe-proven gates** (`scripts/orphans-check.sh` `--probe`, `deny-probes.sh`, `lint-probes.sh`): a planted fault must fire and a control must stay clean before the real run is trusted. `ci.yml:319` runs `--probe && <real>`.
- **Pins with rationale** (`Cargo.toml` `[workspace.dependencies]`, e.g. the `sha2` and `jsonschema` comments): the reason sits beside the pin. This is the natural home for the v1-25 record.
- **Interim seams documented in the code** (`tests/support/home.rs:108-109`, `supervise.rs:1-3`): "the PTY seam replaces the pipe when `viola-pty` lands". This chunk retires both notes.
- **Exit-aware bounded waits** (`home.rs:153-179`, `supervise.rs:100-112`): `try_wait` plus a deadline below 20 s.

## Conventions to follow
- **New members:** `version/edition/rust-version/license/publish.workspace = true`, `[lints] workspace = true` (`tests/contract_lints.rs:33-75`), deps via `workspace = true` only.
- **Sync crates** join `scripts/sync-crates.txt` (`ci.yml:283-286`, `383-386`).
- **Logging** only via `obs_event!` with the catalog's fields (`src/run/mod.rs:22-62`). New fields are already in `schemas/diag-line.v1.json` (`env_stripped_known` is a string, comma-joined).
- **Output:** the workspace denies `print_stdout`/`print_stderr`, and "output modules take a local `#[allow]`" (`Cargo.toml` lints comment).
- **Tests:** inline `#[cfg(test)] mod tests { … }` only (testing.md 2026-09-25), `<subject>_<condition>_<expected>`, `tui_`/`chaos_` prefixes for root binaries, oracles as literals.

## New files to create
- `crates/viola-pty/{Cargo.toml, src/lib.rs}`: the seam. `Pty` trait (spawn · read · write · resize · wait · kill) behind `#[cfg_attr(test, mockall::automock)]`, `PtyError` (thiserror, fixed messages; source boxed for the detail file), the portable-pty `=0.8.1` impl and the `TerminateProcess` kill fallback (windows-sys, **inline** `#[cfg(windows)]` block/module per fact 8). It also holds the host-terminal raw-mode guard (fact 3) and host-size polling for resize forwarding. The deps are portable-pty, windows-sys (`cfg(windows)`, gaining the `Win32_System_Console` feature) and, pending P4, libc (`cfg(unix)`).
- `crates/viola-agent-claude/{Cargo.toml, src/lib.rs}`: the R8 strip (a pure function over an env-name iterator → `{remove: Vec<OsString>, count, known: Vec<&'static str>}`) and the npm-shim resolution (`PATH` + `PATHEXT` search done by viola; `.cmd`/`.bat` → sibling `claude.exe` or refusal). Deps are `viola-core` and thiserror only (the "first consumer creates" rule; `viola-state` / vt100 / serde_path_to_error are not needed yet).
- `tests/support/outer_pty.rs`: the root tui driver over `viola-pty` (test-plan §6 tui row). Root tests `tests/tui_passthrough.rs` (zero-own-bytes, keystrokes, exit on handle) and E2 (`tests/cli_strip.rs` or similar, name fixed at P4).
- A probe mode in `scripts/install-ripgrep.sh` (`--probe`).

## Files to modify
- `Cargo.toml` (root): members are covered by the `crates/*` glob. Add `[workspace.dependencies]` portable-pty `=0.8.1` (+ v1-25 rationale comment) and mockall `=0.15.0` (testing.md). Add windows-sys feature `Win32_System_Console`. Root `[dependencies]` gains viola-pty and viola-agent-claude, the fake agent's console-mode need rides on viola-pty, and `[dev-dependencies]` stays.
- `src/run/mod.rs`: `spawn_child` becomes the PTY spawn through `viola-pty` (cwd = current dir, env strip applied, absolute resolved program), plus the pump threads (stdin → PTY, PTY → stdout) inside `catch_unwind`. `log_child_start` gains `pty_backend`, `env_stripped_count` and `env_stripped_known`, and `log_child_exit` gains the `kill-fallback` source.
- `src/cmd/run.rs::run()`: the only caller of `spawn_child` (graph). It threads resolution, refusal (`batch-script-child` + the stderr line + hint), host raw mode and handle-wait exit.
- `src/bin/viola-fake-agent.rs`: on start, put its own terminal stdin into raw mode like the real CLI (facts 1-2), via viola-pty's raw guard. The `env` receipt is unchanged. It gains a `size` receipt for the resize oracle, pending P4.
- `crates/viola-e2e/Cargo.toml` (+ `viola-pty`) and `crates/viola-e2e/src/harness/supervise.rs`:
  - `Wrapper` holds an outer PTY master and writer.
  - A drain thread runs per master.
  - `stop` writes `\x03` into the PTY.
  - The 4 inline tests move to the PTY form. `fake_agent()` spawns under a PTY, or its stdin stays a pipe where the test only exercises `wait_or_kill`.
- `tests/support/home.rs`: `Wrapper::boot/send/stop` move to the PTY form via `outer_pty`. The module doc (l.108-109) is retired.
- `tests/run_cli.rs::run_viola`: PTY form. The role-shape assertions gain the new `process-start` fields.
- `scripts/sync-crates.txt`: + `viola-pty`, `viola-agent-claude`.
- `scripts/orphans-check.sh`: triple-aware analysis (fact 8), plus a probe case for a `cfg(windows)`/`cfg(unix)` file pair.
- `scripts/install-ripgrep.sh` (download flags + probe) and `.github/workflows/ci.yml` (lint: `install-ripgrep.sh --probe` before the install).
- `tests/contract_lints.rs`: no change (it iterates the members).
- `deny.toml`: no change (the ignore is present; fact 10).
- `schemas/diag-line.v1.json`: no change.

## Open questions
- **R8 strip set** → blocks: plan-decision. Is it every `CLAUDE`-prefixed name (what the S6 spike measurably did, and what obs D-14's "unknown `CLAUDE*` names are counted" presumes), or exactly a known list (arch: "the remaining `CLAUDE*` parent-identity variables on the measured list")? The 14 names are enumerated nowhere (fact 7), and prefix stripping would also remove user configuration such as `CLAUDE_CONFIG_DIR`.
- **Unix legs under the PTY form** → blocks: plan-decision. A cooked Unix tty line-buffers and turns `^C` into SIGINT for the foreground group (POSIX ICANON/ISIG). This is not measurable on this host, and the witness is the ubuntu/macOS CI legs. The PTY-form tests therefore need raw mode on the wrapper's host tty and in the fake agent on Unix too. That needs termios (libc), which arch's `viola-pty` dependency list (portable-pty, windows-sys) does not name.
- **`VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN`** → blocks: plan-decision. E2 expects `VIOLA_NAME`/`VIOLA_DIR` in the child env (test-plan §6 E2). Arch sets them at child spawn after the pinned copy and the instance dir exist, and both are created by "Instance state and start order".
