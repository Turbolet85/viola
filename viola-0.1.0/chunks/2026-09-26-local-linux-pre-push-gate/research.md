# Codebase Research — 2026-09-26-local-linux-pre-push-gate

## Scope
- **Depth:** moderate · **Reads:** 11 (harness `mod.rs`, `run.rs`, `run/mutants.rs` 1–440, `gate.rs` 1–165, `bin/viola-harness.rs`, `scripts/agent-run.sh`, `scripts/agent-run.ps1` 1–60, `ci.yml` 1–260, test-plan §3 611–666, two prior evidence files) · **Globs/Greps:** 12 · **Host probes:** 6 (WSL distro, tools, sudo, vhdx, env forwarding, sync-form timings)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 4 Session Additions applied (the `--in-diff` note, the `--leg windows-2025` rule for `#[cfg(unix)]` diffs, never pipe `boot`, judge a leg by its verdict); `.claude/rules/host-win32.md` (always loaded) — the MSYS `/X` conversion and stop-by-ExecutablePath entries applied.
- **Platform issues consulted:** none — no issue-tracker or release-notes search for a failure signature was run: both CI reads in the plan are post-push conclusion reads of this chunk's own run, and no failure signature exists at P5 (no runner-only bullet was folded; Setup 5a read no completed red).
  - Consulted instead, as context for those reads and not as a signature search:
    - the GitHub REST "List check runs for a Git reference" doc (docs.github.com/en/rest/checks/runs, fetched 2026-09-26): `per_page` "Default: 30" (max 100), `filter` "Default: latest". This repo's pushes carry 15 check runs (Setup 5a on a69c5ef: `total 15`), so the unpaged reads see every run;
    - CI's ubuntu runner image, from job 108482499487's log (run 36270173848, `mutants (ubuntu-latest)`): `Image: ubuntu-24.04`, `Version: 20260920.314.1`, against the local Ubuntu 26.04.1 LTS (M1).

## Files inspected
- `crates/viola-e2e/src/bin/viola-harness.rs` (full) — clap `Cmd` enum (`Boot · Run · Status · Cleanup · Logs · Supervise · SchemaCheck · SecretScan · Gate`, :27-70), `COMMANDS` list (:93-103) used to name the `cmd` of a usage error, `emit` prints one document (:105-108). A new internal subcommand is one variant + one `*_cmd` fn + one `COMMANDS` entry.
- `crates/viola-e2e/src/harness/mod.rs` (full) — `Outcome` (doc + code 0/1/2, :32-53), `Workspace` paths (`agent_run`, `artifacts`, `cargo_target` = `target/harness`, :55-98), `valid_session_id` (:101-108, reused by `parse_legs`).
- `crates/viola-e2e/src/harness/run.rs` (full) — `run_with(ws, sel, filter, chunk_base, leg, runner)` (:108-126) with the `Runner` seam `FnMut(&mut Command) -> (Option<i32>, String)` (:31); `deferred` survivors under `--leg` (:122); `Refusal { reason, detail }` (:93-105) = the `tool-missing` + `detail` pattern; `run_forwarding` sends the tool's stdout to stderr (:216-228).
- `crates/viola-e2e/src/harness/run/mutants.rs` (1–440) — `resolve_base` (:47-56): override → `chunk_flip` → `merge-base HEAD origin/main` → `None` = `base-missing`; `chunk_flip`/`pre_ci_parent` read HEAD's `master-route.md` and history (:61-105); `chunk_diff` = working tree vs merge-base + untracked non-ignored (:116-142); `cargo build --package viola` + `cargo mutants … --in-diff … --copy-target=true` with `CARGO_TARGET_DIR` removed (:335-361), so a run writes `target/` and `mutants.out/` in the tree it runs in; `write_leg_verdict` writes `artifacts/mutants-verdict-<leg>.json`, repo-relative names + outcomes only (:276-302).
- `crates/viola-e2e/src/harness/gate.rs` (1–165) — `gate(artifacts, root, require, legs)` (:59-87); `union` (:120-163) needs every named leg's file (`artifact-missing` otherwise) and judges each mutant by `compiled_legs(root, name, legs)`, falling back to EVERY named leg when none compiles the line (:144-149).
- `crates/viola-e2e/src/harness/cfg_legs.rs` (grep) — leg families by prefix: `windows-` → windows, `ubuntu-` → unix+linux, `macos-` → unix+macos (:23-25); an unknown family keeps the leg.
- `scripts/agent-run.sh` / `agent-run.ps1` — identical thin shims; the internal subcommands are one `case` arm (`supervise|ui-restart|gate|schema-check|secret-scan`, sh :74) and the matching ps1 switch; `cargo run -q -p viola-e2e --bin viola-harness -- "$@"`.
- `.github/workflows/ci.yml` (1–260) — `test` job: `rustup toolchain install` + `llvm-tools-preview` + `cargo-nextest@0.9.146,cargo-mutants@27.1.0,cargo-llvm-cov@0.9.1` (:25-33), `run --coverage` (:36-44), lifecycle smoke (:46-70), G2/G4/secret-scan/uploads, `gate --require coverage,doctest` (:137-140). `mutants` legs: `fetch-depth: 0`, `cargo-nextest@0.9.146,cargo-mutants@27.1.0` (:164-166), `run --mutants --leg "$LEG"` with `LEG` = `ubuntu-latest` / `windows-2025` (:169-173); `mutants-verdict`: `gate --require mutants --mutants-legs ubuntu-latest,windows-2025` (:197-199).
- `.andromeda/test-plan.md` §3 (611–666) — Internal harness subcommands (`supervise`, `ui-restart`, `schema-check`, `secret-scan`, `gate`) and Closed enums (a new `reason` / `detail` / `cmd` value needs a Decisions Log entry, :666).
- `chunks/2026-09-25-pty-wrapper-on-windows/evidence/operator-pass.md` (55–95) — run 36165685381 (17ea8c7): `test (ubuntu-latest)` red on the held-output test AND `mutants-verdict` red on the union while BOTH legs were individually green (:56-67).
- `chunks/2026-09-26-ci-chunk-base-and-union-verdict/evidence/mutants-leg-windows-local.md` — local Windows leg: 79 tested, 71–73 caught, 6–8 unviable on one tree; rust-analyzer held `mutants.out` (os error 5).

## Graph impact
- **`gate`** — callers: `gate_cmd` @ `crates/viola-e2e/src/bin/viola-harness.rs:194` + 17 in-module tests (`tree-query-2026-09-26-local-linux-pre-push-gate.json`, q1). Reused unchanged.
- **`run_with`** — callers: `run_cmd` @ `viola-harness.rs:146`, `run` @ `run.rs:88`, tests in `coverage.rs`, `fuzz.rs`, `mutants.rs` (q1). Reused unchanged.
- **`resolve_base`** — `mutants` @ `run/mutants.rs:316` + tests (q1). Unchanged; the clone only has to satisfy its preconditions.
- **`leg_verdict_path`** — `union` @ `gate.rs:123`, `write_leg_verdict` @ `mutants.rs:298`, `mutants` @ `mutants.rs:313` (q1). Reused to place the copied-back Linux verdict.
- **Crate edges** — `viola-e2e` → `viola-pty`, `viola-core`; no inbound edge (q2): a leaf, so an additive module has zero cross-crate blast radius. No signature changes → no caller threading.

## Measured facts (host, 2026-09-26)
- **M1 distro.** `wsl -l -v`: `docker-desktop` is the DEFAULT (`*`), `Ubuntu` present, both WSL 2. `Ubuntu 26.04.1 LTS`, kernel 6.6.87.2-microsoft-standard-WSL2, 32 cores, 31 GB, user `turbo` (uid 1000). CI's `ubuntu-latest` is a different release: the local leg is a filter, never CI's twin.
- **M2 argv transport.** `wsl -d Ubuntu -- bash -lc '…$t…'` printed `$t` empty: the `--` form re-parses the joined command line through the distro's shell. `wsl -d Ubuntu --exec <prog> <args>` passed arguments verbatim. From Git Bash a `/mnt/c/...` argument is rewritten by MSYS (`C:/Program Files/Git/mnt/c/...`, exit 127) unless `MSYS2_ARG_CONV_EXCL='*'`; a Rust `Command` is not subject to MSYS conversion.
- **M3 tools in Ubuntu.** present: `git 2.53.0`, `rsync`, `tar`, `curl`, `python3`. MISSING: `cc`, `gcc`, `make`, `pkg-config`, `jq`; no `~/.cargo`. `build-essential` candidate `12.12ubuntu2.26.04.2`. `sudo -n true` fails: sudo needs a password. [corrected at P5 by overseer measurement 2026-09-26 23:58, re-read in the phase run dir p5/06.log: `wsl.exe -d Ubuntu -u root --exec /usr/bin/id -u` prints 0 (the control without `-u root` prints 1000), and `apt-get -s install -y build-essential` as root resolves 44 packages — so the install runs as root through WSL, no sudo, no password, no operator step.]
- **M4 PATH leak.** Ubuntu's PATH carries the Windows PATH (WSL `appendWindowsPath` default): `/mnt/d/dev/rust/cargo/bin` (line 36 of the split PATH), Git for Windows' `usr/bin`, the rust-analyzer plugin dir. A command run through `--exec` sources no profile, so a gate must hand WSL an explicit PATH (Linux `~/.cargo/bin` + system dirs).
- **M5 env forwarding.** This Claude session holds 10 `CLAUDE*` variables incl. `CLAUDE_CODE_MESSAGING_TOKEN` / `_SOCKET` (`env | grep '^CLAUDE'`); inside `wsl --exec` the count was 0 (`WSLENV=WT_SESSION:WT_PROFILE_ID:`). The secret does not cross unless `WSLENV` names it.
- **M6 disk.** Ubuntu's vhdx `C:\Users\turbo\AppData\Local\wsl\{6f6e4923-…}\ext4.vhdx` = 1.48 GB; `C:` free 199.1 GB (PowerShell `Get-PSDrive C`); the ext4 fs reports 1007 GB capacity. A vhdx grows and does not shrink on file deletion, so the cap bounds the PEAK. `--copy-target=true` copies `target/` per mutation run (2.9 GB measured on the dev host, test-plan §3 run step 4).
- **M7 XDG.** `XDG_RUNTIME_DIR=/run/user/1000/` (mode 700, owner turbo) inside WSL (systemd=true in `/etc/wsl.conf`). No product code reads `XDG_RUNTIME_DIR` yet (`grep -rn XDG_RUNTIME_DIR crates src tests` → 0 hits), so no socket-dir branch diverges today; the Unix IPC chunk inherits this note.
- **M8 sync forms** (same tree, dirty: 4 modified + 2 untracked dirs; scratch dirs under `~/viola-probe-1`, removed after):

  | form | first sync | repeat sync | ignored files carried | resulting `git status` |
  |---|---|---|---|---|
  | **clone + fetch + temp-index patch** | clone 4.47 s | Windows patch 0.54 s + fetch 0.04 s + reset 0.28 s + apply 0.01 s ≈ 0.9 s | none by construction (`git add -A` honours `.gitignore`) | identical to Windows (4 M, 2 ??) |
  | rsync `-a --delete`, excluding `/target/`, `/mutants.out*` | 9.85 s | 2.74 s (no-op) | every ignored file not excluded by hand | identical to Windows |

  The patch is built on Windows with `GIT_INDEX_FILE=<tmp>`: `git read-tree HEAD` → `git add -A` → `git diff --cached --binary HEAD` (77 863 B, 15 files); the real index was untouched (`git status` after). `git add` printed one CRLF warning per `w/crlf` checkout-artifact file; those files normalised to their HEAD blobs and produced no diff. Clone and rsync trees both held 0 CRLF files under `crates/` and `src/`. The clone's history held the flip (`git log -1 -G ' · complete · '` → `a69c5ef`, = the Windows derivation) and the tree held the new `pending` record: `resolve_base` in the clone reads the same inputs as on Windows.
- **M9 prior Unix reds.** `gh api …/commits/<sha>/check-runs`: 17ea8c7 red on `test (ubuntu-latest)`, `test (macos-latest)`, `mutants-verdict`; c05e6e2 red on `test (macos-latest)` only. The ubuntu test red and the union red are this chunk's target class; macOS stays CI-only.

## Patterns detected
- **Internal subcommand shape** (`bin/viola-harness.rs:56-69, 179-195`): a clap variant, a `*_cmd` fn returning `emit(Outcome)`, one JSON document, exit 0/1/2; shims forward it through one `case`/`switch` arm.
- **Runner seam** (`run.rs:31, 108-115`): every external tool goes through `&mut Runner`, so tests use a stand-in runner and never nest cargo/WSL inside nextest (verification-harness.md: `run_with` runner seam).
- **Typed refusal** (`run.rs:93-105`, test-plan §3 :664): `reason` + optional `detail`, exit 1 (`tool-missing`) or 2 (`fuzz-linux-only` OS refusal).
- **Host-limited arm as a const** (`FUZZ_HOST_SUPPORTED`, `run.rs:23`): an OS gate is a const, not a `cfg!()` fn (the carried 0.8 learning: a `cfg!()`-valued fn is an equivalent mutant on one leg).
- **Union over leg files** (`gate.rs:120-163`): `gate --require mutants --mutants-legs a,b` over `artifacts/mutants-verdict-<leg>.json`, cfg-aware via `compiled_legs`.

## Conventions to follow
- **One JSON document on stdout, progress on stderr** (`run.rs:214-228`, verification-harness.md §Shims).
- **Child env per `Command::env`, never `set_var`** (test-plan §3 boot step 3).
- **Leg names are CI's** (`cfg_legs.rs:23-25`): the Linux leg must be named `ubuntu-latest` to get the unix+linux family; a bare `ubuntu` falls to "unknown family keeps the leg".
- **Stop rust-analyzer by exact ExecutablePath before a host `run --mutants`** (host-win32.md Session Additions 2026-09-26).

## New files to create
- `crates/viola-e2e/src/harness/pre_push.rs` — the orchestration (sync → Linux run → leg copy-back → host leg → union → one document), every external call through the `Runner` seam.
- `scripts/wsl-provision.sh` — runs INSIDE Ubuntu: rustup-init at a pinned version with its published sha256, `rustup toolchain install` from the clone's `rust-toolchain.toml`, `cargo install --locked` of the ci.yml pins, then prints the versions; refuses (typed) when `cc` is absent.
- `crates/viola-e2e/tests/` or in-module tests for `pre_push` (stand-in runner; a scratch git repo for the patch/sync half).

## Files to modify
- `crates/viola-e2e/src/harness/mod.rs` — `pub mod pre_push;`.
- `crates/viola-e2e/src/bin/viola-harness.rs` — a `PrePush` variant, a `pre_push_cmd`, `COMMANDS` gains `pre-push`.
- `scripts/agent-run.sh` (:74) and `scripts/agent-run.ps1` (the internal-subcommand arm) — add `pre-push`; header comments name it.
- `.claude/docs/commands.md`, `.claude/docs/workflow.md`, `.claude/rules/verification-harness.md`, `.claude/rules/testing.md` — leaves re-derived at wrap (the operator-pass wiring); spec masters (test-plan §3 Internal harness subcommands + Closed enums + §12, §9/§10 as needed; architecture tree + Occupied Resources if a repo path is added) are wrap amendments, not touchpoints.
- Companion sweep: `pre-push` / `pre_push` name grep over the workspace → 0 hits today (`grep -rn "pre-push\|pre_push" crates src tests scripts` — new name); the internal-subcommand list is pinned only in the two shims, the bin and test-plan/commands.md/verification-harness.md (`grep -rln ui-restart` → 3 docs + the sources) — no test pins the list.

## Scope widening at implement (operator ruling, 2026-09-26 — scope item 6)
The gate's first real run found a Linux red (`evidence/linux-red-investigation.md`); the overseer folded it into
this chunk, instrument first. Files read for it:
- `src/cmd/run.rs` (full) — `SpawnSpec.size = viola_pty::host_size()` at :63 (the spawn read); `pump(&mut pty,
  stdin, stdout, &mut viola_pty::host_size)` at :69-74, so pump's FIRST call of that closure is its baseline read.
  `run(home, args)` holds the `--home` path, so a closure wrapped here can stamp both reads into the home.
- `crates/viola-pty/src/lib.rs` (:114-124 `Pty` trait; :270-322 `pump`; :450-478 `host_size`; :480-620 tests) —
  `let mut last = host_size();` at :293 is the baseline; a change is forwarded only when a poll differs from it
  (:316). `RESIZE_EVERY` 250 ms (:23), `TICK` 20 ms (:22). The tests drive `pump` with a `MockPty` and a size
  closure (`pump_forwards_a_host_size_change_once` :580).
- `tests/tui_passthrough.rs` (1-121) — the failing test (:93-121): spawn 80×24, wait for the child's `start`
  receipt, resize to 100×30, then press keys until a `size` receipt equals 100×30, asserting within `EXIT_WITHIN`
  (10 s, `tests/support/outer_pty.rs:15`). Receipts carry no timestamp (`src/bin/viola-fake-agent.rs:105-116`), and
  the test compares whole receipt objects, so a timestamp field cannot be added to them.
- **Graph impact** — `pump`: 11 call sites — `run` @ `src/cmd/run.rs:69` and 10 tests in
  `crates/viola-pty/src/lib.rs` (trace `.andromeda/runs/2026-09-26T21-57-01-implement/tree-query-…json`,
  `callee_name = 'pump'`; the query regenerated the stale rust DB first). A signature change threads through all 11.

### Files to modify (added)
- `src/cmd/run.rs` — the temporary instrument (one-shot, removed before the fix lands); then passes the spawn size
  to `pump`.
- `crates/viola-pty/src/lib.rs` — `pump` takes the size the PTY was spawned with as its baseline; the 10 test call
  sites; a deterministic test forcing the resize into the window.
- `tests/tui_passthrough.rs` — the resize test's failure message dumps the child's `size` receipts with the times
  the resize was sent and each receipt observed (permanent), plus the instrument's file while it exists.

## Open questions
- Does the local verdict union BOTH legs (Linux leg in WSL + a Windows leg on the host, then CI's `gate --mutants-legs ubuntu-latest,windows-2025`), or judge the Linux leg alone? A Linux leg alone under `--leg` defers every survivor (`run.rs:122`), and `gate --mutants-legs ubuntu-latest` alone judges a `#[cfg(windows)]` body by the ubuntu leg (no named leg compiles it → every named leg judges, `gate.rs:144-149`) — so a Linux-only verdict needs new judging code or goes red on this very chunk's Windows-only WSL-spawning code → blocks: plan-decision.
- Do the local Unix tests take CI's instrumented form (`run --coverage` + `gate --require coverage,doctest`, floors included, needs `llvm-tools-preview` + `cargo-llvm-cov@0.9.1`) or the plain `run --unit --integration` (+ doctest) → blocks: plan-decision.
- Where the operator-pass wiring lives: the project's leaves + test-plan (wrap) and this plan's own operator pass, vs also editing the Andromeda skill's `plan-template.md` (outside this repo) → blocks: plan-decision.
