# Report — 2026-10-19-overseer-switch-to-viola

**Chunk:** Overseer switch to viola — the overseer drives its builder through the product viola instead of the prototype; v1-33 proven live
**Date:** 2026-10-19
**Commits:** `aa965127 chore(2026-10-19-overseer-switch-to-viola): operator pre-CI commit, for the run this chunk's verdict reads` (the one commit since `last_wrap` 2026-10-19T09:56:16Z; `git log --format='%h %s' f23586fb..HEAD`)

## Changes (structured — detectors read this)
- **Files:** six source/test files (research's Files to modify, all six; `gate.py scope`: `clean — changed 6 · listed 6 · recorded 0`):
  - `crates/viola-agent-claude/src/dialog.rs`
  - `crates/viola-agent-claude/src/hook.rs`
  - `crates/viola-core/src/lib.rs`
  - `crates/viola-state/tests/state_events.rs`
  - `tests/hook_events.rs`
  - `src/run/dialog.rs`

  The chunk folder (`plan.md`, `research.md`, `scope.md`, `evidence/{switch-round.md, switch-readings.ndjson, profraw-red.md, operator-pass.md}`, `inputs/` I1–I20) and `viola-0.1.0/verification-matrix.json` (v1-33) are bookkeeping. No master, key file, leaf or fixture was edited.
- **Symbols / APIs:**
  - **`question` event data** (`viola_agent_claude::dialog::question_data`, private; `dialog.rs` 151-156 per the listing below):
    - Each question gains `option_details`: one `{description, preview}` per option, parallel to `options`. Each field is the option's string or `null`.
    - A missing or non-string value gives `null` and never refuses the dialog.
    - `options` keeps its `[label]` shape.
    - The data reaches every consumer of the question data: `events.ndjson`'s `question` line, `hook.dialog`'s frame, CLI `wait --json` and the MCP `wait` result. Human `wait` lines are unchanged.
    - The doc comment at `dialog.rs:119-120` states the new shape.
  - **`turn-ended` event data** (`viola_agent_claude::hook::data_of`, Stop arm; `hook.rs` 170-173):
    - It gains `background_tasks`: the Stop payload's `background_tasks` array length, or `null` when the key is absent, `null` or not an array.
    - No element is read or carried: `Known.background_tasks` is `Option<Vec<serde::de::IgnoredAny>>` (`hook.rs` 104-105).
    - A non-array value writes a drift entry `{"path":"background_tasks","expected":"array"}` (`hook.rs` 147-152). Before this chunk every drift entry's `expected` was `string`, and the doc comment of `Known` (`hook.rs:92`) is reworded.
  - **`viola_core::EventKind::TurnEnded` data keys** (`lib.rs:119`): `["last_assistant_message", "background_tasks"]`. The `KIND_TABLE` pin follows at `lib.rs:550`.
  - No IPC method, CLI verb, flag, env var, port, socket or exit code changed. No function signature changed, so every caller is unchanged.
- **Crates / modules:** changed `viola-agent-claude` (`dialog`, `hook`) and `viola-core` (`EventKind` data keys); added none, removed none.
- **Dependencies:** none (`Cargo.toml` / `Cargo.lock` untouched; `serde::de::IgnoredAny` is in serde already pinned).
- **Schema / config:**
  - Two additive data keys on product event lines: `question.data.questions[].option_details` and `turn-ended.data.background_tasks`. There is no `v` bump; readers skip unknown fields.
  - `background_tasks` is a count only, by the founder's "count only" (`inputs#I14` Q4), and no task text crosses.
  - `option_details` carries the CLI's option `description` / `preview` text into `events.ndjson`, `wait --json` and the MCP `wait`. That is upstream text crossing a boundary: the "Boundary widening" (playbook) the founder answered himself at P4 fork Q4 (`inputs#I14`). The plan expects ONE halt on it at this wrap, as the answer card.
  - No config key changed. The overseer's home `~/.viola/config.json` carries `dialog_hold_s` 3600 (O2, the overseer's, outside this repo).
- **Spec-master edits:** none.
- **Retired:** none.
- **Counts / qualifiers moved:**
  - **`turn-ended`'s data keys, `{last_assistant_message}` → `{last_assistant_message, background_tasks}`.** Pattern `turnended` (`turn-ended`): 36 standing.
    - `architecture.md:255` (`- \`turn-ended\`: \`{last_assistant_message}\` (string or \`null\`)`): **stale**.
    - `architecture.md:230`: an example `hook.event` frame whose `data` holds only `last_assistant_message`. A Stop's frame now carries `background_tasks` too: **stale example**.
    - The other 34 are true claims sharing the token (the wake set, budget triggers, turn running/ended, CLI line forms): no-change.
    - Pattern `bgtasks`: control unfired over the masters at aa965127 (0 sites; the refusal line `control: bgtasks … never fired over the seven masters' pre-pass text (aa965127)`).
  - **`question`'s per-question data, `{question, header, options:[label], multi_select}` → `+ option_details`.** Pattern `qdata` (`multi_select`): 1 standing, `architecture.md:256`, **stale**. Pattern `optdet`: 1 standing, `security-plan.md:261` ×3 ("option text" of the founder's dialog rulings): a true claim sharing the token, no-change.
  - **A drift entry's `expected` is no longer always `string`** (`array` for `background_tasks`). Pattern `driftexp`: control unfired (0 master sites, the same refusal line). `no master site` beyond the code comment, which is reworded.
  - **The newest stamped CLI.** The verify printed `stamped 2.1.296  24 pass  0 fail` in `~/.viola`, on the Linux dev host, 2026-10-19T11:10:29Z, exit 0 (`evidence/switch-round.md`).
    - Pattern `stamp296`: control unfired (0 master sites).
    - Pattern `stamp287` (`stamped (on )?2.1.287|2.1.287 \(24|24 pass  0 fail`): 12 standing. Every row is a dated historical reading ("stamped on 2.1.287 at chunk …"), still true: no-change.
    - `design-system.md:506` and `layout-templates.md:193` show `stamped 2.1.287  24 pass  0 fail` as the verify wireframe's example line: no-change (an example).
  - **claude 2.1.302:** `viola verify` (the overseer's, 18:18:43Z → 18:21:41Z) printed `stamped 2.1.302  20 pass  4 fail` (`inputs#I20`). The fail rows are `tag-escaping`, `local-command-clear`, `paste-newline` and `local-command-compact`. Pattern `cli302`: control unfired (0 master sites). See Spec claims disproved: no master claims 2.1.302. It has no owner on the route (P5).
  - **The 3600 s ceiling's first live reading.** Pattern `hold3600`: 1 standing, `architecture.md:77` ("A hold near the 3600 s ceiling is first read live at "Overseer switch to viola" … the ceiling is PROVISIONAL").
    - No hold near the ceiling arose in this chunk: the one dialog hold measured is 27 345 ms (`evidence/switch-readings.ndjson` `review-answered`).
    - **Stale placement:** by the P5 card's row 2 (below) the reading re-pins to "Version done-check". The title pattern `title` (`Overseer switch to viola`, ignore_case): 1 standing, the same `architecture.md:77`.
  - **v1-33's state.** The overseer drives viola-builder through the product: proven live, `implemented` with a dated clearance. Pattern `prototype`: 8 standing. Every row is history or rationale ("the prototype proved …", "the viola-lab prototype's live captures"): no-change. `architecture.md:51` "the prototype driver uses both every session" is a rationale for `answer`/`list` and still true of the driver's habit: no-change (detectors may judge it).
  - **Sweep reach:** 15 patterns · 15 moved facts, 15 with a pattern (the report's own count) · owner pattern 1 standing · by line 14 lines · 35 offsets.
    - The `total` lines: 9 + 2 + 4 patterns fired. Five more (`bgtasks`, `stamp296`, `cli302`, `driftexp`, `owner`) were refused unfired, their refusal line the basis of their zeros.
    - The by-line blocks: 5 lines / 10 offsets, 4 lines / 14 offsets and 5 lines / 11 offsets.
    - The generic owner phrase `owner` (`owned by a (route entry|CARRY)`) is unfired, so it is dropped.
    - Trail: `.andromeda/runs/2026-10-19T19-16-36-wrap/cascade-2026-10-19-overseer-switch-to-viola-pre-pass.json`.
- **Dev-tool versions:** **`claude`** (the driven CLI) on the Linux dev host: the role noun is "the installed `claude` CLI".
  - 11:09Z: 2.1.296 (mise `claude/latest` → `2.1.296`; read by `claude --version`).
  - A host restart about 17:29Z brought 2.1.302 to the overseer's launcher: start 8 at 18:07:34Z ran `cli_version 2.1.302`, `cli_verified false`.
  - At 18:26:23Z the founder's pin returned it to 2.1.296. It is a PATH shim `~/.local/viola-claude-pin/bin/claude` → `…/mise/installs/claude/2.1.296/claude`, first on `vb.sh`'s PATH (`inputs#I16`; the I2 drift below).
  - No lockfile resolves it, and no crate shares its stem.
- **Harness / gate surface:** none. Evidence readers only: three `grep -c` probe entries of the plan over the chunk's evidence files.
- **Cross-project / external claims:**
  - **The CI run on this chunk's push.** `ci#39305131020` on `aa965127d51e`: `verdict: green · checks 15/15 · wall 667 s`, run `completed/success` (`evidence/operator-pass.md`). Its jobs: fuzz-replay, lint ×3, msrv, perf ×3, release ×3, supply-chain, test ×3 (macOS, Ubuntu, windows-2025). The wrap commit adds bookkeeping on top of that tree.
  - **The overseer's own steps O1–O6,** outside this repo, reported by the overseer and recorded in `evidence/switch-round.md`. The overseer later added a 2.1.296 PATH pin to `vb.sh` (I16).
    - O1: the product binary at `~/.local/viola-product/bin/viola`, sha256 equal to step 7's `231c0225…db52e3755`.
    - O2: `dialog_hold_s` 3600.
    - O3: `vb.sh` / `vbr.sh` gained `--product` through `vstart.sh --product` (andromedaV3 `0f551a2`).
    - O4: the `up.sh` row.
    - O5: `pwait.sh` (`wait --json`).
    - O6: the switched start at 13:26:43Z.
  - **The inputs** (`inputs.py verify`, wrap P1; 20 entries: unchanged 10 · drifted 5 · n/a 4 · I20 snapped after the verify):
    - I1 · message: the operator, 2026-10-19T10:15Z · copy · n/a, cited.
    - I2 · `../additional/viola-overseer/vb.sh` · copy · **DRIFT**: the start line gained `--product` (O3), and `~/.local/viola-claude-pin/bin` went first on PATH (the founder's 2.1.296 pin, I16).
    - I3 · `../additional/viola-overseer/vbr.sh` · copy · **DRIFT**: `--product` added (O3).
    - I4 · `waitm.sh` · copy · unchanged.
    - I5 · `waitlong.sh` · copy · unchanged.
    - I6 · `scr.sh` · copy · unchanged.
    - I7 · andromedaV3 `andromeda-overseer/docs/tools/vstart.sh` · pointer@8d053dcd · **DRIFT**: committed since. HEAD is 4ca52e5a, and `0f551a2` added `vstart.sh --product` (O3).
    - I8 · andromedaV3 `…/up.sh` · pointer@8d053dcd · **DRIFT**: `0f551a2`, the viola-overseer row (O4).
    - I9 · `driving-guide.md` · copy · unchanged.
    - I10–I13 · the other launchers sharing `vstart.sh` · copy · unchanged: they keep the prototype form.
    - I14 · message: the founder's four P4 answers, relayed by the overseer 10:41Z · n/a, cited.
    - **I15** · message: the operator's P5 word «yes», 10:51Z · n/a. Cited here as `inputs#I15`. It names the P5 review card `.andromeda/runs/2026-10-19T10-15-34-phase/p5-card-1.md`, whose `## Beyond the plan` rows read verbatim, on the operator's word (not the founder's; I15: "It is not the founder's word"):
      > 1. **CARRY on "Windows-only live measurements":** the Windows readings of claude 2.1.296's stamp, and of the two new
      >    event keys on a live CLI. This chunk reads Linux only; CI covers Windows under the fake agent.
      > 2. **CARRY re-pin, only if no hold near the ceiling arises naturally in this chunk:** the 3600 s ceiling's first live
      >    reading stays PROVISIONAL and moves from this entry to "Version done-check".
    - **I16** · `../additional/viola-overseer/relay-switch-step12.md` · copy · **DRIFT**: after the snap the overseer added one line naming the kept verify output (`verify-2.1.302-20261019.out`, now I20). The facts I16 carries are unchanged.
      - Cited here as `inputs#I16`: the turn-waited, final-read and clear-confirmed readings, and the host restart.
      - The founder's words «Держать до смены аккаунта», «Verify 2.1.302, +6 стартов» and «Закрепить 2.1.296, +1 старт», with his own `viola release --budget`.
    - **I17** · message: the overseer's re-entry send, 18:32Z · n/a. Cited here as `inputs#I17`: re-enter at step 12.
    - **I18** · `../additional/viola-overseer/relay-switch-step14.md` · copy · unchanged. Cited here as `inputs#I18`: the takeover readings.
    - **I19** · `../additional/viola-overseer/wrap-switch-directive.md` · copy · unchanged, snapped at this wrap. Cited here as `inputs#I19`: the outcome basis, the complete arm, Epoch 5's last entry, and the three surfaced items.
    - **I20** · `../additional/viola-overseer/verify-2.1.302-20261019.out` · copy, snapped at this wrap. Cited here as `inputs#I20`: `stamped 2.1.302  20 pass  4 fail`, rows 16, 17, 18 and 24 `fail`.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **obs-plan §4 Scenario "Dialog → `answer`", `obs-plan.md:610`:** "`dialog-answered{corr:<dialog_id>, dialog_kind, from, from_trust, deadline_hit, duration_ms}`, `from` / `from_trust` present on an answer and absent on the expiry line".
     - Measured: the overseer's `viola answer viola-builder 1` from a process without `VIOLA_NAME` sent no `from`. The answered line is `dialog-answered{corr 1, dialog_kind question, duration_ms 27345, deadline_hit false}` with **neither `from` nor `from_trust`** (`evidence/switch-readings.ndjson` `review-answered`; the run log line at 13:30:18.345Z).
     - `obs-plan.md:604` itself names "one with no `from`" as an answer shape.
     - Pattern `dafrom` (`dialog-answered[^\n]{0,80}\bfrom\b`): 7 standing.
       - Disproved: `obs-plan.md:610`.
       - True claims: `obs-plan.md:604` (it names the no-`from` answer); `obs-plan.md:615` and `test-plan.md:525` (the expiry line with no `from`).
       - To check: `obs-plan.md:747`, the line-field table listing `from` among `dialog-answered`'s fields; the two key files `log-format-json-schema.md:19` and `log-format.md:13` (a kind list).
     - The same fact bears on v1-33's acceptance ("a `dialog-answered` with `from`"), a matrix entry: `no master site`, see Outcome.
  2. **v1-33's acceptance "`send-confirmed{confirmed:true}`"** (the matrix, not a master). The events line of a confirmed send is `send-confirmed{cursor}`, with no `confirmed` key, which architecture §Standard Contracts states (`architecture.md:265`: "A confirmed send writes `{cursor}`"). The run diagnostics' `send-confirmed` line carries `confirmed: true` (@465 and @9694, `run-viola-builder.ndjson`).
     - Pattern `confirmed`: 7 standing, all consistent with the measurement (`architecture.md:46/139/227/265`, `test-plan.md:668`, `obs-plan.md:695`, key file `snapshot-paste-to-ai-integration.md:13`): no master disproved.
     - The acceptance's wording is imprecise: `no master site — a matrix entry`, routed to P7.3.
  3. **An unstated residual of `viola verify`:** each real-CLI verify leaves one more `~/.claude/projects/` entry.
     - The entry is `-home-<user>--viola-ledger-probes-<pid>`, holding only an empty `memory/` dir and no transcript. It is the CLI's project dir for the print-mode probe's cwd `<home>/ledger/probes/<pid>/` (`src/cmd/verify.rs:311-313`).
     - Measured twice on the Linux dev host: pid 635187 on 2.1.296 and pid 3090040 on 2.1.302 (`find ~/.claude/projects -maxdepth 1 -newermt …`; `evidence/switch-round.md` §CLI residual).
     - Per verify, `~/.claude/projects/` gains 4 entries: Run B's, C's and D's dirs and this one. The transcript files grew 70 → 75 on the 2.1.296 verify, five per round as `architecture.md:110` states.
     - Pattern `residual` (`accepted residual|adds no transcript`): 2 standing. `architecture.md:110` names Runs B–E's residual and the print probe's plugin dir but not this entry. `security-plan.md:261` says "Run A leaves nothing" and "viola itself writes nothing under `~/.claude`", both still true: it is the CLI's write, for the print probe, not Run A. Neither line states this entry: **an omission, not a falsehood**.
  4. **The plan's step-8 premise, not a master's.** The verify was to leave `~/.viola` at 0700. It read `755`, because the verify opens no home log. It narrowed to `700` at the first `run` start (`evidence/switch-readings.ndjson` `verify-stamped`).
     - Pattern `homenarrow`: 4 standing (`architecture.md:290`, `:360`; `security-plan.md:185` ×2; key file `otel-sdk-init.md:22`). All say "opening a log sets an existing home to 0700", consistent with the measurement: no master disproved.
- **Expected amendments (from plan):**
  - **architecture §Standard Contracts, the `question` data's `option_details` and `turn-ended`'s `background_tasks` (count), with the halt as the answer card:** carried in Symbols / APIs and Counts / qualifiers moved. Sites: `qdata` 1 (`architecture.md:256`) and `turnended` (`architecture.md:255`, `:230`) of 36 in architecture.
  - **security-plan §Input Validation / §Data Protection, the same widening, the same answer:** carried in Schema / config. Sites: pattern `qtext` (`question text|question, plan|plan or tool text|upstream text`), 22 standing.
    - security-plan has 10. The candidates are `security-plan.md:390` (the NEVER-log floor's scope: `events.ndjson` and `wait` results carry content by design, and "plan and question text" stays out of logs) and `:527`. The others (`:82`, `:89`, `:208`, `:385`, `:387`, `:451`, `:457`, `:528`) are true claims sharing the token.
    - Architecture: `:252`, `:435` candidates.
    - No-change: obs `:147`, `:332`, `:615`; design `:178`, `:484`, `:562`; layout `:329`.
  - **obs-plan §4 Scenario: `wait` / `last` event-driven readback, the two additive keys:** carried. Pattern `readback`: 28 standing, obs-plan `:282`, `:587`, `:597`. The scenario heading is `obs-plan.md:587`.
  - **test-plan §6 Paths 3–4, the new cases by name, and the switch's live readings stated as a local measurement:** carried (Outcome lists the cases). Pattern `paths34`: 9 standing, test-plan `:673` (Path 3), `:693` (Path 4), `:521`, `:1212`.
  - **a11y-plan §3 Keyboard test harness, the takeover reading on the product once step 14 is read:** carried (Outcome, step 14). Pattern `takeover`: 24 standing.
    - The site is the key file `registries/contracts/a11y-plan/keyboard-test-harness.md:3` @c2195 ("The takeover clause has one live reading on Linux …", a `wtype` key on claude 2.1.287). This chunk adds a second: the founder's own key on claude 2.1.296 under the product.
    - Also a11y-plan `:120`, `:225`, `:227` (no-change candidates).
  - **v1-33's dated note** rides its claim at P5 (`--note-file`), not a wrap ledger note. Written at phase P5, so not carried here. P7.3 adds the premise disposition (Spec claims disproved 1–2).
- **Coverage of new surfaces:**
  - `question.data.questions[].option_details`:
    - validation: ✓ by type check (a non-string becomes `null` and never refuses; the input is already bounded by `MAX_FRAME` on the hook's stdin).
    - instrumentation: n/a (no new op; existing `dialog-raised` lines carry no text).
    - PII: by design the upstream option text lands in `events.ndjson` / `wait` / MCP `wait`, which are content payloads, never a log line (security-plan floor scope). This is the widening the founder answered (`inputs#I14` Q4).
    - tests: unit (`question_option_details_carry_description_and_preview`, `question_option_details_are_null_when_absent_or_not_a_string` ×3) and integ (`hook_a_recorded_question_carries_its_option_details`, `state_events_background_tasks_and_option_details_are_known_keys`); live under the product on 2.1.296 (`question` @3700 with descriptions and previews present).
    - a11y: n/a. tokens: n/a.
  - `turn-ended.data.background_tasks`:
    - validation: ✓ (an array length only; `IgnoredAny` reads no element; a non-array drifts `expected:"array"`).
    - instrumentation: n/a.
    - PII: ✓ (a count; a canary in a task's `description` never reaches the data, asserted by the `turn_ended_background_tasks_counts_the_array` case).
    - tests: unit (`turn_ended_background_tasks_counts_the_array` ×3, `turn_ended_background_tasks_is_null_when_absent_or_not_an_array` ×5) and integ (`hook_a_recorded_stop_carries_its_background_task_count`, the `state_events` case); live (`turn-ended` @5489 `background_tasks` 0).
    - a11y: n/a. tokens: n/a.

## Deviations from intent
1. **The live-start cap went from 8 to 15.** A host restart (about 17:29Z) spent start 8 on claude 2.1.302 (`unverified-cli`). The founder raised the cap twice, both relayed by the overseer (`inputs#I16`): «Verify 2.1.302, +6 стартов» (starts 9–14, the 2.1.302 verify), then «Закрепить 2.1.296, +1 старт» (start 15, under the PATH pin). The builder made none of starts 7–15.
2. **The step-12 `/clear` needed a second try.**
   - The product's budget gate refused the first `/clear` at 13:31:52Z, `send-refused{budget-paused, seven-day}` @8046 at 99 %. That is the gate firing as designed.
   - The founder ruled «Держать до смены аккаунта» and later ran `viola release --budget viola-builder` himself (@9519, override until 2026-10-23T14:00Z).
   - The confirmed `/clear` came at 18:31:57Z: @9694 → `session-start{clear}` @9938, a new id → `send-confirmed` @10127.
3. **The dialog-hold reading is far below the ceiling.** The only hold was 27 345 ms, so the ceiling's first live reading did not arise. By the P5 card's row 2 (the operator's word, `inputs#I15`) it re-pins to "Version done-check" (P5).
4. **`dialog-answered` carries no `from`** (Spec claims disproved 1). The overseer's answering process had no `VIOLA_NAME`. The same holds for every `send` in the chunk (`send-issued` without `from`), so no implicit link was recorded.
5. **The step-8 mode premise** (Spec claims disproved 4): the home read 755 after the verify and 700 after the first run start.
6. **Step 5's fixtures** (`fixtures/claude/2.1.287/`) carry no option `preview` and an empty `background_tasks` array. So the fixture-driven cases read null previews and a count of 0, and the non-null values are held by the unit cases and the live reading.
7. **The live-start ledger's starts 1–6 share one clock reading,** taken before the verify command, not one before each of its six internal spawns.
8. **An extra act line, `restarts`,** joined `switch-readings.ndjson` beside the seven planned acts. It holds the host restart, start 8 on 2.1.302, the 2.1.302 verify, and start 15 on 2.1.296. It is evidence only, outside the reader's regex.

scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 6 · listed 6 · recorded 0 … · excluded 83`).

## Decisions & corrections
- **The founder's own answers to the four P4 forks** (`inputs#I14`, relayed by the overseer 10:41Z):
  - claude 2.1.296 with a cap of 8 live starts.
  - The switch mid-implement.
  - His own key and his own `viola release`, later.
  - Both gaps built, `background_tasks` as a count only; he ratifies the option-text widening.
- **The founder's words between steps 11 and 12** (`inputs#I16`, relayed by the overseer):
  - «Держать до смены аккаунта»: hold the `/clear` until the account switch.
  - «Verify 2.1.302, +6 стартов» (cap 8 → 14).
  - «Закрепить 2.1.296, +1 старт» (cap 14 → 15; a PATH shim pins the CLI for viola-builder only).
  - His own `viola release --budget viola-builder` at 18:31:54Z.
- **The operator's word at P5, «yes» on `p5-card-1.md`** (`inputs#I15`): it approves the plan and the two Beyond-the-plan rows. It is not the founder's word.
- **The operator's direction for the operator pass** (this session, after 18:40Z):
  - Entry 20 until clean, the pre-CI commit (`git add -A`), entry 21 through `--operator 21`, entry 22 as written.
  - Fix commits fast-forward only; stop after entry 22.
  - Then: snap the step-14 relay, record the takeover, `--void 25:block cleared — evidence/switch-readings.ndjson takeover line`, and write the gated clearance.
- **A product finding with no owner: claude 2.1.302 fails four ledger rows** (`inputs#I20`): `tag-escaping`, `local-command-clear`, `paste-newline`, `local-command-compact`. The CLI changed paste and local-command handling. Until an owner rewrites viola against it or the rows' probes, a 2.1.302 home is transport-only (`cli_verified false`). The 2.1.296 pin is the overseer's driving fact, not the product's (`inputs#I16`, `inputs#I19`).
- **A conflict between an acceptance and the inputs rule** (implement surfaced it; `inputs#I19`).
  - The obs acceptance reads "no prompt, skill or dialog text … `last_assistant_message` or `detail-*.ndjson` content anywhere under the chunk folder".
  - The skills' inputs rule snaps every relayed directive verbatim into `chunks/{marker}/inputs/`, so `inputs/I17-relay-1.md.txt` is the 532-byte prompt typed @10391. I1 and I15 are operator messages, which reached the prototype session as prompts.
  - It is a pair of instructions jointly unsatisfiable for a chunk driven through viola.
- **A watched hazard: the overseer's process lacks `VIOLA_NAME`.** A driver CLI process without it sends no `from`, so the answer records no `from` / `from_trust` and no implicit link is made. It is measured here; whose fix it is (the overseer's launcher or a doc) is undecided.
- **A sweep hazard, the grep pattern `turn-ended\{|TurnEnded`:** it reads 0 over the masters. They write the kind in backticks, and its data as `- \`turn-ended\`: \`{…}\``. The plain token `turn-ended` locates the 36 sites.
- **Guard refusals this chunk:**
  - The Bash guard refused one `cd` into the chunk folder at implement step 12 (re-issued with absolute paths). That repeats host-linux.md §Paths.
  - It refused one `cat >> file <<EOF` append at this wrap's site sweep (re-issued through the Edit tool). That repeats the guard's documented arm (host-linux.md 2026-09-28 Extended).
- **The coverage merge refusal at implement's first `pre-push` stays OPEN** (`evidence/profraw-red.md`). It is a `viola hook` killed during its exit-time profile dump: one 118 784 B truncated `.profraw` (pid 552803), with 2958 / 2958 tests passed. The killer is not known. The second firing was green, and that does not close it.
- **The WATCH `run::send::budget::tests::send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4** passed in three nextest runs this chunk (entry 5, entry 19 ×2), with no plain `cargo test` run. That makes this chunk non-advancing for a third consecutive time by the handoff's count.
- **One correction of the chunk's own record:** step 8 attributed 3 of the 4 new `~/.claude/projects` entries. The fourth is the print probe's `*-viola-ledger-probes-635187` (by pid), corrected at step 13 in `evidence/switch-round.md`.

## Outcome
**Acceptance criteria, against the diff:**
- **(arch) `option_details`:** met. In the diff (`dialog.rs` 151-156); held by `question_option_details_*` and `binary(hook_events)`; live @3700.
- **(arch) `background_tasks`:** met (`hook.rs` 170-173, `lib.rs:119`); live @5489, count 0.
- **(tests) unit, integration and E2E `"ok":true`; new cases green; `pre-push` reaches `linux-tests` green:** met on the second firing of `pre-push`. The first firing's red is open (above).
- **(tests) the WATCH green on every run, no retry:** met for the runs made (3 nextest). See the WATCH line.
- **(security) the verify stamp, `cli_verified` true, a non-null answer only on that stamp:**
  - The verify printed `stamped 2.1.296  24 pass  0 fail`.
  - The switched snapshot read `cli_verified true`.
  - `hook-decision{decision_emitted:true}` came on 2.1.296. Met.
- **(security) every exec-form command absolute:** met. 11 of 11 (`hooks.json` 9/9, `.mcp.json` 1/1, `settings.json` 1/1). The plugin's MCP server runs as `~/.viola/bin/0.1.0-231c0225dac912d9/viola mcp` (census).
- **(obs) kinds, cursors, codes and durations only:** met for `evidence/`. **UNMET for `inputs/`:** I1, I15 and I17 hold directive text that reached the session as prompts (Decisions & corrections, the jointly unsatisfiable pair). That is an unlinked criterion, routed as a P2 escalation.
  - Panic lines: 0 in each of `cli-`, `hook-`, `run-viola-builder.ndjson` and `mcp.ndjson` (step 13).
- **(layouts) `wait` ends in a product line or `--json`; `/clear` read back on its post-condition; human line forms byte-identical:** met.
  - `wait --json` woke on `turn-ended`, outcome `turn-ended`.
  - `/clear` → `session-start{clear}` with a new id.
  - No human line form changed (the diff touches no `src/human/` or CLI print site).
- **(a11y) the passthrough property held by the existing outer-PTY cases:** met (the `run --integration` and `run --e2e` entries green).
- **`verification-matrix.json#v1-33`:** `implemented` with its gated clearance (`.andromeda/runs/2026-10-19T18-33-04-implement/cleared-v1-33.md`). It is proven on the Linux dev host with the product binary (sha256 `231c0225…`) and claude 2.1.296 stamped 24/24:
  - the skill sent: `send-confirmed` @1458, 108 ms;
  - the turn waited: `wait` woken by `turn-ended` @5489;
  - `last` read twice, `result_class` ok;
  - the review dialog answered: `dialog-answered`, 27 345 ms;
  - `/clear` read back: `session-start{clear}` with a new id;
  - the takeover:
    - `wheel{human,human-input}` @26032;
    - the overseer's send exit 10 `human-typing` with no `send-issued` (@26181);
    - the founder's own `release` exit 0 → `wheel{driver,release}` @26337.

  Two acceptance words do not match the logs as written: "`dialog-answered` with `from`" (none was sent) and "`send-confirmed{confirmed:true}`" (on the diag line, not the events line). They are **routed to P7.3's post-claim premise escalation** (refine the acceptance with evidence, or the founder's ruling), never a silent flip.

**Gates** (implement's runs `.andromeda/runs/2026-10-19T10-56-42-implement` and `…T18-33-04-implement`; the operator pass in `evidence/operator-pass.md`):

| entry (`run`) | verdict |
|---|---|
| `cargo fmt --all --check` | green |
| `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` | green |
| `rg -n -U --pcre2 --type rust '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .` | green (exit 1, its `expect`) |
| `bash scripts/lint-probes.sh` | green |
| `bash scripts/agent-run.sh run --unit` | green |
| `… run --unit --filter 'test(/question_option_details_\|turn_ended_background_tasks_/)'` | green |
| `bash scripts/agent-run.sh run --integration` | green |
| `… run --integration --filter 'binary(state_events) \| binary(hook_events)'` | green |
| `bash scripts/agent-run.sh run --e2e` | green |
| the `git diff --quiet f23586fbda77 -- . ':!…'` preservation guard | green |
| `cargo deny check` | green |
| `cargo build --release --bin viola` | green, 8.38 s |
| `agent-run.sh cleanup` / `boot` / `status` / `cleanup` (`p-osv-smoke`, the smoke legs) | green ×4 |
| `bash scripts/g2-zero-panics.sh` | green |
| `bash scripts/agent-run.sh schema-check` | green |
| `bash scripts/agent-run.sh pre-push` | **red** on the first firing (`exit 0` ✗, exit 1: `llvm-cov` refused one truncated `.profraw`; 2958 / 2958 tests passed); **green** on the second (`19.2.log`, 88.5 s). The red stays open, its cause owed (`evidence/profraw-red.md`). |
| `gate.py hygiene` (operator) | `hygiene: clean` ×2 (`evidence/operator-pass.md`) |
| `git push origin HEAD` (operator) | green, `f23586fb..aa965127`, only the remote-tracking refs moved |
| `ci.py conclusion --sha HEAD --wait 1800` (operator) | `verdict: green · checks 15/15`, `ci#39305131020` on `aa965127d51e` |
| the `grep -c -E '"act":"(verify-stamped\|…\|clear-confirmed)"'` reader | green, last line 7 |
| the `grep -c 'stamped 2.1.296  24 pass  0 fail'` reader | green, last line 1 |
| the `grep -c '"act":"takeover"'` reader (`defer gated:`) | green, through `--void 25:block cleared — evidence/switch-readings.ndjson takeover line` |

Smoke: the boot path did not change; the smoke legs ran as P2 entries, and the live proof is Part C under the product.

**Watches:** `send_budget_paused…` case_4 has 3 green runs, all nextest (implement run 10-56-42: entry 5, entry 19, entry 19.2), and no plain `cargo test` run. Under the WATCH's own retire rule ("3 green runs") these are 3. Under the reading the last two wraps recorded ("only nextest ran" = non-advancing) it is the third non-advancing chunk. P5's Recurrence-watch row decides.

**Outcome basis:**
- The operator pass ran: `aa965127` (the pre-CI commit) on `f23586fb`, and `ci#39305131020` green 15/15 on it, with no fix commit (`git log --format='%h %s' f23586fb..HEAD`; `evidence/operator-pass.md`; `inputs#I19`).
- After the pass: step 14's takeover line, entry 25's void, and v1-33's ref with the clearance (`.andromeda/runs/2026-10-19T18-33-04-implement/`). They are uncommitted and ride this wrap.
- Implement's persisted outcomes, in dir-name order:
  - `.andromeda/runs/2026-10-19T10-56-42-implement/outcome-…md` (Parts A–B, steps 1–9; superseded for status by the later two).
  - `…T13-27-50-implement/outcome-…md` (steps 10–11).
  - `…T18-33-04-implement/outcome-…md` (steps 12–14, the operator pass; this window holds it whole).

**Process hygiene:**
- Implement's census showed none started by implement still running: the smoke session ended at entry 18 (`processes_gone`, `endpoint_gone`), and the verify and its children exited.
- Re-measured at this wrap with `gate.py census --comm viola-fake-agent --args projects/viola/target/{debug,release}/viola --args target/harness`: `census: none running`.
- Left running by others: the live viola-builder wrapper, its `viola mcp` server, and the overseer's `wait`. Those are the overseer's.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: f23586fb (the parent of the oldest pre-CI commit aa965127) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### crates/viola-agent-claude/src/dialog.rs — added 60 line(s) in 7 range(s)
added: 119-120 · 144 · 146-147 · 151-156 · 161 · 451-457 · 544-584
  - 151-156 «let option_details: Vec<Value> = raw»
  - 549-570 @550 «fn question_option_details_carry_description_and_preview() {»
### crates/viola-agent-claude/src/hook.rs — added 73 line(s) in 8 range(s)
added: 8 · 92 · 104-105 · 147-152 · 170-173 · 457-460 · 479-482 · 484-534
  - 493-494 «{"agent_type": "a", "description": "canary-chain-value-5c1e", "id": "t-1", "status": "running", "type": "…»
  - 496-508 @497 «fn turn_ended_background_tasks_counts_the_array(#[case] tasks: Value, #[case] count: u64) {»
### crates/viola-core/src/lib.rs — added 2 line(s) in 2 range(s)
added: 119 · 550
### crates/viola-state/tests/state_events.rs — added 39 line(s) in 3 range(s)
added: 71 · 103-105 · 265-299
- 265-298 @269 «fn state_events_background_tasks_and_option_details_are_known_keys() {»
  - 271-286 «let written = [»
  - 287-289 «for (kind, data) in &written {»
  - 293-296 «assert_eq!(»
### src/run/dialog.rs — added 2 line(s) in 1 range(s)
added: 625-626
### tests/hook_events.rs — added 83 line(s) in 3 range(s)
added: 209-212 · 238-241 · 252-326
  - 238-241 «assert_eq!(»
- 252-258 @253 «fn recorded(name: &str) -> Value {»
  - 254-256 «let path = workspace_path("fixtures/claude")»
- 260-286 @261 «fn recorded_line(event: &str, variant: &str, kind: &str) -> Value {»
  - 265-275 «let wrapper = Wrapper::boot(»
  - 276-278 «let lines = wait_events(&wrapper.instance_dir(), "the recorded hook's line", |l| {»
  - 279-282 «let line = lines»
- 288-305 @290 «fn hook_a_recorded_stop_carries_its_background_task_count() {»
  - 291-294 «let tasks = recorded("Stop.default.json")["background_tasks"]»
  - 298-304 «assert_eq!(»
- 307-325 @309 «fn hook_a_recorded_question_carries_its_option_details() {»
  - 316-324 «for (carried, asked) in carried.iter().zip(asked) {»
