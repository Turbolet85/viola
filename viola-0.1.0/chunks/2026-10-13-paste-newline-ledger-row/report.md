# Report — 2026-10-13-paste-newline-ledger-row

**Chunk:** the owed capability-ledger rows parked on this entry (an LF inside a paste first), probed by viola verify under the founder's ruling on its child set
**Date:** 2026-10-13
**Commits:** `16b5f5f chore(2026-10-13-paste-newline-ledger-row): operator pre-CI commit, for the run this chunk's verdict reads` (the only commit since `last_wrap` 2026-10-13T20:26:00Z; `git log --format='%h %s' 19483111dbe9..HEAD`)

## Changes (structured — detectors read this)
- **Files:**
  - Product: `crates/viola-agent-claude/src/ledger.rs`, `src/cmd/verify/typed.rs`.
  - Tests:
    - in product files: `crates/viola-agent-claude/src/ledger.rs` (its tests), `src/cmd/verify.rs` (tests only),
      `src/bin/viola-fake-agent.rs` (tests only), `src/run/version_gate.rs` (tests only),
      `crates/viola-e2e/src/harness/run.rs` (the `LEDGER_ROWS` literal and its tests);
    - root tests: `tests/cli_verify.rs`, `tests/contract_ledger_probes.rs`, `tests/support/verify.rs`, and two
      recorded companions, `tests/contract_fixture_hygiene.rs` and `tests/support/home.rs`.
  - New fixture: `fixtures/claude/2.1.287/UserPromptSubmit.paste-3.json`, recorded live and copied alone.
  - Chunk folder: `evidence/live-starts.md`, `evidence/live-round.md`, `evidence/round-213213Z.txt`,
    `evidence/carry2-kills.md`, `evidence/operator.md`, `scope-record.md`, `inputs/` I4 and I5.
  - Basis: `git diff --name-only 19483111dbe9` and `gate.py scope` (changed 12 · listed 10 · recorded 2).
- **Symbols / APIs:**
  - `viola_agent_claude::ledger::LedgerRow::PasteNewline`: new, the eighteenth and last member of
    `LedgerRow::ALL` (now `[Self; 18]`). Rows 1–17 keep their slots.
    - id `paste-newline`; words `an LF inside a paste stays inside the one prompt submitted`.
    - Its `check` arm is `pasted(&probes.trusted, PROBE_NEWLINE_PASTE).is_some()`: the first trusted-run
      UserPromptSubmit whose prompt normalises to the whole text. A CLI that submitted at the LF leaves no capture
      equal to the whole text.
  - `pub const PROBE_NEWLINE_PASTE: &str`: new,
    `"viola verify probe: this paste holds one line break here\nand the next line ends it. Reply with the single word ok"`.
    113 bytes, ASCII, exactly one LF (at neither end), no CR.
  - `FRAMING_TURNS`: now `[(&str, &str); 4]` in paste order — `paste-1` the long text, `paste-2` the tag-like text,
    `paste-3` the newline text, `clear-1` `/clear`.
    - `framing_stem` gives the new text the stem `paste-3`.
    - `framing_variants` names its capture `paste-3` for `--record`.
    - The fake agent reaches the new stem through `framing_stem` with no change of its own.
    - These three readers are the only ones; there is no other caller.
  - `src/cmd/verify/typed.rs` `framing_turns`: Run B's added-paste list is now
    `[PROBE_LONG_PASTE, PROBE_TAG_PASTE, PROBE_NEWLINE_PASTE]`, then the local command.
    - Same per-paste guards: only into the settled input box the turn before left, no paste after the stop, each
      waited for one more UserPromptSubmit and one more Stop.
    - Run B therefore pastes five compiled texts: the probe prompt, the long text, the tag-like text, the newline
      text, `/clear`.
    - Caller: `trusted_run` alone.
  - `viola verify` stdout: `MM` is now 18. The step counter derives from `LedgerRow::ALL.len()` (`check_rows`), so no
    printer changed.
    - It prints eighteen `[NN/18] <row id> <row words>  pass|fail` lines, the last `[18/18] paste-newline …`, then
      `stamped <ver>  N pass  M fail`.
    - The interrupted pair is unchanged.
  - The run gate's verdict (`version_gate::stamps_verdict` over `ledger::verified`) reads the stamp against all
    eighteen ids. A stamp holding only the seventeen older ids, all `pass`, now reads `cli_verified:false`, so every
    home stamped before this chunk is unverified until its next `viola verify`.
  - No new IPC method, endpoint, port, env var, CLI flag or fake-agent option.
- **Crates / modules:** none added or removed; `viola-agent-claude` (ledger) and the root bin (`cmd::verify::typed`)
  changed.
- **Dependencies:** none added or bumped; `Cargo.toml` and `Cargo.lock` are unchanged (the preservation guard's
  diff and `cargo deny check` green).
- **Schema / config:**
  - The fixture set `fixtures/claude/2.1.287/` gains one framing variant, `UserPromptSubmit.paste-3.json`: 621
    bytes, schema-valid, scrubbed. Its `cwd` reads `~/dev/projects/viola/.viola-verify-1474175`. Keys: `cwd`,
    `hook_event_name`, `permission_mode`, `prompt`, `prompt_id`, `scratchpad_dir`, `session_id`,
    `transcript_path`.
  - No schema file changed. The stamp format `ledger/stamps.json` is unchanged in shape and gains the row key
    `paste-newline`.
- **Spec-master edits:** none this chunk before the wrap. The phase's `v1-34` not-claimed note was written to the
  verification matrix at phase P5 step 4 and rode `16b5f5f`; the matrix is a ledger, not a master.
- **Counts / qualifiers moved:**
  - The capability ledger: seventeen rows → **eighteen** (`LedgerRow::ALL`; the eighteenth is `paste-newline`).
    `cli_verified` is gated by eighteen rows.
  - Run B: four compiled pastes → **five** (three added pastes before `/clear` → four added pastes then `/clear`).
  - The 2.1.287 stamped set's framing variants: four → **five** (`paste-1`, `paste-2`, `paste-3`, `clear-1` ×2).
    The set holds 29 files (28 before).
  - `viola verify`'s step lines: `[NN/17]` → `[NN/18]`; a complete set prints `stamped <ver>  18 pass  0 fail`.
  - The harness's `LEDGER_ROWS` (`crates/viola-e2e/src/harness/run.rs`) names eighteen ids, `paste-newline` last.
  - A verify run's hooks under the fake agent: 35 → 37; its Stop hooks: 8 → 9 (Run B's trusted run gains one turn).
    The six spawn pairs are unchanged.
  - The dialog-less stamp (no dialog replay over the committed set) reads `14 pass  4 fail` (was `13 pass  4 fail`).
  - The three relied-on dependences that had no ledger row: the LF typed inside a paste now has its row (and the
    Owed ledger rows entry carries the rest; P5 mints it).
  - Basis: the site sweep `report-patterns.toml` (`cascade.py sweep --pre-pass`, this run dir), patterns and
    standing master/registry rows:
    - `rows17` (`seventeen` · `17 rows` · `17 pass`): 13 —
      - `architecture.md:91` ×4, `:422`, `:472`;
      - `security-plan.md:617`; `design-system.md:811` ×2; `layout-templates.md:489`;
      - `test-plan.md:566`, `:656`, `:666` ×2;
      - registries `contracts/architecture/ci-cd-approach.md:20`, `contracts/architecture/crate-dependency-direction.md:6`,
        `contracts/test-plan/5-command-implementation.md:8`, `:53` ×4.
    - `runb-pastes`: 6 — `security-plan.md:131` ×3, `:600` (the four harness prefixes — a true unrelated
      claim, no change), `test-plan.md:408`, `:438`, `:656`, registries
      `contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10`.
    - `framing-variants`: 12 — `architecture.md:91`, `:362`, `:422` ×2; `test-plan.md:408` ×3, `:438`, `:1056` ×2,
      `:1067`, `:1076`, `:1078`, `:1093`, `:1101` ×2; registries `contracts/architecture/project-directory-structure.md:108`.
    - `ledger-rows-lit`: 1 — registries `contracts/test-plan/5-command-implementation.md:53`.
    - `step-counter`: 13 — `design-system.md:811`; `layout-templates.md:477`–`:488` (nine rows) and `:500`;
      `test-plan.md:656`; `obs-plan.md:739` (the generic `[NN/MM]` form, no count — no change).
    - `inner-lf-owed`: 6 — `architecture.md:49` ×3, `:291`, `:472` ×5; `security-plan.md:248`; `test-plan.md:752` ×3,
      `:1093`.
    - `owed-rows`: 1 — `architecture.md:472`.
    - `paste-newline`: 0 (control unfired over the seven masters at `16b5f5f4`: no master names the new row yet).
    - Leaves: 23 rows over 9 files (the sweep's `leaf` class), for P2's cascade.
- **Dev-tool versions:** none. The live round named `claude` 2.1.287 by path
  (`~/.local/share/mise/installs/claude/2.1.287/claude`, dev host, unchanged); `claude` on `PATH` still resolves to
  2.1.296, not used.
- **Harness / gate surface:**
  - `viola-harness`'s `run --local-live` checks eighteen literal ids (`LEDGER_ROWS`), so the tmpfs live home stamped at
    seventeen rows reads `row-missing` until its next firing re-stamps it (P5 card row 4, `inputs#I3`).
  - `boot` stamps through `viola verify` against the fake agent at eighteen rows (green, 5.36 s, entry `boot`).
  - No new harness command, status shape or CI step.
- **Cross-project / external claims:**
  - **CI:** run `ci#38472492871` (push) on sha `16b5f5f4006b` reads `verdict: green · checks 15/15 · wall 572 s`
    (`evidence/operator.md`, the operator's CI entry fired as written).
  - **The live CLI:** `claude` 2.1.287 on the Linux dev host submitted the 113-byte text holding one inner LF as ONE
    prompt equal to it byte for byte (`evidence/live-round.md`; one round, five starts, the founder's cap). Measured
    on Linux 2.1.287 only, not on Windows (P5 card row 5).
  - **Inputs** (`inputs.py verify`, this run dir):
    - `I1` · message: the operator, `/andromeda-phase` invocation, 2026-10-13T20:31Z · copy message · n/a — cited
      (scope, research, plan).
    - `I2` · message: the founder, by dialog, relayed verbatim by the operator, 2026-10-13T21:09Z · copy message · n/a
      — cited (scope, research, plan).
    - `I3` · message: the operator, in the session, 2026-10-13T21:20Z · copy message · n/a — `inputs#I3`, the
      operator's word on the P5 card `.andromeda/runs/2026-10-13T20-31-55-phase/p5-card-1.md` ("yes. The card stands
      as written, rows 2 to 5 included"; authority: the operator's own word; the card's three P4 forks are the
      founder's own answers, `inputs#I2`). Its `## Beyond the plan` rows, verbatim:
      1. **Route insertion** — a new entry "Owed ledger rows", placed directly after "Paste newline ledger row" in
         `### Epoch 5 — Driver surface`, carrying this entry's CARRY 4, 5, 6, 7 and 8 verbatim (`--resume`, `/compact`,
         the statusline override and `/bin/sh`, `rate_limits`, `claude agents --json`). Each shape needs a child-set
         widening ruling and a live cap of its own at take-up. Authority: the founder's own answer to fork 3
         (`inputs#I2`).
      2. **Deferral** — CARRY 2's cargo-mutants grading of `src/bin/viola-fake-agent.rs` and `src/cmd/verify.rs` moves
         to the Epoch 5 boundary audit (`/andromeda-code-audit`). This chunk shows the kills by hand-applied mutants
         instead, because `inputs#I1` excludes the witness run the CARRY named.
      3. **Owner for a part left out** — CARRY 3 (the three `viola-state` retry-loop mutants graded by the 10 s kill on
         `windows-2025`) is not this entry's work. It is carried to the Epoch 5 boundary audit's `windows-mutants.yml`
         dispatch through the handoff's dispatcher note, with no route entry. Reason: its re-reading needs a dispatch
         (`inputs#I1`), and two Linux cases already kill its six mutants.
      4. **Deferral** — the tmpfs `--local-live` home is re-stamped at eighteen rows by its next `run --local-live` firing
         (five starts, under a cap of its own), not in this chunk (the cap is five, `inputs#I2`). Until then, a live session
         from that home reads `cli_verified:false`.
      5. **Owner for a part left out** — the LF row's Windows reading (stamped here on Linux 2.1.287 only) goes to the
         existing entry "Windows-only live measurements".
    - `I4` · message: the operator, this session, 2026-10-13T21:52Z · copy message · n/a — `inputs#I4`, the
      operator's word at this wrap: plan entries 12 and 13 are wording faults recorded as deviations, neither the
      oracle nor the guard reworded; "Owed ledger rows" is minted directly after this entry from the P5 card, as the
      founder answered (`inputs#I2`).
    - `I5` · message: the operator, this session, 2026-10-13T21:22Z · copy message · n/a — `inputs#I5`, the operator's
      word at implement: the founder's cap of five live starts (one `--record` round on 2.1.287, fired once, never
      retried; each start ledgered with its `date -u` time before it), no mutation run, witness run or workflow
      dispatch, the operator pass with the `ci.py` read.
    - Summary: `inputs: 5 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 5`
      (I3–I5 were uncited before this report). No drift.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - **Plan entry 12's premise:** the literal probe `/17\]|; 17\]|17 pass|\[&str; 17\]|seventeen` over the six files
    reads 0 once no seventeen-row literal is left. Measured false: it reads 1 on the finished tree. The one hit,
    `tests/cli_verify.rs:1093` `"stamped 2.1.0  17 pass  1 fail"`, is the correct eighteen-row oracle (17 pass + 1
    fail) of `verify_without_framing_fails_only_the_clear_row`. Evidence: the gate trail of
    `.andromeda/runs/2026-10-13T21-22-15-implement/`, entry 12 firing 3. A plan statement, not a master's — disposed
    as a deviation on the operator's word (`inputs#I4`).
  - **Plan entry 13's premise:** the preservation guard reads green when the only files changed outside the modify
    set are none. Measured false for a tree carrying the fix-loop's mandatory recorded companions: it reads exit 1
    on exactly `tests/contract_fixture_hygiene.rs` and `tests/support/home.rs` (both in `scope-record.md`). A plan
    statement — disposed as a deviation on the operator's word (`inputs#I4`).
  - **Research's sweep basis** (research.md §Sweeps: 75 hits of the seventeen-row literals, "all six files are on the
    modify list"): as a caught-all claim, measured false. Five sites pinned the row set by shape outside it:
    - in `ledger.rs`, `merge_stamp_writes_the_envelope_for_a_first_version`'s rows object,
      `check_typed_rows_fail_without_a_measurement`'s 17-element verdict vector, and
      `verified_reads_a_fourteen_row_stamp_as_unverified`;
    - `tests/support/home.rs` `dialogless_home`'s `13 pass  4 fail`;
    - `tests/contract_fixture_hygiene.rs`'s four-variant list.

    They read red at the unit, integration and pre-push gates and were fixed in place. A research statement —
    disposed as the deviation below (no master change).
- **Expected amendments (from plan):**
  - architecture §Established Decisions [CLI Version Compatibility] — eighteen rows, `paste-newline` named, Run B's
    five compiled pastes, the founder's R-L29 answer (`inputs#I2`): **carried** — Symbols / APIs and Counts /
    qualifiers moved. Sites: `rows17` `architecture.md:91` ×4, `:422`, `:472`, registries
    `ci-cd-approach.md:20`, `crate-dependency-direction.md:6`; Run B's paste list stated in other words at
    `architecture.md:91` (no `runb-pastes` hit in architecture).
  - architecture §Cross-cutting Patterns (Capability ledger) — the inner-LF dependence rowed; the other owed
    dependences owned by "Owed ledger rows": **carried** — Counts / qualifiers moved (the three relied-on
    dependences). Sites: `inner-lf-owed` `architecture.md:49` ×3, `:291`, `:472` ×5; `owed-rows` `architecture.md:472`.
  - architecture §Occupied Resources (Repository) — the 2.1.287 framing variants count with `paste-3`: **carried** —
    Schema / config and Counts / qualifiers moved. Sites: `framing-variants` `architecture.md:91`, `:362`, `:422` ×2,
    registries `project-directory-structure.md:108`.
  - security-plan §Threat Model Summary / §Data Protection (the verify child-set bullet) — Run B's five pastes with
    the LF text; the founder's own answer to P4 fork 1, relayed verbatim (`inputs#I2`): **carried** — Symbols / APIs,
    Counts / qualifiers moved. Sites: `runb-pastes` `security-plan.md:131` ×3; `rows17` `security-plan.md:617`;
    `inner-lf-owed` `security-plan.md:248`. A boundary widening the founder answered himself at P4 (the answer
    card).
  - test-plan §3 `run` / §5 / §7 — the row count eighteen, `LEDGER_ROWS`' named list, the `paste-3` variant in the
    stamped set: **carried** — Counts / qualifiers moved, Harness / gate surface, Schema / config. Sites: `rows17`
    `test-plan.md:566`, `:656`, `:666` ×2, registries `5-command-implementation.md:8`, `:53` ×4; `ledger-rows-lit`
    `5-command-implementation.md:53`; `runb-pastes` `test-plan.md:408`, `:438`, `:656`,
    `bootstrap-phases-derive-for-route-setup-project.md:10`; `framing-variants` `test-plan.md:408`–`:1101` (eleven
    rows above); `step-counter` `test-plan.md:656`; `inner-lf-owed` `test-plan.md:752` ×3, `:1093`.
  - design-system §Surface: cli pattern 5 — eighteen rows, the new row named: **carried** — Symbols / APIs. Sites:
    `rows17` `design-system.md:811` ×2; `step-counter` `design-system.md:811`.
  - layout-templates §Output structure — `viola verify` — the row list and `MM` eighteen with the `LedgerRow::ALL`
    rule: **carried** — Symbols / APIs. Sites: `step-counter` `layout-templates.md:477`–`:488`, `:500`; `rows17`
    `layout-templates.md:489`.
  - `v1-34` not-claimed note: written at phase P5 step 4, not by the wrap (plan); it rode `16b5f5f`. No P7.3 write
    owed.
- **Coverage of new surfaces:**
  - `LedgerRow::PasteNewline` / Run B's fifth paste → validation n/a (a compiled literal, no external input) ·
    instrumentation n/a (no new process, no new line: the six spawn pairs stand, `verify_with_an_instance_logs_its_six_spawn_pairs`
    green) · PII n/a (synthetic ASCII; the recorded fixture scrubbed, `contract_fixture_hygiene` green) · tests unit
    (three `check_paste_newline_` cases, the ledger and version-gate oracles) / integ (`cli_verify`,
    `contract_ledger_probes`, `contract_fake_agent_drift`, `contract_fixture_hygiene`, `cli_version_gate`) / live (one
    round, `evidence/live-round.md`) · a11y n/a (plain ASCII CLI lines, no SGR; `assert_plain`) · tokens n/a.

## Deviations from intent
- **Plan entry 12 (the seventeen-row literal probe) reads red:** the plan's wording is at fault, on the operator's
  word (`inputs#I4`: "Entries 12 and 13 are wording faults in the plan … record both as deviations with their reasons;
  do not reword the oracle or the guard"; authority: the operator's own word).
  - Reason: its `17 pass` alternative matches the eighteen-row oracle `"stamped 2.1.0  17 pass  1 fail"`
    (`tests/cli_verify.rs:1093`), a correct literal; every seventeen-row literal is gone.
  - Rewording the oracle to dodge the token would break testing.md's literal-oracle rule, so it stands.
- **Plan entry 13 (the preservation guard) reads red:** also the plan's wording, on the same word (`inputs#I4`).
  - Reason: it excludes only research's listed files, while the fix-loop protocol's Trigger 3 makes a companion edit
    of a test pinning the changed data mandatory and recorded.
  - The guard's red is exactly the two recorded companions (`git diff --name-only` outside its exclusions:
    `tests/contract_fixture_hygiene.rs`, `tests/support/home.rs`). No other source, test, schema, plugin, script,
    workflow, config, manifest or lockfile changed.
- **Five shape-pinned sites research's sweep did not list** were fixed in place: the three `ledger.rs` cases and the
  two companions named under Spec claims disproved.
- **Count moves beyond plan step 5's list**, each forced by the fifth paste:
  - `cli_verify.rs`: the identity-strip hook count 35 → 37, the Stop-hook count 8 → 9, the screen-less summary
    `6 pass  12 fail`, the dialog-less summary `14 pass  4 fail`;
  - the tag-turn-modal case now reads `16 pass  2 fail`, with `paste-newline` failing too — the newline text sits
    behind the same input-box guard as `/clear`;
  - the `--record` case lists `UserPromptSubmit.paste-3.json` and asserts its prompt.
- **Step 2's cases:** `clean_trusted` gains the newline turn, so the clear-row and framing-variant cases' indices
  moved by two. `FRAMING_ROWS` gains `PasteNewline`; `framing_variants_name_the_two_prompts_…` is renamed
  `…_the_three_prompts_…`.
- **Step 6, the `ProbeDir::typed` kill case,** builds `ProbeDir` by struct literal, not `ProbeDir::create`. Under the
  `Default::default()` mutant `create` would resolve `plugin/` against the test cwd (the repository root) and could
  overwrite the committed `plugin/hooks/hooks.json`. Each mutant run selected only its killing case;
  `git status --short -- plugin` read empty before and after.
- **Step 6, a third replay case** pins the loop's end with no call recorded
  (`replay_dialogs_replays_nothing_for_a_stem_with_no_call`).
- **The live-start ledger** (`evidence/live-starts.md`, `inputs#I5`): one `viola verify` process makes the five
  starts in sequence, so each carries the one `date -u` time read just before the firing (2026-10-13T21:32:02Z;
  the round started 21:32:13Z).
- **Scope record** (`gate.py scope`: `clean — changed 12 · listed 10 · recorded 2 (companion 2 · mechanical 0 ·
  in-intent 0 · widening 0) · absorbed 0 · excluded 66`):
  - companion: `` `tests/contract_fixture_hygiene.rs` · companion · serves `fixtures/claude/2.1.287/UserPromptSubmit.paste-3.json` · self ``
  - companion: `` `tests/support/home.rs` · companion · serves `crates/viola-agent-claude/src/ledger.rs` · self ``

## Decisions & corrections
- **The founder's cap kept to the letter.** `inputs#I5` carries the cap (`inputs#I2`, fork 2): five live starts, one
  `--record` round on 2.1.287, fired once, never retried, each start ledgered before it.
  - The round fired once through `gate.py run --live-legs`: `round: COMPLETE · legs fired 1/1`,
    `stamped 2.1.287  18 pass  0 fail`.
  - The ledger was written at 2026-10-13T21:32:02Z, before the 21:32:13Z start.
  - No other `claude` start was made: the version check of the binary was `test -x` only. The harness rule
    (verification-harness.md 2026-10-10) would have the rig read `claude --version` by path, which is a start; it was
    not done, by the plan's own note.
- **The operator's word on entries 12 and 13** (`inputs#I4`): both are plan wording faults, recorded as deviations,
  oracle and guard unchanged.
- **The operator's word on the route:** mint "Owed ledger rows" directly after this entry from `p5-card-1.md`
  (`inputs#I4`, carrying the founder's fork-3 answer `inputs#I2`). For P5.
- **Sweep hazard — a count literal can be a correct new count.** A stale-literal probe over `17 pass` matches the
  eighteen-row oracle `17 pass  1 fail` (17 pass + 1 fail). A count probe needs the shape it retires (`/17]`,
  `; 17]`, `17 pass  0 fail`), never the bare `N pass`. Evidence: entry 12's one hit, `tests/cli_verify.rs:1093`.
- **Sweep hazard — a row count is pinned by shape as well as by literal.** A row added to `LedgerRow::ALL` reddened
  five sites no count-literal sweep finds: a `[bool; N]` verdict vector, a stamp's rows JSON object, an older-stamp
  oracle, a dialog-less summary `13 pass  4 fail`, and a framing-variant file list. List the readers of `ALL` and of
  the fixture set, not the literals. Evidence: the unit red (3 cases) and the integration and pre-push reds (2 cases)
  in `.andromeda/runs/2026-10-13T21-22-15-implement/`'s gate trail.
- **Plan defect class — a preservation guard excluding only research's lists contradicts the mandatory recorded
  companion edit.** Trigger 3 requires the edit; the guard reds it by construction. P5's controls did not try a
  recorded companion. Evidence: entry 13 red on exactly the two `scope-record.md` files.
- **Hazard — a hand-applied mutant of a path accessor can write into the repository.** Under
  `ProbeDir::typed` → `Default::default()`, `ProbeDir::create` resolves the run's `plugin/` against the test process's
  cwd, the repository root under nextest, where the committed `plugin/` lives. The killing case builds the struct
  directly, each mutant run selects only its case, and `git status -- plugin` is read before and after
  (`evidence/carry2-kills.md`).
- **A mutant that turns a loop's exit condition false spins its other case.** Under `delete !` in `replay_dialogs`,
  the case of the other dialog event loops with no end. Each mutant run selected only the case that reads it red on
  an assertion. Evidence: `evidence/carry2-kills.md` c1, c2.
- **Two Bash-guard refusals**, each re-issued once:
  - a `cd` into the chunk's `inputs/` dir (it moves the session cwd), re-issued with absolute paths;
  - a call carrying a `cat` heredoc redirected to a file, done with the Edit tool instead.
- **Hygiene caught a temp-dir spelling** in `evidence/live-round.md:59`: a census pattern written with the temp
  dir's root path. It was rewritten as prose ("the OS temp dir's `viola-verify-*`"), and hygiene then read clean
  (`evidence/operator.md`).
- **Standing rules kept:**
  - the rustfmt run in the same call as a scripted `.rs` write (`count18.py`, each mutant);
  - `/proc/<pid>/exe` to tell this tree's `viola` from another's (seven `viola` processes, all another tree's
    prototype build);
  - pipeline output to a file, read whole.

## Outcome
- **Acceptance criteria**, re-asserted against the diff:
  - (arch) `LedgerRow::ALL` holds eighteen rows, the eighteenth `paste-newline`, checked by
    `pasted(trusted, PROBE_NEWLINE_PASTE)`. A seventeen-id stamp, all `pass`, reads unverified; eighteen read
    verified (`stamps_verdict_needs_the_dialog_rows`, `verified_reads_a_fourteen_row_stamp_as_unverified`; unit
    green) — **met**.
  - (security) Run B pastes exactly five compiled texts, each only into a settled input box, none after the stop; no
    byte typed into a dialog. Evidence: the `typed.rs` list, and `verify_window_paste_hint_…` and
    `verify_without_framing_…` asserting the five typed texts in order; the widening is the founder's own answer
    (`inputs#I2`) — **met**.
  - (tests) The `check_paste_newline_` filter reads `"suite":"nextest-unit","passed":3,"failed":0` — **met**.
  - (tests) One live round, five starts, fired once, prints `stamped 2.1.287  18 pass  0 fail`; `paste-3` equals the
    compiled text byte for byte; the census is clean (`evidence/live-round.md`) — **met**.
  - (tests) `fixtures/claude/2.1.287/` gains `UserPromptSubmit.paste-3.json` and no other change (the fixture guard
    green). `contract_ledger_probes` stamps 2.1.287 at `18 pass  0 fail`; drift and hygiene pass it (integration
    94/94) — **met**.
  - (design / layouts / a11y) eighteen `[NN/18]` lines, the last `[18/18] paste-newline …`, then `stamped …  18 pass
    0 fail`, plain ASCII, the interrupted pair unchanged (`cli_verify`) — **met**.
  - (obs) six spawn pairs, subjects in the closed enum, schema-valid, no content in a line (`cli_verify`, G4) —
    **met**.
  - (tests) `LEDGER_ROWS` names eighteen ids and the `LIVE_PASS` unit proof reads eighteen (`run --unit`) —
    **met**.
  - (tests) CARRY 2's four mutants each red by an assertion and green restored (`evidence/carry2-kills.md`); no
    mutation run — **met**.
  - (security) No new dependency, and `cargo deny check` passes — **met**. "No file outside the modify set changes"
    is **not met as worded**: two recorded companion test files changed. That is the entry-13 deviation, routed by
    the operator's word (`inputs#I4`) as a plan wording fault; no product, schema, manifest or workflow file outside
    the set changed.
  - (tests) The pushed HEAD's CI run reads `verdict: green` on all three OSes (`ci#38472492871`, sha `16b5f5f4006b`)
    — **met**.
- **Gates** (implement run `.andromeda/runs/2026-10-13T21-22-15-implement/`, final firings; an entry named by its
  `run`):
  - `cargo fmt --all --check`: green.
  - `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`: green.
  - the G1 instrument `rg`: green (exit 1, no output).
  - `bash scripts/lint-probes.sh`: green.
  - `bash scripts/agent-run.sh run --unit`: green (firing 3, after two reds — the three ledger shape cases).
  - `run --unit --filter 'test(/check_paste_newline_/)'`: green (`passed 3, failed 0`).
  - `test -x …/2.1.287/claude` (round): green.
  - the `verify --record` entry (live): green, `stamped 2.1.287  18 pass  0 fail`.
  - the census (round): green, no output.
  - The round: `round: COMPLETE · legs fired 1/1`, recorded in `evidence/round-213213Z.txt`.
  - the fixture guard: green.
  - `run --integration --filter 'binary(cli_verify) | …'`: green (94 passed in 34.7 s; red at firing 1 on the
    hygiene companion).
  - the seventeen-literal probe: **red · last line '0' ✗ (last: '1')**. The plan wording fault (`inputs#I4`),
    recorded as a deviation — not a tree defect.
  - the preservation guard: **red · exit 0 ✗ (exit 1)**. The plan wording fault (`inputs#I4`), recorded as a
    deviation — the two recorded companions.
  - `cargo deny check`: green.
  - the harness smoke `cleanup` / `boot` / `status` / G2 `g2-zero-panics.sh` / G4 `schema-check` / `cleanup`: green.
  - `bash scripts/agent-run.sh pre-push`: green (firing 2: coverage 2475 passed 0 failed, playwright 1, gate no
    breaches; firing 1 red on the two companions).
  - Operator entries: `gate.py hygiene` green (clean after one rewrite); the push green (history moved
    `origin/build/viola-0.1.0` 19483111→16b5f5f4); `ci.py conclusion` green (`ci#38472492871`, 15/15, 572 s). All in
    `evidence/operator.md`.
- **No defer and no skip.** The integration durations were read as the plan asked: 34.69 s against 35.4 s at P5; no
  kill line moved.
- **Smoke:** ✓ by hand (`p3-pnl-smoke`: boot exit 0, status `ready`, cleanup `processes_gone` and `endpoint_gone`
  true).
- **Watches:** none folded.
- **Outcome basis:**
  - the operator pass's final state: the commit list above, and the final HEAD's CI run `ci#38472492871` on
    `16b5f5f4006b`, recorded in `evidence/operator.md`;
  - implement's P4 report, held whole in this window and persisted as
    `.andromeda/runs/2026-10-13T21-22-15-implement/outcome-2026-10-13-paste-newline-ledger-row.md` (outcome
    `surfaced`, the two plan entries);
  - the operator's word between implement and this report, `inputs#I4`, which settles that surface.
- **Process hygiene** (implement's census, re-measured at this wrap): the five `claude` 2.1.287 starts and
  `viola verify --record` (a leg) are terminated; no `claude` has a probe-dir cwd. The harness sessions `p-pnl-smoke`
  and `p3-pnl-smoke` (this run) are terminated (cleanup `processes_gone:true`). `viola` pids 12692, 13416, 15370,
  16532, 627411, 1552695, 1585891 are left running: another tree's build (`viola-lab/prototype`, by
  `/proc/<pid>/exe`), not this chunk's.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 19483111 (the parent of the oldest pre-CI commit 16b5f5f4) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### crates/viola-agent-claude/src/ledger.rs — added 134 line(s) in 42 range(s)
added: 38 · 42 · 60 · 83 · 109 · 145-146 · 164-165 · 168 · 717-720 · 722-727 · 729 · 788-789 · 1225 · 1247 · 1271 · 1535
       1688 · 1809-1811 · 1838-1843 · 1983 · 2426-2432 · 2442-2443 · 2451 · 2455 · 2506-2554 · 2559 · 2562 · 2564 · 2566
       2569 · 2572 · 2575 · 2590 · 2608-2610 · 2613-2614 · 2616 · 2618 · 2623 · 2627-2635 · 2639 · 2663-2671 · 2674
  - 722-727 «let [»
  - 2426-2428 @2427 «const NEWLINE_PROMPT: &str = "viola verify probe: this paste holds one line break here\nand \»
  - 2506-2515 @2507 «fn check_paste_newline_passes_on_the_one_prompt_holding_the_lf() {»
  - 2517-2536 @2519 «fn check_paste_newline_fails_when_the_lf_split_the_prompt() {»
  - 2538-2553 @2539 «fn check_paste_newline_fails_without_a_matching_capture() {»
### crates/viola-e2e/src/harness/run.rs — added 33 line(s) in 5 range(s)
added: 333 · 351 · 915-933 · 1003 · 1006-1016
  - 915-933 «const LIVE_PASS: &str = "[01/18] shim-resolution claude resolves  pass\n\»
### fixtures/claude/2.1.287/UserPromptSubmit.paste-3.json — new file · 1 line(s)
### src/bin/viola-fake-agent.rs — added 53 line(s) in 1 range(s)
added: 1416-1468
  - 1416-1452 @1418 «fn replayed(events: &[&str]) -> Vec<String> {»
  - 1454-1457 @1455 «fn replay_dialogs_replays_a_call_holding_only_a_pre_tool_use() {»
  - 1459-1462 @1460 «fn replay_dialogs_replays_a_call_holding_only_a_permission_request() {»
  - 1464-1467 @1465 «fn replay_dialogs_replays_nothing_for_a_stem_with_no_call() {»
### src/cmd/verify.rs — added 31 line(s) in 7 range(s)
added: 545-546 · 549-550 · 553-554 · 557-558 · 561-562 · 565-572 · 631-643
  - 631-642 @633 «fn probe_dir_names_each_run_its_own_dir_inside_it() {»
### src/cmd/verify/typed.rs — added 8 line(s) in 5 range(s)
added: 6-7 · 28-29 · 109 · 302-303 · 310
### src/run/version_gate.rs — added 15 line(s) in 3 range(s)
added: 260-263 · 302-307 · 312-316
### tests/cli_verify.rs — added 78 line(s) in 38 range(s)
added: 2 · 26-44 · 46 · 64 · 97 · 102 · 151 · 165 · 168 · 171-182 · 254 · 257 · 288 · 323 · 407 · 546 · 584-588 · 619
       735 · 973 · 985 · 996 · 1020-1021 · 1026-1027 · 1040 · 1077 · 1092-1093 · 1098 · 1102 · 1110 · 1118 · 1139-1142
       1162 · 1170 · 1186-1187 · 1197 · 1207 · 1254
  - 584-588 «assert_eq!(»
### tests/contract_fixture_hygiene.rs — added 2 line(s) in 2 range(s)
added: 364 · 401
### tests/contract_ledger_probes.rs — added 3 line(s) in 3 range(s)
added: 16 · 34 · 103
### tests/support/home.rs — added 4 line(s) in 2 range(s)
added: 375-377 · 386
### tests/support/verify.rs — added 12 line(s) in 5 range(s)
added: 71-75 · 81-83 · 89-90 · 99 · 429
- 71-74 @73 «pub const NEWLINE_PASTE: &str = "viola verify probe: this paste holds one line break here\nand \»
