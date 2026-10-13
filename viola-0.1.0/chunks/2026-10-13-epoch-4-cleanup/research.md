# Codebase Research — 2026-10-13-epoch-4-cleanup

## Scope
- **Depth:** deep on §2's mechanism and §3's two files; survey on §1's 47 sites (the split this research proposes, below, moves §1 to an entry of its own) · **Reads:** 24 · **Globs/Greps:** 21
- **Harness rules consulted:** none — no live leg in this chunk (no live `claude` start, `inputs#I1`; no mutation run, witness run or workflow dispatch, `inputs#I1`)
- **Platform issues consulted:** searched at P5 (validation check 9, the runner-only bullet of scope §2): "actions runner-images windows D:\a workspace drive permissions BUILTIN\Users write ACL inherited" and "github actions windows-2025 runner D: drive folder permissions Users create files write data" — neither returned a runner-images issue or release note stating the workspace drive's ACL (the first returned only Ansible `win_acl` docs; the second's nearest hit, runner-images#8755, is about C: versus D: speed, not ACLs, and was not fetched). No fetched source contradicts or supplements the measurement on disk: ci#37214281447 read the `windows-2025` workspace drive's BUILTIN\Users `0x2`/`0x4` container-inherit ACEs (research.md §2). No instrumentation is planned
- **External inputs:** `inputs#I1` — the operator's take-up direction (size against one window at P5 and propose a split; no mutation run, witness run or workflow dispatch; a dispatch need is a P4 fork; no live `claude` start)

## Files inspected
- `src/cmd/hook/statusline.rs` (44–56, 316–345, 440–560) — `trusted` = `check_instance(&home, &dir).is_ok()`; the test helper `instance_in` (316) lays the home down with plain `fs::create_dir_all(root/vhome/instances/builder)`; `statusline_an_instance_viola_laid_down_passes_its_check` (≈506) then calls `create_private_dir(&instance.dir)` on a dir that already exists
- `crates/viola-state/src/fs.rs` (35–140, 240–300, 380–490) — `create_private_dir` collects only the MISSING ancestors and protects the topmost one it created (`win::protect_outside_profile`, the protected `D:P(A;OICI;FA;;;<user>)(A;OICI;FA;;;SY)` when outside the profile); `replace_private_with`'s retry loop and `retry_replace(raw, HOST_IS_WINDOWS)`
- `crates/viola-state/src/strict.rs` (39–90, 380–470, 575–600) — `check_instance` / `check_path`; the Windows `reading_of` dump; `check_stamps_of_a_home_under_the_workspace_target_passes` lays a home under `target/e2e-home` (the precedent for a Windows DACL case homed in the checkout)
- `crates/viola-e2e/src/harness/run/mutants/scratch.rs` (full) — `prepare` / `wipe`; `wipe` re-creates the scratch with plain `fs::create_dir_all`; the scratch tests (Linux `prepare` → `None` arm; Windows `prepare_refuses_a_repo_whose_scratch_is_an_ancestor`, `prepare_names_a_wipe_that_failed`)
- `crates/viola-e2e/src/harness/run/mutants.rs` (224–265) — `TMP` / `TEMP` set to the scratch for cargo-mutants
- `crates/viola-e2e/Cargo.toml` — no `viola-state` dependency today
- `crates/viola-e2e/src/harness/run/coverage.rs` (20–30) — `cargo llvm-cov nextest --workspace`: CI's `test (windows-2025)` runs viola-e2e's own unit tests
- `.github/workflows/windows-mutants.yml` (full) — nine matrix items; the root package's four are `src/cmd/run.rs`, `src/main.rs src/conpty.rs`, `src/run/env.rs`, `src/panic_frames.rs`
- `.github/workflows/ci.yml` (20–200) — the per-OS `test` job runs `agent-run run --coverage`
- `.config/nextest.toml` (full) — `[profile.mutants]` `fail-fast = { max-fail = 1, … }`; every override filter is a name regex or a binary/package (`test(/send_window_/)`, `test(/verify_window_/)`), none a module path
- `src/run/send.rs` (1–565 whole outline; tests 566–2415 by name) — production 565 lines, tests 1850; 52 test attributes
- `src/human.rs` (1–466 whole outline; tests 467–1241) — production 466 lines, tests 775; 42 test attributes
- `src/main.rs` (4, 7), `src/run/mod.rs` (9) — `mod human;`, `pub(crate) mod send;`
- `crates/viola-pty/src/lib.rs` (196–206, 895–1000, 1183–1275) — `PortablePty::resize` and the three resize tests (§1, surveyed)
- `src/cmd/verify/typed.rs` (255–310, 395–445, 640–680) — the 18 `typed.rs` sites (§1, surveyed)
- `viola-0.1.0/chunks/2026-10-04-dialog-answers-by-dialog-id/evidence/operator-pass.md` (28–62) — the measured runner DACL
- `viola-0.1.0/chunks/2026-10-10-budget-governor/evidence/operator-pass.md` (55–58), `viola-0.1.0/chunks/2026-10-11-the-board-viola-list/evidence/gates.md` (34–40) — the recorded cross-target clippy command and its catch
- `.andromeda/runs/2026-10-12T06-54-29-code-audit/{proposals.md, record.json, c-windows-readback.json, c-mutation-viola-pty.json}` — the survivor list, the Windows readback

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-13T07-07-28-phase/tree-query-2026-10-13-epoch-4-cleanup.json`, `db_state fresh`)
- **`src/human.rs`'s items** — 38 reference rows from 14 other files (`src/cmd/{answer,client,last,list,pause,release,revive,run,send,verify,wait}.rs`, `src/main.rs`): `refuse` (run 5, verify 4, list 2, revive 1), `result` (verify 2, revive 1), `internal_error` (main), the send mirror writers and `send_hint` (cmd/send, cmd/client), the board (`write_board`, `BayBudget`, the two state pairs: cmd/list), wait/last writers, `write_answered` / `answer_hint`, `write_paused`, `write_released` / `write_budget_*` / `override_end`, `escape_message` (revive). A split that re-exports every one under `human::` leaves all 14 caller files unedited.
- **`src/run/send.rs`'s items** — 10 reference rows from 4 other files: `SendSlot` (+ `new`, `attach`, `budget_with`, `links_with`, `send`, `append_hook_event`) from `src/cmd/run.rs`; `parse_from` from `src/run/{dialog,wait,wheel}.rs`. A split that keeps these names at `run::send::` leaves all four unedited.
- **`wipe` / `prepare`** (scratch.rs) — one production caller each (`prepare` ← `mutants.rs:224`; `wipe` ← `prepare`); the change is internal to the file.

## Patterns detected
- **The Windows home lay-down outside the profile** (`crates/viola-state/src/fs.rs:35–60`, `:131–137`): viola protects the topmost folder it CREATES outside the profile; a folder that already exists, or one created by plain `create_dir_all`, keeps its inheritance.
- **The measured runner drive** (chunk 2026-10-04-dialog-answers-by-dialog-id, ci#37214281447, `test (windows-2025)`): a folder created under the checkout inherits BUILTIN\Users `0x2` (write data) and `0x4` (append data), container-inherit, and the strict check refuses it `Err(Writable)` (owner Administrators, trusted; Users read-execute without write; the two write ACEs untrusted).
- **A Windows DACL case homed in the checkout** (`crates/viola-state/src/strict.rs:443–455`): `CARGO_MANIFEST_DIR/../../target/e2e-home`, `tempdir_in`, the reading dumped by SIDs and codes only.
- **The cross-target lint, as run by hand** (budget-governor operator pass, 22:18:30Z; board-viola-list gates firing 3): `CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features fake-agent --target x86_64-pc-windows-msvc -- -D warnings` — check only, nothing linked; exit 0 in 0.12 s warm; it caught a `cfg(unix)`-only constant as Windows dead code (exit 101) before a push.
- **One concern per file under `src/run/`** (`gate.rs`, `wait.rs`, `dialog.rs`, `wheel.rs`, `snapshot.rs`, `budget.rs`, `links.rs`) — the shape the send split takes.

## Conventions to follow
- **Tests inline at the bottom of their source file** (`#[cfg(test)] mod tests`, test-plan §2 naming `<subject>_<condition>_<expected>`): a moved function's tests move with it, names unchanged.
- **Every human writer under `human`**, one `write_all` per refusal pair, no print macro, no `#[allow]` (`src/human.rs:15–45`; `grep -cE 'print!|println!|eprint|#\[allow' src/human.rs` reads 0 at HEAD).
- **Imports used only inside `#[cfg(windows)]` test code go inside that item** (`.claude/rules/testing.md`, 2026-10-03), so the Linux and the Windows lint both read clean.
- **A Windows DACL test homes under `target/e2e-home`, never `%TEMP%`** (test-plan, the 2026-10-04 security sweep).

## Measured facts

### §2 — the Windows baseline red, closed against run 38215104120
- Re-read at P3 (`gh run view 38215104120 --log-failed`, shard `viola-run-env`): `FAILED Unmutated baseline in 82s build + 137s test`; `FAIL [0.013s] (422/1224) viola::bin/viola cmd::hook::statusline::tests::statusline_an_instance_viola_laid_down_passes_its_check`, `panicked at src\cmd\hook\statusline.rs:510:9`; `Summary [136.901s] 425/1224 tests run: 424 passed (5 slow), 1 failed, 0 skipped`. The build dir is `D:\a\viola\viola-mutants-scratch\cargo-mutants-viola-2CPhiJ.tmp\`.
- **The baseline stopped at the first failure** (`[profile.mutants]` `fail-fast = { max-fail = 1 }`): 799 of the root package's 1224 tests never ran in the scratch copy. "Only this test fails there" is not established by the run.
- **The mechanism, as an equality.** For the statusline test's inputs: the home `<tempdir>/vhome` and `instances/` are made by plain `create_dir_all`, and `create_private_dir(&instance.dir)` finds nothing missing, so it protects nothing; every folder of the instance tree carries the tempdir's inheritance. Under CI's checkout run, the tempdir is under `%TEMP%` in the profile (the profile's inheritance: user, SYSTEM, Administrators full, Users read-execute — accepted, `strict.rs` `windows_verdict_takes_the_profile_inheritance`). Under the mutants run, `TMP`/`TEMP` point at `D:\a\viola\viola-mutants-scratch\`, which `wipe` re-creates with plain `create_dir_all` on the runner's workspace drive, so the tempdir inherits BUILTIN\Users `0x2`/`0x4` container-inherit (measured, ci#37214281447) and `check_instance` refuses the home `Err(Writable)`. The check behaves as security-plan writes it; what differs is the scratch's DACL against `%TEMP%`'s.
- **The cause-level fix site.** `wipe` (scratch.rs) re-creates the scratch; created through `viola_state::fs::create_private_dir`, the scratch is the topmost created folder outside the profile and takes the protected user + SYSTEM `OICI` DACL, so every tempdir cargo-mutants' tests lay inside it inherits an owner-only DACL, as one under `%TEMP%` does. This restores the property for the whole suite (the 799 unrun included), where fixing the one test would not. The protected scratch grants user + SYSTEM only, `%TEMP%` also Administrators (trusted) and Users read-execute: a subset of trusted grants, so a lay-down `%TEMP%` passes passes here too; a test that asserts a REFUSAL from `%TEMP%`'s Users read-execute ACE would read differently (none found: the Windows refusal cases build their own DACL, `strict.rs` `everyone_may_write`).
- **The witness without a dispatch.** CI's `test (windows-2025)` runs viola-e2e's unit tests (`coverage.rs:25–27`, `--workspace`). A Windows-only scratch.rs test can lay a scratch down through `wipe` under `target/e2e-home` (the runner's workspace drive) and lay the statusline test's tree inside it — accepted by `check_instance` — beside the same tree laid in a plain-created sibling, refused (the measured drive). The refused control is what keeps the accepted reading from passing vacuously. Its named limit: a Windows checkout under the profile reads the control accepted.
- No `viola-state` dependency in viola-e2e today (`crates/viola-e2e/Cargo.toml`); the root `[workspace.dependencies]` names `viola-state` by path (`Cargo.toml:135`).

### §3 — the two files
- Neither file holds `cfg(windows)` / `cfg(unix)` code (`grep -c` reads 0 in each), so neither is a `windows-mutants.yml` `--file` subject and `tests/contract_windows_mutation_scope.rs` is unaffected.
- No code, script, test or workflow pins either file by path (`grep -rn -e 'human\.rs' -e 'run/send\.rs'` over `scripts tests src crates .github .config`: 0 hits); the docs and masters that name them (`.claude/docs/services/viola.md`, the masters' tree entries and test-plan §4's file citations) are the wrap's reconcile.
- nextest overrides key on name regexes (`send_window_`, `verify_window_`) and binaries; a moved test keeps its name, so none moves out of its override.
- `src/run/send.rs` phases (production 1–565): the classification (`Match`, `Confirms`, `classify` 44–73), the in-flight slot (`InFlight`, `Flight`, `SendSlot` and its `claim` / `session_started` / `settle` / `confirm`, `append_hook_event`, `close`, `window_ms`, `Reserved` 74–289), the params (`parse`, `parse_from` 300–325), the send and its context (`Ctx`, `send`, the refusals, `issue`, `confirm`, `unconfirmable`, `log_confirmed`, `budget_paused` 290–565). Tests (566–2415): shared fixtures 636–1023 (`quiet_gate`, `verified_gate`, `poisoned_gate`, `call`, `events`, `kinds`, `prompt`, `feed`, `occupy`, `paused`, `send_refusal_order`, `confirmed_with`, `confirmed_as`) used across the phases' cases, then cases grouped by the match/relabel, the text refusals, the slot, the turn rung, the local command, the gate wait, the budget gate and the link.
- `src/human.rs` phases (production 1–466): the shared writers (`write_refusal`, `refuse`, `write_internal_error`, `internal_error`, `write_result`, `result` 15–45), the send mirror (`MIRROR_WORD`, `mirror_line`, `write_send_open`, `write_read_back`, `write_send_unconfirmable`, `write_send_unable`, `write_wrapper_fault`, `write_unable` 47–113), the two escapers (`escape_message`, `escape_row` 114–141), the board (`instance_state_pair`, `home_state_pair`, `BayBudget`, the cells, `bay_line`, `board_rows`, `write_board`, `EMPTY_BOARD` 142–317), wait and last (318–369), the verb lines (`write_answered`, `write_paused`, `write_released`, `override_end`, `write_budget_lifted`, `write_budget_open` 370–417), the hints (`HUMAN_TYPING_HINT`, `answer_hint`, `send_hint` 418–466).

### §4 — the cross-target lint
- `rustup target list --installed` reads `x86_64-pc-windows-msvc` and `x86_64-unknown-linux-gnu`; `rust-toolchain.toml` carries no `targets` key, so the Windows std is host preparation (installed, never by a gate).
- The command and its separate target dir are the recorded form above; its first firing in this chunk is a P5 baseline run (a `new` entry).

### §1 — the survivors, surveyed (the proposed split moves this section to an entry of its own)
- `crates/viola-state/src/fs.rs` `replace_private_with` (`:281`, `:285` ×4, `:286`): with `HOST_IS_WINDOWS = cfg!(windows)` false, `retry_replace` is false on Linux, so the loop never retries and each mutant behaves as the original on this host; the retry decision is a const, so the harness never leaves these out (test-plan §3 `run` step 4). A Linux kill needs the decision supplied by the caller (a restatement); otherwise they are reachable only on Windows.
- `crates/viola-e2e/src/harness/run/mutants/scratch.rs` `prepare` (`:48`, `:54`): `HOST_SCRATCH` false returns before both sites on Linux; test-plan §10 already records the pair "not measured here; owed to" a route entry (chunk 2026-10-09-epoch-3-cleanup-ii), and run 38036448183 read them caught on `windows-2025`. §2's edit of `wipe` moves no line of `prepare`; a change to `prepare` would move both coordinates.
- `crates/viola-pty/src/lib.rs` `PortablePty::resize` (`:202`): three Linux-compiled tests resize and wait for the child to report `size 120x40` (`lib.rs:1196`, `:1233`, `:1255`, `wait_line` panics after `CHILD_WITHIN` 7 s); why they read the mutant missed is not known (the unit's baseline test phase read 1.7 s, `c-mutation-viola-pty.json`). Measured by hand-applying the mutant, at the survivors' own take-up.
- `src/cmd/verify/typed.rs` (18): `framing_turns`'s two waits (`count(..) > prompts && count(..) > stops`, `> starts || > prompts`), `max_gap`'s window filter, `SharedPty`'s three forwarders (`resize`, `close`, `child_pid` to the inner PTY), `Feed::take`'s `total > MAX_FRAME`, `end_by_ctrl_c`'s two deadline sums, `wait_pump`, `finish`'s deadline sum.
- The remaining sites (`src/run/wheel.rs` 9, `src/run/dialog.rs` 2, `src/cmd/hook.rs` 1, `src/cmd/send.rs` 1, `src/run/send.rs` 1, `crates/viola-state/src/board.rs` 4, `crates/viola-agent-claude/src/statusline.rs` 2) were not read past their coordinates.
- The disposal forms test-plan holds (tests extract; test-plan §3 `run` step 4, §10): a nextest test in the owning package that fails under the hand-applied mutant; a behaviour-preserving restatement removing the site; for a site this host cannot reach, the record "not measured here; owed to {route entry}" by coordinate. There is no "accepted equivalent" form for a reachable site.

## The size against one window (`inputs#I1`), and the split this research proposes
- The precedent: the Epoch 3 cleanup entry was split in two on the founder's word (`2026-10-09-epoch-3-cleanup` scope, P4 split); its second half — 18 survivors, the root scaffolding shared once, three functions split, the first whole-unit viola-e2e score — was one builder window (`2026-10-09-epoch-3-cleanup-ii`).
- This entry: 47 survivors in 10 files across five packages (two clusters needing a cause read first: `PortablePty::resize`, the `fs.rs` loop), plus two splits that move ~1000 production and ~2600 test lines, plus §2 and §4. Read against the precedent, it does not fit one window.
- The cut research reads as natural: **§2 + §3 + §4 here** (the Windows baseline's cause removed, the two files split by phase, the Windows-target lint as a gate entry — mechanical work with known equalities), and **§1 as its own entry right after this one** at the head of Epoch 5. In that order the survivors' new tests land in the split files (no test written into `src/run/send.rs` and then moved), the Windows-target lint guards their `cfg` code, and §2's `wipe` edit lands before the survivors' entry reads `scratch.rs`.
- This is a proposal for the P5 card (`inputs#I1`); nothing leaves `scope.md` before the word.

## New files to create
- `src/run/send/slot.rs` — the in-flight slot (`InFlight`, `Flight`, `SendSlot` with `claim` / `session_started` / `settle` / `confirm`, `append_hook_event`, `close`, `window_ms`, `Reserved`) and its tests
- `src/run/send/classify.rs` — the local-command decision (`Match`, `Confirms`, `classify`) and its tests
- `src/run/send/params.rs` — `parse` and `parse_from` and their tests
- `src/run/send/budget.rs` — `Ctx::budget_paused` and the budget-gate cases (added at P4, so no split file carries the budget cases' ≈290 test lines beside the root's)
- `src/run/send/test_support.rs` — the fixtures the phase tests share (`#[cfg(test)]`)
- `src/human/mirror.rs` — the send mirror writers and their tests
- `src/human/escape.rs` — `escape_message` and `escape_row` and their tests
- `src/human/board.rs` — the bay line, the board rows and writer, the two state pairs, and their tests
- `src/human/wait.rs` — the wait and last writers and their tests
- `src/human/verbs.rs` — the answer, pause, release and budget lines and their tests
- `src/human/hints.rs` — `HUMAN_TYPING_HINT`, `answer_hint`, `send_hint` and their tests

## Files to modify
- `src/run/send.rs` — the module root: the send and its context (`Ctx`, `send`, the refusals), the `mod` declarations, and the re-exports that keep `run::send::{SendSlot, PasteFn, append_hook_event, parse_from, send}` for their callers
- `src/human.rs` — the module root: the shared writers (`write_refusal`, `refuse`, `write_internal_error`, `internal_error`, `write_result`, `result`), the `mod` declarations, and the re-exports that keep every `human::` name its 14 caller files use
- `crates/viola-e2e/src/harness/run/mutants/scratch.rs` — `wipe` re-creates the scratch through `viola_state::fs::create_private_dir`; a Windows-only test of the lay-down inside a wiped scratch against a plain-created sibling
- `crates/viola-e2e/Cargo.toml` — the `viola-state` path dependency
- `Cargo.lock` — viola-e2e's dependency list gains `viola-state`

## Sweeps
- `human\.rs` · `run/send\.rs` over `scripts tests src crates .github .config` (`grep -rn -e 'human\.rs' -e 'run/send\.rs'`): 0 hits · 0 changed · 0 no-change.
- the same two patterns over `.claude/docs .claude/rules CLAUDE.md` (`grep -rln`): 1 hit (`.claude/docs/services/viola.md`) · 0 changed · 1 no-change (a Tier-3 doc the wrap's cascade re-derives).
- `cfg(windows)\|cfg(unix)` over `src/run/send.rs src/human.rs` (`grep -c`): 0 · 0 · 0.
- `filter` over `.config/nextest.toml`: 6 hits · 0 changed · 6 no-change (name regexes, binaries, a package; no module path).
- `run::send::\|human::tests\|send::tests` over `.config tests scripts crates/viola-e2e/src`: 0 hits.

## Open questions
- none — §3's phase boundaries above are research's reading; a file implement creates outside the two lists is recorded in `scope-record.md` (gate-contract §Scope).
