# Report — 2026-10-05-dialog-rows-and-re-probe

**Chunk:** Dialog rows and re-probe — S3/S7/S8 and dialog-concurrency ledger rows, interactive verify re-probe, the
decision taking effect (the permission end to end split off to "Permission end to end", the founder's ruling)
**Date:** 2026-10-05
**Commits:** (basis `git log --format='%h %s' 75198e5..HEAD`, the operator pass ran; `75198e5` = the oldest pre-CI
commit's parent)
- `43e6245` chore: operator pre-CI commit, for the run this chunk's verdict reads
- `4179973` fix: operator fix after CI run 37316283001, Run C and Run D settle after their last Stop before the kill
- `9629757` test: operator timing-only measurement for CI round 2, verify timing lines on the coverage leg
- `74e719b` test: operator revert of the CI round 2 measurement
- `c914216` test: operator fix after CI run 37320693487, verify-driven tests join the verify_window_ class

## Changes (structured — detectors read this)
- **Files:** (basis `git diff --name-only 75198e5`, 108 paths; outside `.andromeda/` and the version workspace: 51)
  - product: `crates/viola-agent-claude/src/ledger.rs`, `crates/viola-agent-claude/src/dialog.rs`,
    `crates/viola-agent-claude/src/snapshots/viola_agent_claude__dialog__tests__plan_approved.snap`,
    `crates/viola-state/src/fs.rs`, `src/cmd/hook.rs`, `src/cmd/verify.rs`, `src/cmd/verify/typed.rs`;
  - tests only: `src/run/version_gate.rs` (tests module), `src/bin/viola-fake-agent.rs` (the fake agent, feature
    `fake-agent`), `crates/viola-e2e/src/harness/{run,boot}.rs`, `tests/{cli_verify,cli_answer,contract_ledger_probes,
    contract_fixture_hygiene,contract_fake_agent_drift,hook_fail_open}.rs`, `tests/support/{verify,home}.rs`,
    `.config/nextest.toml`;
  - fixtures: 12 recorded dialog variants under each of `fixtures/claude/2.1.288/` and `fixtures/claude/2.1.287/`
    (`PreToolUse`/`PostToolUse` `.questions-1`, `.parallel-1`, `.parallel-2`, `.plan-2`; `PreToolUse.plan-1`;
    `PermissionRequest.plan-1`, `.permission-1`; `PostToolUse.permission-1`), and `fixtures/claude/2.1.287/RELAYED.md`
    (a dated "Superseded" section);
  - chunk folder: `evidence/{live-sessions,step0-dialog-shapes,capture-race,round-131207Z,run-kill-settle,
    operator-pass,ci-rounds,verify-window-class}.md|txt`, `scope-record.md`, this report.
- **Symbols / APIs:**
  - `viola_agent_claude::ledger`:
    - `LedgerRow` gains `QuestionAnswer` (`question-answer`, "a question answered through PreToolUse takes effect"),
      `PlanApproveRevise` (`plan-approve-revise`, "a plan revise and approve each take effect"), `QuestionNotes`
      (`question-notes`, "free text and notes reach the question"), `DialogConcurrency` (`dialog-concurrency`, "two
      parallel questions each raise a dialog"); `ALL: [Self; 14]`, in that order after `confirm-window`.
    - New consts: `DIALOG_PROMPT_QUESTIONS`, `DIALOG_PROMPT_PARALLEL`, `DIALOG_PROMPT_PERMISSION`, `PLAN_PROMPT` (ASCII,
      no tag characters, each starting `viola verify probe: `), `DIALOG_SETTINGS`
      (`{"permissions":{"ask":["Bash(touch viola-probe-permission)"]}}`), `PROBE_FREE_TEXT`, `PROBE_NOTE`,
      `PROBE_REVISE`, `DIALOG_MATCHER` (`AskUserQuestion|ExitPlanMode`), `DIALOG_TURNS`, `PROBE_ANSWERS` (the answer
      table: Run C PreToolUse 1 → `questions-first-and-free-text`, PreToolUse 2-3 → `question-first-option`,
      PermissionRequest 1 → `permit-allow`; Run D PermissionRequest 1 → `plan-revise`, PreToolUse 2 → `plan-approve`;
      Run D PreToolUse 1 unanswered).
    - New fns/types: `plan_settings(plans_dir) -> String` (`{"plansDirectory": <dir>}`), `DialogRun`, `dialog_stem`,
      `ProbeAnswer` (5 kebab ids, `from_id`, exact match), `answer_file_name`, `probe_body(event, payload, answer)`
      (builds a `dialog::Response` keyed by the payload's own question texts and returns `dialog::decision_body`'s
      output, never a second mapping), `ProbePlugin { Spine, Dialog { answers_dir_fwd } }`, `DialogRuns { questions,
      plan }`, `parallel_both_before_first_post`, `dialog_variants`.
    - Changed signatures (every caller updated): `capture_plugin_files(pinned, captures, kind: ProbePlugin)` (sole
      caller `verify.rs`'s `write_plugin`, plus tests); `merge_stamp(…, parallel_both_before_first_post: Option<bool>,
      written_at)` (callers: `verify.rs`, `version_gate.rs` tests, ledger tests); `Probes` gains `dialogs`.
    - `check` gains four arms; a run that raised no dialog fails its rows (unit-witnessed).
  - `viola_agent_claude::dialog::decision_body`: plan approve on PreToolUse now `allow` + `updatedInput` = the tool's own
    input, unchanged (an object; a non-object input gives `{}`), per the founder's ruling after step 0's STOP 7 (a bare
    `allow` left the plan dialog up on live 2.1.288). One production caller, `src/cmd/hook.rs` `body_of`, now also
    reached through `probe_body`. The `plan_approved` insta snapshot changed with it.
  - `viola_state::fs::create_private_new(path) -> io::Result<File>`: create-new at `FILE_MODE` (Unix), fails
    `AlreadyExists`. One production caller (`src/cmd/hook.rs` `file_from`).
  - `src/cmd/hook.rs`: the capture arm claims `<Event>.<k>.json` exclusively (`file_from(event, dir, bytes, start)`:
    the first free `k` from `free_k`, the next `k` on `AlreadyExists`, then `replace_private` over its own claim);
    `HookArgs` gains a hidden `--answers <DIR>` (read only with `--capture`; without it the flag is inert): for a dialog
    event, the event's ordinal `j` in claim order selects `<DIR>/<Event>.<j>` read through `take(64)`, an exact
    `ProbeAnswer` id, `probe_body` over the filed payload, one write to stdout. Every failure prints nothing; no
    `VIOLA_*` read, no obs init, no channel, nothing on stderr, exit 0 (the capture-arm diff guard read 0).
  - `src/cmd/verify.rs` / `typed.rs`: verify runs FOUR interactive PTY runs. Run A (untrusted) and Run B (trusted) are
    unchanged. Run C runs in `<cwd>/.viola-verify-<pid>-dialogs/`, with `--model haiku --settings <DIALOG_SETTINGS>`
    and a dialog plugin; it pastes the three dialog prompts, each after the previous turn's Stop capture. Run D runs in
    `<cwd>/.viola-verify-<pid>-plan/`, with `--model haiku --permission-mode plan --settings {"plansDirectory":"<Run D
    dir>/plans"}` (a 0700 `plans/` inside the 0700 dir) and a dialog plugin; it pastes `PLAN_PROMPT` and waits for its
    turn's Stop.
    - Both new runs wait for the input box with no modal before any paste (a modal start is killed with no key).
    - After the last Stop is captured the screen settles, as Run B's turn does, and the run is ended by a kill, never a
      key.
    - Each wait is bounded by `PROBE_DEADLINE`; both dirs carry a drop guard.
    - `ProbeDir` builds `questions/` and `plan/` roots, each with `plugin/`, `captures/` and an `answers/` dir written
      from `PROBE_ANSWERS` before the spawn.
    - `--record` writes each dialog capture as `<Event>.<variant>.json` (`ledger::dialog_variants`), scrubbed and
      `unclean`-checked like the spine payloads; a dirty payload refuses the whole recording naming the file and the
      code.
    - `measured.dialog_probe.parallel_both_before_first_post` (bool or null) is added to the stamp; `run` reads no field
      of it.
  - Fake agent: new argv modes `--dialogs` (replays a pasted compiled dialog prompt's recorded `<stem>-<n>` variants in
    claim order: a call's PreToolUse fires first; with no decision, its PermissionRequest fires; a decision from either
    fires its PostToolUse) and `--stop-receipt-hold-ms <n>` (capped at 1 000 ms, a test-only hold of the Stop hook's
    receipt). `fire` now reports whether a hook printed. No env seam was added.
  - Test support: `support::verify::{verify_without_dialogs, dialog_set, write_dialog_set, tool_payload, FREE_TEXT,
    NOTE}`, `support::home::dialogless_home`. `verify()` passes `--dialogs` and spawns `viola verify` with no test-side
    bound (`viola_unbounded`; round 4).
- **Crates / modules:** none added or removed; changed as listed above.
- **Dependencies:** none — verified (`git diff 75198e5 -- Cargo.toml Cargo.lock` empty; the deny guard read 0).
- **Schema / config:**
  - The stamps envelope gains `data.versions.<v>.measured.dialog_probe.parallel_both_before_first_post` (additive; no
    `v` bump; `run` ignores it).
  - The capture plugin's dialog-kind `hooks.json` registers PreToolUse (matcher `AskUserQuestion|ExitPlanMode`),
    PermissionRequest and PostToolUse beside the four spine events. The two dialog events carry `--answers`.
  - New recorded fixture class: `fixtures/claude/<ver>/<Event>.<stem>-<n>.json` (stems `questions`, `parallel`,
    `permission`, `plan`). The fixture hygiene walk covers it; `schemas/claude-fixture.v1.json` is unchanged (it already
    admitted the three events).
  - `.config/nextest.toml`, profile `ci`: `test(/verify_window_/)` gets a 45 s kill (15 s × 3); the twelve
    verify-driven binaries (`channel_endpoint cli_answer cli_fake_agent cli_instance_state cli_send cli_verify
    cli_version_gate cli_wheel conpty_sideload contract_fake_agent_drift contract_ledger_probes tui_wheel`) get a 20 s
    kill (10 s × 2). The `mutants` profile's `verify_window_` comment now reads "four times"; its kills are unchanged.
  - No `config.json` key, `VIOLA_*` env var, event kind, refusal, snapshot field or obs subject was added (guards 16 and
    17 read clean).
- **Spec-master edits:** none (no master was edited by implement; `git status --short -- .andromeda/*.md` shows only
  the friction log and run dirs).
- **Counts / qualifiers moved:**
  - ledger rows 10 → 14 and the step counter `/10` → `/14` (basis: `LedgerRow::ALL`). Master sites, from
    `scratchpad sites.py`, pattern `ten rows|ten-row|/10\b|10 pass|\bten ledger`: architecture 5 · security-plan 3 ·
    design-system 3 · layout-templates 5 · test-plan 5 · obs-plan 0 · a11y-plan 0. CLAUDE.md:43 states "the S3/S7/S8
    dialog bodies ride the ten-row stamp until … 'Dialog rows and re-probe'".
  - verify's interactive runs 2 → 4, and its spawn pairs 4 → 6 (version-probe, verify-probe, four `verify-pty-probe`).
    Pattern `two interactive|both interactive|four spawn pairs|four pairs`: architecture 4 · security-plan 2 ·
    test-plan 1 · obs-plan 2.
  - one verify's hook processes against the fake agent 7 → 29 (print 4, Run B 3, Run C 15, Run D 7; the
    identity-strip test asserts 29).
  - verify's wall time against the fake agent: 2.11 s, then 2.73 s with the settle (dev host, harness build); the
    ubuntu coverage leg's median for verify-driven tests went ~1.0 s → ~2.1 s (`evidence/ci-rounds.md`).
- **Dev-tool versions:** none — `claude` re-read at 2.1.288 (PATH, `claude --version` in the round) and 2.1.287 (mise
  install path), both unchanged since the previous chunk.
- **Harness / gate surface:**
  - harness boot step 4 passes `--dialogs` (`crates/viola-e2e/src/harness/boot.rs`, `stamp`).
  - `LEDGER_ROWS: [&str; 14]` and the `local-live` row check (`run.rs`).
  - nextest `ci`-profile kill overrides (Schema / config).
  - The test-side bound: every `viola verify` a test drives (`support::verify::verify`, and through it `stamped_home`
    / `dialogless_home`) runs with no test-side bound, `WITHIN` = 7 s staying for every other wait (round 4; the
    overseer's decision).
  - No `agent-run` verb, CI step or status shape changed.
- **Cross-project / external claims:**
  - `inputs.py verify`: `I1 · message · n/a` (the operator's take-up directive), `I2 · message · n/a` (the structure-only
    census of the viola-lab prototype's interactive 2.1.287 logs), `I3 · message · n/a` (the founder's live P4
    rulings); 3 entries, all cited, unchanged/drifted/vanished/broken 0.
  - Live `claude`: 2.1.288 and 2.1.287 under the founder's cap of 16, 13 sessions spent (`evidence/live-sessions.md`).
    The static reading of the 2.1.288 binary (a hook `allow` satisfies a user-interaction tool only "via
    updatedInput"; `plansDirectory` exists and must lie within the project root) is in `evidence/step0-dialog-shapes.md`.
  - CI runs this chunk's gates read, by sha (`evidence/ci-rounds.md`): `43e6245` red (ci#37316283001); `4179973` green
    (ci#37318179233); `9629757` green (ci#37319370056); `74e719b` red (ci#37320693487); `c914216` green twice
    (ci#37322552375 attempts 1 and 2, 15/15 each).
  - The host-side residuals: each verify adds the CLI's transcripts under `~/.claude/projects/` (the accepted class,
    +2 per run for C and D).
- **Reverted / negative API facts:**
  - `ledger::plan_turn_ended` (Run D ending on the plan's PostToolUse or a Stop) was written, then removed. Ending on
    the PostToolUse raced the fake agent's receipt and would make the drift contract flaky; Run D now ends on its
    turn's Stop.
  - The bare plan approve (`allow` with no `updatedInput`) is replaced; it never shipped in a passing live stamp.
  - The round-2 measurement (`verify-timing` lines and a `ci` success-output override) was pushed and reverted in
    round 3.
- **Insufficient fixes (written, kept, not the remedy):**
  - Round 1's settle before the Run C / Run D kill (kept, red before green) removed the corrupt-profile half of the CI
    red: no corrupt `.profraw` since. It did not remove the timing half: round 3 went red on the same code. The
    remainder's remedy is round 4's `verify_window_` class.
- **Spec claims disproved by measurement:**
  - (1) `dialog.rs`'s doc and the `plan_approved` snapshot stated the S7 approve as "PreToolUse `allow` alone". Live
    2.1.288 (step 0, STOP 7) followed it with a second ExitPlanMode PermissionRequest and no PostToolUse. Disposed:
    the body was changed under the founder's ruling. architecture states only "S7: ExitPlanMode approves only through
    PreToolUse" (1 hit for the pattern `approves only through PreToolUse`), which still holds.
  - (2) The capture arm's naming (`free_k` then rename) was not concurrency-safe (research M2), measured RED by the
    forced-race test (`evidence/capture-race.md`). Disposed: the exclusive claim.
  - (3) The operator-pass HYPOTHESIS that the hook-process count drove the coverage slowdown was falsified by round 2
    (`evidence/ci-rounds.md`). It was a hypothesis, never a spec claim; recorded here for its disposition.
  - (4) The "dialog concurrency" premise: the two "parallel" AskUserQuestion calls ran one after the other on both
    versions (`parallel_both_before_first_post` false). The row as defined (two calls with distinct ids, each answered)
    passes, and no master claims the calls overlap.
- **Expected amendments (from plan):** (basis `sites.py` above, per master)
  - architecture [CLI Version Compatibility] + §Cross-cutting "Capability ledger as the single gate": 14 rows, Run C /
    Run D, the R2 dated exception closed — carried (Counts; Symbols). Sites: ten-rows 5 · R2-dated 1 · two-interactive 4.
  - architecture [Hook Contract]: the capture arm's `--answers` (the founder's ruling) — carried (Symbols: hook.rs).
    Sites: `--capture` 7.
  - architecture [Plugin Scope]: five transient children — carried. Verify now spawns the print probe plus four PTY
    runs (Symbols: verify).
  - architecture §Occupied Resources → Filesystem: the two probe dirs and six spawn pairs — carried. Sites:
    `.viola-verify-` 5 · `verify-pty-probe` 1.
  - architecture §Occupied Resources → Repository: the dialog variants and RELAYED.md superseded — carried (Files;
    Schema). Sites: relayed 9.
  - security-plan §Security Anti-Patterns → Universal: R2's dated gap retired — carried. Sites: R2-dated 1 · ten-rows 3.
  - security-plan §Input Validation › Hook stdin: the capture arm's answer — carried. Sites: `--capture` 5.
  - security-plan §Data Protection: the Run C `touch` and the +2 transcripts residual — carried (Cross-project). Sites:
    `.viola-verify-` 4. The `plansDirectory` fact also belongs here: Run D's plan file goes into the run's own dir, so
    no file lands under `~/.claude/plans`, measured on both versions.
  - test-plan §5 / §7 / §11: `/14`, the dialog variants, the `--dialogs` mode — carried. Sites: ten-rows 5 · relayed 9 ·
    R2-dated 2. The `verify_window_` class extension (round 4) is a test-plan fact too.
  - obs-plan §4 / §10 exemption 5: six spawn pairs; the arm prints a body and is still uninstrumented — carried. Sites:
    `verify-pty-probe` 5 · two-interactive 2 · `--capture` 4 · R2-dated 1.
  - design-system §Surface: cli Component Patterns 5 and layout-templates §Output structure — `viola verify`: the four
    row ids and words, `/14` — carried (Symbols: LedgerRow). Sites: design-system ten-rows 3; layout-templates ten-rows
    5 · never-answers 1.
  - layout-templates: the `--help` paragraph's "never answers" sentence re-read against the ruling — not carried: no
    change. `src/cmd/mod.rs` reads "An unapproved external CLAUDE.md import blocks the probe: its dialog shows in the
    subfolder, and verify never answers it". That is about the CLI-native external-import dialog, which verify still
    never answers. Sites: never-answers in architecture 1, security-plan 1, layout-templates 1.
  - CLAUDE.md:43 (GENERATED:setup:warnings): the dated-exception line, re-derived by the cascade with the architecture
    amendment (plan: no entry of its own).
  - The route minting of "Permission end to end" right after this entry (W3d + W6) — wrap route-resolve's (P5).
- **Coverage of new surfaces:**
  - `hook <event> --capture <dir> --answers <dir>` → validation `take(64)` + exact closed id + absolute dir ✓ ·
    instrumentation n/a (the capture arm is uninstrumented by contract, obs-plan §10 exemption 5) · PII n/a (writes only
    a product-built body) · tests unit (`cmd::hook::tests::answer_*`) + integ (`hook_fail_open`) · a11y n/a · tokens n/a.
  - `viola verify` Run C / Run D → validation: compiled prompts, compiled settings, the R8 strip, `MAX_FRAME`-capped
    screen ✓ · instrumentation: a `verify-pty-probe` pair per spawn ✓ · PII: recorded payloads scrubbed + `unclean`
    refusal ✓ · tests unit + integ (`cli_verify`, `contract_ledger_probes`, drift) + live (13 sessions) · a11y: the
    step lines are uncoloured ASCII with no redraw ✓ (`cli_verify` `assert_plain`) · tokens n/a.
  - `ledger::probe_body` → validation: payload shape checked (`None` on a miss) ✓ · instrumentation n/a (pure) · PII n/a ·
    tests unit · a11y n/a · tokens n/a.
  - `viola_state::fs::create_private_new` → validation n/a · instrumentation n/a · PII n/a · tests unit (`fs::tests`) +
    the race test · a11y n/a · tokens n/a.

## Deviations from intent
- **Step 0 STOP 7** (the bare approve did not take effect, 2.1.288) returned to the operator with 2 of 16 sessions
  spent. The founder ruled live (2026-10-05, via the overseer's AskUserQuestion, relayed by the operator):
  - re-run Run D once, 1 spare session, 13 of 16 planned;
  - approve = PreToolUse `allow` + `updatedInput` set to the tool's own input, unchanged;
  - test `plansDirectory` in Run D's `--settings`, inside its own 0700 dir.
  The re-run met both conditions (the approve took effect; `~/.claude/plans/` 17 → 17 files). The body change and the
  `plansDirectory` setting are this ruling's.
- **Run D ends on its turn's Stop, not the plan's PostToolUse** (plan step 5). Ending on the PostToolUse raced the
  fake agent's receipt, so receipt counts and the dialog drift contract would be flaky. The rows still read the
  PostToolUse.
- **The settle before the Run C / Run D kill** (the overseer's decision, CI round 1): beyond plan step 5's "ended by a
  kill"; red before green in `evidence/run-kill-settle.md`.
- **The verify-driven tests join the `verify_window_` class** (the overseer's decision, CI round 4):
  - no test-side bound on the verify call;
  - a 20 s `ci` per-test kill, 45 s for `verify_window_` tests;
  - this reverses the overseer's earlier "not option 3", recorded with its measured basis (a designed floor, not a
    regression) in `evidence/verify-window-class.md`.
- **A `cli_answer.rs` case for `v1-15`** (a plan first raised by PermissionRequest) was added: the matrix acceptance
  needs it, and no prior test covered it.
- **The fake agent's `--stop-receipt-hold-ms`** is a test-only argv mode added to force a race window open (testing.md
  2026-09-27).
- **`src/cmd/mod.rs` (verify `--help`) unchanged** (see Expected amendments).
- **Scope record** (`gate.py scope` at P1: `clean — changed 45 · listed 43 · recorded 2 (companion 1 · mechanical 0 ·
  in-intent 1 · widening 0)`):
  - in-intent: `tests/hook_fail_open.rs` — serves step 4 · self (the capture arm's fail-open cases the acceptance cites);
  - companion: `.config/nextest.toml` — serves `tests/cli_verify.rs` · self (the `verify_window_` comment, then round
    4's overrides).

## Decisions & corrections
- The founder's live rulings at P4 (inputs#I3): M7 path A, the hook answers, never a key into any dialog; live cap
  16, both versions stamped; W3d + W6 split off to a new Epoch 3 entry "Permission end to end" right after this one.
- The founder's live ruling on STOP 7 (above): the Run D re-run, the echoed-input approve, `plansDirectory` in the
  same session.
- The overseer's decisions, founder-delegated:
  - CI round 0 → option 1 (the settle) now, then a timing-only measurement before removing any contention, never the
    test bound;
  - after the measurement → neither verify change (Run C/D in parallel adds concurrent live claude only to fit a test
    bound; fewer settles is unsafe). Instead the `verify_window_` class, sized from the measured floor plus a 3x tail;
    "read it twice green before the wrap".
- Leave for the overseer desk (the founder's word): step 0's first Run D plan file under `~/.claude/plans/` (one file,
  `viola-verify-probe-make-greedy-island.md`, written twice); the empty `.viola-verify-2095228/` at the repository
  root (from the previous chunk's record runs, about 10:23Z).
- Sweep hazards:
  - `plan-` as a substring matches the relayed `exit-plan-mode` names; the dialog-variant test reads
    `<Event>.<stem>-<digits>.json` instead.
  - `pkill -f <pattern>` matches the calling shell's own command line and killed it (host-win32.md 2026-09-25,
    recurring).
  - `ls -t | head -1` over the task dir can name the watcher's own output file.
- Measured mechanism: the settle is a 300 ms quiet grace, not an absolute guarantee. A hook-receipt hold longer than
  the quiet period still loses the race, so it covers a hook's millisecond exit, the real case.

## Outcome
- **Acceptance criteria**, re-asserted against the diff:
  - (arch) `LedgerRow::ALL` holds 14, the last four as listed, each judged by `check` over Run C / Run D captures; a
    run with no dialog fails its rows (`ledger::tests::check_dialog_rows_fail_when_no_dialog_was_raised`) — MET.
  - (arch, security) both live record entries printed `stamped <ver>  14 pass  0 fail` and their 12 variants landed;
    sessions: step 0 = 3 (the founder's +1), step 11 = 10, total 13 of 16, each row in `evidence/live-sessions.md` —
    MET. The criterion's "step 0 spends 2" reads 3 under the founder's re-run ruling.
  - (security) every live dialog was answered by the probe hook's `decision_body` output, and no byte was typed into a
    dialog; C and D end by a kill; the one `allow` ran `touch viola-probe-permission` inside Run C's 0700 dir; every
    probe dir is gone after each run — MET (step 0 + the round's after-checks).
  - (security, obs) the capture arm exits 0 with empty stderr on every input and writes a body only for a mapped answer,
    with no `VIOLA_*`, obs init or channel; two concurrent captures never share a name — MET (`hook_fail_open` capture
    case, the unit cases, guard 18 read 0, `capture-race.md`).
  - (security, tests) R2 closes: a home stamped `10 pass  4 fail` records `cli_verified:false`, the hook prints nothing,
    and `answer` exits 12 `unverified-cli`; a 14-row home emits the body — MET
    (`cli_answer::path4_a_stamp_failing_the_dialog_rows_leaves_dialogs_to_the_human`, `path4_dialogs_are_…`,
    `version_gate::tests::stamps_verdict_needs_the_dialog_rows`).
  - (tests) `contract_ledger_probes` stamps both versions at `14 pass  0 fail` against `--dialogs`; the drift contract
    replays every variant byte for byte; hygiene passes them with a planted nested-path red; `agent-run.sh run`
    `"ok":true` — MET.
  - (design, layouts, a11y) `[01/14]`…`[14/14]` then `stamped`, uncoloured, no redraw; a failing dialog row prints only
    `fail`, nothing on stderr — MET (`cli_verify` `verify_without_the_dialog_replay_fails_the_four_dialog_rows`).
  - (obs) with `VIOLA_NAME`, six spawn pairs, the four PTY ones `verify-pty-probe`; each passes G4 —
    MET (`cli_verify::verify_with_an_instance_logs_its_six_spawn_pairs`).
  - (tests) `v1-15`: `cli_answer.rs`'s plan cases, both raises, stand on all three CI OSes (`c914216`, twice green), and
    `plan-approve-revise` passes in the 2.1.287 stamp — MET; the matrix reads `implemented`, flipped at P7.3.
- **Gates** (the final local block at the final tree, implement run `2026-10-05T11-48-51`, and the operator pass):
  - `cargo fmt --all --check` green · `cargo clippy … -D warnings` green;
  - the `run --unit --filter` ledger/dialog/verify/hook/fs/version_gate/harness set green (333 passed);
  - `run --unit` green (1 294);
  - `claude --version` green (in the round) · the 2.1.288 record entry green (`stamped 2.1.288  14 pass  0 fail`) · the
    2.1.287 record entry green (`stamped 2.1.287  14 pass  0 fail`) — `round: COMPLETE · legs fired 2/2`, no survivor
    (`evidence/round-131207Z.txt`);
  - the preservation guard (`git diff --quiet 75198e53b918 -- …` pre-existing fixtures) green;
  - the integration filter (eight binaries) green (147 passed) · `run` (default) green;
  - the smoke `cleanup` / `boot` / `status` / `cleanup` green (boot stamped through `--dialogs`; status ready; processes
    and endpoint gone);
  - `CI=true … run --local-live` green (exit 2, `live-in-ci`);
  - the seam/schema guard green · the ignore/retries/env guard green · the capture-arm guard green (exit 1, last line
    0) · the deny guard green (exit 1, 0);
  - `run pre-push` green (coverage 1 609 passed, `"stage":"linux-tests"`);
  - the operator entries (`evidence/operator-pass.md`): hygiene `clean`; the push `75198e5..43e6245`; then four fix
    pushes; `ci.py conclusion` green at the final HEAD `c914216` twice (ci#37322552375 attempts 1-2).
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran. Setup 4's commit list is the five commits above, and the final HEAD's CI
  run is `c914216` green twice (`evidence/ci-rounds.md`, `operator-pass.md`). The conversation is present, and
  implement's P2/P3 results and every deviation come from it.
- **Process hygiene** (implement P4's census, re-measured at this wrap's Setup from the host process list): none left
  running.
  - started by this run and terminated: the scratch step 0 drivers and their `claude` children (3 sessions); both
    record runs' children (the round's census: no survivor); a stray `ugrep` (stopped by exact pid); background
    watchers (stopped).
  - not this run's: the operator's prototype `viola run` sessions and other `claude` sessions, untouched.
