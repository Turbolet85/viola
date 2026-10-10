# Report — 2026-10-09-epoch-3-cleanup-ii

**Chunk:** Epoch 3 cleanup II — test scaffolding shared once, three functions within the cognitive ceiling,
viola-e2e whole-unit score on the dev host, eighteen Linux survivors killed or restated
**Date:** 2026-10-10T01:26Z
**Commits:** `632f6a7` chore(2026-10-09-epoch-3-cleanup-ii): operator pre-CI commit, for the run this chunk's verdict
reads (the one commit since the last wrap's `f0a0dd1` and the setup upgrade `14f1fb5`; basis
`git log 14f1fb5f588d..HEAD`). Every coordinate below is the work tree's; the diff base is `14f1fb5f588d`, the
pre-CI commit's parent.

## Changes (structured — detectors read this)
- **Files:** 20 source and test files, 548 lines added and 527 removed (`git diff --shortstat 14f1fb5 -- src crates
  tests`). Product and test-side sources: `crates/viola-agent-claude/src/ledger.rs` (+75 −30),
  `crates/viola-state/src/events.rs` (+17 −1), `crates/viola-state/src/fs.rs` (+23), `crates/viola-state/src/stamps.rs`
  (+7), `crates/viola-state/src/strict.rs` (+10), `src/bin/viola-fake-agent.rs` (+64 −9), `src/cmd/hook.rs` (+25),
  `src/cmd/verify.rs` (+48 −10). Root tests: `tests/support/cli.rs` (new, 101 lines), `tests/support/events.rs` (new,
  45 lines), `tests/support/mod.rs` (+2), `tests/cli_answer.rs` (+6 −101), `tests/cli_wait_last.rs` (+57 −101),
  `tests/cli_wheel.rs` (+23 −88), `tests/tui_wheel.rs` (+11 −78), `tests/cli_send.rs` (+2 −26),
  `tests/cli_verify.rs` (+22 −52), `tests/hook_events.rs` (+8 −23), `tests/cli_instance_state.rs` (+1 −4),
  `tests/channel_paste_validation.rs` (+1 −4). Beside them: the chunk folder's `evidence/` (15 files, `ls`), `inputs/`,
  the phase run dir and the implement run dir. No file outside research's two lists was edited.
- **Symbols / APIs:**
  - No public product symbol, channel method, CLI verb, flag, exit code, event kind, snapshot field, port, socket
    or environment variable was added, removed or changed. Behaviour is unchanged by intent.
  - **Three functions split in place**, name, signature and callers kept:
    - `dialog_variants` (`ledger.rs`): its `match capture.event` moved into the new private `joined_call`
      (`ledger.rs:878-902`, head `:880`) with the private alias `Call` (inside the added range `:875-903`).
      `dialog_variants` keeps its one product caller, `record` in `src/cmd/verify.rs`.
    - `record` (`src/cmd/verify.rs`, private, one caller `measure`): now calls four new private functions in the
      same file, `named_payloads`, `scrubbed_payloads`, `signature_screens`, `write_recording`, and the private
      alias `Recorded` (the added ranges `:395-413 · 428-436 · 442-444 · 448-455 · 458 · 460 · 465-470 · 476`; the
      bodies between them are moved text). Order kept: every payload and every screen is checked before the first
      file is written, and a refusal returns before any write. The refusal still prints through
      `refuse_recording` → `human::refuse`; no span and no `obs_event!` moved.
    - `submit` (`src/bin/viola-fake-agent.rs`, the test-only fake agent): the end of a turn moved into the new
      private `Agent::end_turn` (`:488-503`, head `:491`), called at `:472`. No option, receipt or written byte
      changed.
  - **Four expressions restated, behaviour unchanged, each with its guarantee in a comment:**
    - `LoggedLines::next_line` (`events.rs:209-211`): `len < self.cap` is now `len != self.cap`.
    - `parallel_both_before_first_post` (`ledger.rs:666-673`): no PostToolUse before the second PreToolUse, with
      no ordering operator between two indexes.
    - `clear_start` (`ledger.rs:688-691 · 694`): the `+ 1` is dropped; the search starts at the paste's own index.
    - the long-paste test in `Agent::end_turn`: `framing.is_some() &&` is dropped.
  - **New shared test support** (test-only, `#[allow(dead_code)]` per binary):
    - `tests/support/events.rs`: `events` (`:14-17`), `wait_events` (`:19-32`), `boot` (`:34-45`, a `builder`
      wrapper over the committed fixtures, its session-start record landed).
    - `tests/support/cli.rs`: `Ran` (`:14-18`), `Running` with `over`, `still_running`, `finish` (`:24-56`) and its
      `Drop` (`:58-65`), `spawn` (`:67-96`), `viola` (`:98-101`). stdout and stderr are two captures beside the exit
      code; `VIOLA_NAME` is removed from the child and set only for `from`; a stdin write tolerates `BrokenPipe`
      alone; the command is built with no `env_clear`.
  - **Removed:** the per-file copies of `events` (7 files), `wait_events` (6), `Running` (2), `Ran` (4), `spawn`
    and `viola` in the eight lifted files; `fixtures_arg` in `cli_answer.rs`. Per-file `boot` functions keep their
    name and signature and call the shared boot; `cli_wait_last`'s keeps its synthetic fixtures. `cli_send`'s
    `start_send` / `feed` / `finish` / `Sent` stay.
  - `tests/cli_verify.rs`: one in-file helper `trusted_run_typed` (`:912-928`, head `:914`) replaces one block in
    four tests.
  - **Fourteen new test cases** (heads from the listing): `ledger.rs:2735`, `:2803` and two rstest cases at
    `:2219-2220`; `events.rs:616`, `:621`; `fs.rs:356` (`cfg(unix)`); `stamps.rs:215`; `strict.rs:530`;
    `viola-fake-agent.rs:1107`, `:1118`, `:1127` (with the helper `whoami` at `:1096`); `src/cmd/hook.rs:885`;
    `tests/cli_wait_last.rs:458`.
- **Crates / modules:** no crate added, removed or re-pointed. Two test-support modules declared
  (`tests/support/mod.rs:6-7`): `cli`, `events`.
- **Dependencies:** none added, none bumped (`Cargo.toml`, `Cargo.lock` and every member manifest read unchanged by
  the preservation guard, `git diff --quiet 14f1fb5f588d -- … Cargo.toml Cargo.lock …`, exit 0).
- **Schema / config:** none. `schemas/`, `fixtures/`, `plugin/`, `.config/nextest.toml` and `.github/` are unchanged
  (the same guard).
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - **Root waits on the one bound `WITHIN`.** `Instant::now() + WITHIN` sites under `tests/` (basis: `grep -c` per
    file, the base read through `git show 14f1fb5:<file>`): 30 sites in 15 files at the base, 22 sites in 16
    files now. Six files lost sites (`cli_answer` 3 → 1, `cli_wait_last` 3 → 1, `cli_wheel` 3 → 1, `tui_wheel`
    3 → 1, `cli_send` 3 → 2, `hook_events` 1 → 0) and the two new support files hold one each.
    `architecture.md:407` says the bound is "used by the 9 root waits on a child". That count's own rule was not
    re-derived by this report: this pattern does not read 9 at either point, so the 9 counts something narrower.
    Read at the resumed wrap, on the operator's word (`inputs#I5`): the 9 is a named list, not a pattern count.
    Eight are the sites chunk 2026-09-27-browser-verdict-reachability moved from a 10 s bound onto `WITHIN`
    (`fake.rs` `wait_for`, `home.rs` `wait_ready`, `outer_pty.rs` `wait_exit`, `run_cli.rs` `wait_raw` and the
    `claude-child` wait, `cli_instance_state.rs`'s `-STOP` wait, `contract_diag_schema.rs`'s raw and exit waits:
    that chunk's `report.md`), and the ninth is `wait_endpoint_gone` (chunk
    2026-09-29-verify-stamped-test-homes-and-harness's `report.md`). No wrap re-took the list after that. The
    same claim stands in the test-plan key file `registries/contracts/test-plan/5-command-implementation.md`
    ("shared by all 9 root waits on a child", "the ninth wait"). The pattern above is narrower than "every wait
    on the bound": it does not match the `EXIT_WITHIN` sites (`tests/support/outer_pty.rs`,
    `tests/tui_pty_seam.rs`) or the two `support::watch::WITHIN` sites in `tests/cli_instance_state.rs`
    (`grep -rn WITHIN tests/`, read at the resumed wrap).
  - **`tests/support/` files:** 9 → 11 (`ls`). `test-plan.md:456` names five of them, `events.rs` among them; that
    file did not exist before this chunk and does now; `cli.rs` is not named there.
  - **Test counts:** unit 1471 → 1484, integration 353 → 354, the coverage run 1824 → 1838 (the harness documents:
    the last wrap's light gate at `f0a0dd1`, this chunk's block entries 3, 4 and 18). CI's own counts on the final
    sha: ubuntu 1838, windows 1861, macOS 1834.
  - **The first measured whole-unit `viola-e2e` score on the dev host:** 718 mutants, wall 5356 s (89 min 16 s),
    656 caught, 2 missed, 0 timeout, 60 unviable (`evidence/e2e-score.md`). The earlier whole-unit wall on record
    is 78 min for 711 mutants.
  - **The two audit scalars** (no master records either; the instrument is the audit's): duplication 3.36 % →
    2.84 % with named fragments 7 → 0; `complexity.over_ceiling` 4 → 1 (`evidence/scalars.md`).
  - **cargo-mutants' generated counts on the edited files** (`cargo mutants --list`, the witness documents):
    the four viola-state files 205 → 203, `ledger.rs` 304 → 303, the four root files 255 → 263.
- **Dev-tool versions:** none — `jscpd` re-read at 5.0.16, `rust-code-analysis-cli` at 0.0.25, `cargo-mutants` at
  27.1.0, all on the dev host, unchanged.
- **Harness / gate surface:** none. `scripts/`, `crates/viola-e2e` and the workflows are unchanged (the guard). The
  four mutation runs used the existing `run --mutants --package <member> [--file …]` arm as built.
- **Cross-project / external claims:**
  - CI: `ci#38012420489`, event `push`, on `632f6a7edf29`, conclusion `success`, 15 of 15 checks, `run_attempt` 1,
    the only run on that sha (`evidence/operator-pass.md`). The verdict was taken on that tree; this wrap's
    commit adds to it.
  - The dev host's pressure record is read with the overseer's `hostwatch.py`, which lives outside this
    repository: QUIET for viola over every mutation run's window (`evidence/e2e-score.md`, `evidence/survivors.md`).
  - Inputs (`inputs.py verify`, 2026-10-10T01:25Z: 4 entries, n/a 4, drifted 0, vanished 0, broken 0):
    `I1 · message, the operator at take-up · copy · n/a — a message has no live source` (cited by scope, research,
    plan); `I2 · message, the operator in the P4 fork dialog · copy · n/a` (cited by scope, research, plan);
    `I3 · message, the operator in the implement invocation · copy · n/a`, cited here: `inputs#I3` is the word
    under which the implementer drove the operator pass and journalled each mutation run; `I4 · message, the
    operator in this wrap's invocation · copy · n/a`, snapped at this wrap and cited here: `inputs#I4` (P1 only in
    this window, the wrap resumed in a cleared one). `verify` printed I3 and I4 `UNCITED` before this report
    existed. `I5 · message, the operator in the resumed wrap invocation · copy · n/a`, snapped at the resume
    (2026-10-10T01:32Z, the manifest's `read_at`) and cited here: `inputs#I5` (the wrap resumed at Phase 2 in its first run dir, with the
    route items for P5).
- **Reverted / negative API facts:** `parallel_both_before_first_post` was first rewritten with a slice pattern in
  place of `if pres.len() != 2`; that took one generated mutant out of the function for no reason the restatement
  needed, so the length check was put back before any run. It was never committed. Nothing else was written and
  removed.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none in a master. One plan-level count read differently: plan step 12
  and research say the receipt-reading block of `tests/cli_verify.rs` stands in three tests; it stood in four
  (`grep -c trusted_run_typed tests/cli_verify.rs` reads 5: the helper and four call sites).
- **Expected amendments (from plan):**
  - *test-plan §2 Test directory + naming conventions — `tests/support/` gains `cli.rs` beside the `events.rs` it
    already names, and the per-file copies are gone*: **carried**, by "Symbols / APIs" (the new support files) and
    "Counts / qualifiers moved" (9 → 11 files). Search: `grep -c 'tests/support'` per master: architecture 7,
    security-plan 1, test-plan 9, the other four 0; under `.andromeda/registries/` three test-plan key files hold
    it (`test-data-bootstrap.md`, `5-command-implementation.md`,
    `bootstrap-phases-derive-for-route-setup-project.md`). The one site that lists the helper files by name is
    `test-plan.md:456`.
  - *test-plan §10 Mutation gate — the first measured whole-unit `viola-e2e` score on the dev host with its wall
    and counts, beside the 78 min reading it carries*: **carried**, by "Counts / qualifiers moved". Search:
    `78 min` reads 0 in all seven master bodies and 0 in every file under `.andromeda/registries/`; `whole-unit`
    reads 0 in every master body. (Corrected at the resumed wrap, 2026-10-10T01:36Z: this line first said the
    `78 min` search read 1 each in two test-plan key files; a fixed-string read of each file and a recursive
    `grep -c` over the masters and the registries both read 0. The reading the plan's entry names is spelled
    `78 m` and stands once, in the test-plan key file
    `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md` (line 16, "a full Linux
    viola-e2e run took 78 m against 23 m under `immediate`"); `78 m` reads 0 in the seven master bodies and in
    every other key file. The Mutation gate paragraph, `test-plan.md:1212`, holds no whole-unit wall or count
    for `viola-e2e`.)
  - *architecture §Occupied Resources — the count of root waits that share `WITHIN`, if the lift changes how many
    call sites there are*: **carried**, by "Counts / qualifiers moved" (30 → 22 sites by pattern; the master's 9
    not re-derived). Search: `grep -c 'WITHIN'` reads 1 in architecture (`:407`), 0 in the other six, and 1 each
    in the same two test-plan key files.
- **Coverage of new surfaces:** no new external surface, hot-path operation or UI element.
  - `joined_call`, `named_payloads`, `scrubbed_payloads`, `signature_screens`, `write_recording` (private product
    helpers split out of existing functions) → validation n/a · instrumentation n/a (no span added; none moved) ·
    PII n/a (the refusal text still names a file and a check code, never content) · tests unit + integ (every
    mutant of each reads caught or unviable in the witness runs) · a11y n/a · tokens n/a
  - `tests/support/cli.rs`, `tests/support/events.rs` (test-only) → validation n/a · instrumentation n/a · PII n/a
    · tests integ (the eight lifted files run through them: 354 of 354) · a11y n/a · tokens n/a

## Deviations from intent
- **`tests/cli_verify.rs`'s helper serves four tests, the plan names three.** The fourth,
  `verify_without_framing_fails_only_the_clear_row`, holds the same block. Each test keeps its expected list as a
  literal.
- **The instrument's population is tracked plus untracked `*.rs` files.** The plan gives it the audit's
  `git ls-files` rule, which leaves the two new files out until they are committed. On the untouched tree both
  rules read the same 124 files; `evidence/scalars.md` states it.
- **The shared `Running` has a public constructor, `Running::over`.** `cli_answer.rs`'s two hook runners build
  their own command and need the guard; research did not list that use.
- **The four mutation runs' stderr files were moved out of the run dir** to the session scratchpad before hygiene.
  The plan files them under the run dir; each opens with cargo build lines that carry the repository's absolute
  path. Their outcome lines are in `evidence/`.
- **`evidence/survivors-build.py` was added**: the script that built `survivors.ndjson` from the three outcome
  files. The plan names no producer for that record.
- **The smoke was not driven a second time by hand.** The plan lists pre-clean, boot, status and cleanup as
  ordinary smoke entries, and the gate tool fired them in the block.
- **Five of the fourteen kill rows name their killing test by construction, not from a log** (the viola-state
  rows): the next witness run replaced that run's per-mutant logs before they were read. Their `caught` lines are
  measured; the other nine attributions are read from logs.
- scope record: none — `gate.py scope` clean, changed 20 · listed 20 · recorded 0 (read at implement P4, after the
  pre-CI commit, and again at this wrap's P1).

## Decisions & corrections
- **The operator's word for the implement run** (`inputs#I3`): no live session; each mutation run journalled
  before its wait (start, output files, next step) and re-read from disk at its return; a red where no assertion
  failed on a value, or a timeout in a mutation record, is read against the backing and `hostwatch.py` before
  anything else, never re-run for green; the implementer drives the operator pass, reading the attempt number.
  All four runs were journalled so (`mutation-runs.md` in the implement run dir).
- **The operator's word for this wrap** (`inputs#I4`): P1 only in this window; the wrap resumes in a cleared one,
  where its route items are given.
- **The operator's word for the resumed wrap** (`inputs#I5`): the two missed `viola-e2e` mutants go onto "Windows
  mutation grade"; the four root mutants graded caught only by the 10 s kill need a route owner, the options
  shown recommended-first and each priced; no epoch boundary is minted inside Epoch 4 (the founder's word of
  2026-10-09); the architecture count of root waits is amended only after its counting rule is read.
- **The operator's answer in the wrap's route dialogue** (`inputs#I6`, four placements shown with their prices):
  the four root mutants caught only by the 10 s kill are owned by the route entry "Paste newline ledger row",
  which edits `viola verify`'s child set and the fake agent; "Windows mutation grade" keeps its size.
- **Found, for the route:** in the root witness run, four mutants on lines this chunk did not edit are graded
  `caught` by the `mutants` profile's 10 s kill alone, with no failing assertion
  (`viola-fake-agent.rs:378:16` and `:378:38`, `:97:17`, `src/cmd/verify.rs:284:9`). They were read against the
  backing and the host record and not re-run. They have no route owner yet.
- **Found, for the route:** the two `viola-e2e` mutants the score read missed are in `prepare`
  (`harness/run/mutants/scratch.rs:48:5`, `:54:8`), reachable only on a Windows host. Recorded "not measured here;
  owed to Windows mutation grade".
- **Found:** a mutation run of `viola-e2e` leaves session homes on the shared test-home base. The tool's copy of
  the tree carries the `target/e2e-home` link, so harness tests under a mutated `boot` or `cleanup` leave theirs
  behind (26 after the score run). No process was left.
- **Found:** cargo-mutants keeps the per-mutant logs of one earlier run only (`mutants.out.old/`). Which test
  failed under a mutant is read from `mutants.out/log/` before the next mutation run starts.
- **Found:** a formatter width above its range panics at run time (`Formatting argument out of range`); a 16 MiB
  padding is built with `repeat`.
- **Sweep hazards:** (1) `grep 'in <function>$'` over cargo-mutants' outcome lines misses the whole-function
  mutants, whose text ends `with <value>` and names no `in`. (2) A mutation text plus its function is not a
  unique key: two rows share it inside `both_parallel_answered` and inside `has_email`, and `clear_start` keeps a
  `+ → *` line on another expression after the restated one is gone; a row is pinned by its coordinate too.
- **Recurrences of carried learnings, both in the implement run:** a time was written into the journal ahead of
  the clock (`01:09Z` for an act at `01:07Z`, corrected in place); one Bash call carried a heredoc with a file
  target and was refused by the guard.

## Outcome
**Acceptance criteria, each re-asserted against the diff. All twenty-one are met.**
1. `run` exits 0 with `"ok":true`, counts the base's plus the new cases, none dropped — met: 1484 and 354 against
   1471 and 353; both in `evidence/scalars.md`.
2. No root test file outside `tests/support/` defines `events`, `wait_events`, `Running` or `Ran` — met: the count
   entry reads 0 (19 at the base).
3. (audit M1) duplication below 3.13 % with no named fragment — met: 2.84 %, 0.
4. (audit M2) `over_ceiling` 1 and none of the three functions or a split-out helper over 15 — met: 1; 10, 2, 11.
5. (audit M3) both scalars before and after beside the three earlier records — met: `evidence/scalars.md`.
6. The `viola-e2e` score is one package run under the scratch `TMPDIR`, with its document, counts, missed mutants
   by name and the host-unreachable wording — met: `evidence/e2e-score.md`; the score reader exits 0. The score
   itself reads red by the verdict rule (2 missed, both host-unreachable); the criterion asks for the record.
7. Each of the eighteen reads `caught` by a test in its owning package or gone by restatement, none missed, none
   called equivalent — met: 14 and 4; the survivor reader exits 0.
8. Each witness run reads `timeout == 0` and `unviable <= caught`, every missed line accounted for, no new missed
   mutant in the restated files — met: 0 timeouts; 9 ≤ 129, 20 ≤ 283, 25 ≤ 238; the 65 viola-state misses are 59
   audit not-measured and 6 `replace_private_with` rows.
9. (security) `fs.rs:290:19` caught by the same-bytes case, other bytes still an error — met (`fs.rs:356`).
10. (security) `strict.rs:34:23` caught by a case asserting `Refused::Unreadable` — met (`strict.rs:530`).
11. (security) the `--record` refusal tests pass with no expected literal edited — met: `tests/cli_verify.rs`'s
    diff touches only the receipt-reading block and its helper.
12. (obs) `src/cmd/mod.rs:133:5` caught by a test; the record states obs for each row — met (`obs` field; row 17).
13. (obs) G2 zero panic lines and `schema-check` `"ok":true` — met: `g2: clean`; 145 files, 1952 lines.
14. (arch) no manifest, lockfile, workflow, nextest profile, fixture, schema or plugin change; no feature, env var
    or fake-agent option; `viola-e2e` untouched — met: the preservation guard exits 0.
15. (arch) fmt and clippy pass with no new `allow` and no print macro in `src/cmd/verify.rs` — met.
16. (arch) no file under `evidence/` holds an absolute path — met: hygiene `clean`.
17. (design) stdout-only and stderr-only assertions kept; the shared runner exposes two captures beside the exit
    code — met (`tests/support/cli.rs:14-18`).
18. (layouts) `viola verify`'s step lines, summary and fixture refusal unchanged — met: no expected-output literal
    in `tests/cli_verify.rs` edited; `human.rs` under the guard.
19. (a11y) each tui boundary case and the gate-wait case still its own case; `cli_output_plain.rs` untouched — met.
20. `pre-push` exits 0 with `"ok":true` — met, twice (in the block and before the push).
21. The final HEAD's CI run reads green on all three OSes — met: `ci#38012420489` on `632f6a7edf29`.

**Gates** (the block's 21 entries by `run`, at the implement run's firing, 2026-10-10T01:09Z, 18 green · 0 red ·
3 not run; then the operator pass):
- `cargo fmt --all --check` — green, exit 0
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green, exit 0
- `bash scripts/agent-run.sh run --unit` — green, exit 0, `"ok":true`
- `bash scripts/agent-run.sh run` — green, exit 0, `"ok":true`
- `cat tests/*.rs | grep -c -E '^(pub )?fn (events|wait_events)\(|^struct (Running|Ran)\b'` — green, exit 1, last
  line `0`
- `python3 …/evidence/scalars.py selftest` — green, `selftest: 0 mismatches`
- `python3 …/evidence/scalars.py duplication` — green, `named fragments 0`
- `python3 …/evidence/scalars.py complexity` — green, `over_ceiling 1`
- `git diff --quiet 14f1fb5f588d -- .config .github Cargo.toml Cargo.lock …` (the preservation guard) — green
- `jq -e -s 'length == 18 and …' …/evidence/survivors.ndjson` — green, `true`
- `jq -e '.mutants.verdict == "package" and …' …/evidence/e2e-score.json` — green, `true`
- `bash scripts/agent-run.sh cleanup --session p-e3c2-smoke` (pre-clean) — green
- `bash scripts/agent-run.sh boot --session p-e3c2-smoke --instance builder` — green, 4.8 s
- `bash scripts/agent-run.sh status --session p-e3c2-smoke` — green, `state:"ready"`
- `bash scripts/g2-zero-panics.sh` — green, `g2: clean`
- `bash scripts/agent-run.sh schema-check` — green
- `bash scripts/agent-run.sh cleanup --session p-e3c2-smoke` — green, `processes_gone` and `endpoint_gone` true
- `bash scripts/agent-run.sh pre-push` — green, `"stage":"linux-tests"`; green again through `--entry` at 01:12:55Z
- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`,
  driven by hand: exit 0, `hygiene: clean`, three readings (`evidence/operator-pass.md`)
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD` — `leg = 'operator'`: exit 0,
  `14f1fb5..632f6a7  HEAD -> build/viola-0.1.0`
- `python -X utf8 …/ci.py conclusion --sha HEAD --wait 1800` — `leg = 'operator'`: exit 0, `verdict: green · checks
  15/15`, `run_attempt` 1
- No `defer`, no deferral, no `--skip`, no `--void`. Smoke: the boot path changed (the fake agent's split); the
  session booted, read ready and cleaned up inside the block.

**Watches:** none folded.

**Outcome basis:** the operator pass ran. The verdicts above rest on its final state: one commit, `632f6a7`
(`git log 14f1fb5f588d..HEAD`), and the final HEAD's CI run recorded in `evidence/operator-pass.md`. Implement's
report as given in this conversation is the basis for what only it holds (the deviations, the census). No fix
commit was made and no directive changed the tree between implement's report and this one. Written after the
pre-CI commit, for this wrap's commit to carry: the last sections of `evidence/operator-pass.md`, one sentence each
in `evidence/survivors.md` and `evidence/e2e-score.md`, the implement run dir's journal and gate trail.

**Process hygiene** (implement P4's census, the host's process list read at 2026-10-10T01:23Z):
- four `run --mutants` harness runs · started by this run · terminated (each exited by itself)
- the gate block, its smoke session `p-e3c2-smoke`, the `pre-push` re-run · this run · terminated
  (`processes_gone:true`)
- the CI read (`ci.py`) · this run · terminated
- No process of this repository was alive at the read; every `viola`-named process belonged to another tree's
  build, decided by its executable path. Not re-measured at this wrap's P1.
- Left on disk, not removed: 34 `.tmp*` directories in the mutation scratch (250 MB); 26 `viola-session-*` homes
  on the tmpfs behind `target/e2e-home`; `mutants.out/` and `mutants.out.old/` (ignored by git); the four raw
  stderr captures in the implement session's scratchpad.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 14f1fb5f (the parent of the oldest pre-CI commit 632f6a7e) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### crates/viola-agent-claude/src/ledger.rs — added 75 line(s) in 9 range(s)
added: 666-673 · 688-691 · 694 · 875-903 · 915 · 917-918 · 2219-2220 · 2733-2741 · 2800-2818
  - 669-673 «Some(»
- 878-902 @880 «fn joined_call<'a>(calls: &mut Vec<Call<'a>>, capture: &'a Capture) -> Option<usize> {»
  - 883-901 «match capture.event {»
  - 2733-2740 @2735 «fn check_dialog_concurrency_with_a_post_for_a_third_id_is_not_answered() {»
  - 2800-2817 @2803 «fn dialog_variants_with_a_post_of_an_unknown_id_beside_an_identified_call_records_no_post() {»
### crates/viola-state/src/events.rs — added 17 line(s) in 2 range(s)
added: 209-211 · 610-623
  - 615-618 @616 «fn events_current_len_of_a_log_that_cannot_be_statted_is_an_error() {»
  - 620-623 @621 «fn events_read_of_a_log_that_cannot_be_opened_is_an_error() {»
### crates/viola-state/src/fs.rs — added 23 line(s) in 1 range(s)
added: 351-373
  - 351-372 @356 «fn replace_private_shared_in_a_read_only_dir_is_done_only_over_the_same_bytes() {»
### crates/viola-state/src/stamps.rs — added 7 line(s) in 1 range(s)
added: 212-218
  - 212-217 @215 «fn read_stamps_of_a_file_that_cannot_be_opened_is_an_error() {»
### crates/viola-state/src/strict.rs — added 10 line(s) in 1 range(s)
added: 527-536
  - 527-535 @530 «fn check_stamps_of_a_path_that_cannot_be_statted_is_unreadable() {»
### src/bin/viola-fake-agent.rs — added 64 line(s) in 4 range(s)
added: 472 · 488-504 · 800 · 1094-1138
  - 488-503 @491 «fn end_turn(&self, framing: Option<&str>) {»
  - 1095-1104 @1096 «fn whoami() -> String {»
  - 1106-1115 @1107 «fn run_hook_after_close_hooks_runs_nothing() {»
  - 1117-1124 @1118 «fn run_hook_of_a_stop_with_the_receipt_hold_takes_at_least_the_hold() {»
  - 1126-1138 @1127 «fn submit_of_the_long_paste_with_a_paste_hint_takes_at_least_the_hint() {»
### src/cmd/hook.rs — added 25 line(s) in 1 range(s)
added: 882-906
  - 882-905 @885 «fn handle_dialog_with_a_payload_at_the_frame_bound_prints_the_decision_body() {»
### src/cmd/verify.rs — added 48 line(s) in 8 range(s)
added: 395-413 · 428-436 · 442-444 · 448-455 · 458 · 460 · 465-470 · 476
  - 395-399 @396 «let mut files = match scrubbed_payloads(named_payloads(probes), &home, &user) {»
  - 400-403 «match signature_screens(&probes.typed, &home, &user) {»
### tests/channel_paste_validation.rs — added 1 line(s) in 1 range(s)
added: 14
### tests/cli_answer.rs — added 6 line(s) in 5 range(s)
added: 19 · 24-25 · 50 · 605 · 626
### tests/cli_instance_state.rs — added 1 line(s) in 1 range(s)
added: 16
### tests/cli_send.rs — added 2 line(s) in 2 range(s)
added: 19 · 36
### tests/cli_verify.rs — added 22 line(s) in 5 range(s)
added: 912-929 · 955 · 986 · 1021 · 1050
- 912-928 @914 «fn trusted_run_typed(receipt: &Path) -> Vec<String> {»
  - 916-921 «let starts: Vec<usize> = lines»
  - 923-927 «lines[starts[2]..starts[3]]»
### tests/cli_wait_last.rs — added 57 line(s) in 17 range(s)
added: 16-17 · 173 · 183 · 205-206 · 220 · 227 · 261 · 270 · 292-297 · 316-321 · 337 · 353 · 387-392 · 404 · 413 · 446
       454-477
  - 292-297 «let before = json_ok(&viola(»
  - 316-321 «let after = json_ok(&viola(»
  - 387-392 «let piped = viola(»
- 454-476 @458 «fn wait_in_a_home_whose_diagnostics_is_a_file_keeps_the_chain_in_the_detail_file() {»
  - 466-471 «let detail = home»
### tests/cli_wheel.rs — added 23 line(s) in 22 range(s)
added: 15-16 · 18 · 35 · 83 · 88 · 92 · 105 · 110 · 120 · 129 · 132 · 138 · 141 · 147 · 211 · 216 · 222 · 230 · 232
       234 · 288 · 307
### tests/hook_events.rs — added 8 line(s) in 3 range(s)
added: 18 · 163-165 · 309-312
  - 310-312 «let lines = wait_events(&wrapper.instance_dir(), "the hook events", |l| {»
### tests/support/cli.rs — new file · 101 line(s)
- 14-18 «pub struct Ran {»
- 24-56 «impl Running {»
  - 25-28 @26 «pub fn over(child: Child) -> Self {»
  - 30-33 «pub fn still_running(&mut self) -> bool {»
  - 35-55 @36 «pub fn finish(mut self) -> Ran {»
- 58-65 «impl Drop for Running {»
  - 59-64 «fn drop(&mut self) {»
- 67-96 @70 «pub fn spawn(home: &Path, args: &[&str], stdin: Option<&str>, from: Option<&str>) -> Running {»
  - 72-83 «command»
  - 84-86 «if let Some(from) = from {»
  - 88-94 «if let Some(text) = stdin {»
- 98-101 @99 «pub fn viola(home: &Path, args: &[&str], stdin: Option<&str>, from: Option<&str>) -> Ran {»
### tests/support/events.rs — new file · 45 line(s)
- 14-17 @15 «pub fn events(instance_dir: &Path) -> Vec<Value> {»
- 19-32 @20 «pub fn wait_events(instance_dir: &Path, what: &str, pred: impl Fn(&[Value]) -> bool) -> Vec<Value> {»
  - 23-31 «loop {»
- 34-45 @36 «pub fn boot(stamped: StampedHome, script: Option<&str>, extra: &[&str]) -> Wrapper {»
  - 41-43 «wait_events(&wrapper.instance_dir(), "the session-start record", |l| {»
### tests/support/mod.rs — added 2 line(s) in 1 range(s)
added: 6-7
### tests/tui_wheel.rs — added 11 line(s) in 10 range(s)
added: 17-18 · 20 · 30 · 85 · 128 · 137 · 139 · 148 · 260 · 283
