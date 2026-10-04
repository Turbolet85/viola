# Codebase Research — 2026-10-03-mutation-scoring-completion

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 19 · **Host measurements:** 5 (C3 two-sided, M3 two-sided ×2, cfg(unix) list, TUI set, `channel_frames` ×3)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (auto-loaded on the harness reads), 7 Session Additions applied (the `--in-diff` regeneration note, the cfg-excluded-miss note, the never-pipe-`boot` note, verdict-not-counts, the `tests::name` filter form, the unbuilt-selector rule, the same-day control-pair rule); `.claude/rules/testing.md` — read in full (its `paths:` cover `crates/*/tests/**`, which item 7 touches), 22 Session Additions applied (notably :56 per-OS I/O error call sites, :58 cfg(unix) minimal OS reader, :60 remove-the-guard runs, :64 crate-local security tests, :69 never reshape to dodge mutants)
- **Platform issues consulted:** filled at P5 (validation check 9; scope carries item 7's runner-only bullet). Queries:
  - `macOS unix domain socket write ENOTCONN "Socket is not connected" after peer closed os error 57 rust`;
  - `actions runner-images macos ENOTCONN unix socket write "os error 57"`.

  The hosted platform's own tracker (actions/runner-images) returned no matching issue. The one fetched source,
  bobmatnyc/trusty-tools PR #8851, states that "On macOS `shutdown(SHUT_WR)` returns ENOTCONN once the peer has
  closed", that "Linux's `shutdown` returns 0 in this situation", and that the fix lets the code "fall through to
  the response read, which returns the reply", the reply being "already buffered".

  That source names `shutdown`, not the `write` this test hit (`channel_frames.rs:92:28`). It supports the
  hypothesis that the reply stays readable after a macOS ENOTCONN close race, but does not prove it for `write`. The
  witness stays the `test (macos-latest)` job. The recorded run's log was read with
  `gh run view 37157981452 --log-failed`.
- **Code-graph:** built cold this phase (`scripts/code-graph.py refresh`, PATH prepended with `~/.local/viola-node/bin` and `~/.local/viola-venv/bin`): rust 2604 nodes / 11836 edges (72 s), ts 7 nodes / 1 edge. Trace `.andromeda/runs/2026-10-03T22-20-19-phase/tree-query-2026-10-03-mutation-scoring-completion.json` (7 queries).

## Files inspected
- `crates/viola-e2e/src/harness/run/mutants.rs` (1–210, test names) — the `run --mutants` arm: root prebuild `cargo build --package viola --features fake-agent` into `<repo>/target/mutants` (:132–:140), then `cargo mutants --workspace --features fake-agent --in-diff <chunk.diff> --test-tool=nextest --copy-target=true` with relative `CARGO_TARGET_DIR=target/mutants`, `NEXTEST_PROFILE=mutants`, `AGENT_RUN_KEEP_HOMES=0`/`AGENT_RUN_KEEP_FAILED=0` (:141–:165). It is diff-scoped only (`--in-diff` always); there is no whole-unit form.
- `crates/viola-e2e/src/harness/run/mutants/base.rs` (100–189) — `chunk_diff` runs `git diff --no-color --no-ext-diff <merge-base>` (:124–:127) and `git diff --no-color --no-ext-diff --no-index -- /dev/null <file>` per untracked file (:130–:144), neither pinning prefixes; `diff_paths` keys on `strip_prefix("diff --git a/")` + `rsplit_once(" b/")` (:152–:158); `diff_files` counts `diff --git ` (prefix-agnostic). `chunk.diff` is also cargo-mutants' own `--in-diff` input.
- `crates/viola-e2e/src/harness/run/mutants/scratch.rs` (10–50) — `HOST_SCRATCH = cfg!(windows)` (:11); `prepare` returns `Ok(None)` off Windows (:48) → `mutants.out/` at the repo root. Inert on this host (no `../viola-mutants-scratch` exists).
- `crates/viola-e2e/src/harness/pre_push.rs` (1–230, test names) — `PRE_PUSH_HOST_SUPPORTED = cfg!(windows)` (:24); off Windows the `host` stage refuses `pre-push-windows-only`, exit 2 (:74–:77); stages `tools → sync → cache → linux-tests → vm-release → windows-tests` (:126–:157); `release_vm` shells `powershell.exe` + `wsl.exe --terminate` (:160–:184); `windows_tests` = `run --coverage` + `gate --require coverage,doctest` on the host (:188–:202). Tests :417–:552 pin the Windows stages.
- `crates/viola-e2e/src/harness/pre_push/linux.rs` (1–330, test names) — `Linux::cmd` = `wsl.exe -d Ubuntu [--cd D] --exec /usr/bin/env -i HOME=<distro home> PATH=<home>/.cargo/bin:<home>/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin` (:38–:52); `wsl_path` (:62–:76); `ci_pins` / `node_pin` file-text readers (:80–:102); `distro_home` via `printenv HOME` (:115–:122); `tools` checks `cc`, rustc + ci.yml test-job pins + node (:126–:165); `sync` (temp-index binary patch into the clone, tree-id equality, :169–:242); `cache` (40 GiB cap, :256–:265); `linux_tests` = `run --coverage` → `run --browser` → `gate --require coverage,doctest,playwright` read from each child's last JSON line (:306–:328).
- `crates/viola-e2e/src/harness/mod.rs` (116) and `supervise.rs` (201–205) — viola-e2e finds the root bins through `bin_dir_from_exe(current_exe)`: the test exe's `target/<profile>/`, so a copied tree needs the root bins in ITS target dir.
- `Cargo.toml` (12–24) — `viola-fake-agent` is a root `[[bin]]` with `required-features = ["fake-agent"]`.
- `crates/viola-channel/tests/channel_frames.rs` (70–118) — `channel_frame_one_byte_over_is_refused_and_closed`: the write arm tolerates `BrokenPipe | ConnectionReset` only (:90–:93), then asserts the `-32600 invalid request` reply and the end of stream (`cfg!(windows)` → `Ok(0)`; else `Ok(0)` or `ConnectionReset`, :103–:117).
- `.andromeda/runs/2026-10-01T09-18-50-code-audit/{proposals.md,mut.sh,mut-e2e.sh,mut-driver.log}` — the boundary tier's own form is `cargo mutants -p <unit> [--features fake-agent] --test-tool=nextest -j N --output …` (no prebuild, no `--copy-target`); the viola-e2e re-invocation added `--test-workspace=true` and still read `baseline-test-failure`, 727 planned. The 13 not-measured mutants are listed at proposals.md:27–:38 by coordinate.
- `scripts/wsl-exec.sh`, `scripts/wsl-provision.sh` — the WSL operator aid and provisioner (headers read); `scripts/install-node.sh:4` and `.github/workflows/ci.yml:10` carry WSL-provisioning comments only.

## Measured on the host (2026-10-04, Linux, btrfs)
- **C3, two-sided** (`cargo nextest run -p viola-e2e -E 'test(/mutants/)'`, a 42-test selection): host config (`diff.mnemonicprefix=true` in `~/.config/git/config`) → **29 passed / 13 failed**; `GIT_CONFIG_GLOBAL=/dev/null` → **42 passed / 0**. The 13 are the `run --mutants` tests whose throwaway repo diff reads `no-rust-delta` (panic text: `"verdict":"no-rust-delta"` where `counted` was expected). Re-derived: the relay's 25/13 vs 38/0 was another selection; the 13 and the two-sidedness match.
- **M3, two-sided, config isolated** (`GIT_CONFIG_GLOBAL=/dev/null`, `NEXTEST_PROFILE=mutants`, one `--re mutants_exit_reason` filter, 22 mutants): (A) the audit form `cargo mutants -p viola-e2e --features fake-agent --test-tool=nextest` → exit 4, baseline **216/245 run, 32 failed**, each `fake agent: Spawn(… /tmp/cargo-mutants-viola-*.tmp/target/debug/viola-fake-agent … ENOENT)` at `supervise.rs:205`; (B) root prebuild `CARGO_TARGET_DIR=$PWD/target/mutants cargo build -p viola --features fake-agent` (6 s, warm), then `CARGO_TARGET_DIR=target/mutants cargo mutants -p viola-e2e --features fake-agent --test-tool=nextest --copy-target=true` → exit 0, baseline **245/245 passed in 20.5 s**, **22 caught / 0 missed / 0 timeout / 0 unviable**, 64 s wall. Logs: session scratchpad `p3m3iso/`.
- **M3 with the host config in place:** both forms read baseline-red on the C3 tests first (A: 29 failed of 175 run; B: 27 of 171), so **C3 must land before any viola-e2e scoring on this host**.
- **Copy size:** `du -sb target` = 12 339 008 691 B before the prebuild, 12 869 606 479 B after (target/debug 4.5 G, llvm-cov-target 3.5 G, harness 3.4 G); B's whole run incl. the copy took 64 s on btrfs — the copy is per job (`-j N` → N copies).
- **viola-e2e unit size:** `cargo mutants --list --package viola-e2e --features fake-agent | wc -l` → **727** (equals the audit's planned_len).
- **The 12 cfg(unix) mutants** (`cargo mutants --list --workspace --features fake-agent --line-col=true --file …`): every audit coordinate is present unchanged at HEAD — viola-pty `lib.rs:315:9` ×2, `:317:64`, `:322:71`, `:367:76`; viola-channel `client.rs:231:5`, `endpoint.rs:76:5`, `server.rs:75:9`, `server.rs:108:5`; `src/panic_frames.rs:69:5` (`vec![0]`), `:80:5` (`None`, `Some((String::new(), 1))`). The 13th, `src/cmd/run.rs:318:5` arm (`sideload_outcome`, `cfg(all(windows, not(target_arch = "x86_64")))`), also lists. The unix arms carry further mutants beyond the 12 (e.g. `panic_frames.rs:69:5` ×3, `:80:5` ×5, `:82:77`; `lib.rs:290:9`–`:309:10`, `:347:5`, `:355:87`; `client.rs:182:5`–`:215:12`; `server.rs:83:5`, `:94:25`) — the Windows audit graded those as other outcomes; a native run grades them all.
- **In-repo TUI set** (`cargo nextest run --workspace --features fake-agent -E 'binary(/^tui_/)'`): **12 passed / 0** across `tui_pty_seam` (3), `tui_env_strip` (3), `tui_passthrough` (5), `tui_channel_fds` (1). None asserts focus / mouse / wheel (`grep -ln 'focus|mouse|1004|wheel' tests/tui_*.rs` → no file): a11y-plan §4 P4 clause (3) has no case at HEAD — owned by the wheel/driving-verbs entries, not this chunk.
- **`channel_frames` on Linux:** 3 rounds × 4/4 passed — the macOS red does not reproduce here (its expected state: a runner-only subject).
- **Gate tools under the native `env -i`:** `env -i HOME=$HOME PATH=$HOME/.cargo/bin:$HOME/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin` resolves `cc` (GCC 16.2.1), rustc 1.98.1, cargo-nextest 0.9.146, cargo-mutants 27.1.0, cargo-llvm-cov 0.9.1, node v24.21.0 — every ci.yml pin (`ci.yml:12`, `:41`; `rust-toolchain.toml` 1.98.1); Playwright's Chromium is under `$HOME/.cache/ms-playwright` (chromium-1243). The child env is exactly `HOME`, `PATH` (+ the shell's own `PWD`/`SHLVL`/`_`). The pre-push leg spawns only cargo, git, npm, node (grep `Command::new(` over `crates/viola-e2e/src`): **no python, so the venv is not needed on the gate's PATH**; it serves only the code-graph refresh.

## Graph impact (from the code-graph query)
- **chunk_diff** — 5 call sites in `run/mutants.rs` and `run/mutants/base.rs` (+ the `run.rs` re-export) — the C3 pin is local to its two `git diff` argvs; no signature change.
- **diff_paths** — 6 sites (`run.rs` re-export, `mutants.rs`, `base.rs` incl. tests) — unchanged by the pin.
- **pre_push_with** — 3 (`pre_push.rs` + its tests); **stages** — 1 (`pre_push_with`); **wsl_path** — 3 (`pre_push`, its test) — the pre-push reshape is contained in `harness/pre_push*` + `bin/viola-harness.rs:13,:244`.
- **bin_dir_from_exe** — 8 (`mod.rs`, `supervise.rs`, `tests/cli.rs`, `tests/harness_lifecycle.rs`) — the spawn-path seam M3 depends on; unchanged.
- **mutants** — 1 (`run.rs` `tool_arms`).

## Patterns detected
- **Runner seam** (`run.rs` `Runner` / `run_with`; `pre_push_with(…, runner)`): every external command goes through an injectable runner; tests assert argv strings (`linux.rs:585` `pre_push_every_wsl_call_uses_exec_and_a_clean_env`) and use a `Fake` answer table (`pre_push.rs:286`–`:392`).
- **OS gates as consts** (`PRE_PUSH_HOST_SUPPORTED`, `HOST_SCRATCH`): a `const` = `cfg!(…)`, pinned by a test comparing to `std::env::consts::OS` (`pre_push.rs:510`).
- **Throwaway repos in tests** (`run.rs:505` `init_repo`, `mini`) run real `git` with `-c user.name/-c user.email` — they inherit the host's global git config, which is how C3 reached them.
- **Labelled case tables** in viola-e2e (no rstest — viola-e2e has no dev-dependencies; test-plan history `2026-09-27-epoch-2-cleanup` trap).

## Conventions to follow
- **Closed enums** for `pre-push` `stage` / `reason` / `detail` (test-plan §3): a retired or new value is recorded for the wrap's amendment, never invented silently.
- **One JSON document** per harness command, codes/counts/repo-relative names only (obs-plan §8 item 6; `linux.rs:281` `summary` strips suite `artifact` paths).
- **`env -i` crossing = HOME + PATH constants** (`linux.rs:34`–`:52` doc comment): the native stage keeps the same PATH shape with the host HOME in place of the distro HOME.

## New files to create
- none

## Files to modify
- `crates/viola-e2e/src/harness/run/mutants/base.rs` — C3: `--src-prefix=a/ --dst-prefix=b/` on both `chunk_diff` `git diff` calls; hostile-config cases (`diff.mnemonicprefix`, `diff.noprefix`) set in the throwaway repo's own config
- `crates/viola-e2e/src/harness/run/mutants.rs` — M3: the whole-unit boundary form (P4 fork; only if the harness carries it)
- `crates/viola-e2e/src/harness/run.rs` — M3: the selection flag for the whole-unit form (P4 fork; only if the harness carries it)
- `crates/viola-e2e/src/bin/viola-harness.rs` — the `pre-push` host gate wiring and, under the harness branch of the M3 fork, its clap flag
- `crates/viola-e2e/src/harness/pre_push.rs` — native Linux reshape: host gate, stage list, retire `vm-release` / `windows-tests`, document sections, tests
- `crates/viola-e2e/src/harness/pre_push/linux.rs` — native launcher (`env -i HOME PATH` without `wsl.exe`), `tools` without the distro probe, retire or keep `sync`/`cache` per the P4 fork, tests
- `crates/viola-channel/tests/channel_frames.rs` — item 7: the close-race write arm
- `scripts/wsl-exec.sh` — retired (deleted) with the WSL tooling
- `scripts/wsl-provision.sh` — retired (deleted) with the WSL tooling
- `scripts/install-node.sh` — header comment: the WSL provisioning reference
- `.github/workflows/ci.yml` — comment line 10 only: the WSL pre-push provisioning reference
- `crates/viola-e2e/src/harness/` — survivor kill tests from the viola-e2e boundary run (provisional: which files depends on the survivors)
- `crates/viola-e2e/tests/` — survivor kill tests from the viola-e2e boundary run (provisional)
- `crates/viola-pty/src/lib.rs` — cfg(unix) survivor kill tests, inline (provisional)
- `crates/viola-channel/src/` — cfg(unix) survivor kill tests in `client.rs` / `endpoint.rs` / `server.rs`, inline (provisional)
- `src/panic_frames.rs` — cfg(unix) survivor kill tests, inline (provisional)
- `viola-0.1.0/chunks/2026-10-03-mutation-scoring-completion/` — evidence records (the scoring outcomes, the 13th's not-measurable record, the witness logs)

## Open questions
- Where the boundary tier's viola-e2e form lives — a harness arm the audit calls (e.g. a whole-unit `run --mutants` selection) vs a recorded recipe the audit runs directly (the measured B form) → blocks: plan-decision
- Whether the native `pre-push` stage runs in the working tree in place or in a native clone synced as before (the clone was WSL's filesystem bridge; in place drops `sync` / `cache`) → blocks: plan-decision
- Whether the native stage's `env -i` PATH stays exactly the WSL shape (no widening — measured sufficient) — shown to the founder at P4 as the operator directed → blocks: plan-decision
