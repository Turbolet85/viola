# Report — 2026-10-07-test-homes-off-the-contended-volume

**Chunk:** Test homes off the contended volume — the dev host's test homes on a tmpfs backing, the path and CI's own
fsync unchanged, proved by a contended reading
**Date:** 2026-10-07T08:21Z
**Commits:** `230f5dc chore(2026-10-07-test-homes-off-the-contended-volume): operator pre-CI commit, for the run this
chunk's verdict reads` (the one commit since `b329083`, the last wrap's; basis `git log --format='%h %s'
b329083..HEAD`)

## Changes (structured — detectors read this)
- **Files:** six, all in research's lists (basis `git diff --name-only b329083`, the chunk folder, run dirs and
  bookkeeping aside):
  - `tests/support/home.rs` — the root keeper `prepare_home_base`, called from `TestHome::new`;
  - `tests/cli_instance_state.rs` — six `home_base_backing_` cases;
  - `crates/viola-e2e/src/harness/mod.rs` — the harness keeper `Workspace::ensure_e2e_home`, six cases;
  - `crates/viola-e2e/src/harness/boot.rs` — `start` calls the keeper in place of `fs::create_dir_all`;
  - `crates/viola-e2e/src/harness/run.rs` — `local_live` calls the keeper after the build, one case;
  - `scripts/g2-zero-panics.sh` — `count_panics`' two `find` start points read `"$scope/"`.
- **Symbols / APIs:** test-side and harness-side only, no product surface.
  - `pub fn prepare_home_base(base: &Path)` (`tests/support/home.rs`): one caller, `TestHome::new`; the six cases
    call it directly.
  - `pub fn Workspace::ensure_e2e_home(&self) -> io::Result<()>` (`viola-e2e`, the harness crate): two callers,
    `boot::start` and `run::local_live`. The readers and the remover (`schema_check`, `secret_scan::of`,
    `cleanup::remove_session_home`) do not call it and did not change.
  - The two keepers hold one contract. A plain base (absent, or a directory) is `create_dir_all`, as before. On Unix
    a base that is a link: its target must be absolute; a non-recursive `DirBuilder` with mode 0700 is tried on the
    target (an existing one answers already-exists and is left as it is; never a chmod after, never a parent); then
    one `lstat` of the target is the verdict: a real directory with no group or other bit, else refused. Elsewhere
    (Windows) the function is the plain `create_dir_all`.
  - A refusal: the root keeper panics with a fixed message naming the failed check and no path; the harness keeper
    returns an `io::Error` (`InvalidInput`) with the same fixed messages. Three messages, the same in both: the
    link's target is relative · is not a real directory (a target that is itself a link, or not a directory, by the
    one `lstat`) · is not owner-only.
  - A refused base reads `build-failed` at `boot` (the path a failed `create_dir_all` took) and `verify-exit-none`
    at `run --local-live`, the verify not started. No failure reason or code was added to either closed list.
  - No IPC method, endpoint, event, socket, port or env var was added. No `viola` build reads a new variable, no flag
    was added, and no harness variable either: the backing is a property of the path.
- **Crates / modules:** none added or removed. `viola-e2e` changed (three files); the root package's test support
  and one root test file changed. No product crate changed: `git diff --quiet b329083e0266 -- src
  crates/viola-core crates/viola-pty crates/viola-channel crates/viola-state crates/viola-agent-claude
  crates/viola-mcp crates/viola-ui … Cargo.toml Cargo.lock` exits 0 (the preservation guard, a gate entry).
- **Dependencies:** none. `Cargo.toml` and `Cargo.lock` are unchanged (the same guard); the harness keeper uses
  std's Unix extensions, which `secret_scan.rs` already used.
- **Schema / config:** none. No config key, no schema, no redaction shape.
- **Spec-master edits:** none before this wrap.
- **Counts / qualifiers moved:**
  - The Linux dev host's test count: `run --coverage` inside `pre-push` reads 1675 passed (1662 at the last chunk's
    operator pass); the difference is this chunk's 13 cases. CI's `test` legs on `230f5dc`: ubuntu-latest 1675,
    macos-latest 1671, windows-2025 1699 (basis: the three job logs of ci#37592258366, `evidence/operator-pass.md`).
    On windows-2025 two of the 13 exist (the plain-base case of each keeper; the rest are `#[cfg(unix)]`).
  - `nextest-unit` 1336 passed on the dev host (the gate block's `run --unit`).
  - No documented count of harness failure reasons, env vars, commands or gate steps moved.
- **Dev-tool versions:** none — no host tool installed, upgraded or read changed.
- **Harness / gate surface:**
  - `boot` step "choose the home": the base `target/e2e-home` is now prepared by `Workspace::ensure_e2e_home`
    before the `tempdir_in`; on a plain base the behaviour is the old `create_dir_all`.
  - `run --local-live`: the same call after the build arm and before the verify; a refusal is `verify-exit-none`.
  - G2 (`scripts/g2-zero-panics.sh`): the walk's start point is `"$scope/"` in both `find` calls, so a scope that is
    a link to a directory is descended. On a real directory the walk and its output are the same, and a missing
    scope still reads empty. `--probe` is unchanged, and `ci.yml`, which runs the script in its `test` and `perf` jobs,
    is unchanged (the guard exits 0 over `.github`).
  - The five agent commands, their documents, the status shape and the log format are unchanged. `pre-push`, its
    launcher and its stages are unchanged (the guard exits 0 over `pre_push.rs` and `pre_push/`).
- **The Linux dev host's backing (an environment fact this chunk set, outside harness code):**
  - `target/e2e-home` is a symbolic link to `/tmp/viola-e2e-home-<uid>` (the user's numeric id), a directory with
    mode 0700 owned by the user, on tmpfs (`findmnt -n -o FSTYPE -T target/e2e-home/` reads `tmpfs`; it read `btrfs`
    before). Set by the builder with `mv`, `mkdir -m 700` and `ln -s`, no root (`evidence/backing.md`).
  - The path string `<workspace>/target/e2e-home` is unchanged for every home class: `viola-test-*`,
    `viola-session-*`, `viola-live-<pid>`.
  - Through the link the root fixture chain and the harness create and delete outside the working directory: the
    owner sweep and a test home's drop (`tests/support/home.rs`), and harness `cleanup`'s removal of the
    `viola-session-*` dir. Each removes only dirs the test side itself created (by owner record, by its own
    `TempDir`, by the `viola-session-` name). This is a boundary widening, ratified by the founder's own live answer
    of 2026-10-07T07:25Z, shown to him as creating and deleting outside the working directory through the link,
    relayed by the overseer (inputs#I6); the overseer's disposition at this wrap names the same word as the
    ratification of the widening amendments.
  - `viola` itself never reads or writes the backing directory by that name: a product process gets a `--home`
    under the link and resolves nothing (`pin_exe` takes `std::path::absolute`; research).
  - No CI runner has a link, so CI's homes stay on the runner's disk and the pinned copy still ends in
    `sync_all()`. `replace_private` is unchanged.
  - The 13 entries that were in `target/e2e-home` (545M) are in `target/e2e-home.disk`, nothing deleted,
    `viola-test-WYNVH7` among them. Both paths are under the ignored `target/`.
  - What the link does not cover: after `cargo clean` the link is gone and the next start puts a plain directory
    on the shared volume with no signal; the gate entry `test -L target/e2e-home && findmnt …` reads it red, and the
    recipe is one `ln -s`. After a reboot the target is gone and the next start re-creates it (the keepers). A home
    kept for a red reading (`AGENT_RUN_KEEP_HOMES=1`, `AGENT_RUN_KEEP_FAILED=1`) lives in memory, does not survive
    a reboot, and `/tmp` ages out what is untouched for ten days.
- **Cross-project / external claims:**
  - CI: ci#37592258366 on `230f5dc77eae`, `verdict: green · checks 15/15 · wall 405 s` (`ci.py conclusion`,
    2026-10-07T08:12:49Z to 08:19:34Z). The sha is the record: this wrap's own commit adds to that tree.
  - The contended reading rests on another project's build on the same host (`other_builds` names `escher`, 129 and
    123 build processes alive at the two opens); nothing was started or changed in that tree.
  - `inputs.py verify` (this wrap, P1): `inputs: 6 entries — unchanged 4 · drifted 0 · vanished 0 · broken 0 ·
    altered 0 · unreachable 0 · n/a 2 · uncited 0 · unparsed 0`.
    - `I1 · message: the operator, the /andromeda-phase invocation, 2026-10-07T06:13Z · copy · n/a` (a message);
    - `I2 · ../additional/viola-overseer/d94-route-adaptation.md · copy · unchanged`;
    - `I3 · ../additional/andromedaV3:…/F167-host-contention-2026-10-07/plan.md · pointer @1ed4a6ab · unchanged`;
    - `I4 · …/start_probe.py · pointer @1ed4a6ab · unchanged` (the probe the driver ran by path; the sibling's HEAD
      has moved since, the file has not);
    - `I5 · …/window_watch.py · pointer @1ed4a6ab · unchanged`;
    - `I6 · message: the overseer, this session AskUserQuestion, 2026-10-07T07:27Z (question 1 carries the founder's
      live answer of 07:25Z as relayed) · copy · n/a`.
- **Reverted / negative API facts:** none shipped and removed. Step 3's and the third pair's neutralising edits were
  one-shot controls, restored and confirmed (`evidence/keeper-control.md`); no net change.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - obs-plan §9 CI Integration, Gate commands (`.andromeda/obs-plan.md:1033-1034`) writes G2's walk as
    `find target/e2e-home -path …`. Over a linked scope that start point finds nothing: measured in step 7 with a
    session booted, the old start point printed 0 files and `find target/e2e-home/ …` printed 3
    (`evidence/smoke-on-backing.md`). The script now reads `"$scope/"`; the master's two command lines still show
    the old start point.
  - The plan's own step 5 check (`pgrep -x viola` empty) could not read empty on this host: 9 processes of that
    name are another tree's `viola` build, the bridge the overseer pair runs on. Read by executable path instead
    (this repository 0). A plan-text fact, in no master.
  - The plan's step 3 names a call-site swap its cases cannot reach; see Deviations. A plan-text fact, in no master.
- **Expected amendments (from plan):** six entries, plus two notes that are not master amendments. Search basis for
  the sites: `grep -c -F 'e2e-home'` over the seven masters reads architecture 3 · security-plan 0 · design-system
  0 · layout-templates 0 · test-plan 8 · obs-plan 9 · a11y-plan 0; over `.andromeda/registries/**` it reads
  `contracts/architecture/ci-cd-approach.md` 1 · `contracts/architecture/project-directory-structure.md` 1 ·
  `contracts/test-plan/test-data-bootstrap.md` 1 · `contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md`
  1 · `contracts/test-plan/5-command-implementation.md` 8.
  1. architecture §Occupied Resources → Repository (the `target/e2e-home/…` entry) — **carried**: the "Linux dev
     host's backing" and "Symbols / APIs" bullets above (the link, the two keepers, creation and removal through
     the link, the ratification). Site: `.andromeda/architecture.md:423` (1 hit of the entry itself; the other two
     architecture hits are `:384` the keep variable and `:408` the chaos home, both unchanged facts).
  2. architecture §Occupied Resources → Filesystem (test-side sites outside the repository) — **carried**: the
     backing directory `/tmp/viola-e2e-home-<uid>`, its writers (the builder's `mkdir`, then the keepers), and that
     `viola` never reads or writes it by that name. Site: the test-only bullets beside `.andromeda/architecture.md:408`
     (`grep -n 'Test-only, outside the viola home'`: 1 hit).
  3. security-plan §Security Anti-Patterns → Code Patterns — **carried**: the harness's home removals reach the
     link's target on the Linux dev host (the backing bullet). Site: `.andromeda/security-plan.md:599`, the one
     sanctioned wipe outside the workspace (`grep -n 'viola-mutants-scratch' .andromeda/security-plan.md`: 1 hit;
     `e2e-home`: 0 hits in this master, so the amendment adds the fact, it corrects no existing sentence).
  4. test-plan §5 Setup / teardown lifecycle and §3 Test data bootstrap (Cleanup) — **carried**: the dev-host
     backing as an operator-set environment fact, the keeper, the path statement holding by path, a kept home's
     lifetime (the backing bullet). Sites: `.andromeda/test-plan.md:626` (§5) and the key file
     `registries/contracts/test-plan/test-data-bootstrap.md:16` (Cleanup); `5-command-implementation.md:6` states
     `boot`'s home choice.
  5. obs-plan §3 Log file location — **carried**: on the dev host a kept home's diagnostics end at a reboot. Site:
     the key file `registries/contracts/obs-plan/log-file-location.md` (`grep -rn -i 'log file location'
     .andromeda/registries`: the key and its file; `e2e-home`: 0 hits in that key file, so the fact is added).
  6. obs-plan §9 CI Integration (Gate commands, G2) — **carried**: the walk's start point as landed (Harness / gate
     surface, and Spec claims disproved). Site: `.andromeda/obs-plan.md:1033-1034` (2 hits of `find target/e2e-home`).
  - For the wrap's docs pass, not a master (the plan's note): `.claude/docs/commands.md:75` and `:91` and the Test
    data bullet of `.claude/rules/testing.md:34` (`grep -rn -F 'e2e-home' CLAUDE.md .claude/rules .claude/docs`: 7
    hits in 4 files, the other four being `verification-harness.md:28`, `:43`, `commands.md:70` and
    `tests-summary.md:20`).
  - `.andromeda/architecture-amendments.md` stands at 119 962 B, 38 B under its 120 000 B whole-read bound (`wc
    -c`): this chunk's first architecture entry crosses it (the plan's note; a wrap mechanics fact).
- **Coverage of new surfaces** (no external surface, hot-path operation or UI element is new; the two keepers are
  test-side filesystem preparation):
  - `prepare_home_base` (root fixture chain) → validation mechanism✓ (absolute target, `lstat`, owner-only) ·
    instrumentation n/a (test code, a panic names the check) · PII n/a (fixed messages, no path) · tests integ
    (six cases; remove-the-guard reds recorded) · a11y n/a · tokens n/a
  - `Workspace::ensure_e2e_home` (harness) → validation mechanism✓ (the same three checks) · instrumentation n/a
    (the harness prints one JSON document; a refusal reads the existing `build-failed` / `verify-exit-none`) · PII
    n/a (fixed messages, no path, never printed) · tests unit (seven cases; remove-the-guard reds recorded) · a11y
    n/a · tokens n/a
  - G2's start point → validation n/a · instrumentation n/a · PII n/a · tests (`--probe` green; the old-against-new
    one-shot in `evidence/smoke-on-backing.md`; G2 green in CI's three `test` logs) · a11y n/a · tokens n/a

## Deviations from intent
- **Step 3, where the guard was removed.** The plan names the call site; the cases call the keeper itself over
  their own temp dirs (as steps 1 and 2 require), so a call-site swap cannot turn case 2 red. The link arm was
  switched off inside each keeper instead, which leaves exactly the bare `create_dir_all`: both read
  `AlreadyExists` red and green restored (`evidence/keeper-control.md`, pairs 1 and 2).
- **Step 3, a third control pair.** `.claude/rules/testing.md` (2026-09-25) asks a remove-the-guard run of every
  new guard test, so the three refusal checks and the `local_live` call were read too: 3 and 4 red with the checks
  off, 6 and 7 green restored. With the relative-target check off the run made two empty `backing` directories
  (the repository root and `crates/viola-e2e/`), removed with `rmdir` before the restore.
- **Step 5, the quiet check.** Read by executable path, not by process name (Spec claims disproved, second
  bullet); `lsof +D target/e2e-home` listed no open file and `agent-run.sh cleanup` read `"ok":true`.
- **Keeper shape.** A target that is a link and a target that is not a directory are refused by one `lstat` check
  with one message, where the plan lists them as two conditions. The create result is ignored so the `lstat` is
  the only verdict (`.claude/rules/testing.md`, 2026-09-27, on a guard that repeats what the call guarantees; it
  also keeps concurrent starts after a reboot from racing on the `mkdir`).
- **Step 8's notice** also named the forced control's own 260 MB on the shared volume, which the plan's wording of
  the notice leaves out.
- **Order of the checkpoints.** The smoke (step 7) ran before the reader entry could (it reads step 8's record), so
  the block ran as entries 1 to 16, then whole once the record existed.
- scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 6 · listed 6 · recorded 0 ·
  absorbed 0 · excluded 64`, base `b329083e`, read at this wrap's P1).

## Decisions & corrections
- The overseer's dispositions, given with the wrap invocation (2026-10-07, relayed by the operator's session):
  1. the widening amendments are ratified by the founder's live answer of 2026-10-07T07:25Z, shown to him as
     creating and deleting outside the working directory through the link (inputs#I6);
  2. the handoff's operator desk names `target/e2e-home.disk` whole, the old `viola-test-WYNVH7` line retired into
     it;
  3. the copied-tree question for a mutation run (whether a copied tree carries the link or a plain directory) goes
     to the Epoch 3 boundary audit CARRY on the last Epoch 3 entry;
  4. a stalled-start red from now on is a finding about the backing: stop and report, never re-run for green.
- The operator's word at implement: the notices of steps 5 and 8 are notices, not questions; at the 60 minute cap
  with no contended window, stop and ask before any synthetic writer (not reached: the window came at 16 minutes).
- Corrections to the builder this session:
  - a time written into an evidence file ahead of the clock was refused by the stamp hook; times are read from
    `date -u`, never estimated;
  - a scratchpad script written through a `cat` heredoc with a file target was refused by the Bash guard (the rule
    `host-win32.md` already carries); the Write tool, then run by path;
  - a compound that carried `rm -rf` beside other steps was denied whole; the removal was dropped and the rest
    re-fired.
- Sweep hazards found:
  - `pgrep -x viola` on this host matches the overseer pair's own bridge (another tree's build of the same name):
    a liveness check by process name reads 9 where this repository has 0. Read `/proc/<pid>/exe` against the
    repository root.
  - `find <link>` with no trailing slash does not descend a link to a directory on this host's `find` (bfs 4.1.1):
    0 files against 3 with the slash.
  - With a relative-target check neutralised, a keeper resolves the target against the test's working directory
    and makes a directory there: a mutation run of the harness keeper will leave `backing/` in its copied tree.
- A natural window is found by its trigger, not by its first line: the first window (control median 6.471 s, max
  7.206 s) stayed under the 7 s line and is kept in the record uncounted; the next one, 59 s later, read 10.999 s.

## Outcome
**Acceptance criteria, each against the diff and the evidence:**
- (tests) `target/e2e-home` is still the `tempdir_in` root and each home is still created by viola: **met**.
  `pre-push` reads `"ok":true` at `linux-tests` on the backing, three times (the block twice, the operator pass).
- (tests) No kill, no `WITHIN`, no retry moved: **met**. The preservation guard exits 0 over `.config/nextest.toml`
  and `tests/support/watch.rs`.
- (tests) The contended reading: **met**. `evidence/contended-reading.ndjson` holds a window (08:08:52Z) whose
  control stalled 10.999 s median on the shared volume, with the backing probe at 0.285 s and `binary(cli_verify)`
  at 29 passed, 0 failed; the reader entry exits 0. No suite member carries an elapsed-time verdict.
- (tests) The keepers hold their contract: **met**. 7 and 6 passed by their prefix filters; `evidence/keeper-control.md`
  holds each keeper's red and green.
- (arch) No product crate touched, no dependency added: **met** (the guard). `replace_private` is unchanged.
- (arch) No new environment variable, flag or harness variable: **met** (the diff adds none).
- (arch) `ci.yml` unchanged and CI green on the final HEAD: **met**. ci#37592258366 on `230f5dc`, the HEAD the
  operator pass pushed; no fix commit followed.
- (security) Modes on the backing and no `strict-modes-failed` line: **met** (`evidence/smoke-on-backing.md`: 700,
  600, 700; 0 lines).
- (security) Every pre-push launcher call still carries exactly `HOME` and `PATH`: **met** (the guard over
  `pre_push.rs` and `pre_push/`).
- (security) Neither the link, its target nor `target/e2e-home.disk` is tracked or untracked-unignored: **met**
  (`evidence/backing.md`: no `git status --short` line; `git check-ignore -v` answers `.gitignore:15:target/`).
- (security) The link's target is a real, owner-only directory or the start is refused: **met** (cases 4, 5 and 6
  of each keeper, with their remove-the-guard reds in pair 3).
- (obs) Over the link G2's walk finds the role files and G2 and G4 read green: **met** (`g2: clean`,
  `schema-check` `"ok":true`, 3 files, 22 lines; the old start point's empty reading recorded).
- (obs) `g2-zero-panics.sh --probe` exits 0 after the edit: **met**.
- (obs) The reading in `evidence/` is numbers and project names, no absolute host path: **met** (`hygiene: clean`:
  a preview before the pass, twice in the pass before the commit, and once at implement's P4 against the chunk's
  base `b329083e`).
- (a11y) `run --browser` still boots, runs and cleans up inside `pre-push`; no suite gained a retry, a sleep, a
  widened timeout or a skip: **met** (playwright 1/1 in each `pre-push`; the diff adds none).

**Gates** (implement's one full run on the final tree, 08:09Z to 08:10Z; `entries 20 · green 17 · red 0 · recorded 0
· timeout 0 · not-run 3`):
- `cargo fmt --all --check` — green, exit 0.
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green, exit 0.
- `bash scripts/agent-run.sh run --unit` — green (`"ok":true`; 1336 passed).
- `bash scripts/agent-run.sh run --unit --filter 'test(/e2e_home_backing_/)'` — green (`"passed":7,"failed":0`).
- `bash scripts/agent-run.sh run --integration --filter 'test(/home_base_backing_/)'` — green
  (`"passed":6,"failed":0`).
- `bash scripts/g2-zero-panics.sh --probe` — green, exit 0.
- `test -L target/e2e-home && findmnt -n -o FSTYPE -T target/e2e-home/` — green (last line `tmpfs`).
- `git diff --quiet b329083e0266 -- src crates/viola-core …` — green, exit 0.
- `bash scripts/agent-run.sh cleanup --session p-homes-smoke` (pre-clean) — green.
- `bash scripts/agent-run.sh boot --session p-homes-smoke --instance builder` — green.
- `bash scripts/agent-run.sh status --session p-homes-smoke` — green.
- `bash scripts/g2-zero-panics.sh` — green (`g2: clean`).
- `bash scripts/agent-run.sh schema-check` — green (`"ok":true`).
- `bash scripts/agent-run.sh cleanup --session p-homes-smoke` — green (`"processes_gone":true`,
  `"endpoint_gone":true`).
- `bash scripts/agent-run.sh run --integration --filter 'binary(cli_verify)'` — green (`"passed":29,"failed":0`,
  21.2 s).
- `bash scripts/agent-run.sh pre-push` — green (`"ok":true`, `"stage":"linux-tests"`; coverage 1675/1675,
  playwright 1/1, no breach; 43.0 s).
- `jq -e -s 'map(select(.kind == "reading" and .control_write_med_s >= 7)) | …' …/evidence/contended-reading.ndjson`
  — green, exit 0.
- `python -X utf8 …/gate.py hygiene` (`leg = 'operator'`) — driven by hand in the operator pass: exit 0, `hygiene:
  clean`, both atoms hold (`evidence/operator-pass.md`, entry 18).
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (`leg = 'operator'`) — the operator
  pass: exit 0, `b329083..230f5dc` (`evidence/operator-pass.md`, entry 19).
- `python -X utf8 …/ci.py conclusion --sha HEAD --wait 1800` (`leg = 'operator'`) — the operator pass: exit 0,
  `verdict: green · checks 15/15`, both atoms hold (`evidence/operator-pass.md`, entry 20).
- No `defer`, no skip, no deferral under the source-delta rule: the chunk has Rust delta and every entry ran.
- Smoke (P3): fired by the harness-listed entries and by P3's own boot on the backing; `ready`, the three
  one-shot readings recorded, cleanup with the home removed.

**Watches:**
- `pre-push`'s coverage merge failing on truncated profiles while another build writes to the volume · 3 green
  runs in this chunk [the block's first run, 41.8 s, quiet host; the block's full run, 43.0 s, started 40 s after
  the contended window with the other build still alive; the operator pass, 42 s] · not recurred. Coverage
  profiles are written under `target/`, not under a home, so this chunk does not move it.

**Outcome basis:** the operator pass ran (the pre-CI commit `230f5dc`, the only commit from `b329083`), so the gate
verdicts rest on its final state: the tree of `230f5dc`, CI's run on it recorded in `evidence/operator-pass.md`.
Implement's conversation is present in this window (the same session), and its P4 report stands as given. After
the push the tree gained only the CI section of `evidence/operator-pass.md` and tool trails; no source moved.

**Process hygiene** (implement P4's census, measured at 08:12Z; re-measured at this wrap's P1, 08:23Z, by the same
read of `/proc`: no executable of this repository alive, no driver or probe, 8 processes of another tree named
`viola`):

| process | started by | final state |
|---|---|---|
| harness sessions `p-homes-smoke` (two gate runs, one P3 boot) | this run | terminated (`processes_gone:true` each; no executable of this repository alive) |
| the contended driver, its probes and its suite (the control and the graded run) | this run | terminated (none alive); `target/contended-control/` removed |
| nine `viola` processes of another tree | not this run | left running: the overseer pair's own bridge |
