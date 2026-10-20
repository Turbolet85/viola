proposals:
  - detector: D-tests-framework
    severity: warning
    section: "§3 → Bootstrap phases (derive for route / setup-project)"
    change: "test-runner-install's `[profile.mutants]` item: four `[[profile.mutants.overrides]]`, the first matching winning, in the order `verify_window_` (slow-timeout period 20s, terminate-after 3: a 60 s kill, the class's `ci` kill, still first) · `package(viola-e2e)` · `send_window_` (15s, terminate-after 2) · `test(/send_under_the_paste_hint_/)` (15s, terminate-after 2: a 30 s kill, last, pinned by `mutants_paste_hint_override_holds_the_class_above_the_default_kill` in `tests/contract_lints.rs`); the no-screen root case's 34.3 s reading becomes 'below the 60 s kill', with the measured floor beside it (42.874 / 43.078 / 43.714 s on the CI legs, ci#39383206773, and 42.952 s on the dev host), the paste-hint pair's floor 8.7–9.9 s on three OSes under `ci` against the 10 s default kill, and the planted-hang pairs of chunk 2026-10-20-epoch-5-cleanup (TIMEOUT 30.003 s / PASS 48.413 s; TIMEOUT 60.003 s / PASS 112.961 s)"
    sidecar: "2026-10-20 (chunk 2026-10-20-epoch-5-cleanup): [profile.mutants] 'plus three overrides' with `verify_window_` at a 45 s kill superseded by four overrides, `verify_window_` at 60 s first and `send_under_the_paste_hint_` at 30 s last"
    rationale: "Report Changes → Schema / config (.config/nextest.toml [profile.mutants]: verify_window_ 15s×3 → 20s×3, a new fourth override send_under_the_paste_hint_ at 15s×2, four overrides) and Counts / qualifiers moved (F1-verify-kill, F2-paste-hint, F3-mutants-prof list this line as stale for both the 45 s kill and the override list); Outcome records the kill-line planted-hang pairs"
    basis: ".andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:11"
  - detector: D-tests-framework
    severity: warning
    section: "§3 → Bootstrap phases (derive for route / setup-project)"
    change: "the kill-line list in the `[profile.mutants]` fail-fast paragraph reads (10 s; the `send_under_the_paste_hint_` class 30 s; viola-e2e 30 s; the `verify_window_` class 60 s, one `viola-e2e` harness test in it since chunk 2026-10-09-epoch-3-cleanup)"
    sidecar: "2026-10-20 (chunk 2026-10-20-epoch-5-cleanup): mutants kill list '10 s; viola-e2e 30 s; the verify_window_ class 45 s' superseded by one adding the paste-hint class at 30 s and the verify_window_ class at 60 s"
    rationale: "Report Counts / qualifiers moved: F1 names `:12` («the `verify_window_` class 45 s») stale and F2 names `:12`'s kill list stale for the missing paste-hint class"
    basis: ".andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:12"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§3 → Bootstrap phases (derive for route / setup-project)"
    change: "the viola-e2e override sub-bullet: `verify_window_boot_with_an_unknown_cli_version_is_verify_failed` takes that class's 60 s kill through the override that stands first"
    sidecar: "2026-10-20 (chunk 2026-10-20-epoch-5-cleanup): 'takes that class's 45 s kill' superseded by 60 s"
    rationale: "Report Counts / qualifiers moved, F1: `:20` «takes that class's 45 s kill» is stale (mutants verify_window_ kill 45 s → 60 s)"
    basis: ".andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:20"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§3 → Bootstrap phases (derive for route / setup-project)"
    change: "the Run E sentence of the `[profile.ci]` item keeps its history and states the current kill: 'against the class's 60 s kill and the `mutants` profile's 45 s then (about 2 s of margin locally); chunk 2026-10-20-epoch-5-cleanup moved the `mutants` kill to 60 s, the class's `ci` kill, on the measured floor 42.874 / 43.078 / 43.714 s (CI legs, ci#39383206773) and 42.952 s (dev host)'"
    sidecar: "2026-10-20 (chunk 2026-10-20-epoch-5-cleanup): Run E's 'the mutants profile's 45 s' read as standing superseded; the mutants kill of the verify_window_ class is 60 s"
    rationale: "Report Counts / qualifiers moved, F1: `:10 @c3888` («against the class's 60 s kill and the `mutants` profile's 45 s (about 2 s of margin locally)») is listed stale"
    basis: ".andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§3 → Bootstrap phases (derive for route / setup-project)"
    change: "the Dev-deps bullet 'in `viola-e2e`' also lists `libc` (the workspace pin `=0.2.189`) under `[target.'cfg(unix)'.dependencies]` since chunk 2026-10-20-epoch-5-cleanup, read by the keeper's backing free-space check (`statvfs`)"
    sidecar: "2026-10-20 (chunk 2026-10-20-epoch-5-cleanup): viola-e2e's dependency list (rmcp, tokio, syn, proc-macro2) superseded by one adding the cfg(unix) libc edge"
    rationale: "Report Changes → Dependencies: `viola-e2e` gains `libc.workspace = true` under `[target.'cfg(unix)'.dependencies]`; the report's F9 disposition read only architecture's crate-dependency-direction key, while this key file's Dev-deps bullet lists viola-e2e's dependencies"
    basis: "crates/viola-e2e/Cargo.toml:38-41"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Test data bootstrap"
    change: "Mechanism's `prepare_home_base` bullet: on Unix a linked base, after its three checks (absolute target; made 0700 when gone; one `lstat` reads a real, owner-only directory), is also refused when the backing's free space (`statvfs`, `f_bavail × f_frsize`) is below `HEADROOM_FLOOR_MIB` = 9 GiB, panicking with `headroom_refusal`'s fixed two-count message `e2e-home backing: {free} MiB free, below the {floor} MiB floor` (no path); the floor's basis is the EDQUOT reading of about 8 GiB free plus 1 GiB (`evidence/headroom.md` of chunk 2026-10-20-epoch-5-cleanup); the harness keeper `Workspace::ensure_e2e_home` holds the same floor and message; no env var, flag or config sets it"
    sidecar: "2026-10-20 (chunk 2026-10-20-epoch-5-cleanup): the keepers' three checks superseded by four, the 9 GiB backing headroom floor added"
    rationale: "Report Symbols / APIs (tests/support/home.rs HEADROOM_FLOOR_MIB, headroom_refusal, free_bytes; prepare_home_base's link branch panics after its three existing checks), Counts / qualifiers moved F8-headroom (test-data-bootstrap.md:13 stale-incomplete), Expected amendments (§3 Test data bootstrap (Mechanism), the floor with its basis)"
    basis: "tests/support/home.rs:154-159"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → 5-command implementation"
    change: "`boot` step 2's `Workspace::ensure_e2e_home` sentence: on Unix a linked base accepted by its `lstat` check is also refused when the backing's free space is below the 9 GiB `HEADROOM_FLOOR_MIB` (an `io::Error` of kind `InvalidInput` carrying `e2e-home backing: {free} MiB free, below the {floor} MiB floor`, two counts, no path); `boot` reads it as its existing `reason:build-failed` and `run --local-live` as `verify-exit-none`, no reason added"
    sidecar: "2026-10-20 (chunk 2026-10-20-epoch-5-cleanup): ensure_e2e_home's three checks superseded by four, the headroom floor read as build-failed / verify-exit-none"
    rationale: "Report Symbols / APIs (crates/viola-e2e/src/harness/mod.rs: the link branch returns io::Error InvalidInput below the floor; boot reads build-failed, run --local-live verify-exit-none; no reason added) and Counts / qualifiers moved F8-headroom naming 5-command-implementation.md:6 stale-incomplete"
    basis: "crates/viola-e2e/src/harness/mod.rs:120-122"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → 5-command implementation"
    change: "the `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` sentence: the same tree in a plain sibling explicitly granted `BUILTIN\\Users` write (a direct spawn of `%SystemRoot%\\System32\\icacls.exe <plain> /grant *S-1-5-32-545:(OI)(CI)(W)`, its exit asserted 0, since chunk 2026-10-20-epoch-5-cleanup) reads `Writable`; green on windows-2025 at ci#39400402749, the mutation-copy reading owed to the next epoch boundary's Windows leg"
    sidecar: "2026-10-20 (chunk 2026-10-20-epoch-5-cleanup): 'a plain-created sibling reads Writable' superseded by a sibling granted BUILTIN\\Users write explicitly"
    rationale: "Report Symbols / APIs (scratch.rs: the cfg(windows) case's plain sibling is granted BUILTIN\\Users write by icacls, exit asserted 0) and Counts / qualifiers moved F11-scratch-win naming `5-command-implementation.md:35 @c3573` stale; Outcome: 0.061 s on windows-2025, ci#39400402749"
    basis: ".andromeda/registries/contracts/test-plan/5-command-implementation.md:35"
  - detector: D-tests-coverage
    severity: warning
    section: "§7 Test Data & Fixtures"
    change: "the fake agent's exit bullet: before it exits it still waits for a hook still running and starts no other, but on Unix every command `run_receipted` spawns (each hook and the status line command) runs in its own process group (`process_group(0)`, as `hold_inherited_stdout` already did) since chunk 2026-10-20-epoch-5-cleanup, so the terminal hangup at the agent's exit as session leader no longer reaches it (measured: 17 refused of 726 profiles → 0 of 800 over 200 runs of `chaos_wait_parked_wrapper_killed_exits_21`, `evidence/profraw-cause.md`); the earlier foreground-group reading stays as history"
    sidecar: "2026-10-20 (chunk 2026-10-20-epoch-5-cleanup): 'a hook is in its PTY's foreground process group' superseded: on Unix the fake agent's hooks run in their own process group"
    rationale: "Report Symbols / APIs (src/bin/viola-fake-agent.rs run_receipted takes process_group(0) on Unix) and Counts / qualifiers moved F7-hook-group: test-plan.md `:963` stale, the wait for a running hook still stands; Decisions & corrections records 17/726 → 0/800"
    basis: ".andromeda/test-plan.md:963"
  - detector: D-tests-coverage
    severity: warning
    section: "§10 Quality Gates & Coverage Targets (Stack adjustments)"
    change: "the short-profile paragraph names a second measured mechanism: after a test the terminal hangup kills the wrapper and reached hooks the fake agent ran in the PTY's foreground process group, cutting their exit-time profile write (17 refused of 726 profiles → 0 of 800 after `run_receipted`'s own process group, chunk 2026-10-20-epoch-5-cleanup, `evidence/profraw-cause.md`); and the witness gains its second form `scripts/profraw-census.sh --case <binary stem> <test name> <runs> <workers> [<name>]` (profiles `r<i>-%p-%<nproc>m.profraw`, each offered alone to the toolchain's `llvm-profdata merge --sparse -o /dev/null`, refused files kept under `refused/`, last line `profraw-census: runs N · passed N · profiles P · refused R`, exit 0 only when every run passed and R is 0, else 1, 2 on usage or with no instrumented binary or no `llvm-profdata`); both forms stay a dev-host script with no CI job, `pre-push` stage or harness command, no unit test, only recorded one-shot controls"
    sidecar: "2026-10-20 (chunk 2026-10-20-epoch-5-cleanup): the one-mechanism, first-form-only census statement superseded: the hook process-group hangup named and the census's --case form added"
    rationale: "Report Symbols / APIs (scripts/profraw-census.sh second form; run_receipted process group), Counts / qualifiers moved F6-profraw / F13-census (test-plan.md `:1092` names one mechanism and the census's first form only), Expected amendments (test-plan §10 Stack adjustments, item (8)'s finding); Coverage of new surfaces: the --case form has no unit test, as §10 says of the script"
    basis: "scripts/profraw-census.sh:30-37"
  - detector: D-retired
    severity: warning
    section: "§6 E2E Test Strategy → Scenario: E4 — SSE `Last-Event-ID` resume"
    change: "the E4 heading no longer names a standing SSE `Last-Event-ID` resume scenario over its one-line deferral stub: retitle it as a deferral (e.g. 'E4 — frontend (deferred to 0.2.0)') keeping the stub «The frontend is deferred to 0.2.0; its technology is not chosen.» and the E-numbering"
    sidecar: "2026-10-20 (chunk 2026-10-20-epoch-5-cleanup): the E4 'SSE Last-Event-ID resume' scenario heading retired as a web leftover, as obs-plan §4's GUI view heading"
    rationale: "Report Retired: obs-plan §4's scenario heading for the live SSE tape with `Last-Event-ID` resume, standing over a one-line deferral stub, is named the standing web leftover (item (10), the restructure's web leftovers); test-plan's E4 heading restates the same SSE `Last-Event-ID` resume scenario over the same stub"
note: "D-tests-obs-harness: no drift — the report states no harness verb, status shape or log format changed (the keeper's refusal reads the existing build-failed / verify-exit-none), so §3 and obs-plan §3 still agree. Holds per the report: test-plan.md :562, :591, :624, :652, :671, :772, :893, :962, :1095 and the key file's other `:10` offsets. The E4 proposal is a reading of the Retired bullet (it names obs-plan's heading, not test-plan's) — the orchestrator may drop it if it scopes D-retired to the named passage only."
